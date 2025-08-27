use std::collections::HashMap;
use crate::configuration::*;
use crate::hardware::HardwareReport;
use crate::errors::LxHwError;

pub struct KernelParameterGenerator {
    parameter_rules: HashMap<String, Vec<ParameterRule>>,
    hardware_optimizations: HashMap<String, Vec<OptimizationRule>>,
    distribution_specifics: HashMap<String, DistributionConfig>,
}

#[derive(Debug, Clone)]
pub struct ParameterRule {
    pub parameter_name: String,
    pub default_value: Option<String>,
    pub conditions: Vec<HardwareCondition>,
    pub purpose: String,
    pub boot_order: u8,
    pub risk_level: RiskLevel,
}

#[derive(Debug, Clone)]
pub struct OptimizationRule {
    pub optimization_type: String,
    pub parameters: Vec<KernelParameter>,
    pub hardware_requirements: Vec<HardwareCondition>,
    pub expected_benefit: String,
}

#[derive(Debug, Clone)]
pub struct HardwareCondition {
    pub component_type: String,
    pub vendor_pattern: Option<String>,
    pub model_pattern: Option<String>,
    pub feature_required: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DistributionConfig {
    pub distribution_name: String,
    pub bootloader_type: String,
    pub config_file_path: String,
    pub parameter_prefix: String,
}

impl KernelParameterGenerator {
    pub fn new() -> Result<Self, LxHwError> {
        let mut generator = Self {
            parameter_rules: HashMap::new(),
            hardware_optimizations: HashMap::new(),
            distribution_specifics: HashMap::new(),
        };

        generator.initialize_parameter_rules()?;
        generator.initialize_hardware_optimizations()?;
        generator.initialize_distribution_configs()?;

        Ok(generator)
    }

    pub fn generate_parameters(&self, hardware: &HardwareReport) -> Result<Vec<KernelParameter>, LxHwError> {
        let mut parameters = Vec::new();

        // Generate parameters based on detected hardware
        parameters.extend(self.generate_cpu_parameters(hardware)?);
        parameters.extend(self.generate_gpu_parameters(hardware)?);
        parameters.extend(self.generate_memory_parameters(hardware)?);
        parameters.extend(self.generate_storage_parameters(hardware)?);
        parameters.extend(self.generate_network_parameters(hardware)?);
        parameters.extend(self.generate_power_management_parameters(hardware)?);
        parameters.extend(self.generate_security_parameters(hardware)?);

        // Apply distribution-specific adjustments
        let default_distribution = "unknown".to_string();
        let distribution = hardware.system.distribution.as_ref().unwrap_or(&default_distribution);
        if let Some(distribution_config) = self.distribution_specifics.get(distribution) {
            parameters = self.apply_distribution_adjustments(parameters, distribution_config)?;
        }

        // Sort by boot order priority
        parameters.sort_by_key(|p| p.boot_order);

        Ok(parameters)
    }

