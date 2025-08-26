//! Kernel-based hardware support verification
//! 
//! This module leverages Linux kernel information to verify hardware support
//! by checking modules.alias files, sysfs information, and kernel device tables.

use crate::errors::{Result, LxHwError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

/// Kernel support verification data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelSupportData {
    pub kernel_version: String,
    pub supported_devices: Vec<DeviceSupport>,
    pub module_aliases: HashMap<String, Vec<String>>,
    pub config_options: HashMap<String, bool>,
}

/// Individual device support information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSupport {
    pub device_id: String, // PCI ID format: vendor:device
    pub driver_module: String,
    pub support_level: SupportLevel,
    pub kernel_version_added: Option<String>,
    pub config_dependencies: Vec<String>,
}

/// Level of hardware support
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SupportLevel {
    /// Officially supported with dedicated driver
    Supported,
    /// Experimental or staging driver
    Experimental, 
    /// Generic driver support (e.g., USB HID)
    Generic,
    /// Similar device supported, might work
    Similar,
    /// No kernel support found
    Unsupported,
}

/// Kernel support verifier
pub struct KernelSupportVerifier {
    kernel_version: String,
    modules_alias_path: String,
    config_path: Option<String>,
}

impl KernelSupportVerifier {
    /// Create a new kernel support verifier
    pub fn new() -> Result<Self> {
        let kernel_version = Self::get_kernel_version()?;
        let modules_alias_path = format!("/lib/modules/{}/modules.alias", kernel_version);
        let config_path = Self::find_kernel_config(&kernel_version);

        Ok(Self {
            kernel_version,
            modules_alias_path,
            config_path,
        })
    }

