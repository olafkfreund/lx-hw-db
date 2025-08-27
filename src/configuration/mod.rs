use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::hardware::HardwareReport;
use crate::errors::LxHwError;

pub mod engine;
pub mod drivers;
pub mod kernel_params;
pub mod dkms;
pub mod packages;
pub mod recommendations;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub system_id: String,
    pub target_distribution: String,
    pub kernel_version: String,
    pub hardware_profile: HardwareProfile,
    pub driver_recommendations: Vec<DriverRecommendation>,
    pub kernel_parameters: Vec<KernelParameter>,
    pub package_installations: Vec<PackageInstallation>,
    pub dkms_modules: Vec<DkmsModule>,
    pub configuration_files: HashMap<String, ConfigurationFile>,
    pub performance_optimizations: Vec<PerformanceOptimization>,
    pub compatibility_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareProfile {
    pub cpu: Option<CpuProfile>,
    pub gpu: Vec<GpuProfile>,
    pub network: Vec<NetworkProfile>,
    pub storage: Vec<StorageProfile>,
    pub audio: Vec<AudioProfile>,
    pub usb_controllers: Vec<UsbControllerProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuProfile {
    pub vendor: String,
    pub model: String,
    pub architecture: String,
    pub features: Vec<String>,
    pub microcode_needed: bool,
    pub power_management: PowerManagement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuProfile {
    pub vendor: String,
    pub model: String,
    pub device_id: String,
    pub driver_options: Vec<String>,
    pub performance_profile: String,
    pub display_outputs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkProfile {
    pub interface_type: String,
    pub vendor: String,
    pub model: String,
    pub device_id: String,
    pub driver_name: Option<String>,
    pub firmware_needed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageProfile {
    pub device_type: String,
    pub interface: String,
    pub vendor: String,
    pub model: String,
    pub capacity: Option<u64>,
    pub optimizations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioProfile {
    pub vendor: String,
    pub codec: String,
    pub device_id: String,
    pub pulse_config: Option<String>,
    pub alsa_config: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbControllerProfile {
    pub version: String,
    pub vendor: String,
    pub device_id: String,
    pub power_management: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverRecommendation {
    pub hardware_id: String,
    pub component_type: String,
    pub recommended_driver: String,
    pub alternative_drivers: Vec<String>,
    pub driver_source: DriverSource,
    pub installation_priority: u8,
    pub compatibility_notes: Option<String>,
    pub kernel_modules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DriverSource {
    KernelBuiltin,
    DistributionPackage { package_name: String },
    ThirdParty { source_url: String },
    Dkms { module_name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelParameter {
    pub parameter: String,
    pub value: Option<String>,
    pub purpose: String,
    pub hardware_target: Option<String>,
    pub distribution_specific: Option<String>,
    pub boot_order: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInstallation {
    pub package_name: String,
    pub package_description: String,
    pub package_category: PackageCategory,
    pub installation_reason: InstallationReason,
    pub installation_command: String,
    pub post_install_commands: Vec<String>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PackageCategory {
    Driver,
    Firmware,
    Utility,
    Development,
    System,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InstallationReason {
    HardwareSupport,
    PerformanceOptimization,
    FeatureEnhancement,
    SecurityUpdate,
    DependencyRequirement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PackageManager {
    Apt,
    Dnf,
    Pacman,
    Zypper,
    Nix,
    Portage,
    Apk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DkmsModule {
    pub module_name: String,
    pub version: String,
    pub source_url: String,
    pub build_dependencies: Vec<String>,
    pub kernel_versions: Vec<String>,
    pub installation_steps: Vec<String>,
    pub auto_rebuild: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationFile {
    pub file_path: String,
    pub content: String,
    pub backup_original: bool,
    pub file_permissions: String,
    pub validation_command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceOptimization {
    pub optimization_type: String,
    pub description: String,
    pub configuration_changes: Vec<ConfigurationChange>,
    pub expected_improvement: String,
    pub risk_level: RiskLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationChange {
    pub file_path: String,
    pub parameter: String,
    pub old_value: Option<String>,
    pub new_value: String,
    pub comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerManagement {
    pub cpu_governor: String,
    pub scaling_driver: String,
    pub idle_states: Vec<String>,
    pub turbo_boost: bool,
}

pub trait ConfigurationEngine {
    fn generate_configuration(&self, hardware: &HardwareReport, target_distribution: &str) -> Result<Configuration, LxHwError>;
    fn analyze_compatibility(&self, hardware: &HardwareReport) -> Result<f64, LxHwError>;
    fn recommend_drivers(&self, hardware: &HardwareReport) -> Result<Vec<DriverRecommendation>, LxHwError>;
    fn generate_kernel_parameters(&self, hardware: &HardwareReport) -> Result<Vec<KernelParameter>, LxHwError>;
    fn suggest_packages(&self, hardware: &HardwareReport, distribution: &str) -> Result<Vec<PackageInstallation>, LxHwError>;
}