    fn initialize_parameter_rules(&mut self) -> Result<(), LxHwError> {
        // CPU-related parameters
        let mut cpu_rules = Vec::new();
        
        cpu_rules.push(ParameterRule {
            parameter_name: "intel_pstate".to_string(),
            default_value: Some("enable".to_string()),
            conditions: vec![HardwareCondition {
                component_type: "CPU".to_string(),
                vendor_pattern: Some("Intel".to_string()),
                model_pattern: None,
                feature_required: None,
            }],
            purpose: "Enable Intel P-State driver for better power management".to_string(),
            boot_order: 1,
            risk_level: RiskLevel::Low,
        });

        cpu_rules.push(ParameterRule {
            parameter_name: "amd_pstate".to_string(),
            default_value: Some("active".to_string()),
            conditions: vec![HardwareCondition {
                component_type: "CPU".to_string(),
                vendor_pattern: Some("AMD".to_string()),
                model_pattern: None,
                feature_required: None,
            }],
            purpose: "Enable AMD P-State driver for modern AMD CPUs".to_string(),
            boot_order: 1,
            risk_level: RiskLevel::Low,
        });

        cpu_rules.push(ParameterRule {
            parameter_name: "mitigations".to_string(),
            default_value: Some("auto".to_string()),
            conditions: vec![HardwareCondition {
                component_type: "CPU".to_string(),
                vendor_pattern: None,
                model_pattern: None,
                feature_required: None,
            }],
            purpose: "Control CPU vulnerability mitigations".to_string(),
            boot_order: 2,
            risk_level: RiskLevel::Medium,
        });

        self.parameter_rules.insert("cpu".to_string(), cpu_rules);

        // GPU-related parameters
        let mut gpu_rules = Vec::new();
        
        gpu_rules.push(ParameterRule {
            parameter_name: "nouveau.modeset".to_string(),
            default_value: Some("0".to_string()),
            conditions: vec![HardwareCondition {
                component_type: "GPU".to_string(),
                vendor_pattern: Some("NVIDIA".to_string()),
                model_pattern: None,
                feature_required: None,
            }],
            purpose: "Disable nouveau for proprietary NVIDIA driver".to_string(),
            boot_order: 3,
            risk_level: RiskLevel::Low,
        });

        gpu_rules.push(ParameterRule {
            parameter_name: "i915.enable_psr".to_string(),
            default_value: Some("0".to_string()),
            conditions: vec![HardwareCondition {
                component_type: "GPU".to_string(),
                vendor_pattern: Some("Intel".to_string()),
                model_pattern: None,
                feature_required: None,
            }],
            purpose: "Disable Panel Self Refresh to fix display issues".to_string(),
            boot_order: 4,
            risk_level: RiskLevel::Low,
        });

        gpu_rules.push(ParameterRule {
            parameter_name: "amdgpu.dc".to_string(),
            default_value: Some("1".to_string()),
            conditions: vec![HardwareCondition {
                component_type: "GPU".to_string(),
                vendor_pattern: Some("AMD".to_string()),
                model_pattern: None,
                feature_required: None,
            }],
            purpose: "Enable Display Core for modern AMD GPUs".to_string(),
            boot_order: 4,
            risk_level: RiskLevel::Low,
        });

        self.parameter_rules.insert("gpu".to_string(), gpu_rules);

        // Memory-related parameters
        let mut memory_rules = Vec::new();
        
        memory_rules.push(ParameterRule {
            parameter_name: "transparent_hugepage".to_string(),
            default_value: Some("madvise".to_string()),
            conditions: vec![HardwareCondition {
                component_type: "Memory".to_string(),
                vendor_pattern: None,
                model_pattern: None,
                feature_required: None,
            }],
            purpose: "Optimize memory allocation with transparent hugepages".to_string(),
            boot_order: 5,
            risk_level: RiskLevel::Low,
        });

        self.parameter_rules.insert("memory".to_string(), memory_rules);

        // Storage-related parameters
        let mut storage_rules = Vec::new();
        
        storage_rules.push(ParameterRule {
            parameter_name: "elevator".to_string(),
            default_value: Some("none".to_string()),
            conditions: vec![HardwareCondition {
                component_type: "Storage".to_string(),
                vendor_pattern: None,
                model_pattern: Some("SSD".to_string()),
                feature_required: None,
            }],
            purpose: "Disable I/O scheduler for SSDs".to_string(),
            boot_order: 6,
            risk_level: RiskLevel::Low,
        });

        self.parameter_rules.insert("storage".to_string(), storage_rules);

        Ok(())
    }

