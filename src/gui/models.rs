//! Application state models and data structures

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::hardware::{HardwareReport, PrivacyLevel};
use crate::configuration::Configuration;
use crate::errors::LxHwError;

/// Main application state
#[derive(Debug, Clone)]
pub struct AppState {
    /// Current hardware detection progress (0.0 to 1.0)
    pub detection_progress: f64,
    
    /// Current detection status message
    pub detection_status: String,
    
    /// Whether hardware detection is currently running
    pub is_detecting: bool,
    
    /// Latest hardware report
    pub hardware_report: Option<HardwareReport>,
    
    /// Generated configuration recommendations
    pub configuration: Option<Configuration>,
    
    /// Current privacy level
    pub privacy_level: PrivacyLevel,
    
    /// Available detection tools and their status
    pub available_tools: HashMap<String, ToolStatus>,
    
    /// Application preferences
    pub preferences: AppPreferences,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            detection_progress: 0.0,
            detection_status: "Ready".to_string(),
            is_detecting: false,
            hardware_report: None,
            configuration: None,
            privacy_level: PrivacyLevel::Basic,
            available_tools: HashMap::new(),
            preferences: AppPreferences::default(),
        }
    }
}

/// Status of a detection tool
#[derive(Debug, Clone)]
pub struct ToolStatus {
    pub name: String,
    pub available: bool,
    pub version: Option<String>,
    pub requires_root: bool,
    pub enabled: bool,
}

/// Application preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppPreferences {
    /// Privacy settings
    pub privacy: PrivacyPreferences,
    
    /// Detection tool settings
    pub tools: ToolPreferences,
    
    /// Export settings
    pub export: ExportPreferences,
    
    /// UI settings
    pub ui: UiPreferences,
}

impl Default for AppPreferences {
    fn default() -> Self {
        Self {
            privacy: PrivacyPreferences::default(),
            tools: ToolPreferences::default(),
            export: ExportPreferences::default(),
            ui: UiPreferences::default(),
        }
    }
}

/// Privacy-related preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyPreferences {
    pub level: PrivacyLevel,
    pub include_hardware_info: bool,
    pub include_kernel_info: bool,
    pub include_performance_data: bool,
    pub salt_rotation_hours: u32,
}

impl Default for PrivacyPreferences {
    fn default() -> Self {
        Self {
            level: PrivacyLevel::Basic,
            include_hardware_info: true,
            include_kernel_info: true,
            include_performance_data: false,
            salt_rotation_hours: 24,
        }
    }
}

/// Detection tool preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPreferences {
    pub enabled_tools: Vec<String>,
    pub timeout_seconds: u32,
    pub parallel_execution: bool,
}

impl Default for ToolPreferences {
    fn default() -> Self {
        Self {
            enabled_tools: vec![
                "lshw".to_string(),
                "dmidecode".to_string(),
                "lspci".to_string(),
                "lsusb".to_string(),
            ],
            timeout_seconds: 30,
            parallel_execution: true,
        }
    }
}

/// Export preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPreferences {
    pub default_format: String,
    pub default_location: String,
    pub include_metadata: bool,
    pub compress_output: bool,
}

impl Default for ExportPreferences {
    fn default() -> Self {
        Self {
            default_format: "yaml".to_string(),
            default_location: "~/Documents".to_string(),
            include_metadata: true,
            compress_output: false,
        }
    }
}

/// UI preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiPreferences {
    pub window_width: i32,
    pub window_height: i32,
    pub sidebar_visible: bool,
    pub detailed_view: bool,
}

impl Default for UiPreferences {
    fn default() -> Self {
        Self {
            window_width: 1000,
            window_height: 700,
            sidebar_visible: true,
            detailed_view: false,
        }
    }
}

/// Shared application state type
pub type SharedAppState = Arc<Mutex<AppState>>;

/// Create a new shared application state
pub fn create_app_state() -> SharedAppState {
    Arc::new(Mutex::new(AppState::default()))
}

