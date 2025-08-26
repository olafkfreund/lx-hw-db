//! Hardware detection tools and interfaces

use crate::errors::Result;
use async_trait::async_trait;
use std::process::Output;
use std::time::Duration;

pub mod lshw;
pub mod dmidecode;  
pub mod lspci;
pub mod lsusb;
pub mod inxi;

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
    Dmidecode(dmidecode::DmidecodeData),
    Lspci(lspci::LspciData),
    Lsusb(lsusb::LsusbData),
    Inxi(inxi::InxiData),
}

/// Registry for managing multiple hardware detectors
pub struct DetectorRegistry {
    detectors: Vec<Box<dyn HardwareDetector>>,
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
        }
    }

    /// Get all available detectors on this system
    pub async fn get_available_detectors(&self) -> Vec<&dyn HardwareDetector> {
        let mut available = Vec::new();
        for detector in &self.detectors {
            if detector.is_available().await {
                available.push(detector.as_ref());
            }
        }
        available
    }

    /// Run all available detectors in parallel
    pub async fn detect_all(&self) -> Result<Vec<DetectionResult>> {
        let available = self.get_available_detectors().await;
        let mut results = Vec::new();

        for detector in available {
            match detector.execute().await {
                Ok(output) => {
                    match detector.parse_output(&output) {
                        Ok(result) => results.push(result),
                        Err(e) => {
                            results.push(DetectionResult {
                                tool_name: detector.name().to_string(),
                                success: false,
                                data: DetectionData::Lshw(lshw::LshwData::default()),
                                errors: vec![e.to_string()],
                            });
                        }
                    }
                }
                Err(e) => {
                    results.push(DetectionResult {
                        tool_name: detector.name().to_string(),
                        success: false,
                        data: DetectionData::Lshw(lshw::LshwData::default()),
                        errors: vec![e.to_string()],
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