    fn initialize_hardware_optimizations(&mut self) -> Result<(), LxHwError> {
        // Gaming optimizations
        let gaming_optimizations = vec![
            OptimizationRule {
                optimization_type: "gaming".to_string(),
                parameters: vec![
                    KernelParameter {
                        parameter: "preempt".to_string(),
                        value: Some("voluntary".to_string()),
                        purpose: "Reduce latency for gaming".to_string(),
                        hardware_target: Some("CPU".to_string()),
                        distribution_specific: None,
                        boot_order: 2,
                    },
                    KernelParameter {
                        parameter: "clocksource".to_string(),
                        value: Some("tsc".to_string()),
                        purpose: "Use TSC for precise timing".to_string(),
                        hardware_target: Some("CPU".to_string()),
                        distribution_specific: None,
                        boot_order: 3,
                    },
                ],
                hardware_requirements: vec![
                    HardwareCondition {
                        component_type: "GPU".to_string(),
                        vendor_pattern: Some("NVIDIA|AMD".to_string()),
                        model_pattern: None,
                        feature_required: None,
                    },
                ],
                expected_benefit: "Lower input latency and smoother gaming performance".to_string(),
            },
        ];

        self.hardware_optimizations.insert("gaming".to_string(), gaming_optimizations);

        // Server optimizations
        let server_optimizations = vec![
            OptimizationRule {
                optimization_type: "server".to_string(),
                parameters: vec![
                    KernelParameter {
                        parameter: "nohz_full".to_string(),
                        value: Some("1-3".to_string()),
                        purpose: "Isolate CPU cores from kernel ticks".to_string(),
                        hardware_target: Some("CPU".to_string()),
                        distribution_specific: None,
                        boot_order: 1,
                    },
                    KernelParameter {
                        parameter: "rcu_nocbs".to_string(),
                        value: Some("1-3".to_string()),
                        purpose: "Move RCU callbacks off isolated cores".to_string(),
                        hardware_target: Some("CPU".to_string()),
                        distribution_specific: None,
                        boot_order: 1,
                    },
                ],
                hardware_requirements: vec![
                    HardwareCondition {
                        component_type: "CPU".to_string(),
                        vendor_pattern: None,
                        model_pattern: None,
                        feature_required: Some("cores >= 4".to_string()),
                    },
                ],
                expected_benefit: "Better CPU isolation for high-performance workloads".to_string(),
            },
        ];

        self.hardware_optimizations.insert("server".to_string(), server_optimizations);

        Ok(())
    }

    fn initialize_distribution_configs(&mut self) -> Result<(), LxHwError> {
        self.distribution_specifics.insert("Ubuntu".to_string(), DistributionConfig {
            distribution_name: "Ubuntu".to_string(),
            bootloader_type: "grub".to_string(),
            config_file_path: "/etc/default/grub".to_string(),
            parameter_prefix: "GRUB_CMDLINE_LINUX_DEFAULT=".to_string(),
        });

        self.distribution_specifics.insert("Debian".to_string(), DistributionConfig {
            distribution_name: "Debian".to_string(),
            bootloader_type: "grub".to_string(),
            config_file_path: "/etc/default/grub".to_string(),
            parameter_prefix: "GRUB_CMDLINE_LINUX_DEFAULT=".to_string(),
        });

        self.distribution_specifics.insert("Fedora".to_string(), DistributionConfig {
            distribution_name: "Fedora".to_string(),
            bootloader_type: "grub".to_string(),
            config_file_path: "/etc/default/grub".to_string(),
            parameter_prefix: "GRUB_CMDLINE_LINUX=".to_string(),
        });

        self.distribution_specifics.insert("Arch Linux".to_string(), DistributionConfig {
            distribution_name: "Arch Linux".to_string(),
            bootloader_type: "systemd-boot".to_string(),
            config_file_path: "/boot/loader/entries/arch.conf".to_string(),
            parameter_prefix: "options ".to_string(),
        });

        self.distribution_specifics.insert("NixOS".to_string(), DistributionConfig {
            distribution_name: "NixOS".to_string(),
            bootloader_type: "systemd-boot".to_string(),
            config_file_path: "/etc/nixos/configuration.nix".to_string(),
            parameter_prefix: "boot.kernelParams = [".to_string(),
        });

        Ok(())
    }

