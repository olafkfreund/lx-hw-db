//! lspci hardware detection implementation

use super::{HardwareDetector, DetectionResult, DetectionData};
use crate::errors::Result;
use async_trait::async_trait;
use std::process::{Output, Command};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LspciData {
    // Placeholder - will be implemented in Phase 2
}

pub struct LspciDetector;

impl LspciDetector {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl HardwareDetector for LspciDetector {
    fn name(&self) -> &'static str {
        "lspci"
    }

    async fn is_available(&self) -> bool {
        Command::new("which")
            .arg("lspci")
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
            data: DetectionData::Lspci(LspciData::default()),
            errors: vec!["Not yet implemented".to_string()],
        })
    }
}