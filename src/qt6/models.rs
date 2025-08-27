//! Qt6 data models for hardware information and UI state

use cxx_qt_lib::{QString, QVariant};
use serde::{Deserialize, Serialize};

/// Hardware device model for QML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceModel {
    pub id: String,
    pub display_name: String,
    pub vendor: String,
    pub model: String,
    pub category: String,
    pub pci_id: String,
    pub compatibility_status: String,
    pub compatibility_notes: String,
    pub driver_info: String,
    pub testable: bool,
}

impl DeviceModel {
    pub fn get_status_color(&self) -> QString {
        match self.compatibility_status.as_str() {
            "supported" => QString::from("#4CAF50"),
            "partial" => QString::from("#FF9800"),
            "unsupported" => QString::from("#F44336"),
            _ => QString::from("#9E9E9E"),
        }
    }

    pub fn get_status_icon(&self) -> QString {
        match self.compatibility_status.as_str() {
            "supported" => QString::from("check-circle"),
            "partial" => QString::from("warning"),
            "unsupported" => QString::from("error"),
            _ => QString::from("help"),
        }
    }

    pub fn get_support_url(&self) -> QString {
        QString::from(format!(
            "https://www.google.com/search?q=linux+support+{}+{}",
            urlencoding::encode(&self.vendor),
            urlencoding::encode(&self.model)
        ))
    }
}

/// Hardware category model for QML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryModel {
    pub name: String,
    pub icon: String,
    pub color: String,
    pub expanded: bool,
    pub devices: Vec<DeviceModel>,
}

impl CategoryModel {
    pub fn get_support_summary(&self) -> Vec<StatusSummary> {
        let mut summary = std::collections::HashMap::new();
        
        for device in &self.devices {
            *summary.entry(device.compatibility_status.clone()).or_insert(0) += 1;
        }

        summary.into_iter().map(|(status, count)| {
            StatusSummary {
                status,
                count,
                color: match status.as_str() {
                    "supported" => "#4CAF50".to_string(),
                    "partial" => "#FF9800".to_string(),
                    "unsupported" => "#F44336".to_string(),
                    _ => "#9E9E9E".to_string(),
                },
            }
        }).collect()
    }
}

/// Status summary for category indicators
#[derive(Debug, Clone)]
pub struct StatusSummary {
    pub status: String,
    pub count: i32,
    pub color: String,
}

/// Detection tool model for progress tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolModel {
    pub name: String,
    pub description: String,
    pub status: String, // "pending", "running", "complete", "error"
    pub progress: f64,  // 0.0 to 1.0
    pub device_count: i32,
    pub processing_time: String,
}

impl ToolModel {
    pub fn get_status_color(&self) -> QString {
        match self.status.as_str() {
            "pending" => QString::from("#9E9E9E"),
            "running" => QString::from("#2196F3"),
            "complete" => QString::from("#4CAF50"),
            "error" => QString::from("#F44336"),
            _ => QString::from("#9E9E9E"),
        }
    }

    pub fn get_status_icon(&self) -> QString {
        match self.status.as_str() {
            "pending" => QString::from("schedule"),
            "running" => QString::from("refresh"),
            "complete" => QString::from("check-circle"),
            "error" => QString::from("error"),
            _ => QString::from("help"),
        }
    }
}

/// Driver recommendation model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverModel {
    pub id: String,
    pub display_name: String,
    pub description: String,
    pub source: String, // "official", "community", "proprietary"
    pub license: String,
    pub installable: bool,
    pub install_command: String,
    pub devices: Vec<String>,
}

impl DriverModel {
    pub fn get_status_color(&self) -> QString {
        match self.source.as_str() {
            "official" => QString::from("#4CAF50"),
            "community" => QString::from("#2196F3"),
            "proprietary" => QString::from("#FF9800"),
            _ => QString::from("#9E9E9E"),
        }
    }

    pub fn get_source_color(&self) -> QString {
        match self.source.as_str() {
            "official" => QString::from("#4CAF50"),
            "community" => QString::from("#2196F3"),
            "proprietary" => QString::from("#FF9800"),
            _ => QString::from("#9E9E9E"),
        }
    }

    pub fn get_icon(&self) -> QString {
        match self.source.as_str() {
            "official" => QString::from("verified"),
            "community" => QString::from("group"),
            "proprietary" => QString::from("business"),
            _ => QString::from("extension"),
        }
    }
}