    fn generate_cpu_parameters(&self, hardware: &HardwareReport) -> Result<Vec<KernelParameter>, LxHwError> {
        let mut parameters = Vec::new();

        if let Some(cpu_info) = &hardware.cpu {
            let vendor = &cpu_info.vendor;
            let vendor_lower = vendor.to_lowercase();

            // Intel-specific parameters
            if vendor_lower.contains("intel") {
                parameters.push(KernelParameter {
                    parameter: "intel_pstate".to_string(),
                    value: Some("enable".to_string()),
                    purpose: "Enable Intel P-State driver for better power management".to_string(),
                    hardware_target: Some("Intel CPU".to_string()),
                    distribution_specific: None,
                    boot_order: 1,
                });

                parameters.push(KernelParameter {
                    parameter: "intel_iommu".to_string(),
                    value: Some("on".to_string()),
                    purpose: "Enable Intel IOMMU for virtualization and security".to_string(),
                    hardware_target: Some("Intel CPU".to_string()),
                    distribution_specific: None,
                    boot_order: 2,
                });
            }

            // AMD-specific parameters
            if vendor_lower.contains("amd") {
                parameters.push(KernelParameter {
                    parameter: "amd_pstate".to_string(),
                    value: Some("active".to_string()),
                    purpose: "Enable AMD P-State driver for modern AMD CPUs".to_string(),
                    hardware_target: Some("AMD CPU".to_string()),
                    distribution_specific: None,
                    boot_order: 1,
                });

                parameters.push(KernelParameter {
                    parameter: "amd_iommu".to_string(),
                    value: Some("on".to_string()),
                    purpose: "Enable AMD IOMMU for virtualization and security".to_string(),
                    hardware_target: Some("AMD CPU".to_string()),
                    distribution_specific: None,
                    boot_order: 2,
                });
            }

            // General CPU security mitigations
            parameters.push(KernelParameter {
                parameter: "mitigations".to_string(),
                value: Some("auto".to_string()),
                purpose: "Enable automatic CPU vulnerability mitigations".to_string(),
                hardware_target: Some("CPU".to_string()),
                distribution_specific: None,
                boot_order: 3,
            });
        }

        Ok(parameters)
    }

    fn generate_gpu_parameters(&self, hardware: &HardwareReport) -> Result<Vec<KernelParameter>, LxHwError> {
        let mut parameters = Vec::new();

        for gpu in &hardware.graphics {
            let vendor = &gpu.vendor;
            let vendor_lower = vendor.to_lowercase();

            if vendor_lower.contains("nvidia") {
                parameters.push(KernelParameter {
                    parameter: "nouveau.modeset".to_string(),
                    value: Some("0".to_string()),
                    purpose: "Disable nouveau driver for NVIDIA proprietary driver".to_string(),
                    hardware_target: Some("NVIDIA GPU".to_string()),
                    distribution_specific: None,
                    boot_order: 4,
                });

                parameters.push(KernelParameter {
                    parameter: "rd.driver.blacklist".to_string(),
                    value: Some("nouveau".to_string()),
                    purpose: "Blacklist nouveau driver in initramfs".to_string(),
                    hardware_target: Some("NVIDIA GPU".to_string()),
                    distribution_specific: None,
                    boot_order: 4,
                });
            } else if vendor_lower.contains("intel") {
                parameters.push(KernelParameter {
                    parameter: "i915.enable_psr".to_string(),
                    value: Some("0".to_string()),
                    purpose: "Disable Panel Self Refresh to prevent display issues".to_string(),
                    hardware_target: Some("Intel GPU".to_string()),
                    distribution_specific: None,
                    boot_order: 5,
                });

                parameters.push(KernelParameter {
                    parameter: "i915.fastboot".to_string(),
                    value: Some("1".to_string()),
                    purpose: "Enable fast boot for Intel graphics".to_string(),
                    hardware_target: Some("Intel GPU".to_string()),
                    distribution_specific: None,
                    boot_order: 5,
                });
            } else if vendor_lower.contains("amd") {
                parameters.push(KernelParameter {
                    parameter: "amdgpu.dc".to_string(),
                    value: Some("1".to_string()),
                    purpose: "Enable Display Core for modern AMD GPUs".to_string(),
                    hardware_target: Some("AMD GPU".to_string()),
                    distribution_specific: None,
                    boot_order: 5,
                });
            }
        }

        Ok(parameters)
    }

