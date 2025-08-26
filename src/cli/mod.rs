//! Command-line interface for the hardware detection tool

use crate::errors::{LxHwError, Result};
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
        /// Available tools: lshw, dmidecode, lspci, lsusb, inxi
        /// Example: --tools lshw,lspci
        #[arg(short, long, value_delimiter = ',')]
        tools: Option<Vec<String>>,

        /// Timeout for each detection tool in seconds
        /// Individual detectors will be terminated if they exceed this time limit
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

    /// Validate hardware report files
    Validate(crate::validation::cli::ValidateArgs),

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

    /// Submit hardware report directly to GitHub
    Submit {
        /// GitHub username (will prompt if not provided)
        #[arg(long)]
        github_username: Option<String>,

        /// GitHub personal access token (will prompt if not provided)
        #[arg(long)]
        github_token: Option<String>,

        /// Hardware report file to submit (if not provided, will generate automatically)
        #[arg(short, long)]
        report: Option<PathBuf>,

        /// Brief description of the hardware setup
        #[arg(short, long)]
        description: Option<String>,

        /// Skip interactive confirmation prompts
        #[arg(short, long)]
        yes: bool,

        /// Fork repository if not already forked
        #[arg(long)]
        auto_fork: bool,

        /// Specific tools to use for detection (if generating report)
        #[arg(short, long, value_delimiter = ',')]
        tools: Option<Vec<String>>,

        /// Use draft pull request
        #[arg(long)]
        draft: bool,
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

impl Default for CliHandler {
    fn default() -> Self {
        Self::new()
    }
}

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
            Commands::Detect { format, output, tools, timeout, no_anonymize } => {
                self.handle_detect(cli.global.privacy, format, output, tools, timeout, no_anonymize)
                    .await
            }
            Commands::Check { detailed } => self.handle_check(detailed).await,
            Commands::Validate(validate_args) => {
                crate::validation::cli::execute_validate(validate_args).await?;
                Ok(())
            }
            Commands::Analyze { device, kernel_source, kernel_repo, recommendations } => {
                self.handle_analyze(device, kernel_source, kernel_repo, recommendations).await
            }
            Commands::Config { command } => self.handle_config(command).await,
            Commands::Submit {
                github_username,
                github_token,
                report,
                description,
                yes,
                auto_fork,
                tools,
                draft,
            } => {
                self.handle_submit(
                    github_username,
                    github_token,
                    report,
                    description,
                    yes,
                    auto_fork,
                    tools,
                    draft,
                    &cli.global,
                )
                .await
            }
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
    #[allow(clippy::too_many_arguments)]
    async fn handle_detect(
        &self,
        privacy: PrivacyLevel,
        format: OutputFormat,
        output: Option<PathBuf>,
        tools: Option<Vec<String>>,
        timeout: u64,
        no_anonymize: bool,
    ) -> Result<()> {
        use crate::detectors::integration::HardwareAnalyzer;
        use crate::output::OutputRenderer;
        use std::time::Duration;

        log::info!("Starting hardware detection and analysis...");
        println!("Detecting hardware and analyzing kernel compatibility...\n");

        // Create hardware analyzer with privacy settings and configure tools/timeout
        let mut analyzer = HardwareAnalyzer::new(privacy)?;
        
        // Configure tool filtering if specified
        if let Some(tool_names) = &tools {
            analyzer.set_enabled_tools(tool_names.clone())?;
            println!("Using only specified tools: {}", tool_names.join(", "));
        }
        
        // Configure timeout if specified
        analyzer.set_detection_timeout(Duration::from_secs(timeout));
        if timeout != 30 {
            println!("Using custom timeout: {}s per detector", timeout);
        }

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
                std::fs::write(&path, output_content).map_err(LxHwError::IoError)?;
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

    /// Handle the analyze command
    async fn handle_analyze(
        &self,
        device: Option<String>,
        kernel_source: bool,
        kernel_repo: Option<PathBuf>,
        recommendations: bool,
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
                if device_support.support_level
                    != crate::detectors::kernel::SupportLevel::Unsupported
                {
                    continue;
                }

                println!("Searching kernel source for device {}...", device_support.device_id);
                match source_analyzer.search_device_support(&device_support.device_id).await {
                    Ok(source_info) => {
                        self.display_kernel_source_info(source_info);
                    }
                    Err(e) => {
                        println!("  Error searching source: {}", e);
                    }
                }
                println!();
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
        recommendations: &crate::detectors::kernel::UserRecommendations,
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

    /// Display kernel source information
    fn display_kernel_source_info(
        &self,
        source_info: Vec<crate::detectors::kernel_source::HardwareSupportInfo>,
    ) {
        if source_info.is_empty() {
            println!("  No kernel source support found");
            return;
        }

        for info in source_info {
            println!("  Found in: {}", info.driver_path);
            println!("  Driver: {}", info.driver_name);
            if info.experimental {
                println!("  Status: EXPERIMENTAL");
            }
        }
    }

    /// Display upgrade recommendations
    fn display_upgrade_recommendations(
        &self,
        recommendations: &crate::detectors::kernel::UserRecommendations,
    ) {
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

                std::fs::write(&output, config_toml).map_err(LxHwError::IoError)?;

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

    /// Handle the submit command
    async fn handle_submit(
        &self,
        github_username: Option<String>,
        github_token: Option<String>,
        report: Option<PathBuf>,
        description: Option<String>,
        yes: bool,
        auto_fork: bool,
        tools: Option<Vec<String>>,
        _draft: bool,
        global: &GlobalOptions,
    ) -> Result<()> {
        use crate::github_submit::{setup_github_config, GitHubSubmitter, SubmissionInfo};
        use chrono::Utc;
        use std::fs;
        use tempfile::NamedTempFile;

        println!("🚀 Starting automated GitHub submission...\n");

        // Step 1: Setup GitHub configuration
        let mut github_config = setup_github_config(github_username, github_token)?;
        github_config.auto_fork = auto_fork;

        // Step 2: Generate or use existing report
        let report_path = if let Some(report_path) = report {
            if !report_path.exists() {
                return Err(LxHwError::Validation(format!(
                    "Report file not found: {}",
                    report_path.display()
                )));
            }
            report_path
        } else {
            println!("📊 No report file provided, generating hardware report...");

            // Create temporary file for the report
            let temp_file = NamedTempFile::new()
                .map_err(|e| LxHwError::Io(format!("Failed to create temporary file: {}", e)))?;

            // Generate report using the detect functionality
            let mut registry = crate::detectors::DetectorRegistry::new();

            // Configure detection tools if specified
            if let Some(tool_names) = tools {
                registry.set_enabled_tools(tool_names)?;
            }

            // Run detection
            let detection_results = registry.detect_all().await?;

            // Generate hardware report  
            let mut hardware_analyzer = crate::detectors::integration::HardwareAnalyzer::new(global.privacy)?;
            let hardware_report = hardware_analyzer.analyze_system().await?;

            // Write report to temporary file
            let report_json = serde_json::to_string_pretty(&hardware_report)
                .map_err(|e| LxHwError::SerializationError(e.to_string()))?;

            fs::write(temp_file.path(), report_json)
                .map_err(|e| LxHwError::Io(format!("Failed to write report: {}", e)))?;

            println!("✅ Hardware report generated successfully");
            temp_file.path().to_path_buf()
        };

        // Step 3: Get description if not provided
        let description = if let Some(desc) = description {
            desc
        } else {
            // Extract basic system info for default description
            let content = fs::read_to_string(&report_path)
                .map_err(|e| LxHwError::Io(format!("Failed to read report: {}", e)))?;

            let report: crate::hardware::HardwareReport = serde_json::from_str(&content)
                .map_err(|e| LxHwError::Validation(format!("Invalid report format: {}", e)))?;

            let cpu_info = report.cpu
                .as_ref()
                .map(|cpu| format!("{} {}", cpu.vendor, cpu.model))
                .unwrap_or_else(|| "Unknown CPU".to_string());

            let gpu_info = report.graphics
                .first()
                .map(|gpu| format!("{} {}", gpu.vendor, gpu.model))
                .unwrap_or_else(|| "Unknown GPU".to_string());

            format!("{} with {} on {} {}",
                cpu_info,
                gpu_info,
                report.system.distribution.as_ref().unwrap_or(&"Unknown".to_string()),
                report.system.kernel_version)
        };

        // Step 4: Create submission info
        let submission = SubmissionInfo {
            description,
            report_path: report_path.clone(),
            generated_at: Utc::now(),
            privacy_level: global.privacy,
            tools_used: Vec::new(), // Will be populated from report
        };

        // Step 5: Submit to GitHub
        let mut submitter = GitHubSubmitter::new(github_config);
        let pr_url = submitter.submit_report(submission, yes).await?;

        println!("\n🎉 Submission completed successfully!");
        println!("📋 Pull Request: {}", pr_url);
        println!("\nNext steps:");
        println!("1. Your submission will be automatically validated");
        println!("2. Community members will review your hardware report");
        println!("3. Once approved, it will be merged into the database");
        println!("\nThank you for contributing to the Linux Hardware Database! 🐧");

        Ok(())
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
        Self { enabled: true, timeout: None, extra_args: Vec::new() }
    }
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self { salt_rotation_hours: None, preserve_vendor: true, preserve_model: true }
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
            Self::Yaml => clap::builder::PossibleValue::new("yaml").help("YAML format output"),
            Self::Json => clap::builder::PossibleValue::new("json").help("JSON format output"),
            Self::Markdown => {
                clap::builder::PossibleValue::new("markdown").help("Markdown with YAML frontmatter")
            }
        })
    }
}