/// Hardware detection progress callback
pub type ProgressCallback = Box<dyn Fn(f64, String) + Send + Sync>;

/// Hardware detection result
#[derive(Debug)]
pub enum DetectionResult {
    Progress { fraction: f64, message: String },
    Complete { report: HardwareReport },
    Error { error: LxHwError },
}

/// GUI event types for state updates
#[derive(Debug, Clone)]
pub enum GuiEvent {
    DetectionStarted,
    DetectionProgress { fraction: f64, message: String },
    DetectionComplete { report: HardwareReport },
    DetectionError { error: String },
    ConfigurationGenerated { config: Configuration },
    PreferencesChanged { preferences: AppPreferences },
    PrivacyLevelChanged { level: PrivacyLevel },
}

/// Hardware category for display organization
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HardwareCategory {
    System,
    Processor,
    Memory,
    Graphics,
    Network,
    Audio,
    Storage,
    Usb,
    Other,
}

impl HardwareCategory {
    /// Get display name for category
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::System => "System & Motherboard",
            Self::Processor => "CPU & Memory",
            Self::Memory => "Memory",
            Self::Graphics => "Graphics",
            Self::Network => "Network",
            Self::Audio => "Audio",
            Self::Storage => "Storage",
            Self::Usb => "USB & Peripherals",
            Self::Other => "Other Devices",
        }
    }

    /// Get icon name for category
    pub fn icon_name(&self) -> &'static str {
        match self {
            Self::System => "computer-symbolic",
            Self::Processor => "cpu-symbolic",
            Self::Memory => "media-memory-symbolic",
            Self::Graphics => "video-display-symbolic",
            Self::Network => "network-wired-symbolic",
            Self::Audio => "audio-speakers-symbolic",
            Self::Storage => "drive-harddisk-symbolic",
            Self::Usb => "usb-symbolic",
            Self::Other => "applications-other-symbolic",
        }
    }
}

/// Device compatibility status for display
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompatibilityStatus {
    Supported,
    PartialSupport,
    RequiresDriver,
    Unsupported,
    Unknown,
}

impl CompatibilityStatus {
    /// Get display text for status
    pub fn display_text(&self) -> &'static str {
        match self {
            Self::Supported => "Fully Supported",
            Self::PartialSupport => "Partial Support",
            Self::RequiresDriver => "Driver Required",
            Self::Unsupported => "Not Supported",
            Self::Unknown => "Unknown",
        }
    }

    /// Get CSS class for status styling
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Supported => "status-supported",
            Self::PartialSupport => "status-partial",
            Self::RequiresDriver => "status-warning",
            Self::Unsupported => "status-error",
            Self::Unknown => "status-unknown",
        }
    }

    /// Get icon name for status
    pub fn icon_name(&self) -> &'static str {
        match self {
            Self::Supported => "emblem-ok-symbolic",
            Self::PartialSupport => "dialog-warning-symbolic",
            Self::RequiresDriver => "software-update-urgent-symbolic",
            Self::Unsupported => "emblem-important-symbolic",
            Self::Unknown => "help-about-symbolic",
        }
    }
}

/// Display model for hardware devices in the UI
#[derive(Debug, Clone)]
pub struct HardwareDeviceDisplay {
    pub id: String,
    pub name: String,
    pub category: HardwareCategory,
    pub vendor: String,
    pub model: String,
    pub status: CompatibilityStatus,
    pub details: HashMap<String, String>,
    pub recommendations: Vec<String>,
}

