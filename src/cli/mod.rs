//! Command-line interface for the hardware detection tool

use crate::errors::{Result, LxHwError};
use crate::hardware::PrivacyLevel;
use crate::output::OutputFormat;
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Linux Hardware Detection CLI Tool  
#[derive(Parser, Debug)]
#[command(
    name = "lx-hw-detect",
    version,
    about = "Privacy-preserving Linux hardware detection and compatibility reporting",
    long_about = "A privacy-first hardware detection tool that collects system information using multiple Linux utilities while implementing comprehensive anonymization and privacy protection."
)]
pub struct Cli {
    /// Global options
    #[command(flatten)]
    pub global: GlobalOptions,

    /// Commands
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub struct GlobalOptions {
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress all output except errors
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,

    /// Privacy level for data collection
    #[arg(
        short = 'p',
        long,
        global = true,
        value_enum,
        default_value_t = PrivacyLevel::Basic
    )]
    pub privacy: PrivacyLevel,

    /// Configuration file path
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Detect hardware and generate compatibility report
    Detect {
        /// Output format
        #[arg(short = 'f', long, value_enum, default_value_t = OutputFormat::Markdown)]
        format: OutputFormat,

        /// Output file path (default: stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Specific tools to run (default: all available)
        #[arg(short, long, value_delimiter = ',')]
        tools: Option<Vec<String>>,

        /// Timeout for each detection tool in seconds
        #[arg(long, default_value_t = 30)]
        timeout: u64,

        /// Skip privacy anonymization (for debugging)
        #[arg(long)]
        no_anonymize: bool,
    },

    /// Check which detection tools are available
    Check {
        /// Show detailed information about each tool
        #[arg(short, long)]
        detailed: bool,
    },

    /// Validate a hardware report file
    Validate {
        /// Path to the report file to validate
        file: PathBuf,

        /// Check schema compliance only
        #[arg(short, long)]
        schema_only: bool,
    },

    /// Analyze kernel support and provide upgrade recommendations
    Analyze {
        /// Analyze only specific device (vendor:device format)
        #[arg(long)]
        device: Option<String>,

        /// Include detailed kernel source analysis
        #[arg(long)]
        kernel_source: bool,

        /// Path to local Linux kernel repository for faster analysis
        #[arg(long)]
        kernel_repo: Option<PathBuf>,

        /// Show upgrade recommendations
        #[arg(long)]
        recommendations: bool,
    },

    /// Generate configuration templates
    Config {
        /// Generate default configuration file
        #[command(subcommand)]
        command: ConfigCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Generate default configuration file
    Init {
        /// Output path for configuration file
        #[arg(short, long, default_value = "lx-hw-detect.toml")]
        output: PathBuf,

        /// Overwrite existing configuration file
        #[arg(short, long)]
        force: bool,
    },

    /// Show current configuration
    Show,
}

/// CLI implementation for handling command execution
pub struct CliHandler;

impl CliHandler {
    /// Create a new CLI handler
    pub fn new() -> Self {
        Self
    }

    /// Run the CLI with the given arguments
    pub async fn run(&self, cli: Cli) -> Result<()> {
        // Initialize logging based on verbosity
        self.init_logging(cli.global.verbose, cli.global.quiet)?;

        // Load configuration
        let _config = self.load_config(cli.global.config.as_ref())?;

        // Execute the command
        match cli.command {
            Commands::Detect {
                format,
                output,
                tools,
                timeout,
                no_anonymize,
            } => {
                self.handle_detect(
                    cli.global.privacy,
                    format,
                    output,
                    tools,
                    timeout,
                    no_anonymize,
                )
                .await
            }
            Commands::Check { detailed } => self.handle_check(detailed).await,
            Commands::Validate { file, schema_only } => {
                self.handle_validate(file, schema_only).await
            }
            Commands::Analyze { device, kernel_source, kernel_repo, recommendations } => {
                self.handle_analyze(device, kernel_source, kernel_repo, recommendations).await
            }
            Commands::Config { command } => self.handle_config(command).await,
        }
    }

