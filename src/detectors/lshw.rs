//! lshw hardware detection implementation

use super::{DetectionData, DetectionResult, HardwareDetector};
use crate::errors::{LxHwError, Result};
use async_trait::async_trait;
use log::{debug, error, warn};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::process::{Command, Output};
use std::time::Duration;

/// Complete hardware information from lshw
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LshwData {
    /// Individual hardware components detected by lshw
    pub components: Vec<LshwComponent>,
    /// Summary statistics and metadata
    pub summary: Option<LshwSummary>,
}

/// Individual hardware component from lshw JSON output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LshwComponent {
    pub id: String,
    pub class: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claimed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub businfo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<HashMap<String, Value>>,
}

/// Summary metadata for lshw detection run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LshwSummary {
    pub total_components: usize,
    pub components_by_class: HashMap<String, usize>,
    pub privileged_execution: bool,
    pub warnings: Vec<String>,
}

pub struct LshwDetector;

impl Default for LshwDetector {
    fn default() -> Self {
        Self
    }
}

impl LshwDetector {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl HardwareDetector for LshwDetector {
    fn name(&self) -> &'static str {
        "lshw"
    }

    async fn is_available(&self) -> bool {
        // Try multiple methods to check for lshw availability
        let which_result = Command::new("which")
            .arg("lshw")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false);

        if which_result {
            return true;
        }

        // Also check common installation paths
        for path in &["/usr/bin/lshw", "/usr/sbin/lshw", "/sbin/lshw"] {
            if std::path::Path::new(path).exists() {
                return true;
            }
        }

        false
    }

    async fn execute(&self) -> Result<Output> {
        debug!("Executing lshw hardware detection");

        let output = tokio::process::Command::new("lshw")
            .arg("-json") // Request JSON output
            .arg("-quiet") // Suppress header information
            .arg("-sanitize") // Remove sensitive information like serials by default
            .arg("-class")
            .arg("system,memory,processor,bridge,network,storage,multimedia,display")
            .output()
            .await
            .map_err(|e| LxHwError::SystemCommandError {
                command: format!("lshw: {}", e)
            })?;

        debug!("lshw execution completed with status: {}", output.status);

        // lshw may return non-zero exit status with warnings but still provide useful data
        if !output.status.success() && output.stdout.is_empty() {
            return Err(LxHwError::DetectionError(format!(
                "lshw failed with exit code: {} and stderr: {}",
                output.status.code().unwrap_or(-1),
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(output)
    }

    fn parse_output(&self, output: &Output) -> Result<DetectionResult> {
        let mut errors = Vec::new();

        // Handle stderr warnings (lshw often warns about privileges)
        if !output.stderr.is_empty() {
            let stderr_str = String::from_utf8_lossy(&output.stderr);
            for line in stderr_str.lines() {
                if !line.trim().is_empty() {
                    warn!("lshw stderr: {}", line);
                    errors.push(format!("Warning: {}", line));
                }
            }
        }

        // Parse stdout JSON
        if output.stdout.is_empty() {
            return Ok(DetectionResult {
                tool_name: self.name().to_string(),
                success: false,
                data: DetectionData::Lshw(LshwData::default()),
                errors: vec!["Empty output from lshw".to_string()],
            });
        }

        let stdout_str = String::from_utf8_lossy(&output.stdout);
        debug!("Parsing lshw JSON output ({} bytes)", stdout_str.len());

        match self.parse_json(&stdout_str) {
            Ok(lshw_data) => {
                debug!("Successfully parsed {} components from lshw", lshw_data.components.len());
                Ok(DetectionResult {
                    tool_name: self.name().to_string(),
                    success: true,
                    data: DetectionData::Lshw(lshw_data),
                    errors,
                })
            }
            Err(e) => {
                error!("Failed to parse lshw JSON: {}", e);
                errors.push(format!("JSON parsing failed: {}", e));
                Ok(DetectionResult {
                    tool_name: self.name().to_string(),
                    success: false,
                    data: DetectionData::Lshw(LshwData::default()),
                    errors,
                })
            }
        }
    }

    fn timeout(&self) -> Duration {
        Duration::from_secs(30)
    }
}

impl LshwDetector {
    /// Parse lshw JSON output into structured data
    fn parse_json(&self, json_str: &str) -> Result<LshwData> {
        // Parse the JSON array of components
        let components: Vec<LshwComponent> = serde_json::from_str(json_str).map_err(|e| {
            LxHwError::SerializationError(format!("lshw JSON parsing failed: {}", e))
        })?;

        // Generate summary statistics
        let summary = self.generate_summary(&components);

        Ok(LshwData { components, summary: Some(summary) })
    }

    /// Generate summary statistics from components
    fn generate_summary(&self, components: &[LshwComponent]) -> LshwSummary {
        let mut components_by_class = HashMap::new();
        let mut warnings = Vec::new();
        let mut privileged_execution = true;

        for component in components {
            *components_by_class.entry(component.class.clone()).or_insert(0) += 1;
        }

        // Check for common indicators of unprivileged execution
        let has_memory = components_by_class.contains_key("memory");
        let has_system = components_by_class.contains_key("system");

        if !has_memory || !has_system {
            privileged_execution = false;
            warnings.push("Some hardware information may be missing due to insufficient privileges. Run as root for complete detection.".to_string());
        }

        LshwSummary {
            total_components: components.len(),
            components_by_class,
            privileged_execution,
            warnings,
        }
    }
}
