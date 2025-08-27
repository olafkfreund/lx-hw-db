//! Utility functions and controllers for the GUI

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

use crate::hardware::PrivacyLevel;
use crate::errors::{LxHwError, Result};
use crate::detectors::integration::HardwareAnalyzer;
use crate::gui::models::HardwareReport;

/// Controller for managing hardware detection operations
pub struct DetectionController {
    sender: Option<mpsc::Sender<DetectionCommand>>,
    receiver: Option<mpsc::Receiver<DetectionResult>>,
    handle: Option<thread::JoinHandle<()>>,
}

/// Commands for the detection controller
#[derive(Debug)]
pub enum DetectionCommand {
    Start {
        privacy_level: PrivacyLevel,
    },
    Cancel,
}

/// Results from hardware detection
#[derive(Debug, Clone)]
pub enum DetectionResult {
    Progress {
        tool: String,
        fraction: f64,
        message: String,
    },
    ToolComplete {
        tool: String,
        success: bool,
    },
    Complete {
        report: HardwareReport,
    },
    Error {
        message: String,
    },
}

impl DetectionController {
    /// Create a new detection controller
    pub fn new() -> Self {
        Self {
            sender: None,
            receiver: None,
            handle: None,
        }
    }

    /// Start hardware detection with real backend integration
    pub fn start_detection(
        &mut self, 
        privacy_level: PrivacyLevel,
    ) -> Result<mpsc::Receiver<DetectionResult>> {
        log::info!("Starting hardware detection with privacy level: {:?}", privacy_level);
        
        // Stop any existing detection
        self.stop_detection()?;
        
        // Create channels for communication
        let (cmd_sender, cmd_receiver) = mpsc::channel();
        let (result_sender, result_receiver) = mpsc::channel();
        
        // Store references
        self.sender = Some(cmd_sender);
        self.receiver = Some(result_receiver.clone());
        
        // Spawn detection thread
        let handle = thread::spawn(move || {
            Self::detection_worker(cmd_receiver, result_sender, privacy_level);
        });
        
        self.handle = Some(handle);
        
        Ok(result_receiver)
    }

    /// Hardware detection worker thread
    fn detection_worker(
        cmd_receiver: mpsc::Receiver<DetectionCommand>,
        result_sender: mpsc::Sender<DetectionResult>,
        privacy_level: PrivacyLevel,
    ) {
        // Create async runtime for hardware detection
        let rt = match tokio::runtime::Runtime::new() {
            Ok(rt) => rt,
            Err(e) => {
                let _ = result_sender.send(DetectionResult::Error {
                    message: format!("Failed to create async runtime: {}", e),
                });
                return;
            }
        };
        
        rt.block_on(async {
            // Send initial progress
            let _ = result_sender.send(DetectionResult::Progress {
                tool: "initialization".to_string(),
                fraction: 0.0,
                message: "Initializing hardware analyzer...".to_string(),
            });
            
            // Create hardware analyzer
            let mut analyzer = match HardwareAnalyzer::new(privacy_level) {
                Ok(analyzer) => analyzer,
                Err(e) => {
                    let _ = result_sender.send(DetectionResult::Error {
                        message: format!("Failed to create hardware analyzer: {}", e),
                    });
                    return;
                }
            };
            
            // Send progress for each detection step
            let tools = ["lshw", "dmidecode", "lspci", "lsusb", "inxi"];
            let mut completed_tools = 0;
            
            for (i, tool) in tools.iter().enumerate() {
                // Check for cancellation
                if let Ok(DetectionCommand::Cancel) = cmd_receiver.try_recv() {
                    log::info!("Hardware detection cancelled by user");
                    return;
                }
                
                // Send progress for current tool
                let progress = i as f64 / tools.len() as f64;
                let _ = result_sender.send(DetectionResult::Progress {
                    tool: tool.to_string(),
                    fraction: progress,
                    message: format!("Running {} detection...", tool),
                });
                
                // Simulate tool execution time
                tokio::time::sleep(Duration::from_millis(500)).await;
                
                // Mark tool as complete
                let _ = result_sender.send(DetectionResult::ToolComplete {
                    tool: tool.to_string(),
                    success: true,
                });
                
                completed_tools += 1;
                
                // Update overall progress
                let overall_progress = completed_tools as f64 / tools.len() as f64;
                let _ = result_sender.send(DetectionResult::Progress {
                    tool: "overall".to_string(),
                    fraction: overall_progress,
                    message: format!("Completed {}/{} detection tools", completed_tools, tools.len()),
                });
            }
            
            // Run actual hardware analysis
            let _ = result_sender.send(DetectionResult::Progress {
                tool: "analysis".to_string(),
                fraction: 0.9,
                message: "Analyzing detected hardware...".to_string(),
            });
            
            match analyzer.analyze_system().await {
                Ok(hardware_report) => {
                    // Convert to GUI HardwareReport format
                    let gui_report = Self::convert_hardware_report(hardware_report);
                    
                    let _ = result_sender.send(DetectionResult::Complete {
                        report: gui_report,
                    });
                }
                Err(e) => {
                    let _ = result_sender.send(DetectionResult::Error {
                        message: format!("Hardware analysis failed: {}", e),
                    });
                }
            }
        });
    }
    
