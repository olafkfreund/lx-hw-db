use std::collections::HashMap;
use serde_json::Value;
use crate::configuration::*;
use crate::hardware::HardwareReport;
use crate::errors::LxHwError;
use crate::configuration::drivers::DriverMapper;
use crate::configuration::kernel_params::KernelParameterGenerator;
use crate::configuration::packages::PackageMapper;

pub struct ConfigurationEngineImpl {
    driver_mapper: DriverMapper,
    kernel_param_generator: KernelParameterGenerator,
    package_mapper: PackageMapper,
    community_configurations: HashMap<String, Vec<Configuration>>,
}

impl ConfigurationEngineImpl {
    pub fn new() -> Result<Self, LxHwError> {
        Ok(Self {
            driver_mapper: DriverMapper::new()?,
            kernel_param_generator: KernelParameterGenerator::new()?,
            package_mapper: PackageMapper::new()?,
            community_configurations: HashMap::new(),
        })
    }

    pub fn load_community_configurations(&mut self, configurations: Vec<Configuration>) -> Result<(), LxHwError> {
        for config in configurations {
            let hardware_key = self.generate_hardware_key(&config.hardware_profile)?;
            self.community_configurations
                .entry(hardware_key)
                .or_insert_with(Vec::new)
                .push(config);
        }
        Ok(())
    }

    fn generate_hardware_key(&self, profile: &HardwareProfile) -> Result<String, LxHwError> {
        let mut key_parts = Vec::new();
        
        if let Some(cpu) = &profile.cpu {
            key_parts.push(format!("cpu:{}:{}", cpu.vendor, cpu.architecture));
        }
        
        for gpu in &profile.gpu {
            key_parts.push(format!("gpu:{}:{}", gpu.vendor, gpu.device_id));
        }
        
        for network in &profile.network {
            key_parts.push(format!("net:{}:{}", network.vendor, network.device_id));
        }
        
        key_parts.sort();
        Ok(key_parts.join("|"))
    }

    fn extract_hardware_profile(&self, hardware: &HardwareReport) -> Result<HardwareProfile, LxHwError> {
        let mut profile = HardwareProfile {
            cpu: None,
            gpu: Vec::new(),
            network: Vec::new(),
            storage: Vec::new(),
            audio: Vec::new(),
            usb_controllers: Vec::new(),
        };

        // Extract CPU information
        if let Some(cpu_info) = &hardware.cpu {
            profile.cpu = Some(CpuProfile {
                vendor: cpu_info.vendor.clone(),
                model: cpu_info.model.clone(),
                architecture: hardware.system.architecture.clone(),
                features: self.extract_cpu_features(cpu_info)?,
                microcode_needed: self.needs_microcode(cpu_info)?,
                power_management: self.detect_power_management(cpu_info)?,
            });
        }

        // Extract GPU information
        for gpu in &hardware.graphics {
            profile.gpu.push(self.extract_gpu_profile_from_device(gpu)?);
        }

        // Extract network information
        for network in &hardware.network {
            profile.network.push(self.extract_network_profile_from_device(network)?);
        }

        // Extract storage information
        for storage in &hardware.storage {
            profile.storage.push(self.extract_storage_profile_from_device(storage)?);
        }

        // Extract audio information
        for audio in &hardware.audio {
            profile.audio.push(self.extract_audio_profile_from_device(audio)?);
        }

        // Extract USB controller information from kernel support
        if let Some(kernel_support) = &hardware.kernel_support {
            for device in &kernel_support.device_support_details {
                if device.device_name.to_lowercase().contains("usb") {
                    profile.usb_controllers.push(self.extract_usb_controller_profile(device)?);
                }
            }
        }

        Ok(profile)
    }

    fn extract_cpu_features(&self, cpu_info: &crate::hardware::CpuInfo) -> Result<Vec<String>, LxHwError> {
        Ok(cpu_info.flags.clone())
    }

    fn needs_microcode(&self, cpu_info: &crate::hardware::CpuInfo) -> Result<bool, LxHwError> {
        let vendor_lower = cpu_info.vendor.to_lowercase();
        Ok(vendor_lower.contains("intel") || vendor_lower.contains("amd"))
    }