    /// Get current kernel version
    fn get_kernel_version() -> Result<String> {
        let output = Command::new("uname")
            .arg("-r")
            .output()
            .map_err(|e| LxHwError::SystemCommandError { 
                command: format!("uname -r: {}", e) 
            })?;

        if !output.status.success() {
            return Err(LxHwError::SystemCommandError {
                command: "uname -r".to_string(),
            });
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    /// Find kernel configuration file
    fn find_kernel_config(kernel_version: &str) -> Option<String> {
        let possible_paths = vec![
            format!("/boot/config-{}", kernel_version),
            "/proc/config.gz".to_string(),
            "/boot/config".to_string(),
        ];

        possible_paths.into_iter().find(|path| Path::new(path).exists())
    }

    /// Verify support for a specific PCI device
    pub fn verify_pci_support(&self, vendor_id: &str, device_id: &str) -> Result<DeviceSupport> {
        let pci_id = format!("{vendor_id}:{device_id}").to_lowercase();
        
        // Check modules.alias for exact match
        if let Some(module) = self.check_modules_alias(&pci_id)? {
            let config_deps = self.get_config_dependencies(&module)?;
            return Ok(DeviceSupport {
                device_id: pci_id,
                driver_module: module,
                support_level: SupportLevel::Supported,
                kernel_version_added: None, // TODO: Could be extracted from git history
                config_dependencies: config_deps,
            });
        }

        // Check for generic driver support
        if let Some(generic) = self.check_generic_support(vendor_id, device_id)? {
            return Ok(generic);
        }

        Ok(DeviceSupport {
            device_id: pci_id,
            driver_module: "none".to_string(),
            support_level: SupportLevel::Unsupported,
            kernel_version_added: None,
            config_dependencies: vec![],
        })
    }

    /// Check modules.alias for device support
    fn check_modules_alias(&self, device_id: &str) -> Result<Option<String>> {
        let alias_content = match fs::read_to_string(&self.modules_alias_path) {
            Ok(content) => content,
            Err(_) => {
                log::warn!("Could not read modules.alias at {}", self.modules_alias_path);
                return Ok(None);
            }
        };

        // Parse modules.alias format: alias pci:v00001B21d00000612sv*sd*bc*sc*i* ahci
        for line in alias_content.lines() {
            if line.starts_with("alias pci:") {
                if let Some((alias, _module)) = line.split_once(' ') {
                    if let Some(module) = alias.split(' ').nth(1) {
                        // Extract vendor and device IDs from alias
                        if self.matches_pci_alias(device_id, alias) {
                            return Ok(Some(module.trim().to_string()));
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    /// Check if PCI device ID matches alias pattern
    fn matches_pci_alias(&self, device_id: &str, alias: &str) -> bool {
        // Convert device_id "1b21:0612" to match alias format "v00001B21d00000612"
        if let Some((vendor, device)) = device_id.split_once(':') {
            let vendor_pattern = format!("v0000{:0>4}", vendor.to_uppercase());
            let device_pattern = format!("d0000{:0>4}", device.to_uppercase());
            
            alias.contains(&vendor_pattern) && alias.contains(&device_pattern)
        } else {
            false
        }
    }

    /// Check for generic driver support (USB HID, mass storage, etc.)
    fn check_generic_support(&self, _vendor_id: &str, _device_id: &str) -> Result<Option<DeviceSupport>> {
        // USB mass storage: class 08
        // USB HID: class 03
        // This would need to be expanded with actual class checking from sysfs
        
        // Placeholder implementation - in real version we'd check device class
        Ok(None)
    }

    /// Get kernel configuration dependencies for a module
    fn get_config_dependencies(&self, module: &str) -> Result<Vec<String>> {
        // This would parse modinfo output for dependencies and map to CONFIG_ options
        let output = Command::new("modinfo")
            .arg(module)
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let info = String::from_utf8_lossy(&output.stdout);
                let mut deps = Vec::new();
                
                for line in info.lines() {
                    if line.starts_with("depends:") {
                        let depends = line.strip_prefix("depends:").unwrap_or("").trim();
                        if !depends.is_empty() {
                            deps.extend(depends.split(',').map(|s| s.trim().to_string()));
                        }
                    }
                }
                
                return Ok(deps);
            }
        }

        Ok(vec![])
    }

    /// Check if a specific kernel configuration option is enabled
    pub fn check_config_option(&self, config_name: &str) -> Result<bool> {
        if let Some(ref config_path) = self.config_path {
            let config_content = fs::read_to_string(config_path)
                .map_err(|_| LxHwError::ConfigError("Could not read kernel config".to_string()))?;

            // Look for CONFIG_OPTION=y or CONFIG_OPTION=m
            let search_pattern = format!("{}=", config_name);
            for line in config_content.lines() {
                if line.starts_with(&search_pattern) {
                    return Ok(line.ends_with("=y") || line.ends_with("=m"));
                }
            }
        }
        
        // If no config file or option not found, assume it could be enabled
        Ok(false)
    }

    /// Get comprehensive support data for all detected devices
    pub fn get_support_data(&self, device_ids: Vec<(String, String)>) -> Result<KernelSupportData> {
        let mut supported_devices = Vec::new();
        let mut module_aliases = HashMap::new();

        for (vendor_id, device_id) in device_ids {
            let support = self.verify_pci_support(&vendor_id, &device_id)?;
            
            // Group by module for aliases
            let module = support.driver_module.clone();
            module_aliases.entry(module.clone())
                .or_insert_with(Vec::new)
                .push(format!("{}:{}", vendor_id, device_id));
                
            supported_devices.push(support);
        }

        Ok(KernelSupportData {
            kernel_version: self.kernel_version.clone(),
            supported_devices,
            module_aliases,
            config_options: HashMap::new(), // TODO: populate with relevant CONFIG_ options
        })
    }

    /// Generate user-friendly recommendations for hardware support
    pub fn generate_user_recommendations(&self, support_data: &KernelSupportData) -> UserRecommendations {
        let mut recommendations = UserRecommendations::new(self.kernel_version.clone());
        
        let mut unsupported_devices = Vec::new();
        let mut kernel_upgrade_needed = false;
        let mut missing_modules = Vec::new();
        let mut configuration_changes = Vec::new();

        for device in &support_data.supported_devices {
            match device.support_level {
                SupportLevel::Unsupported => {
                    unsupported_devices.push(device.clone());
                    
                    // Check if newer kernels might support this device
                    if let Some(upgrade_rec) = self.check_kernel_upgrade_benefit(&device.device_id) {
                        kernel_upgrade_needed = true;
                        recommendations.kernel_upgrades.push(upgrade_rec);
                    }
                },
                SupportLevel::Experimental => {
                    configuration_changes.push(UserAction {
                        action_type: ActionType::EnableExperimental,
                        description: format!("Enable experimental driver for {}", device.device_id),
                        commands: vec![format!("sudo modprobe {}", device.driver_module)],
                        risk_level: RiskLevel::Medium,
                        explanation: "This device has experimental kernel support. It may work but could be unstable.".to_string(),
                    });
                },
                SupportLevel::Supported => {
                    // Check if module is loaded
                    if !self.is_module_loaded(&device.driver_module) {
                        missing_modules.push(UserAction {
                            action_type: ActionType::LoadModule,
                            description: format!("Load driver module for {}", device.device_id),
                            commands: vec![format!("sudo modprobe {}", device.driver_module)],
                            risk_level: RiskLevel::Low,
                            explanation: "This device is supported but the driver module is not currently loaded.".to_string(),
                        });
                    }
                },
                _ => {}
            }

            // Check for missing kernel configuration
            for config_dep in &device.config_dependencies {
                if let Ok(enabled) = self.check_config_option(&format!("CONFIG_{}", config_dep.to_uppercase())) {
                    if !enabled {
                        configuration_changes.push(UserAction {
                            action_type: ActionType::ReconfigureKernel,
                            description: format!("Enable {} in kernel configuration", config_dep),
                            commands: vec![
                                "# This requires kernel recompilation".to_string(),
                                format!("# Enable CONFIG_{} in kernel config", config_dep.to_uppercase()),
                                "# Or install a kernel with this option enabled".to_string(),
                            ],
                            risk_level: RiskLevel::High,
                            explanation: format!("The {} module requires CONFIG_{} to be enabled in the kernel.", device.driver_module, config_dep.to_uppercase()),
                        });
                    }
                }
            }
        }

        recommendations.unsupported_devices = unsupported_devices.len();
        recommendations.module_actions = missing_modules;
        recommendations.configuration_actions = configuration_changes;
        recommendations.needs_kernel_upgrade = kernel_upgrade_needed;

        // Add general recommendations
        self.add_general_recommendations(&mut recommendations);

        recommendations
    }

    /// Check if a kernel module is currently loaded
    fn is_module_loaded(&self, module_name: &str) -> bool {
        std::fs::read_to_string("/proc/modules")
            .map(|content| content.lines().any(|line| line.starts_with(module_name)))
            .unwrap_or(false)
    }

    /// Check if upgrading kernel would benefit this device
    fn check_kernel_upgrade_benefit(&self, device_id: &str) -> Option<KernelUpgradeRecommendation> {
        // This is a simplified version - in a real implementation, this would
        // query a database of kernel versions and hardware support
        
        let current_version = self.parse_kernel_version(&self.kernel_version);
        
        // Example logic: if current kernel is older than 5.15 and device is unsupported,
        // recommend upgrade to latest LTS
        if current_version.0 < 5 || (current_version.0 == 5 && current_version.1 < 15) {
            return Some(KernelUpgradeRecommendation {
                device_id: device_id.to_string(),
                current_kernel: self.kernel_version.clone(),
                recommended_kernel: "6.1 LTS or newer".to_string(),
                reason: "Hardware support significantly improved in newer kernels".to_string(),
                upgrade_method: self.suggest_upgrade_method(),
                estimated_support_probability: 70, // Percentage chance of support
            });
        }

        None
    }

    /// Parse kernel version string to (major, minor) tuple
    fn parse_kernel_version(&self, version: &str) -> (u32, u32) {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() >= 2 {
            let major = parts[0].parse().unwrap_or(0);
            let minor = parts[1].parse().unwrap_or(0);
            (major, minor)
        } else {
            (0, 0)
        }
    }

    /// Suggest kernel upgrade method based on distribution
    fn suggest_upgrade_method(&self) -> Vec<String> {
        // Detect distribution and suggest appropriate upgrade method
        if std::path::Path::new("/etc/debian_version").exists() {
            vec![
                "# For Ubuntu/Debian:".to_string(),
                "sudo apt update".to_string(),
                "sudo apt install linux-generic-hwe-22.04".to_string(),
                "# Or for latest mainline:".to_string(),
                "# Use mainline kernel PPA or download from kernel.org".to_string(),
            ]
        } else if std::path::Path::new("/etc/redhat-release").exists() {
            vec![
                "# For RHEL/CentOS/Fedora:".to_string(),
                "sudo dnf update kernel".to_string(),
                "# Or enable newer kernel streams:".to_string(),
                "sudo dnf install kernel-ml".to_string(),
            ]
        } else if std::path::Path::new("/etc/arch-release").exists() {
            vec![
                "# For Arch Linux:".to_string(),
                "sudo pacman -Syu linux".to_string(),
                "# Or install LTS kernel:".to_string(),
                "sudo pacman -S linux-lts".to_string(),
            ]
        } else {
            vec![
                "# Generic kernel upgrade:".to_string(),
                "# Check your distribution's documentation".to_string(),
                "# Or compile from source at kernel.org".to_string(),
            ]
        }
    }

    /// Add general system recommendations
    fn add_general_recommendations(&self, recommendations: &mut UserRecommendations) {
        // Check system health indicators
        let kernel_age = self.estimate_kernel_age(&self.kernel_version);
        
        if kernel_age > 365 { // Older than 1 year
            recommendations.general_advice.push(
                "Consider upgrading to a newer kernel for better hardware support and security updates.".to_string()
            );
        }

        // Check for common issues
        if !std::path::Path::new("/sys/firmware/efi").exists() {
            recommendations.general_advice.push(
                "System appears to be using legacy BIOS. Consider UEFI for better hardware support.".to_string()
            );
        }

        // Check for virtualization
        if self.is_virtualized() {
            recommendations.general_advice.push(
                "Running in virtualized environment. Some hardware may be emulated or not directly accessible.".to_string()
            );
        }
    }

    /// Estimate kernel age in days (simplified)
    fn estimate_kernel_age(&self, kernel_version: &str) -> u32 {
        // This is a simplified estimation - real implementation would use a database
        let version = self.parse_kernel_version(kernel_version);
        match version.0 {
            6 => 30,   // Very recent
            5 => 200,  // Recent
            4 => 800,  // Old
            _ => 1000, // Very old
        }
    }

    /// Check if system is virtualized
    fn is_virtualized(&self) -> bool {
        std::fs::read_to_string("/proc/cpuinfo")
            .map(|content| content.contains("hypervisor") || content.contains("QEMU"))
            .unwrap_or(false)
    }

    /// Extract device IDs from sysfs for verification
    pub fn extract_system_device_ids(&self) -> Result<Vec<(String, String)>> {
        let mut device_ids = Vec::new();
        let pci_devices_path = "/sys/bus/pci/devices";

        if !Path::new(pci_devices_path).exists() {
            return Ok(device_ids);
        }

        let entries = fs::read_dir(pci_devices_path)
            .map_err(LxHwError::IoError)?;

        for entry in entries {
            let entry = entry.map_err(LxHwError::IoError)?;
            let device_path = entry.path();
            
            // Read vendor and device files
            let vendor_path = device_path.join("vendor");
            let device_id_path = device_path.join("device");
            
            if let (Ok(vendor), Ok(device)) = (
                fs::read_to_string(&vendor_path),
                fs::read_to_string(&device_id_path)
            ) {
                let vendor = vendor.trim().strip_prefix("0x").unwrap_or(&vendor).trim();
                let device = device.trim().strip_prefix("0x").unwrap_or(&device).trim();
                device_ids.push((vendor.to_string(), device.to_string()));
            }
        }

        Ok(device_ids)
    }
}

impl Default for KernelSupportData {
    fn default() -> Self {
        Self {
            kernel_version: "unknown".to_string(),
            supported_devices: Vec::new(),
            module_aliases: HashMap::new(),
            config_options: HashMap::new(),
        }
    }
}

/// User recommendations for hardware support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRecommendations {
    pub kernel_version: String,
    pub total_devices: usize,
    pub supported_devices: usize,
    pub unsupported_devices: usize,
    pub needs_kernel_upgrade: bool,
    pub kernel_upgrades: Vec<KernelUpgradeRecommendation>,
    pub module_actions: Vec<UserAction>,
    pub configuration_actions: Vec<UserAction>,
    pub general_advice: Vec<String>,
}

/// Kernel upgrade recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelUpgradeRecommendation {
    pub device_id: String,
    pub current_kernel: String,
    pub recommended_kernel: String,
    pub reason: String,
    pub upgrade_method: Vec<String>,
    pub estimated_support_probability: u8, // 0-100%
}

/// User action recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAction {
    pub action_type: ActionType,
    pub description: String,
    pub commands: Vec<String>,
    pub risk_level: RiskLevel,
    pub explanation: String,
}

/// Types of actions users can take
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    LoadModule,
    EnableExperimental,
    ReconfigureKernel,
    UpgradeKernel,
    InstallFirmware,
}

/// Risk level for user actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,    // Safe, reversible
    Medium, // Some risk, usually reversible
    High,   // Significant risk, may require recovery
}

impl UserRecommendations {
    pub fn new(kernel_version: String) -> Self {
        Self {
            kernel_version,
            total_devices: 0,
            supported_devices: 0,
            unsupported_devices: 0,
            needs_kernel_upgrade: false,
            kernel_upgrades: Vec::new(),
            module_actions: Vec::new(),
            configuration_actions: Vec::new(),
            general_advice: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pci_alias_matching() {
        let verifier = KernelSupportVerifier {
            kernel_version: "test".to_string(),
            modules_alias_path: "/test".to_string(),
            config_path: None,
        };

        // Test vendor:device format matching
        assert!(verifier.matches_pci_alias(
            "1b21:0612", 
            "alias pci:v00001B21d00000612sv*sd*bc*sc*i* ahci"
        ));
        
        assert!(!verifier.matches_pci_alias(
            "1234:5678", 
            "alias pci:v00001B21d00000612sv*sd*bc*sc*i* ahci"
        ));
    }

    #[test]
    fn test_support_level_serialization() {
        let support = DeviceSupport {
            device_id: "1234:5678".to_string(),
            driver_module: "test".to_string(),
            support_level: SupportLevel::Supported,
            kernel_version_added: Some("5.15".to_string()),
            config_dependencies: vec!["USB_SUPPORT".to_string()],
        };

        let serialized = serde_json::to_string(&support).unwrap();
        let deserialized: DeviceSupport = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(support.device_id, deserialized.device_id);
        assert_eq!(support.driver_module, deserialized.driver_module);
    }
}