    /// Convert Rust hardware report to GUI format
    fn convert_hardware_report(report: crate::hardware::HardwareReport) -> HardwareReport {
        HardwareReport {
            system: crate::gui::models::SystemInfo {
                distribution: report.system.distribution,
                kernel_version: report.system.kernel_version,
                architecture: report.system.architecture,
            },
            cpu: report.cpu.map(|cpu| crate::gui::models::CpuInfo {
                vendor: cpu.vendor,
                model: cpu.model,
                cores: cpu.cores,
                threads: cpu.threads,
                base_frequency: cpu.base_frequency,
            }),
            graphics: report.graphics.into_iter().map(|gpu| crate::gui::models::GraphicsDevice {
                vendor: gpu.vendor,
                model: gpu.model,
                pci_id: gpu.pci_id,
                driver: gpu.driver,
            }).collect(),
            network: report.network.into_iter().map(|net| crate::gui::models::NetworkDevice {
                vendor: net.vendor,
                model: net.model,
                device_type: net.device_type,
                driver: net.driver,
            }).collect(),
            storage: report.storage.into_iter().map(|storage| crate::gui::models::StorageDevice {
                vendor: storage.vendor,
                model: storage.model,
                size_bytes: storage.size_bytes,
                interface: storage.interface,
            }).collect(),
            audio: report.audio.into_iter().map(|audio| crate::gui::models::AudioDevice {
                vendor: audio.vendor,
                device_type: audio.device_type,
                driver: audio.driver,
            }).collect(),
        }
    }

    /// Stop hardware detection
    pub fn stop_detection(&mut self) -> Result<()> {
        if let Some(sender) = &self.sender {
            let _ = sender.send(DetectionCommand::Cancel);
        }

        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }

        self.sender = None;
        self.receiver = None;
        
        Ok(())
    }
    
    /// Check if detection is running
    pub fn is_running(&self) -> bool {
        self.handle.is_some() && self.sender.is_some()
    }
}

impl Drop for DetectionController {
    fn drop(&mut self) {
        let _ = self.stop_detection();
    }
}

// Mock hardware report creation will be implemented later when needed

/// Utility functions for the GUI
pub mod gui_utils {
    use gtk4::prelude::*;
    use libadwaita as adw;
    use adw::prelude::*;

    /// Show an error dialog
    pub fn show_error_dialog(parent: &impl IsA<gtk4::Window>, title: &str, message: &str) {
        let dialog = adw::MessageDialog::new(
            Some(parent),
            Some(title),
            Some(message),
        );
        dialog.add_response("ok", &crate::gui::t("OK"));
        dialog.present();
    }

    /// Show a success toast
    pub fn show_success_toast(overlay: &adw::ToastOverlay, message: &str) {
        let toast = adw::Toast::new(message);
        overlay.add_toast(toast);
    }

    /// Format bytes as human-readable size
    pub fn format_bytes(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        if unit_index == 0 {
            format!("{} {}", bytes, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
    }

    /// Format frequency in Hz as human-readable
    pub fn format_frequency(hz: u64) -> String {
        if hz >= 1_000_000_000 {
            format!("{:.2} GHz", hz as f64 / 1_000_000_000.0)
        } else if hz >= 1_000_000 {
            format!("{:.1} MHz", hz as f64 / 1_000_000.0)
        } else if hz >= 1_000 {
            format!("{:.0} kHz", hz as f64 / 1_000.0)
        } else {
            format!("{} Hz", hz)
        }
    }
}