    /// Initialize logging based on verbosity settings
    fn init_logging(&self, verbose: bool, quiet: bool) -> Result<()> {
        let log_level = if quiet {
            "error"
        } else if verbose {
            "debug"
        } else {
            "info"
        };

        std::env::set_var("RUST_LOG", format!("lx_hw_detect={}", log_level));
        env_logger::init();
        Ok(())
    }

    /// Load configuration from file
    fn load_config(&self, _config_path: Option<&PathBuf>) -> Result<AppConfig> {
        // Placeholder implementation
        Ok(AppConfig::default())
    }

    /// Handle the detect command
    async fn handle_detect(
        &self,
        privacy: PrivacyLevel,
        format: OutputFormat,
        output: Option<PathBuf>,
        _tools: Option<Vec<String>>, // TODO: Use to filter specific tools
        _timeout: u64, // TODO: Apply timeout to individual detectors
        no_anonymize: bool,
    ) -> Result<()> {
        use crate::detectors::integration::HardwareAnalyzer;
        use crate::output::OutputRenderer;
        
        log::info!("Starting hardware detection and analysis...");
        println!("Detecting hardware and analyzing kernel compatibility...\n");
        
        // Create hardware analyzer with privacy settings
        let mut analyzer = HardwareAnalyzer::new(privacy)?;
        
        // Run complete analysis
        let report = analyzer.analyze_system().await?;
        
        // Render output
        let renderer = OutputRenderer::new(format);
        let output_content = if no_anonymize {
            log::warn!("Privacy anonymization disabled - report contains identifying information");
            renderer.render_debug(&report)?
        } else {
            renderer.render(&report)?
        };
        
        // Write to file or stdout
        match output {
            Some(path) => {
                std::fs::write(&path, output_content)
                    .map_err(LxHwError::IoError)?;
                println!("Hardware report saved to: {:?}", path);
            }
            None => {
                println!("{}", output_content);
            }
        }
        
        Ok(())
    }

    /// Handle the check command
    async fn handle_check(&self, detailed: bool) -> Result<()> {
        use crate::detectors::DetectorRegistry;
        
        log::info!("Checking hardware detection tool availability...");
        println!("Checking availability of hardware detection tools...\n");
        
        let registry = DetectorRegistry::new();
        let detectors = registry.list_detectors();
        
        let mut available_count = 0;
        let total_count = detectors.len();
        
        for detector in detectors {
            let is_available = detector.is_available().await;
            let status = if is_available { "✓ Available" } else { "✗ Not found" };
            
            println!("{:<12} {}", detector.name(), status);
            
            if detailed && is_available {
                println!("    Timeout: {:?}", detector.timeout());
                println!();
            }
            
            if is_available {
                available_count += 1;
            }
        }
        
        println!("\nSummary: {}/{} detection tools available", available_count, total_count);
        
        if available_count == 0 {
            println!("Warning: No hardware detection tools found. Install lshw, dmidecode, lspci, lsusb, or inxi for hardware detection.");
        } else if available_count < total_count {
            println!("Note: Install missing tools for more comprehensive hardware detection.");
        }
        
        Ok(())
    }

    /// Handle the validate command
    async fn handle_validate(&self, _file: PathBuf, _schema_only: bool) -> Result<()> {
        // Placeholder implementation
        log::info!("Report validation will be implemented in Phase 2");
        println!("Report validation feature is not yet implemented.");
        Ok(())
    }

