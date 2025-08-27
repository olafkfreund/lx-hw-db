use std::collections::HashMap;
use serde_json::Value;
use crate::configuration::*;
use crate::hardware::{HardwareReport, DeviceCompatibility};
use crate::errors::LxHwError;

pub struct DriverMapper {
    driver_database: DriverDatabase,
    vendor_mappings: HashMap<String, VendorInfo>,
    device_class_mappings: HashMap<String, DeviceClassInfo>,
}

#[derive(Debug, Clone)]
pub struct DriverDatabase {
    pub pci_drivers: HashMap<String, DriverInfo>,
    pub usb_drivers: HashMap<String, DriverInfo>,
    pub generic_drivers: HashMap<String, DriverInfo>,
}

#[derive(Debug, Clone)]
pub struct DriverInfo {
    pub driver_name: String,
    pub driver_type: DriverType,
    pub kernel_modules: Vec<String>,
    pub firmware_files: Vec<String>,
    pub package_names: HashMap<String, String>, // distribution -> package name
    pub configuration_hints: Vec<String>,
    pub known_issues: Vec<String>,
    pub alternatives: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum DriverType {
    InKernel,
    External,
    Proprietary,
    Dkms,
}

#[derive(Debug, Clone)]
pub struct VendorInfo {
    pub vendor_name: String,
    pub vendor_id: String,
    pub typical_drivers: Vec<String>,
    pub firmware_prefix: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DeviceClassInfo {
    pub class_name: String,
    pub class_code: String,
    pub common_drivers: Vec<String>,
    pub subsystem: String,
}

impl DriverMapper {
    pub fn new() -> Result<Self, LxHwError> {
        let mut mapper = Self {
            driver_database: DriverDatabase {
                pci_drivers: HashMap::new(),
                usb_drivers: HashMap::new(),
                generic_drivers: HashMap::new(),
            },
            vendor_mappings: HashMap::new(),
            device_class_mappings: HashMap::new(),
        };

        mapper.initialize_driver_database()?;
        mapper.initialize_vendor_mappings()?;
        mapper.initialize_device_class_mappings()?;

        Ok(mapper)
    }

    pub fn map_drivers(&self, hardware: &HardwareReport) -> Result<Vec<DriverRecommendation>, LxHwError> {
        let mut recommendations = Vec::new();

        // Map drivers for CPU
        if let Some(cpu) = &hardware.cpu {
            recommendations.extend(self.map_cpu_drivers_from_device(cpu)?);
        }

        // Map drivers for graphics
        for gpu in &hardware.graphics {
            recommendations.extend(self.map_gpu_drivers_from_device(gpu)?);
        }

        // Map drivers for network devices
        for network in &hardware.network {
            recommendations.extend(self.map_network_drivers_from_device(network)?);
        }

        // Map drivers for audio devices
        for audio in &hardware.audio {
            recommendations.extend(self.map_audio_drivers_from_device(audio)?);
        }

        // Map drivers from kernel support information
        if let Some(kernel_support) = &hardware.kernel_support {
            for device in &kernel_support.device_support_details {
                recommendations.extend(self.map_kernel_device_drivers(device)?);
            }
        }

        Ok(recommendations)
    }