    fn generate_memory_parameters(&self, hardware: &HardwareReport) -> Result<Vec<KernelParameter>, LxHwError> {
        let mut parameters = Vec::new();

        // General memory optimizations
        parameters.push(KernelParameter {
            parameter: "transparent_hugepage".to_string(),
            value: Some("madvise".to_string()),
            purpose: "Use transparent hugepages only when requested by applications".to_string(),
            hardware_target: Some("Memory".to_string()),
            distribution_specific: None,
            boot_order: 6,
        });

        // Check if we have memory information to make smarter decisions
        if let Some(_memory_info) = &hardware.memory {
            parameters.push(KernelParameter {
                parameter: "vm.swappiness".to_string(),
                value: Some("10".to_string()),
                purpose: "Reduce swappiness for better responsiveness".to_string(),
                hardware_target: Some("Memory".to_string()),
                distribution_specific: None,
                boot_order: 6,
            });
        }

        Ok(parameters)
    }

    fn generate_storage_parameters(&self, hardware: &HardwareReport) -> Result<Vec<KernelParameter>, LxHwError> {
        let mut parameters = Vec::new();

        // Check for SSD/NVMe drives
        for storage in &hardware.storage {
            let product = &storage.model;
            let product_lower = product.to_lowercase();

            if product_lower.contains("ssd") || product_lower.contains("nvme") {
                parameters.push(KernelParameter {
                    parameter: "elevator".to_string(),
                    value: Some("none".to_string()),
                    purpose: "Disable I/O scheduler for SSD/NVMe drives".to_string(),
                    hardware_target: Some("SSD/NVMe".to_string()),
                    distribution_specific: None,
                    boot_order: 7,
                });
                break; // Only need to set this once
            }
        }

        Ok(parameters)
    }

    fn generate_network_parameters(&self, hardware: &HardwareReport) -> Result<Vec<KernelParameter>, LxHwError> {
        let mut parameters = Vec::new();

        // Check for Intel wireless cards that may need specific parameters
        for network in &hardware.network {
            let vendor = &network.vendor;
            let product = &network.model;
            
            let vendor_lower = vendor.to_lowercase();
            let product_lower = product.to_lowercase();

            if vendor_lower.contains("intel") && (product_lower.contains("wireless") || product_lower.contains("wifi")) {
                parameters.push(KernelParameter {
                    parameter: "iwlwifi.power_save".to_string(),
                    value: Some("0".to_string()),
                    purpose: "Disable power saving for Intel wireless to improve stability".to_string(),
                    hardware_target: Some("Intel Wireless".to_string()),
                    distribution_specific: None,
                    boot_order: 8,
                });
            }
        }

        Ok(parameters)
    }

    fn generate_power_management_parameters(&self, _hardware: &HardwareReport) -> Result<Vec<KernelParameter>, LxHwError> {
        let mut parameters = Vec::new();

        // General power management parameters
        parameters.push(KernelParameter {
            parameter: "pcie_aspm".to_string(),
            value: Some("off".to_string()),
            purpose: "Disable PCIe Active State Power Management to prevent issues".to_string(),
            hardware_target: Some("PCIe".to_string()),
            distribution_specific: None,
            boot_order: 9,
        });

        Ok(parameters)
    }

    fn generate_security_parameters(&self, _hardware: &HardwareReport) -> Result<Vec<KernelParameter>, LxHwError> {
        let mut parameters = Vec::new();

        // Enable KASLR by default
        parameters.push(KernelParameter {
            parameter: "kaslr".to_string(),
            value: None,
            purpose: "Enable Kernel Address Space Layout Randomization".to_string(),
            hardware_target: None,
            distribution_specific: None,
            boot_order: 10,
        });

        Ok(parameters)
    }

    fn apply_distribution_adjustments(&self, parameters: Vec<KernelParameter>, _distribution_config: &DistributionConfig) -> Result<Vec<KernelParameter>, LxHwError> {
        // Apply any distribution-specific parameter adjustments
        // For now, we'll just return the parameters as-is
        // In the future, this could modify parameters based on distribution quirks
        
        Ok(parameters)
    }
}