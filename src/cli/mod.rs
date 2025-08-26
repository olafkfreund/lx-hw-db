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
        _privacy: PrivacyLevel,
        _format: OutputFormat,
        _output: Option<PathBuf>,
        _tools: Option<Vec<String>>,
        _timeout: u64,
        _no_anonymize: bool,
    ) -> Result<()> {
        // Placeholder implementation - will be completed in Phase 2
        log::info!("Hardware detection will be implemented in Phase 2");
        println!("Hardware detection feature is not yet implemented.");
        println!("This is the foundation phase - detection will be added next.");
        Ok(())
    }

    /// Handle the check command
    async fn handle_check(&self, _detailed: bool) -> Result<()> {
        // Placeholder implementation
        log::info!("Tool availability check will be implemented in Phase 2");
        println!("Tool availability check feature is not yet implemented.");
        Ok(())
    }

    /// Handle the validate command
    async fn handle_validate(&self, _file: PathBuf, _schema_only: bool) -> Result<()> {
        // Placeholder implementation
        log::info!("Report validation will be implemented in Phase 2");
        println!("Report validation feature is not yet implemented.");
        Ok(())
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