    fn initialize_driver_database(&mut self) -> Result<(), LxHwError> {
        // Initialize GPU drivers
        self.driver_database.pci_drivers.insert("nvidia".to_string(), DriverInfo {
            driver_name: "nvidia".to_string(),
            driver_type: DriverType::Proprietary,
            kernel_modules: vec!["nvidia".to_string(), "nvidia_modeset".to_string(), "nvidia_uvm".to_string()],
            firmware_files: vec![],
            package_names: {
                let mut packages = HashMap::new();
                packages.insert("ubuntu".to_string(), "nvidia-driver-470".to_string());
                packages.insert("debian".to_string(), "nvidia-driver".to_string());
                packages.insert("fedora".to_string(), "akmod-nvidia".to_string());
                packages.insert("arch".to_string(), "nvidia".to_string());
                packages.insert("nixos".to_string(), "nvidia_x11".to_string());
                packages
            },
            configuration_hints: vec![
                "Add nvidia to /etc/modules".to_string(),
                "Configure X11 to use nvidia driver".to_string(),
            ],
            known_issues: vec![
                "May conflict with nouveau driver".to_string(),
                "Requires specific kernel version compatibility".to_string(),
            ],
            alternatives: vec!["nouveau".to_string()],
        });

        self.driver_database.pci_drivers.insert("nouveau".to_string(), DriverInfo {
            driver_name: "nouveau".to_string(),
            driver_type: DriverType::InKernel,
            kernel_modules: vec!["nouveau".to_string()],
            firmware_files: vec![],
            package_names: HashMap::new(), // In-kernel driver
            configuration_hints: vec!["Usually works out of the box".to_string()],
            known_issues: vec!["Limited performance compared to proprietary nvidia driver".to_string()],
            alternatives: vec!["nvidia".to_string()],
        });

        // Initialize AMD GPU drivers
        self.driver_database.pci_drivers.insert("amdgpu".to_string(), DriverInfo {
            driver_name: "amdgpu".to_string(),
            driver_type: DriverType::InKernel,
            kernel_modules: vec!["amdgpu".to_string()],
            firmware_files: vec!["amdgpu".to_string()],
            package_names: {
                let mut packages = HashMap::new();
                packages.insert("ubuntu".to_string(), "firmware-amd-graphics".to_string());
                packages.insert("debian".to_string(), "firmware-amd-graphics".to_string());
                packages.insert("fedora".to_string(), "amd-gpu-firmware".to_string());
                packages.insert("arch".to_string(), "linux-firmware".to_string());
                packages.insert("nixos".to_string(), "linux-firmware".to_string());
                packages
            },
            configuration_hints: vec!["Ensure AMD firmware is installed".to_string()],
            known_issues: vec![],
            alternatives: vec!["radeon".to_string()],
        });

        // Initialize Intel GPU drivers
        self.driver_database.pci_drivers.insert("i915".to_string(), DriverInfo {
            driver_name: "i915".to_string(),
            driver_type: DriverType::InKernel,
            kernel_modules: vec!["i915".to_string()],
            firmware_files: vec!["i915".to_string()],
            package_names: {
                let mut packages = HashMap::new();
                packages.insert("ubuntu".to_string(), "intel-microcode".to_string());
                packages.insert("debian".to_string(), "intel-microcode".to_string());
                packages.insert("fedora".to_string(), "intel-gpu-firmware".to_string());
                packages.insert("arch".to_string(), "intel-ucode".to_string());
                packages.insert("nixos".to_string(), "intel-microcode".to_string());
                packages
            },
            configuration_hints: vec!["Enable early microcode loading".to_string()],
            known_issues: vec![],
            alternatives: vec!["xe".to_string()],
        });

        // Initialize network drivers
        self.driver_database.pci_drivers.insert("e1000e".to_string(), DriverInfo {
            driver_name: "e1000e".to_string(),
            driver_type: DriverType::InKernel,
            kernel_modules: vec!["e1000e".to_string()],
            firmware_files: vec![],
            package_names: HashMap::new(),
            configuration_hints: vec!["Usually works out of the box".to_string()],
            known_issues: vec![],
            alternatives: vec![],
        });

        self.driver_database.pci_drivers.insert("iwlwifi".to_string(), DriverInfo {
            driver_name: "iwlwifi".to_string(),
            driver_type: DriverType::InKernel,
            kernel_modules: vec!["iwlwifi".to_string()],
            firmware_files: vec!["iwlwifi".to_string()],
            package_names: {
                let mut packages = HashMap::new();
                packages.insert("ubuntu".to_string(), "linux-firmware".to_string());
                packages.insert("debian".to_string(), "firmware-iwlwifi".to_string());
                packages.insert("fedora".to_string(), "iwl*-firmware".to_string());
                packages.insert("arch".to_string(), "linux-firmware".to_string());
                packages.insert("nixos".to_string(), "linux-firmware".to_string());
                packages
            },
            configuration_hints: vec!["Ensure Intel wireless firmware is installed".to_string()],
            known_issues: vec!["Some cards require specific firmware versions".to_string()],
            alternatives: vec![],
        });

        Ok(())
    }

