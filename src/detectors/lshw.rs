//! lshw hardware detection implementation

use super::{HardwareDetector, DetectionResult, DetectionData};
use crate::errors::Result;
use async_trait::async_trait;
use std::process::{Output, Command};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LshwData {
    // Placeholder - will be implemented in Phase 2
}

pub struct LshwDetector;

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
        Command::new("which")
            .arg("lshw")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    async fn execute(&self) -> Result<Output> {
        // Placeholder implementation
        Ok(Output {
            status: std::process::ExitStatus::default(),
            stdout: Vec::new(),
            stderr: Vec::new(),
        })
    }

    fn parse_output(&self, _output: &Output) -> Result<DetectionResult> {
        // Placeholder implementation
        Ok(DetectionResult {
            tool_name: self.name().to_string(),
            success: false,
            data: DetectionData::Lshw(LshwData::default()),
            errors: vec!["Not yet implemented".to_string()],
        })
    }
}