    /// Handle the analyze command
    async fn handle_analyze(
        &self, 
        device: Option<String>, 
        kernel_source: bool, 
        kernel_repo: Option<PathBuf>,
        recommendations: bool
    ) -> Result<()> {
        use crate::detectors::kernel::KernelSupportVerifier;
        use crate::detectors::kernel_source::KernelSourceAnalyzer;
        
        println!("Analyzing kernel hardware support...\n");

        // Initialize kernel support verifier
        let verifier = match KernelSupportVerifier::new() {
            Ok(v) => v,
            Err(e) => {
                println!("Warning: Could not initialize kernel verifier: {}", e);
                println!("Some features may be limited.\n");
                return Ok(());
            }
        };

        // Get device IDs to analyze
        let device_ids = if let Some(device_filter) = device {
            if let Some((vendor, device)) = device_filter.split_once(':') {
                vec![(vendor.to_string(), device.to_string())]
            } else {
                println!("Error: Device format should be vendor:device (e.g., '8086:1234')");
                return Ok(());
            }
        } else {
            println!("Scanning system for PCI devices...");
            verifier.extract_system_device_ids().unwrap_or_else(|e| {
                println!("Warning: Could not scan system devices: {}", e);
                Vec::new()
            })
        };

        if device_ids.is_empty() {
            println!("No devices found to analyze.");
            return Ok(());
        }

        println!("Found {} device(s) to analyze\n", device_ids.len());

        // Analyze kernel support
        let support_data = verifier.get_support_data(device_ids)?;
        let user_recommendations = verifier.generate_user_recommendations(&support_data);

        // Display results
        self.display_kernel_analysis(&support_data, &user_recommendations);

        // Perform kernel source analysis if requested
        if kernel_source {
            println!("\n=== KERNEL SOURCE ANALYSIS ===\n");
            let mut source_analyzer = if let Some(repo_path) = kernel_repo {
                KernelSourceAnalyzer::new().with_local_repo(repo_path.to_string_lossy().to_string())
            } else {
                KernelSourceAnalyzer::new()
            };

            for device_support in &support_data.supported_devices {
                if device_support.support_level == crate::detectors::kernel::SupportLevel::Unsupported {
                    println!("Searching kernel source for device {}...", device_support.device_id);
                    match source_analyzer.search_device_support(&device_support.device_id).await {
                        Ok(source_info) => {
                            if !source_info.is_empty() {
                                for info in source_info {
                                    println!("  Found in: {}", info.driver_path);
                                    println!("  Driver: {}", info.driver_name);
                                    if info.experimental {
                                        println!("  Status: EXPERIMENTAL");
                                    }
                                }
                            } else {
                                println!("  No kernel source support found");
                            }
                        }
                        Err(e) => {
                            println!("  Error searching source: {}", e);
                        }
                    }
                    println!();
                }
            }
        }

        // Show upgrade recommendations if requested
        if recommendations {
            self.display_upgrade_recommendations(&user_recommendations);
        }

        Ok(())
    }

    /// Display kernel analysis results
    fn display_kernel_analysis(
        &self, 
        support_data: &crate::detectors::kernel::KernelSupportData, 
        recommendations: &crate::detectors::kernel::UserRecommendations
    ) {
        println!("=== KERNEL SUPPORT ANALYSIS ===");
        println!("Kernel Version: {}", support_data.kernel_version);
        println!("Total Devices: {}", support_data.supported_devices.len());
        println!("Supported: {}", recommendations.supported_devices);
        println!("Unsupported: {}", recommendations.unsupported_devices);
        
        if recommendations.unsupported_devices > 0 {
            println!("\nUnsupported Devices:");
            for device in &support_data.supported_devices {
                if device.support_level == crate::detectors::kernel::SupportLevel::Unsupported {
                    println!("  {} - No driver found", device.device_id);
                }
            }
        }

        if !recommendations.module_actions.is_empty() {
            println!("\nModule Actions Needed:");
            for action in &recommendations.module_actions {
                println!("  {} (Risk: {:?})", action.description, action.risk_level);
                for cmd in &action.commands {
                    println!("    {}", cmd);
                }
            }
        }

        if !recommendations.general_advice.is_empty() {
            println!("\nGeneral Advice:");
            for advice in &recommendations.general_advice {
                println!("  - {}", advice);
            }
        }
    }

    /// Display upgrade recommendations
    fn display_upgrade_recommendations(&self, recommendations: &crate::detectors::kernel::UserRecommendations) {
        if recommendations.needs_kernel_upgrade {
            println!("\n=== UPGRADE RECOMMENDATIONS ===");
            
            for upgrade in &recommendations.kernel_upgrades {
                println!("Device: {}", upgrade.device_id);
                println!("Current Kernel: {}", upgrade.current_kernel);
                println!("Recommended: {}", upgrade.recommended_kernel);
                println!("Reason: {}", upgrade.reason);
                println!("Success Probability: {}%", upgrade.estimated_support_probability);
                println!("\nUpgrade Commands:");
                for cmd in &upgrade.upgrade_method {
                    println!("  {}", cmd);
                }
                println!();
            }
        } else {
            println!("\nNo kernel upgrades needed for better hardware support.");
        }
    }

