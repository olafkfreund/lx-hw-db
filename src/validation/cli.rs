//! CLI integration for hardware report validation

use crate::validation::{HardwareReportValidator, ValidationConfig, ValidationResult};
use crate::hardware::{HardwareReport, PrivacyLevel};
use crate::LxHwError;
use clap::Args;
use std::path::{Path, PathBuf};
use std::fs;

/// CLI arguments for validation command
#[derive(Args, Debug)]
pub struct ValidateArgs {
    /// Path to hardware report file(s) to validate
    #[arg(value_name = "FILES")]
    pub files: Vec<PathBuf>,
    
    /// Enable strict validation mode
    #[arg(short, long)]
    pub strict: bool,
    
    /// Require specific privacy level (basic, enhanced, strict)
    #[arg(long, value_name = "LEVEL")]
    pub privacy_level: Option<String>,
    
    /// Require kernel support information
    #[arg(long)]
    pub require_kernel_support: bool,
    
    /// Minimum device count required
    #[arg(long, value_name = "COUNT")]
    pub minimum_devices: Option<u32>,
    
    /// Skip hardware compatibility validation
    #[arg(long)]
    pub skip_compatibility: bool,
    
    /// Output format (text, json, yaml)
    #[arg(short, long, default_value = "text")]
    pub format: String,
    
    /// Show only errors (suppress warnings and suggestions)
    #[arg(short, long)]
    pub quiet: bool,
    
    /// Show detailed validation information
    #[arg(short, long)]
    pub verbose: bool,
}

/// Execute validation command
pub async fn execute_validate(args: ValidateArgs) -> Result<(), LxHwError> {
    if args.files.is_empty() {
        return Err(LxHwError::InvalidInput { 
            message: "No input files specified".to_string() 
        });
    }
    
    // Parse privacy level if specified
    let privacy_level = args.privacy_level
        .as_deref()
        .map(parse_privacy_level)
        .transpose()?;
    
    // Configure validator
    let config = ValidationConfig {
        strict_mode: args.strict,
        privacy_level_required: privacy_level,
        require_kernel_support: args.require_kernel_support,
        minimum_device_count: args.minimum_devices,
        validate_hardware_compatibility: !args.skip_compatibility,
    };
    
    let validator = HardwareReportValidator::with_config(config);
    
    // Validate each file
    let mut total_files = 0;
    let mut valid_files = 0;
    let mut total_errors = 0;
    let mut total_warnings = 0;
    
    for file_path in &args.files {
        total_files += 1;
        
        if args.verbose {
            println!("Validating: {}", file_path.display());
        }
        
        match validate_single_file(&validator, file_path, &args).await {
            Ok(result) => {
                if result.valid {
                    valid_files += 1;
                }
                total_errors += result.errors.len();
                total_warnings += result.warnings.len();
            }
            Err(e) => {
                eprintln!("Error processing {}: {}", file_path.display(), e);
                total_errors += 1;
            }
        }
    }
    
    // Print summary
    if !args.quiet {
        println!("\n--- Validation Summary ---");
        println!("Files processed: {}", total_files);
        println!("Valid files: {}", valid_files);
        println!("Invalid files: {}", total_files - valid_files);
        println!("Total errors: {}", total_errors);
        println!("Total warnings: {}", total_warnings);
        
        if valid_files == total_files {
            println!("All files passed validation!");
        } else {
            println!("Some files failed validation");
        }
    }
    
    // Exit with error code if any files failed validation
    if valid_files != total_files {
        std::process::exit(1);
    }
    
    Ok(())
}

/// Validate a single file
async fn validate_single_file(
    validator: &HardwareReportValidator,
    file_path: &PathBuf,
    args: &ValidateArgs,
) -> Result<ValidationResult, LxHwError> {
    // Read and parse the report file
    let content = fs::read_to_string(file_path)
        .map_err(|e| LxHwError::SystemError {
            message: format!("Failed to read {}: {}", file_path.display(), e)
        })?;
    
    let report: HardwareReport = parse_report_file(&content, file_path)?;
    
    // Validate the report
    let result = validator.validate(&report);
    
    // Output results
    match args.format.as_str() {
        "json" => print_json_results(file_path, &result, args)?,
        "yaml" => print_yaml_results(file_path, &result, args)?,
        _ => print_text_results(file_path, &result, args)?,
    }
    
    Ok(result)
}

/// Print validation results in text format
fn print_text_results(
    file_path: &Path,
    result: &ValidationResult,
    args: &ValidateArgs,
) -> Result<(), LxHwError> {
    let status = if result.valid { "VALID" } else { "INVALID" };
    println!("{} {} (confidence: {:.1}%)", status, file_path.display(), result.confidence_score * 100.0);
    
    // Print errors
    if !result.errors.is_empty() {
        println!("  Errors:");
        for error in &result.errors {
            println!("    ERROR: {}", error);
        }
    }
    
    // Print warnings (unless quiet mode)
    if !result.warnings.is_empty() && !args.quiet {
        println!("  Warnings:");
        for warning in &result.warnings {
            println!("    WARNING: {}", warning);
        }
    }
    
    // Print suggestions (only in verbose mode)
    if !result.suggestions.is_empty() && args.verbose {
        println!("  Suggestions:");
        for suggestion in &result.suggestions {
            println!("    💡 {}", suggestion);
        }
    }
    
    if args.verbose || !result.errors.is_empty() {
        println!(); // Empty line for separation
    }
    
    Ok(())
}

