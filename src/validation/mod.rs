//! Hardware report validation system
//!
//! This module provides comprehensive validation for hardware reports including:
//! - JSON schema validation against predefined schemas
//! - Business logic validation for realistic hardware configurations
//! - Privacy anonymization verification to ensure PII protection
//! - Data consistency checks across different hardware components
//! - Hardware compatibility validation against kernel support data
//!
//! # Examples
//!
//! Basic usage:
//! ```rust
//! use lx_hw_db::validation::{validate_report, ValidationConfig, HardwareReportValidator};
//! use lx_hw_db::hardware::HardwareReport;
//!
//! // Quick validation with defaults
//! let result = validate_report(&report);
//! if result.valid {
//!     println!("Report is valid with {:.1}% confidence", result.confidence_score * 100.0);
//! }
//!
//! // Custom validation with configuration
//! let config = ValidationConfig {
//!     strict_mode: true,
//!     minimum_device_count: Some(5),
//!     ..ValidationConfig::default()
//! };
//! let validator = HardwareReportValidator::with_config(config);
//! let result = validator.validate(&report);
//! ```

use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::hardware::{HardwareReport, PrivacyLevel};

pub mod schema;
pub mod business_logic;
pub mod privacy;
pub mod consistency;
pub mod cli;
pub mod constants;

/// Validation errors that can occur during report validation
#[derive(Debug, Clone, Error, Serialize, Deserialize, PartialEq, Eq)]
#[non_exhaustive]
pub enum ValidationError {
    #[error("Schema validation failed: {message}")]
    SchemaError { message: String },
    
    #[error("Business logic validation failed: {field} - {message}")]
    BusinessLogicError { field: String, message: String },
    
    #[error("Privacy validation failed: {field} - {message}")]
    PrivacyError { field: String, message: String },
    
    #[error("Data consistency validation failed: {field} - {message}")]
    ConsistencyError { field: String, message: String },
    
    #[error("Hardware compatibility validation failed: {device} - {message}")]
    CompatibilityError { device: String, message: String },
}

/// Validation result with confidence score
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValidationResult {
    pub valid: bool,
    pub confidence_score: f64, // 0.0 to 1.0
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<String>,
    pub suggestions: Vec<String>,
}

/// Validation configuration with builder pattern support
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationConfig {
    pub strict_mode: bool,
    pub privacy_level_required: Option<PrivacyLevel>,
    pub require_kernel_support: bool,
    pub minimum_device_count: Option<u32>,
    pub validate_hardware_compatibility: bool,
}

/// Confidence score impact constants for different validation failures
mod confidence_impact {
    pub const SCHEMA_ERROR: f64 = 0.5;
    pub const BUSINESS_LOGIC_ERROR: f64 = 0.8;
    pub const PRIVACY_ERROR: f64 = 0.7;
    pub const CONSISTENCY_ERROR: f64 = 0.9;
    pub const COMPATIBILITY_ERROR: f64 = 0.85;
    pub const WARNING_PENALTY_PER_WARNING: f64 = 0.05;
}

/// Main hardware report validator
#[derive(Debug, Clone)]
pub struct HardwareReportValidator {
    config: ValidationConfig,
}

impl HardwareReportValidator {
    /// Create a new validator with default configuration
    pub fn new() -> Self {
        Self {
            config: ValidationConfig::default(),
        }
    }
    
    /// Create a new validator with custom configuration
    pub fn with_config(config: ValidationConfig) -> Self {
        Self { config }
    }
    
    /// Validate a hardware report comprehensively
    pub fn validate(&self, report: &HardwareReport) -> ValidationResult {
        let mut result = ValidationResultBuilder::new();
        
        // 1. JSON Schema Validation
        self.validate_schema(report)
            .unwrap_or_else(|e| result.add_error(e, confidence_impact::SCHEMA_ERROR));
        
        // 2. Business Logic Validation
        self.validate_business_logic(report)
            .map(|warnings| result.add_warnings(warnings))
            .unwrap_or_else(|e| result.add_error(e, confidence_impact::BUSINESS_LOGIC_ERROR));
        
        // 3. Privacy Validation
        self.validate_privacy(report)
            .unwrap_or_else(|e| result.add_error(e, confidence_impact::PRIVACY_ERROR));
        
        // 4. Data Consistency Validation
        self.validate_consistency(report)
            .map(|warnings| result.add_warnings(warnings))
            .unwrap_or_else(|e| result.add_error(e, confidence_impact::CONSISTENCY_ERROR));
        
        // 5. Hardware Compatibility Validation
        if self.config.validate_hardware_compatibility {
            self.validate_hardware_compatibility(report)
                .map(|suggestions| result.add_suggestions(suggestions))
                .unwrap_or_else(|e| result.add_error(e, confidence_impact::COMPATIBILITY_ERROR));
        }
        
        result.build()
    }
    
    /// Validate report against JSON schema
    fn validate_schema(&self, report: &HardwareReport) -> Result<(), ValidationError> {
        schema::validate_report_schema(report)
    }
    