    fn initialize_vendor_mappings(&mut self) -> Result<(), LxHwError> {
        self.vendor_mappings.insert("10de".to_string(), VendorInfo {
            vendor_name: "NVIDIA Corporation".to_string(),
            vendor_id: "10de".to_string(),
            typical_drivers: vec!["nvidia".to_string(), "nouveau".to_string()],
            firmware_prefix: None,
        });

        self.vendor_mappings.insert("1002".to_string(), VendorInfo {
            vendor_name: "Advanced Micro Devices".to_string(),
            vendor_id: "1002".to_string(),
            typical_drivers: vec!["amdgpu".to_string(), "radeon".to_string()],
            firmware_prefix: Some("amdgpu".to_string()),
        });

        self.vendor_mappings.insert("8086".to_string(), VendorInfo {
            vendor_name: "Intel Corporation".to_string(),
            vendor_id: "8086".to_string(),
            typical_drivers: vec!["i915".to_string(), "e1000e".to_string(), "iwlwifi".to_string()],
            firmware_prefix: Some("intel".to_string()),
        });

        self.vendor_mappings.insert("1022".to_string(), VendorInfo {
            vendor_name: "Advanced Micro Devices".to_string(),
            vendor_id: "1022".to_string(),
            typical_drivers: vec!["amd64_edac".to_string(), "k10temp".to_string()],
            firmware_prefix: Some("amd".to_string()),
        });

        Ok(())
    }

    fn initialize_device_class_mappings(&mut self) -> Result<(), LxHwError> {
        self.device_class_mappings.insert("0300".to_string(), DeviceClassInfo {
            class_name: "VGA compatible controller".to_string(),
            class_code: "0300".to_string(),
            common_drivers: vec!["nvidia".to_string(), "nouveau".to_string(), "amdgpu".to_string(), "i915".to_string()],
            subsystem: "pci".to_string(),
        });

        self.device_class_mappings.insert("0200".to_string(), DeviceClassInfo {
            class_name: "Ethernet controller".to_string(),
            class_code: "0200".to_string(),
            common_drivers: vec!["e1000e".to_string(), "r8169".to_string(), "bnx2".to_string()],
            subsystem: "pci".to_string(),
        });

        self.device_class_mappings.insert("0280".to_string(), DeviceClassInfo {
            class_name: "Network controller".to_string(),
            class_code: "0280".to_string(),
            common_drivers: vec!["iwlwifi".to_string(), "ath9k".to_string(), "rtw88".to_string()],
            subsystem: "pci".to_string(),
        });

        self.device_class_mappings.insert("0403".to_string(), DeviceClassInfo {
            class_name: "Audio device".to_string(),
            class_code: "0403".to_string(),
            common_drivers: vec!["snd_hda_intel".to_string(), "snd_hda_codec_realtek".to_string()],
            subsystem: "pci".to_string(),
        });

        Ok(())
    }

    #[allow(dead_code)]
    fn map_cpu_drivers(&self, cpu_info: &Value) -> Result<Vec<DriverRecommendation>, LxHwError> {
        let mut recommendations = Vec::new();
        
        if let Some(vendor) = cpu_info.get("vendor").and_then(|v| v.as_str()) {
            let vendor_lower = vendor.to_lowercase();
            
            if vendor_lower.contains("intel") {
                recommendations.push(DriverRecommendation {
                    hardware_id: "cpu:intel".to_string(),
                    component_type: "CPU".to_string(),
                    recommended_driver: "intel-microcode".to_string(),
                    alternative_drivers: vec![],
                    driver_source: DriverSource::DistributionPackage {
                        package_name: "intel-microcode".to_string(),
                    },
                    installation_priority: 9,
                    compatibility_notes: Some("Early microcode loading recommended".to_string()),
                    kernel_modules: vec!["microcode".to_string()],
                });
            } else if vendor_lower.contains("amd") {
                recommendations.push(DriverRecommendation {
                    hardware_id: "cpu:amd".to_string(),
                    component_type: "CPU".to_string(),
                    recommended_driver: "amd64-microcode".to_string(),
                    alternative_drivers: vec![],
                    driver_source: DriverSource::DistributionPackage {
                        package_name: "amd64-microcode".to_string(),
                    },
                    installation_priority: 9,
                    compatibility_notes: Some("Microcode updates for security and stability".to_string()),
                    kernel_modules: vec!["microcode".to_string()],
                });
            }
        }

        Ok(recommendations)
    }