/// Print validation results in JSON format
fn print_json_results(
    file_path: &Path,
    result: &ValidationResult,
    _args: &ValidateArgs,
) -> Result<(), LxHwError> {
    #[derive(serde::Serialize)]
    struct JsonOutput {
        file: String,
        result: ValidationResult,
    }
    
    let output = JsonOutput {
        file: file_path.display().to_string(),
        result: result.clone(),
    };
    
    let json_output = serde_json::to_string_pretty(&output)
        .map_err(|e| LxHwError::SystemError {
            message: format!("Failed to serialize JSON output: {}", e),
        })?;
    println!("{}", json_output);
    Ok(())
}

/// Print validation results in YAML format
fn print_yaml_results(
    file_path: &Path,
    result: &ValidationResult,
    _args: &ValidateArgs,
) -> Result<(), LxHwError> {
    #[derive(serde::Serialize)]
    struct YamlOutput {
        file: String,
        result: ValidationResult,
    }
    
    let output = YamlOutput {
        file: file_path.display().to_string(),
        result: result.clone(),
    };
    
    let yaml_output = serde_yaml::to_string(&output)
        .map_err(|e| LxHwError::SystemError {
            message: format!("Failed to serialize YAML output: {}", e),
        })?;
    println!("{}", yaml_output);
    Ok(())
}

/// Parse report file content based on extension or content detection
fn parse_report_file(content: &str, file_path: &Path) -> Result<HardwareReport, LxHwError> {
    match file_path.extension().and_then(|ext| ext.to_str()) {
        Some("json") => serde_json::from_str(content)
            .map_err(|e| LxHwError::InvalidInput {
                message: format!("Invalid JSON in {}: {}", file_path.display(), e)
            }),
        Some("yaml") | Some("yml") => serde_yaml::from_str(content)
            .map_err(|e| LxHwError::InvalidInput {
                message: format!("Invalid YAML in {}: {}", file_path.display(), e)
            }),
        _ => {
            // Try JSON first, then YAML
            serde_json::from_str(content).or_else(|_| {
                serde_yaml::from_str(content).map_err(|e| LxHwError::InvalidInput {
                    message: format!("Could not parse {} as JSON or YAML: {}", file_path.display(), e)
                })
            })
        }
    }
}

/// Parse privacy level string
fn parse_privacy_level(level_str: &str) -> Result<PrivacyLevel, LxHwError> {
    match level_str.to_lowercase().as_str() {
        "basic" => Ok(PrivacyLevel::Basic),
        "enhanced" => Ok(PrivacyLevel::Enhanced),
        "strict" => Ok(PrivacyLevel::Strict),
        _ => Err(LxHwError::InvalidInput {
            message: format!("Invalid privacy level '{}'. Must be: basic, enhanced, or strict", level_str),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::{ReportMetadata, SystemInfo};
    use chrono::Utc;
    use tempfile::NamedTempFile;
    use std::io::Write;
    
    fn create_test_report() -> HardwareReport {
        HardwareReport {
            metadata: ReportMetadata {
                version: "1.0.0".to_string(),
                generated_at: Utc::now(),
                privacy_level: PrivacyLevel::Basic,
                tools_used: vec!["lshw".to_string()],
                anonymized_system_id: "test_id_123456".to_string(),
            },
            system: SystemInfo {
                anonymized_hostname: "test_host_456789".to_string(),
                kernel_version: "6.16.0".to_string(),
                distribution: Some("NixOS 25.11".to_string()),
                architecture: "x86_64".to_string(),
                boot_time: Some(Utc::now()),
            },
            cpu: None,
            memory: None,
            storage: Vec::new(),
            graphics: Vec::new(),
            network: Vec::new(),
            usb: Vec::new(),
            audio: Vec::new(),
            kernel_support: None,
        }
    }
    
    #[tokio::test]
    async fn test_validate_json_file() {
        let report = create_test_report();
        let json_content = serde_json::to_string_pretty(&report).unwrap();
        
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", json_content).unwrap();
        
        let args = ValidateArgs {
            files: vec![temp_file.path().to_path_buf()],
            strict: false,
            privacy_level: None,
            require_kernel_support: false,
            minimum_devices: None,
            skip_compatibility: false,
            format: "text".to_string(),
            quiet: true,
            verbose: false,
        };
        
        let validator = HardwareReportValidator::new();
        let result = validate_single_file(&validator, &args.files[0], &args).await;
        
        assert!(result.is_ok());
        assert!(result.unwrap().valid);
    }
    
    #[test]
    fn test_parse_privacy_level() {
        assert_eq!(parse_privacy_level("basic").unwrap(), PrivacyLevel::Basic);
        assert_eq!(parse_privacy_level("Enhanced").unwrap(), PrivacyLevel::Enhanced);
        assert_eq!(parse_privacy_level("STRICT").unwrap(), PrivacyLevel::Strict);
        
        assert!(parse_privacy_level("invalid").is_err());
    }
}