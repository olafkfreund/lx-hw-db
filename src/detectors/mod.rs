//! Hardware detection tools and interfaces

use crate::errors::Result;
use async_trait::async_trait;
use std::process::Output;
use std::time::Duration;

pub mod dmidecode;
pub mod integration;
pub mod inxi;
pub mod kernel;
pub mod kernel_source;
pub mod lshw;
pub mod lspci;
pub mod lsusb;

/// Trait for hardware detection tools
#[async_trait]
pub trait HardwareDetector: Send + Sync {
    /// Name of this detector
    fn name(&self) -> &'static str;

    /// Check if this detector is available on the system
    async fn is_available(&self) -> bool;

    /// Execute the detection tool and return raw output
    async fn execute(&self) -> Result<Output>;

    /// Parse the raw output into hardware information
    fn parse_output(&self, output: &Output) -> Result<DetectionResult>;

    /// Get the timeout for this detector
    fn timeout(&self) -> Duration {
        Duration::from_secs(30)
    }
}

/// Result from a hardware detection tool
#[derive(Debug, Clone)]
pub struct DetectionResult {
    pub tool_name: String,
    pub success: bool,
    pub data: DetectionData,
    pub errors: Vec<String>,
}

/// Data extracted by detection tools
#[derive(Debug, Clone)]
pub enum DetectionData {
    Lshw(lshw::LshwData),
    Dmidecode(Box<dmidecode::DmidecodeData>),
    Lspci(lspci::LspciData),
    Lsusb(lsusb::LsusbData),
    Inxi(Box<inxi::InxiData>),
    Kernel(kernel::KernelSupportData),
}

/// Registry for managing multiple hardware detectors
pub struct DetectorRegistry {
    detectors: Vec<Box<dyn HardwareDetector>>,
    enabled_tools: Option<Vec<String>>,
    custom_timeout: Option<Duration>,
}

impl DetectorRegistry {
    /// Create a new detector registry with default detectors
    pub fn new() -> Self {
        Self {
            detectors: vec![
                Box::new(lshw::LshwDetector::new()),
                Box::new(dmidecode::DmidecodeDetector::new()),
                Box::new(lspci::LspciDetector::new()),
                Box::new(lsusb::LsusbDetector::new()),
                Box::new(inxi::InxiDetector::new()),
            ],
            enabled_tools: None,
            custom_timeout: None,
        }
    }

    /// Set specific tools to enable (filters out others)
    pub fn set_enabled_tools(&mut self, tool_names: Vec<String>) -> Result<()> {
        // Validate that all requested tools exist
        let available_tools: Vec<&str> = self.detectors.iter().map(|d| d.name()).collect();

        let mut invalid_tools = Vec::new();
        for tool in &tool_names {
            if !available_tools.contains(&tool.as_str()) {
                invalid_tools.push(tool.clone());
            }
        }

        if !invalid_tools.is_empty() {
            return Err(crate::errors::LxHwError::ConfigError(format!(
                "Unknown detection tools: {}. Available tools: {}",
                invalid_tools.join(", "),
                available_tools.join(", ")
            )));
        }

        self.enabled_tools = Some(tool_names);
        Ok(())
    }

    /// Set custom timeout for all detectors
    pub fn set_detection_timeout(&mut self, timeout: Duration) {
        self.custom_timeout = Some(timeout);
    }

    /// Check if a tool is enabled based on filtering configuration
    fn is_tool_enabled(&self, tool_name: &str) -> bool {
        match &self.enabled_tools {
            Some(enabled) => enabled.contains(&tool_name.to_string()),
            None => true, // If no filter is set, all tools are enabled
        }
    }

    /// Get the effective timeout for a detector
    fn get_effective_timeout(&self, detector: &dyn HardwareDetector) -> Duration {
        self.custom_timeout.unwrap_or_else(|| detector.timeout())
    }

    /// Create default data variant for a specific detector type
    fn default_data_for_detector(detector_name: &str) -> DetectionData {
        match detector_name {
            "lshw" => DetectionData::Lshw(lshw::LshwData::default()),
            "dmidecode" => DetectionData::Dmidecode(Box::default()),
            "lspci" => DetectionData::Lspci(lspci::LspciData::default()),
            "lsusb" => DetectionData::Lsusb(lsusb::LsusbData::default()),
            "inxi" => DetectionData::Inxi(Box::default()),
            _ => DetectionData::Lshw(lshw::LshwData::default()), // fallback
        }
    }

    /// Get all available detectors on this system (respecting enabled tool filter)
    pub async fn get_available_detectors(&self) -> Vec<&dyn HardwareDetector> {
        let mut available = Vec::new();
        for detector in &self.detectors {
            // Check if tool is enabled by filter and available on system
            if self.is_tool_enabled(detector.name()) && detector.is_available().await {
                available.push(detector.as_ref());
            }
        }
        available
    }

    /// Get list of all registered detectors (for checking availability)
    pub fn list_detectors(&self) -> Vec<&dyn HardwareDetector> {
        self.detectors.iter().map(|d| d.as_ref()).collect()
    }

    /// Run all available detectors with timeout handling
    pub async fn detect_all(&self) -> Result<Vec<DetectionResult>> {
        let available = self.get_available_detectors().await;
        let mut results = Vec::new();

        for detector in available {
            let timeout = self.get_effective_timeout(detector);

            // Execute with timeout
            let execution_result = tokio::time::timeout(timeout, detector.execute()).await;

            match execution_result {
                Ok(Ok(output)) => {
                    // Successful execution within timeout
                    match detector.parse_output(&output) {
                        Ok(result) => results.push(result),
                        Err(e) => {
                            results.push(DetectionResult {
                                tool_name: detector.name().to_string(),
                                success: false,
                                data: Self::default_data_for_detector(detector.name()),
                                errors: vec![e.to_string()],
                            });
                        }
                    }
                }
                Ok(Err(e)) => {
                    // Execution failed
                    results.push(DetectionResult {
                        tool_name: detector.name().to_string(),
                        success: false,
                        data: Self::default_data_for_detector(detector.name()),
                        errors: vec![e.to_string()],
                    });
                }
                Err(_) => {
                    // Execution timed out
                    results.push(DetectionResult {
                        tool_name: detector.name().to_string(),
                        success: false,
                        data: Self::default_data_for_detector(detector.name()),
                        errors: vec![format!("Execution timed out after {:?}", timeout)],
                    });
                }
            }
        }

        Ok(results)
    }
}

impl Default for DetectorRegistry {
    fn default() -> Self {
        Self::new()
    }
}