    #[allow(dead_code)]
    fn map_gpu_drivers(&self, gpu_info: &Value) -> Result<Vec<DriverRecommendation>, LxHwError> {
        let mut recommendations = Vec::new();
        
        let vendor = gpu_info.get("vendor").and_then(|v| v.as_str()).unwrap_or("unknown");
        let _product = gpu_info.get("product").and_then(|v| v.as_str()).unwrap_or("unknown");
        let device_id = self.extract_pci_id(gpu_info)?;

        let vendor_lower = vendor.to_lowercase();
        
        if vendor_lower.contains("nvidia") {
            // Recommend proprietary NVIDIA driver for better performance
            recommendations.push(DriverRecommendation {
                hardware_id: device_id.clone(),
                component_type: "GPU".to_string(),
                recommended_driver: "nvidia".to_string(),
                alternative_drivers: vec!["nouveau".to_string()],
                driver_source: DriverSource::DistributionPackage {
                    package_name: "nvidia-driver".to_string(),
                },
                installation_priority: 8,
                compatibility_notes: Some("Proprietary driver with better performance".to_string()),
                kernel_modules: vec!["nvidia".to_string(), "nvidia_modeset".to_string()],
            });

            // Also include open-source alternative
            recommendations.push(DriverRecommendation {
                hardware_id: device_id.clone(),
                component_type: "GPU".to_string(),
                recommended_driver: "nouveau".to_string(),
                alternative_drivers: vec!["nvidia".to_string()],
                driver_source: DriverSource::KernelBuiltin,
                installation_priority: 6,
                compatibility_notes: Some("Open-source driver, limited performance".to_string()),
                kernel_modules: vec!["nouveau".to_string()],
            });
        } else if vendor_lower.contains("amd") || vendor_lower.contains("ati") {
            recommendations.push(DriverRecommendation {
                hardware_id: device_id.clone(),
                component_type: "GPU".to_string(),
                recommended_driver: "amdgpu".to_string(),
                alternative_drivers: vec!["radeon".to_string()],
                driver_source: DriverSource::KernelBuiltin,
                installation_priority: 8,
                compatibility_notes: Some("Modern AMD GPUs supported by amdgpu".to_string()),
                kernel_modules: vec!["amdgpu".to_string()],
            });
        } else if vendor_lower.contains("intel") {
            recommendations.push(DriverRecommendation {
                hardware_id: device_id.clone(),
                component_type: "GPU".to_string(),
                recommended_driver: "i915".to_string(),
                alternative_drivers: vec!["xe".to_string()],
                driver_source: DriverSource::KernelBuiltin,
                installation_priority: 8,
                compatibility_notes: Some("Intel integrated graphics driver".to_string()),
                kernel_modules: vec!["i915".to_string()],
            });
        }

        Ok(recommendations)
    }

    #[allow(dead_code)]
    fn map_network_drivers(&self, network_info: &Value) -> Result<Vec<DriverRecommendation>, LxHwError> {
        let mut recommendations = Vec::new();
        
        let vendor = network_info.get("vendor").and_then(|v| v.as_str()).unwrap_or("unknown");
        let product = network_info.get("product").and_then(|v| v.as_str()).unwrap_or("unknown");
        let device_id = self.extract_pci_id(network_info)?;

        let vendor_lower = vendor.to_lowercase();
        let product_lower = product.to_lowercase();
        
        if vendor_lower.contains("intel") {
            if product_lower.contains("wireless") || product_lower.contains("wifi") {
                recommendations.push(DriverRecommendation {
                    hardware_id: device_id.clone(),
                    component_type: "Wireless Network".to_string(),
                    recommended_driver: "iwlwifi".to_string(),
                    alternative_drivers: vec![],
                    driver_source: DriverSource::KernelBuiltin,
                    installation_priority: 8,
                    compatibility_notes: Some("Requires firmware package".to_string()),
                    kernel_modules: vec!["iwlwifi".to_string()],
                });
            } else if product_lower.contains("ethernet") {
                recommendations.push(DriverRecommendation {
                    hardware_id: device_id.clone(),
                    component_type: "Ethernet".to_string(),
                    recommended_driver: "e1000e".to_string(),
                    alternative_drivers: vec![],
                    driver_source: DriverSource::KernelBuiltin,
                    installation_priority: 8,
                    compatibility_notes: Some("Intel Ethernet adapter".to_string()),
                    kernel_modules: vec!["e1000e".to_string()],
                });
            }
        } else if vendor_lower.contains("realtek") {
            recommendations.push(DriverRecommendation {
                hardware_id: device_id.clone(),
                component_type: "Network".to_string(),
                recommended_driver: "r8169".to_string(),
                alternative_drivers: vec![],
                driver_source: DriverSource::KernelBuiltin,
                installation_priority: 8,
                compatibility_notes: Some("Realtek network adapter".to_string()),
                kernel_modules: vec!["r8169".to_string()],
            });
        }

        Ok(recommendations)
    }