    /// Validate business logic constraints
    fn validate_business_logic(&self, report: &HardwareReport) -> Result<Vec<String>, ValidationError> {
        business_logic::validate_business_rules(report, &self.config)
    }
    
    /// Validate privacy and anonymization
    fn validate_privacy(&self, report: &HardwareReport) -> Result<(), ValidationError> {
        privacy::validate_privacy_compliance(report, &self.config)
    }
    
    /// Validate data consistency
    fn validate_consistency(&self, report: &HardwareReport) -> Result<Vec<String>, ValidationError> {
        consistency::validate_data_consistency(report)
    }
    
    /// Validate hardware compatibility information
    fn validate_hardware_compatibility(&self, report: &HardwareReport) -> Result<Vec<String>, ValidationError> {
        // This would validate against known hardware compatibility database
        // For now, return basic suggestions
        let mut suggestions = Vec::new();
        
        if let Some(kernel_support) = &report.kernel_support {
            if kernel_support.unsupported_devices > 0 {
                suggestions.push(format!(
                    "Consider upgrading kernel to improve support for {} unsupported devices",
                    kernel_support.unsupported_devices
                ));
            }
            
            if kernel_support.experimental_devices > 0 {
                suggestions.push(format!(
                    "{} devices have experimental support - monitor for stability issues",
                    kernel_support.experimental_devices
                ));
            }
        }
        
        Ok(suggestions)
    }
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            strict_mode: false,
            privacy_level_required: None,
            require_kernel_support: false,
            minimum_device_count: None,
            validate_hardware_compatibility: true,
        }
    }
}

impl Default for HardwareReportValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper for building validation results
#[derive(Debug)]
struct ValidationResultBuilder {
    errors: Vec<ValidationError>,
    warnings: Vec<String>,
    suggestions: Vec<String>,
    confidence_score: f64,
}

impl ValidationResultBuilder {
    fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
            confidence_score: 1.0,
        }
    }
    
    fn add_error(&mut self, error: ValidationError, impact: f64) {
        self.errors.push(error);
        self.confidence_score *= impact;
    }
    
    fn add_warnings<I>(&mut self, warnings: I)
    where
        I: IntoIterator<Item = String>,
    {
        let new_warnings: Vec<_> = warnings.into_iter().collect();
        let warning_impact = 1.0 - (new_warnings.len() as f64 * confidence_impact::WARNING_PENALTY_PER_WARNING);
        self.confidence_score *= warning_impact;
        self.warnings.extend(new_warnings);
    }
    
    fn add_suggestions<I>(&mut self, suggestions: I)
    where
        I: IntoIterator<Item = String>,
    {
        self.suggestions.extend(suggestions);
    }
    
    fn build(self) -> ValidationResult {
        ValidationResult {
            valid: self.errors.is_empty(),
            confidence_score: self.confidence_score.clamp(0.0, 1.0),
            errors: self.errors,
            warnings: self.warnings,
            suggestions: self.suggestions,
        }
    }
}

/// Utility function to validate multiple reports
pub fn validate_reports(
    reports: &[HardwareReport], 
    config: Option<ValidationConfig>
) -> Vec<ValidationResult> {
    let validator = config
        .map(HardwareReportValidator::with_config)
        .unwrap_or_default();
    
    reports
        .iter()
        .map(|report| validator.validate(report))
        .collect()
}

/// Quick validation function for single reports
pub fn validate_report(report: &HardwareReport) -> ValidationResult {
    let validator = HardwareReportValidator::new();
    validator.validate(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::{ReportMetadata, SystemInfo};
    use chrono::Utc;
    
    fn create_minimal_report() -> HardwareReport {
        HardwareReport {
            metadata: ReportMetadata {
                version: "1.0.0".to_string(),
                generated_at: Utc::now(),
                privacy_level: PrivacyLevel::Basic,
                tools_used: vec!["lshw".to_string()],
                anonymized_system_id: "test_id_123".to_string(),
            },
            system: SystemInfo {
                anonymized_hostname: "test_host_456".to_string(),
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
    
    #[test]
    fn test_minimal_report_validation() {
        let report = create_minimal_report();
        let result = validate_report(&report);
        
        // Debug what's failing
        if !result.valid {
            eprintln!("Validation failed with errors: {:?}", result.errors);
            eprintln!("Warnings: {:?}", result.warnings);
        }
        
        assert!(result.valid, "Validation failed: {:?}", result.errors);
        assert!(result.confidence_score > 0.0);
    }
    
    #[test]
    fn test_validator_configuration() {
        let config = ValidationConfig {
            strict_mode: true,
            privacy_level_required: Some(PrivacyLevel::Enhanced),
            require_kernel_support: true,
            minimum_device_count: Some(5),
            validate_hardware_compatibility: true,
        };
        
        let validator = HardwareReportValidator::with_config(config);
        let report = create_minimal_report();
        let result = validator.validate(&report);
        
        // Should have validation errors due to strict requirements
        assert!(!result.errors.is_empty());
    }
}