impl HardwareDeviceDisplay {
    /// Create display model from hardware report data
    pub fn from_hardware_report(report: &HardwareReport) -> Vec<Self> {
        let mut devices = Vec::new();
        
        // Add system information
        devices.push(HardwareDeviceDisplay {
            id: "system".to_string(),
            name: format!("System - {}", report.system.distribution.clone().unwrap_or("Unknown".to_string())),
            category: HardwareCategory::System,
            vendor: "System".to_string(),
            model: report.system.distribution.clone().unwrap_or("Unknown".to_string()),
            status: CompatibilityStatus::Supported,
            details: [
                ("Distribution".to_string(), report.system.distribution.clone().unwrap_or("Unknown".to_string())),
                ("Kernel".to_string(), report.system.kernel_version.clone()),
                ("Architecture".to_string(), report.system.architecture.clone()),
            ].into(),
            recommendations: vec![],
        });

        // Add CPU information
        if let Some(cpu) = &report.cpu {
            devices.push(HardwareDeviceDisplay {
                id: "cpu".to_string(),
                name: cpu.model.clone(),
                category: HardwareCategory::Processor,
                vendor: cpu.vendor.clone(),
                model: cpu.model.clone(),
                status: CompatibilityStatus::Supported,
                details: [
                    ("Cores".to_string(), cpu.cores.to_string()),
                    ("Threads".to_string(), cpu.threads.to_string()),
                    ("Frequency".to_string(), format!("{:.2} GHz", cpu.base_frequency.unwrap_or(0.0) / 1000.0)),
                ].into(),
                recommendations: vec![],
            });
        }

        // Add graphics devices
        for (i, gpu) in report.graphics.iter().enumerate() {
            devices.push(HardwareDeviceDisplay {
                id: format!("gpu_{}", i),
                name: gpu.model.clone(),
                category: HardwareCategory::Graphics,
                vendor: gpu.vendor.clone(),
                model: gpu.model.clone(),
                status: if gpu.driver.is_some() {
                    CompatibilityStatus::Supported
                } else {
                    CompatibilityStatus::RequiresDriver
                },
                details: [
                    ("PCI ID".to_string(), gpu.pci_id.clone()),
                    ("Driver".to_string(), gpu.driver.clone().unwrap_or("Not loaded".to_string())),
                ].into(),
                recommendations: if gpu.driver.is_none() {
                    vec!["Install appropriate graphics driver".to_string()]
                } else {
                    vec![]
                },
            });
        }

        // Add network devices
        for (i, net) in report.network.iter().enumerate() {
            devices.push(HardwareDeviceDisplay {
                id: format!("net_{}", i),
                name: net.model.clone(),
                category: HardwareCategory::Network,
                vendor: net.vendor.clone(),
                model: net.model.clone(),
                status: if net.driver.is_some() {
                    CompatibilityStatus::Supported
                } else {
                    CompatibilityStatus::RequiresDriver
                },
                details: [
                    ("Type".to_string(), net.device_type.clone()),
                    ("Driver".to_string(), net.driver.clone().unwrap_or("Not loaded".to_string())),
                ].into(),
                recommendations: vec![],
            });
        }

        // Add storage devices
        for (i, storage) in report.storage.iter().enumerate() {
            devices.push(HardwareDeviceDisplay {
                id: format!("storage_{}", i),
                name: storage.model.clone(),
                category: HardwareCategory::Storage,
                vendor: storage.vendor.clone().unwrap_or("Unknown".to_string()),
                model: storage.model.clone(),
                status: CompatibilityStatus::Supported,
                details: [
                    ("Size".to_string(), format!("{:.1} GB", storage.size_bytes as f64 / (1024.0 * 1024.0 * 1024.0))),
                    ("Interface".to_string(), storage.interface.clone().unwrap_or("Unknown".to_string())),
                ].into(),
                recommendations: vec![],
            });
        }

        // Add audio devices
        for (i, audio) in report.audio.iter().enumerate() {
            devices.push(HardwareDeviceDisplay {
                id: format!("audio_{}", i),
                name: format!("{} Audio", audio.vendor),
                category: HardwareCategory::Audio,
                vendor: audio.vendor.clone(),
                model: audio.device_type.clone(),
                status: CompatibilityStatus::Supported,
                details: [
                    ("Type".to_string(), audio.device_type.clone()),
                    ("Driver".to_string(), audio.driver.clone().unwrap_or("Built-in".to_string())),
                ].into(),
                recommendations: vec![],
            });
        }

        devices
    }
}