    #[allow(dead_code)]
    fn map_audio_drivers(&self, audio_info: &Value) -> Result<Vec<DriverRecommendation>, LxHwError> {
        let mut recommendations = Vec::new();
        
        let _vendor = audio_info.get("vendor").and_then(|v| v.as_str()).unwrap_or("unknown");
        let _product = audio_info.get("product").and_then(|v| v.as_str()).unwrap_or("unknown");
        let device_id = self.extract_pci_id(audio_info)?;

        // Most audio devices use HD Audio (Intel HDA) standard
        recommendations.push(DriverRecommendation {
            hardware_id: device_id.clone(),
            component_type: "Audio".to_string(),
            recommended_driver: "snd_hda_intel".to_string(),
            alternative_drivers: vec!["snd_hda_codec_generic".to_string()],
            driver_source: DriverSource::KernelBuiltin,
            installation_priority: 7,
            compatibility_notes: Some("Standard HD Audio driver".to_string()),
            kernel_modules: vec!["snd_hda_intel".to_string()],
        });

        Ok(recommendations)
    }

    fn map_kernel_device_drivers(&self, device: &DeviceCompatibility) -> Result<Vec<DriverRecommendation>, LxHwError> {
        let mut recommendations = Vec::new();

        // Only create recommendations for devices without existing driver assignments
        if device.driver_module.is_empty() {
            let driver_name = self.infer_driver_from_device(device)?;
            
            if !driver_name.is_empty() && driver_name != "unknown" {
                recommendations.push(DriverRecommendation {
                    hardware_id: device.device_id.clone(),
                    component_type: "Device".to_string(),
                    recommended_driver: driver_name.clone(),
                    alternative_drivers: vec![],
                    driver_source: DriverSource::KernelBuiltin,
                    installation_priority: 5,
                    compatibility_notes: Some(format!("Inferred driver for {}", device.device_name)),
                    kernel_modules: vec![driver_name],
                });
            }
        }

        Ok(recommendations)
    }

    fn infer_driver_from_device(&self, device: &DeviceCompatibility) -> Result<String, LxHwError> {
        let device_name_lower = device.device_name.to_lowercase();
        let device_id_parts: Vec<&str> = device.device_id.split(':').collect();

        // Check for specific device patterns
        if device_name_lower.contains("ethernet") {
            return Ok("e1000e".to_string()); // Generic ethernet driver
        } else if device_name_lower.contains("wireless") || device_name_lower.contains("wifi") {
            return Ok("iwlwifi".to_string()); // Common wireless driver
        } else if device_name_lower.contains("audio") || device_name_lower.contains("sound") {
            return Ok("snd_hda_intel".to_string()); // HD Audio driver
        } else if device_name_lower.contains("vga") || device_name_lower.contains("display") {
            // Infer graphics driver from vendor ID
            if let Some(vendor_id) = device_id_parts.get(0) {
                if *vendor_id == "10de" {
                    return Ok("nouveau".to_string()); // NVIDIA
                } else if *vendor_id == "1002" {
                    return Ok("amdgpu".to_string()); // AMD
                } else if *vendor_id == "8086" {
                    return Ok("i915".to_string()); // Intel
                }
            }
        } else if device_name_lower.contains("usb") {
            return Ok("usbhid".to_string()); // Generic USB HID driver
        }

        Ok("unknown".to_string())
    }

    #[allow(dead_code)]
    fn extract_pci_id(&self, device_info: &Value) -> Result<String, LxHwError> {
        if let Some(businfo) = device_info.get("businfo") {
            if let Some(businfo_str) = businfo.as_str() {
                if businfo_str.starts_with("pci@") {
                    return Ok(businfo_str.replace("pci@", ""));
                }
            }
        }
        
        // Fallback to constructing from vendor/product info
        let vendor = device_info.get("vendor").and_then(|v| v.as_str()).unwrap_or("unknown");
        let product = device_info.get("product").and_then(|v| v.as_str()).unwrap_or("unknown");
        Ok(format!("{}:{}", vendor, product))
    }