    fn detect_power_management(&self, cpu_info: &crate::hardware::CpuInfo) -> Result<PowerManagement, LxHwError> {
        // Default power management settings based on CPU info
        let vendor = cpu_info.vendor.to_lowercase();
        
        let (governor, scaling_driver) = if vendor.contains("intel") {
            ("performance".to_string(), "intel_pstate".to_string())
        } else if vendor.contains("amd") {
            ("performance".to_string(), "amd_pstate".to_string())
        } else {
            ("ondemand".to_string(), "acpi-cpufreq".to_string())
        };

        Ok(PowerManagement {
            cpu_governor: governor,
            scaling_driver,
            idle_states: vec!["C1".to_string(), "C2".to_string(), "C3".to_string()],
            turbo_boost: true,
        })
    }

    #[allow(dead_code)]
    fn extract_gpu_profile(&self, gpu_info: &Value) -> Result<GpuProfile, LxHwError> {
        let vendor = gpu_info.get("vendor").and_then(|v| v.as_str()).unwrap_or("unknown");
        let product = gpu_info.get("product").and_then(|v| v.as_str()).unwrap_or("unknown");
        let device_id = self.extract_device_id(gpu_info)?;

        let driver_options = self.get_gpu_driver_options(vendor)?;
        let performance_profile = if vendor.to_lowercase().contains("nvidia") {
            "performance"
        } else if vendor.to_lowercase().contains("amd") {
            "auto"
        } else {
            "powersave"
        }.to_string();

        Ok(GpuProfile {
            vendor: vendor.to_string(),
            model: product.to_string(),
            device_id,
            driver_options,
            performance_profile,
            display_outputs: vec!["HDMI".to_string(), "DisplayPort".to_string()],
        })
    }

    #[allow(dead_code)]
    fn extract_network_profile(&self, network_info: &Value) -> Result<NetworkProfile, LxHwError> {
        let vendor = network_info.get("vendor").and_then(|v| v.as_str()).unwrap_or("unknown");
        let product = network_info.get("product").and_then(|v| v.as_str()).unwrap_or("unknown");
        let device_id = self.extract_device_id(network_info)?;
        
        let interface_type = if product.to_lowercase().contains("wireless") || product.to_lowercase().contains("wifi") {
            "wireless"
        } else if product.to_lowercase().contains("ethernet") {
            "ethernet"
        } else {
            "unknown"
        }.to_string();

        let firmware_needed = self.requires_firmware(vendor, &interface_type)?;

        Ok(NetworkProfile {
            interface_type,
            vendor: vendor.to_string(),
            model: product.to_string(),
            device_id,
            driver_name: network_info.get("configuration")
                .and_then(|c| c.get("driver"))
                .and_then(|d| d.as_str())
                .map(|s| s.to_string()),
            firmware_needed,
        })
    }

    #[allow(dead_code)]
    fn extract_storage_profile(&self, storage_info: &Value) -> Result<StorageProfile, LxHwError> {
        let vendor = storage_info.get("vendor").and_then(|v| v.as_str()).unwrap_or("unknown");
        let product = storage_info.get("product").and_then(|v| v.as_str()).unwrap_or("unknown");
        
        let device_type = if product.to_lowercase().contains("ssd") {
            "ssd"
        } else if product.to_lowercase().contains("nvme") {
            "nvme"
        } else if product.to_lowercase().contains("hdd") {
            "hdd"
        } else {
            "unknown"
        }.to_string();

        let interface = if product.to_lowercase().contains("nvme") {
            "nvme"
        } else if product.to_lowercase().contains("sata") {
            "sata"
        } else {
            "unknown"
        }.to_string();

        let optimizations = self.get_storage_optimizations(&device_type)?;

        Ok(StorageProfile {
            device_type,
            interface,
            vendor: vendor.to_string(),
            model: product.to_string(),
            capacity: storage_info.get("size")
                .and_then(|s| s.as_u64()),
            optimizations,
        })
    }

