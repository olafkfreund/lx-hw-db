//! Utility functions and controllers for the GUI

use std::thread;
use std::sync::mpsc;

use crate::hardware::PrivacyLevel;
use crate::errors::LxHwError;

/// Controller for managing hardware detection operations
pub struct DetectionController {
    sender: Option<mpsc::Sender<DetectionCommand>>,
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

impl DetectionController {
    /// Create a new detection controller
    pub fn new() -> Self {
        Self {
            sender: None,
            handle: None,
        }
    }

    /// Start hardware detection
    pub fn start_detection(
        &mut self, 
        privacy_level: PrivacyLevel,
    ) -> Result<(), LxHwError> {
        // For now, just return success - actual implementation would start detection
        println!("Starting hardware detection with privacy level: {:?}", privacy_level);
        Ok(())
    }

    /// Stop hardware detection
    pub fn stop_detection(&mut self) -> Result<(), LxHwError> {
        if let Some(sender) = &self.sender {
            let _ = sender.send(DetectionCommand::Cancel);
        }

        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }

        self.sender = None;
        Ok(())
    }

    // Detection implementation will be added later
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