    // New methods that work with structured hardware types
    fn map_cpu_drivers_from_device(&self, cpu: &crate::hardware::CpuInfo) -> Result<Vec<DriverRecommendation>, LxHwError> {
        let mut recommendations = Vec::new();
        let vendor = cpu.vendor.to_lowercase();

        let (driver_name, priority) = if vendor.contains("intel") {
            ("intel_pstate", 8)
        } else if vendor.contains("amd") {
            ("amd_pstate", 8)
        } else {
            ("acpi-cpufreq", 6)
        };

        recommendations.push(DriverRecommendation {
            hardware_id: format!("{}:{}", cpu.vendor, cpu.model),
            component_type: "CPU".to_string(),
            recommended_driver: driver_name.to_string(),
            alternative_drivers: vec![],
            driver_source: DriverSource::KernelBuiltin,
            installation_priority: priority,
            compatibility_notes: Some(format!("CPU frequency scaling driver for {}", cpu.vendor)),
            kernel_modules: vec![driver_name.to_string()],
        });

        Ok(recommendations)
    }

    fn map_gpu_drivers_from_device(&self, gpu: &crate::hardware::GraphicsDevice) -> Result<Vec<DriverRecommendation>, LxHwError> {
        let mut recommendations = Vec::new();
        let vendor = gpu.vendor.to_lowercase();

        let (driver_name, priority, source) = if vendor.contains("nvidia") {
            ("nvidia", 9, DriverSource::DistributionPackage { 
                package_name: "nvidia-driver".to_string() 
            })
        } else if vendor.contains("amd") || vendor.contains("ati") {
            ("amdgpu", 8, DriverSource::KernelBuiltin)
        } else if vendor.contains("intel") {
            ("i915", 8, DriverSource::KernelBuiltin)
        } else {
            ("generic", 5, DriverSource::KernelBuiltin)
        };

        recommendations.push(DriverRecommendation {
            hardware_id: gpu.pci_id.clone(),
            component_type: "GPU".to_string(),
            recommended_driver: driver_name.to_string(),
            alternative_drivers: vec![],
            driver_source: source,
            installation_priority: priority,
            compatibility_notes: Some(format!("Graphics driver for {}", gpu.vendor)),
            kernel_modules: vec![driver_name.to_string()],
        });

        Ok(recommendations)
    }

    fn map_network_drivers_from_device(&self, network: &crate::hardware::NetworkDevice) -> Result<Vec<DriverRecommendation>, LxHwError> {
        let mut recommendations = Vec::new();
        let vendor = network.vendor.to_lowercase();
        let device_type = &network.device_type;

        let (driver_name, priority) = if device_type == "wifi" || device_type == "wireless" {
            if vendor.contains("intel") {
                ("iwlwifi", 8)
            } else if vendor.contains("broadcom") {
                ("brcmfmac", 7)
            } else if vendor.contains("realtek") {
                ("rtl8xxx", 7)
            } else {
                ("generic-wifi", 6)
            }
        } else { // ethernet
            if vendor.contains("intel") {
                ("e1000e", 8)
            } else if vendor.contains("realtek") {
                ("r8169", 8)
            } else if vendor.contains("broadcom") {
                ("tg3", 7)
            } else {
                ("generic-ethernet", 6)
            }
        };

        recommendations.push(DriverRecommendation {
            hardware_id: network.anonymized_mac.clone(),
            component_type: "Network".to_string(),
            recommended_driver: driver_name.to_string(),
            alternative_drivers: vec![],
            driver_source: DriverSource::KernelBuiltin,
            installation_priority: priority,
            compatibility_notes: Some(format!("{} network driver for {}", device_type, network.vendor)),
            kernel_modules: vec![driver_name.to_string()],
        });

        Ok(recommendations)
    }

    fn map_audio_drivers_from_device(&self, audio: &crate::hardware::AudioDevice) -> Result<Vec<DriverRecommendation>, LxHwError> {
        let mut recommendations = Vec::new();
        let vendor = audio.vendor.to_lowercase();

        let driver_name = if vendor.contains("intel") {
            "snd_hda_intel"
        } else if vendor.contains("amd") || vendor.contains("ati") {
            "snd_hda_generic"
        } else if vendor.contains("nvidia") {
            "snd_hda_intel"
        } else {
            "snd_hda_generic"
        };

        recommendations.push(DriverRecommendation {
            hardware_id: format!("{}:{}", audio.vendor, audio.model),
            component_type: "Audio".to_string(),
            recommended_driver: driver_name.to_string(),
            alternative_drivers: vec![],
            driver_source: DriverSource::KernelBuiltin,
            installation_priority: 6,
            compatibility_notes: Some(format!("Audio driver for {}", audio.vendor)),
            kernel_modules: vec![driver_name.to_string(), "snd_pcm".to_string()],
        });

        Ok(recommendations)
    }
}