    #[allow(dead_code)]
    fn extract_audio_profile(&self, audio_info: &Value) -> Result<AudioProfile, LxHwError> {
        let vendor = audio_info.get("vendor").and_then(|v| v.as_str()).unwrap_or("unknown");
        let product = audio_info.get("product").and_then(|v| v.as_str()).unwrap_or("unknown");
        let device_id = self.extract_device_id(audio_info)?;

        Ok(AudioProfile {
            vendor: vendor.to_string(),
            codec: product.to_string(),
            device_id,
            pulse_config: None,
            alsa_config: None,
        })
    }

    fn extract_gpu_profile_from_device(&self, gpu: &crate::hardware::GraphicsDevice) -> Result<GpuProfile, LxHwError> {
        let driver_options = self.get_gpu_driver_options(&gpu.vendor)?;
        let performance_profile = if gpu.vendor.to_lowercase().contains("nvidia") {
            "performance"
        } else if gpu.vendor.to_lowercase().contains("amd") {
            "auto"
        } else {
            "powersave"
        }.to_string();

        Ok(GpuProfile {
            vendor: gpu.vendor.clone(),
            model: gpu.model.clone(),
            device_id: gpu.pci_id.clone(),
            driver_options,
            performance_profile,
            display_outputs: vec!["HDMI".to_string(), "DisplayPort".to_string()],
        })
    }

    fn extract_network_profile_from_device(&self, network: &crate::hardware::NetworkDevice) -> Result<NetworkProfile, LxHwError> {
        let interface_type = network.device_type.clone();
        let firmware_needed = self.requires_firmware(&network.vendor, &interface_type)?;

        Ok(NetworkProfile {
            interface_type,
            vendor: network.vendor.clone(),
            model: network.model.clone(),
            device_id: network.anonymized_mac.clone(),
            driver_name: network.driver.clone(),
            firmware_needed,
        })
    }

    fn extract_storage_profile_from_device(&self, storage: &crate::hardware::StorageDevice) -> Result<StorageProfile, LxHwError> {
        let device_type = storage.device_type.clone();
        let interface = storage.interface.clone().unwrap_or_else(|| "unknown".to_string());
        let optimizations = self.get_storage_optimizations(&device_type)?;

        Ok(StorageProfile {
            device_type,
            interface,
            vendor: storage.vendor.clone().unwrap_or_else(|| "unknown".to_string()),
            model: storage.model.clone(),
            capacity: Some(storage.size_bytes),
            optimizations,
        })
    }

    fn extract_audio_profile_from_device(&self, audio: &crate::hardware::AudioDevice) -> Result<AudioProfile, LxHwError> {
        Ok(AudioProfile {
            vendor: audio.vendor.clone(),
            codec: audio.model.clone(),
            device_id: format!("{}:{}", audio.vendor, audio.model),
            pulse_config: None,
            alsa_config: None,
        })
    }

    fn extract_usb_controller_profile(&self, device_info: &crate::hardware::DeviceCompatibility) -> Result<UsbControllerProfile, LxHwError> {
        let version = if device_info.device_name.contains("USB 3") || device_info.device_name.contains("xHCI") {
            "3.0"
        } else if device_info.device_name.contains("USB 2") || device_info.device_name.contains("EHCI") {
            "2.0"
        } else {
            "1.1"
        }.to_string();

        Ok(UsbControllerProfile {
            version,
            vendor: device_info.device_id.split(':').next().unwrap_or("unknown").to_string(),
            device_id: device_info.device_id.clone(),
            power_management: true,
        })
    }

    #[allow(dead_code)]
    fn extract_device_id(&self, device_info: &Value) -> Result<String, LxHwError> {
        if let Some(businfo) = device_info.get("businfo") {
            if let Some(businfo_str) = businfo.as_str() {
                // Extract PCI device ID from businfo like "pci@0000:01:00.0"
                if businfo_str.starts_with("pci@") {
                    return Ok(businfo_str.replace("pci@", ""));
                }
            }
        }
        
        // Fallback to product name or vendor info
        if let Some(product) = device_info.get("product").and_then(|p| p.as_str()) {
            Ok(product.to_string())
        } else {
            Ok("unknown".to_string())
        }
    }

