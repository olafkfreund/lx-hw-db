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

/// Output renderer with enhanced formatting capabilities
pub struct OutputRenderer {
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

impl OutputRenderer {
    /// Create a new output renderer with the specified format
    pub fn new(format: OutputFormat) -> Self {
        Self { format }
    }

    /// Render a hardware report
    pub fn render(&self, report: &HardwareReport) -> Result<String> {
        match self.format {
            OutputFormat::Yaml => self.render_yaml(report),
            OutputFormat::Json => self.render_json(report),
            OutputFormat::Markdown => self.render_markdown(report),
        }
    }

    /// Render a hardware report without privacy anonymization (debug mode)
    pub fn render_debug(&self, report: &HardwareReport) -> Result<String> {
        // Same as regular render for now - could include raw data in the future
        self.render(report)
    }

    fn render_yaml(&self, report: &HardwareReport) -> Result<String> {
        Ok(serde_yaml::to_string(report)?)
    }

    fn render_json(&self, report: &HardwareReport) -> Result<String> {
        Ok(serde_json::to_string_pretty(report)?)
    }

    fn render_markdown(&self, report: &HardwareReport) -> Result<String> {
        let mut output = String::new();
        
        // YAML frontmatter
        output.push_str("---\n");
        output.push_str(&serde_yaml::to_string(&report.metadata)?);
        output.push_str("---\n\n");
        
        // Markdown content
        output.push_str("# Hardware Compatibility Report\n\n");
        output.push_str(&format!("Generated: {}\n", report.metadata.generated_at));
        output.push_str(&format!("Privacy Level: {:?}\n", report.metadata.privacy_level));
        output.push_str(&format!("Tools Used: {}\n\n", report.metadata.tools_used.join(", ")));
        
        // System information
        output.push_str("## System Information\n\n");
        output.push_str(&format!("- **Hostname:** {}\n", report.system.anonymized_hostname));
        output.push_str(&format!("- **Kernel:** {}\n", report.system.kernel_version));
        output.push_str(&format!("- **Architecture:** {}\n", report.system.architecture));
        if let Some(ref distro) = report.system.distribution {
            output.push_str(&format!("- **Distribution:** {}\n", distro));
        }
        
        // Kernel compatibility
        if let Some(ref kernel_support) = report.kernel_support {
            output.push_str("\n## Kernel Compatibility\n\n");
            output.push_str(&format!("- **Total Devices:** {}\n", kernel_support.total_devices_detected));
            output.push_str(&format!("- **Supported:** {}\n", kernel_support.supported_devices));
            output.push_str(&format!("- **Unsupported:** {}\n", kernel_support.unsupported_devices));
            output.push_str(&format!("- **Experimental:** {}\n", kernel_support.experimental_devices));
            
            if !kernel_support.device_support_details.is_empty() {
                output.push_str("\n### Device Details\n\n");
                for device in &kernel_support.device_support_details {
                    output.push_str(&format!("#### {}\n\n", device.device_name));
                    output.push_str(&format!("- **Device ID:** {}\n", device.device_id));
                    output.push_str(&format!("- **Status:** {}\n", device.support_status));
                    output.push_str(&format!("- **Driver:** {}\n", device.driver_module));
                    if let Some(ref since) = device.since_kernel_version {
                        output.push_str(&format!("- **Since Kernel:** {}\n", since));
                    }
                    if let Some(ref notes) = device.notes {
                        output.push_str(&format!("- **Notes:** {}\n", notes));
                    }
                    output.push('\n');
                }
            }
        }
        
        Ok(output)
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