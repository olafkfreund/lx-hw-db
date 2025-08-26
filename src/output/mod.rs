//! Output formatting and report generation

use crate::errors::Result;
use crate::hardware::HardwareReport;
use serde_yaml;
use std::fmt;

/// Output format for hardware reports
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Yaml,
    Json,  
    Markdown,
}

/// Report generator for different output formats
pub struct ReportGenerator {
    format: OutputFormat,
}

impl ReportGenerator {
    /// Create a new report generator with the specified format
    pub fn new(format: OutputFormat) -> Self {
        Self { format }
    }

    /// Generate a formatted report from hardware data
    pub fn generate(&self, report: &HardwareReport) -> Result<String> {
        match self.format {
            OutputFormat::Yaml => self.generate_yaml(report),
            OutputFormat::Json => self.generate_json(report),
            OutputFormat::Markdown => self.generate_markdown(report),
        }
    }

    fn generate_yaml(&self, report: &HardwareReport) -> Result<String> {
        Ok(serde_yaml::to_string(report)?)
    }

    fn generate_json(&self, report: &HardwareReport) -> Result<String> {
        Ok(serde_json::to_string_pretty(report)?)
    }

    fn generate_markdown(&self, report: &HardwareReport) -> Result<String> {
        // Generate YAML frontmatter + markdown body
        let frontmatter = serde_yaml::to_string(report)?;
        Ok(format!("---\n{}---\n\n# Hardware Compatibility Report\n\nGenerated on: {}\n", 
                   frontmatter, report.metadata.generated_at))
    }
}

impl Default for ReportGenerator {
    fn default() -> Self {
        Self::new(OutputFormat::Markdown)
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::Yaml => write!(f, "yaml"),
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Markdown => write!(f, "markdown"),
        }
    }
}