    /// Handle the config command
    async fn handle_config(&self, command: ConfigCommands) -> Result<()> {
        match command {
            ConfigCommands::Init { output, force } => {
                log::info!("Generating configuration file at: {:?}", output);
                if output.exists() && !force {
                    return Err(LxHwError::ConfigError(format!(
                        "Configuration file already exists: {:?}. Use --force to overwrite.",
                        output
                    )));
                }

                let config = AppConfig::default();
                let config_toml = toml::to_string_pretty(&config)
                    .map_err(|e| LxHwError::SerializationError(e.to_string()))?;

                std::fs::write(&output, config_toml)
                    .map_err(|e| LxHwError::IoError(e))?;

                println!("Configuration file generated: {:?}", output);
                Ok(())
            }
            ConfigCommands::Show => {
                log::info!("Showing current configuration");
                let config = AppConfig::default();
                println!("{}", toml::to_string_pretty(&config).unwrap());
                Ok(())
            }
        }
    }
}

/// Application configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppConfig {
    /// Default privacy level
    pub privacy_level: PrivacyLevel,
    /// Default output format  
    pub output_format: String,
    /// Tool-specific settings
    pub tools: ToolConfig,
    /// Privacy settings
    pub privacy: PrivacyConfig,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolConfig {
    /// Default timeout for tools in seconds
    pub timeout: u64,
    /// Specific tool configurations
    pub lshw: ToolSettings,
    pub dmidecode: ToolSettings,
    pub lspci: ToolSettings,
    pub lsusb: ToolSettings,
    pub inxi: ToolSettings,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolSettings {
    /// Whether this tool is enabled
    pub enabled: bool,
    /// Custom timeout for this tool
    pub timeout: Option<u64>,
    /// Additional command-line arguments
    pub extra_args: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]  
pub struct PrivacyConfig {
    /// Custom salt rotation period in hours
    pub salt_rotation_hours: Option<u64>,
    /// Whether to preserve hardware vendor information
    pub preserve_vendor: bool,
    /// Whether to preserve device model information
    pub preserve_model: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            privacy_level: PrivacyLevel::Basic,
            output_format: "markdown".to_string(),
            tools: ToolConfig::default(),
            privacy: PrivacyConfig::default(),
        }
    }
}

impl Default for ToolConfig {
    fn default() -> Self {
        Self {
            timeout: 30,
            lshw: ToolSettings::default(),
            dmidecode: ToolSettings::default(),
            lspci: ToolSettings::default(),
            lsusb: ToolSettings::default(),
            inxi: ToolSettings::default(),
        }
    }
}

impl Default for ToolSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: None,
            extra_args: Vec::new(),
        }
    }
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            salt_rotation_hours: None,
            preserve_vendor: true,
            preserve_model: true,
        }
    }
}

// Implement ValueEnum for PrivacyLevel to work with clap
impl ValueEnum for PrivacyLevel {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Basic, Self::Enhanced, Self::Strict]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Basic => clap::builder::PossibleValue::new("basic")
                .help("Basic privacy with 24-hour salt rotation"),
            Self::Enhanced => clap::builder::PossibleValue::new("enhanced") 
                .help("Enhanced privacy with 12-hour salt rotation"),
            Self::Strict => clap::builder::PossibleValue::new("strict")
                .help("Strict privacy with 1-hour salt rotation"),
        })
    }
}

// Implement ValueEnum for OutputFormat to work with clap  
impl ValueEnum for OutputFormat {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Yaml, Self::Json, Self::Markdown]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Yaml => clap::builder::PossibleValue::new("yaml")
                .help("YAML format output"),
            Self::Json => clap::builder::PossibleValue::new("json")
                .help("JSON format output"),
            Self::Markdown => clap::builder::PossibleValue::new("markdown")
                .help("Markdown with YAML frontmatter"),
        })
    }
}