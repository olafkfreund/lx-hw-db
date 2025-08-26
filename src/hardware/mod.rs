//! Hardware data structures and system information types

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Privacy levels for hardware data collection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrivacyLevel {
    /// Basic privacy with 24-hour salt rotation
    Basic,
    /// Enhanced privacy with 12-hour salt rotation  
    Enhanced,
    /// Strict privacy with 1-hour salt rotation and maximum anonymization
    Strict,
}

/// Complete hardware report containing all detected information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareReport {
    pub metadata: ReportMetadata,
    pub system: SystemInfo,
    pub cpu: Option<CpuInfo>,
    pub memory: Option<MemoryInfo>,
    pub storage: Vec<StorageDevice>,
    pub graphics: Vec<GraphicsDevice>,
    pub network: Vec<NetworkDevice>,
    pub usb: Vec<UsbDevice>,
    pub audio: Vec<AudioDevice>,
    pub kernel_support: Option<KernelCompatibilityInfo>,
}

/// Report metadata and privacy settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMetadata {
    pub version: String,
    pub generated_at: DateTime<Utc>,
    pub privacy_level: PrivacyLevel,
    pub tools_used: Vec<String>,
    pub anonymized_system_id: String,
}

/// System-level information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub anonymized_hostname: String,
    pub kernel_version: String,
    pub distribution: Option<String>,
    pub architecture: String,
    pub boot_time: Option<DateTime<Utc>>,
}

/// CPU information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    pub model: String,
    pub vendor: String,
    pub cores: u32,
    pub threads: u32,
    pub base_frequency: Option<f64>,
    pub max_frequency: Option<f64>,
    pub cache_l1: Option<u64>,
    pub cache_l2: Option<u64>,
    pub cache_l3: Option<u64>,
    pub flags: Vec<String>,
}

/// Memory information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub dimms: Vec<MemoryDimm>,
}

/// Individual memory DIMM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryDimm {
    pub size_bytes: u64,
    pub speed_mhz: Option<u32>,
    pub memory_type: Option<String>,
    pub manufacturer: Option<String>,
}

/// Storage device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDevice {
    pub anonymized_serial: String,
    pub device_type: String, // SSD, HDD, NVMe, etc.
    pub size_bytes: u64,
    pub model: String,
    pub vendor: Option<String>,
    pub interface: Option<String>,
}

/// Graphics device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsDevice {
    pub vendor: String,
    pub model: String,
    pub driver: Option<String>,
    pub memory_bytes: Option<u64>,
    pub pci_id: String,
}

/// Network device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDevice {
    pub device_type: String, // ethernet, wifi, bluetooth
    pub vendor: String,
    pub model: String,
    pub driver: Option<String>,
    pub anonymized_mac: String,
}

/// USB device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbDevice {
    pub vendor_id: String,
    pub product_id: String,
    pub vendor_name: Option<String>,
    pub product_name: Option<String>,
    pub usb_version: Option<String>,
}

/// Audio device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    pub vendor: String,
    pub model: String,
    pub driver: Option<String>,
    pub device_type: String, // playback, capture, etc.
}

/// Kernel compatibility and support information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelCompatibilityInfo {
    pub kernel_version: String,
    pub total_devices_detected: u32,
    pub supported_devices: u32,
    pub unsupported_devices: u32,
    pub experimental_devices: u32,
    pub device_support_details: Vec<DeviceCompatibility>,
    pub missing_modules: Vec<String>,
    pub config_recommendations: Vec<String>,
}

/// Individual device compatibility information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCompatibility {
    pub device_id: String,
    pub device_name: String,
    pub support_status: String, // "supported", "experimental", "unsupported"
    pub driver_module: String,
    pub since_kernel_version: Option<String>,
    pub config_dependencies: Vec<String>,
    pub notes: Option<String>,
}

impl Default for PrivacyLevel {
    fn default() -> Self {
        Self::Basic
    }
}