    fn get_gpu_driver_options(&self, vendor: &str) -> Result<Vec<String>, LxHwError> {
        let vendor_lower = vendor.to_lowercase();
        if vendor_lower.contains("nvidia") {
            Ok(vec!["nvidia".to_string(), "nouveau".to_string()])
        } else if vendor_lower.contains("amd") || vendor_lower.contains("ati") {
            Ok(vec!["amdgpu".to_string(), "radeon".to_string()])
        } else if vendor_lower.contains("intel") {
            Ok(vec!["i915".to_string(), "xe".to_string()])
        } else {
            Ok(vec!["generic".to_string()])
        }
    }

    fn requires_firmware(&self, vendor: &str, interface_type: &str) -> Result<bool, LxHwError> {
        if interface_type == "wireless" {
            Ok(true) // Most wireless cards need firmware
        } else if vendor.to_lowercase().contains("broadcom") {
            Ok(true) // Broadcom typically needs firmware
        } else {
            Ok(false)
        }
    }

    fn get_storage_optimizations(&self, device_type: &str) -> Result<Vec<String>, LxHwError> {
        match device_type {
            "ssd" | "nvme" => Ok(vec![
                "discard".to_string(),
                "noatime".to_string(),
                "scheduler=none".to_string(),
            ]),
            "hdd" => Ok(vec![
                "relatime".to_string(),
                "scheduler=mq-deadline".to_string(),
            ]),
            _ => Ok(Vec::new()),
        }
    }

    fn calculate_compatibility_score(&self, hardware: &HardwareReport, recommendations: &[DriverRecommendation]) -> Result<f64, LxHwError> {
        let total_devices = if let Some(kernel_support) = &hardware.kernel_support {
            kernel_support.total_devices_detected as f64
        } else {
            1.0 // Avoid division by zero
        };

        let supported_devices = recommendations.iter()
            .filter(|r| r.recommended_driver != "unknown")
            .count() as f64;

        let compatibility_ratio = supported_devices / total_devices;
        
        // Weight the score based on critical components
        let mut weighted_score = compatibility_ratio * 0.7; // Base compatibility
        
        // Add weights for critical components
        if hardware.cpu.is_some() {
            weighted_score += 0.1;
        }
        
        if !hardware.graphics.is_empty() {
            weighted_score += 0.1;
        }
        
        if !hardware.network.is_empty() {
            weighted_score += 0.1;
        }

        // Cap at 100%
        Ok((weighted_score * 100.0).min(100.0))
    }
}

impl ConfigurationEngine for ConfigurationEngineImpl {
    fn generate_configuration(&self, hardware: &HardwareReport, target_distribution: &str) -> Result<Configuration, LxHwError> {
        let hardware_profile = self.extract_hardware_profile(hardware)?;
        let driver_recommendations = self.recommend_drivers(hardware)?;
        let kernel_parameters = self.generate_kernel_parameters(hardware)?;
        let package_installations = self.suggest_packages(hardware, target_distribution)?;
        let compatibility_score = self.calculate_compatibility_score(hardware, &driver_recommendations)?;

        Ok(Configuration {
            system_id: hardware.metadata.anonymized_system_id.clone(),
            target_distribution: target_distribution.to_string(),
            kernel_version: hardware.system.kernel_version.clone(),
            hardware_profile,
            driver_recommendations,
            kernel_parameters,
            package_installations,
            dkms_modules: Vec::new(), // TODO: Implement DKMS module detection
            configuration_files: HashMap::new(), // TODO: Implement config file generation
            performance_optimizations: Vec::new(), // TODO: Implement performance optimizations
            compatibility_score,
        })
    }

    fn analyze_compatibility(&self, hardware: &HardwareReport) -> Result<f64, LxHwError> {
        let recommendations = self.recommend_drivers(hardware)?;
        self.calculate_compatibility_score(hardware, &recommendations)
    }

    fn recommend_drivers(&self, hardware: &HardwareReport) -> Result<Vec<DriverRecommendation>, LxHwError> {
        self.driver_mapper.map_drivers(hardware)
    }

    fn generate_kernel_parameters(&self, hardware: &HardwareReport) -> Result<Vec<KernelParameter>, LxHwError> {
        self.kernel_param_generator.generate_parameters(hardware)
    }

    fn suggest_packages(&self, hardware: &HardwareReport, distribution: &str) -> Result<Vec<PackageInstallation>, LxHwError> {
        self.package_mapper.map_packages(hardware, distribution)
    }
}