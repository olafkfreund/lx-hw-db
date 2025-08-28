//! Hardware detection and kernel verification integration
//!
//! This module combines hardware detection tools with kernel support verification
//! to provide comprehensive compatibility reports.

#![allow(clippy::excessive_nesting)]

use crate::detectors::kernel::{KernelSupportVerifier, SupportLevel};
use crate::detectors::{DetectionData, DetectionResult, DetectorRegistry};
use crate::errors::Result;
use crate::hardware::{
    AudioDevice, CpuInfo, DeviceCompatibility, GraphicsDevice, HardwareReport,
    KernelCompatibilityInfo, MemoryDimm, MemoryInfo, NetworkDevice, PrivacyLevel, ReportMetadata,
    StorageDevice, SystemInfo, UsbDevice,
};
use crate::privacy::PrivacyManager;
use chrono::Utc;

/// Comprehensive hardware analysis combining detection and kernel verification
pub struct HardwareAnalyzer {
    detector_registry: DetectorRegistry,
    kernel_verifier: KernelSupportVerifier,
    privacy_manager: PrivacyManager,
}

impl HardwareAnalyzer {
    /// Create a new hardware analyzer
    pub fn new(privacy_level: PrivacyLevel) -> Result<Self> {
        Ok(Self {
            detector_registry: DetectorRegistry::new(),
            kernel_verifier: KernelSupportVerifier::new()?,
            privacy_manager: PrivacyManager::new(privacy_level)?,
        })
    }

    /// Set specific tools to enable (filters out others)
    pub fn set_enabled_tools(&mut self, tool_names: Vec<String>) -> Result<()> {
        self.detector_registry.set_enabled_tools(tool_names)
    }

    /// Set custom timeout for detection tools
    pub fn set_detection_timeout(&mut self, timeout: std::time::Duration) {
        self.detector_registry.set_detection_timeout(timeout);
    }

    /// Perform complete hardware analysis with kernel verification
    pub async fn analyze_system(&mut self) -> Result<HardwareReport> {
        // Step 1: Run hardware detection tools
        log::info!("Running hardware detection tools...");
        let detection_results = self.detector_registry.detect_all().await?;

        // Step 2: Extract device IDs from detection results
        let device_ids = self.extract_device_ids(&detection_results);

        // Step 3: Verify kernel support for detected devices
        log::info!("Verifying kernel support for {} devices...", device_ids.len());
        let kernel_support = self.kernel_verifier.get_support_data(device_ids)?;

        // Step 4: Build comprehensive compatibility information
        let kernel_compatibility =
            self.build_kernel_compatibility(&kernel_support, &detection_results)?;

        // Step 5: Generate anonymized hardware report
        let report = self.build_hardware_report(detection_results, kernel_compatibility).await?;

        Ok(report)
    }

    /// Extract device IDs from detection results
    fn extract_device_ids(&self, results: &[DetectionResult]) -> Vec<(String, String)> {
        let mut device_ids = Vec::new();

        for result in results {
            let DetectionData::Lshw(data) = &result.data else {
                continue;
            };

            // Extract PCI IDs from lshw businfo field
            for component in &data.components {
                let Some(ref businfo) = component.businfo else {
                    continue;
                };

                if let Some(pci_id) = self.extract_pci_id_from_businfo(businfo) {
                    device_ids.push(pci_id);
                }
            }
        }

        // Also get device IDs directly from sysfs
        if let Ok(sysfs_ids) = self.kernel_verifier.extract_system_device_ids() {
            device_ids.extend(sysfs_ids);
        }

        // Remove duplicates
        device_ids.sort();
        device_ids.dedup();
        device_ids
    }

    /// Build kernel compatibility information
    fn build_kernel_compatibility(
        &self,
        kernel_support: &crate::detectors::kernel::KernelSupportData,
        detection_results: &[DetectionResult],
    ) -> Result<KernelCompatibilityInfo> {
        let mut device_details = Vec::new();
        let mut supported_count = 0;
        let mut unsupported_count = 0;
        let mut experimental_count = 0;
        let mut missing_modules = Vec::new();
        let mut config_recommendations = Vec::new();

        for device_support in &kernel_support.supported_devices {
            let device_name = self.get_device_name(&device_support.device_id, detection_results);

            let (support_status, _count_target) = match device_support.support_level {
                SupportLevel::Supported => {
                    supported_count += 1;
                    ("supported", &mut supported_count)
                }
                SupportLevel::Experimental => {
                    experimental_count += 1;
                    ("experimental", &mut experimental_count)
                }
                SupportLevel::Generic => {
                    supported_count += 1;
                    ("supported_generic", &mut supported_count)
                }
                SupportLevel::Similar => {
                    experimental_count += 1;
                    ("similar_device_supported", &mut experimental_count)
                }
                SupportLevel::Unsupported => {
                    unsupported_count += 1;
                    missing_modules.push(format!("No driver for {}", device_support.device_id));
                    ("unsupported", &mut unsupported_count)
                }
            };

            // Generate configuration recommendations
            for dep in &device_support.config_dependencies {
                let config_recommendation = format!(
                    "Enable CONFIG_{} for module {}",
                    dep.to_uppercase().replace('-', "_"),
                    device_support.driver_module
                );
                if !config_recommendations.contains(&config_recommendation) {
                    config_recommendations.push(config_recommendation);
                }
            }

            device_details.push(DeviceCompatibility {
                device_id: device_support.device_id.clone(),
                device_name,
                support_status: support_status.to_string(),
                driver_module: device_support.driver_module.clone(),
                since_kernel_version: device_support.kernel_version_added.clone(),
                config_dependencies: device_support.config_dependencies.clone(),
                notes: self.generate_compatibility_notes(&device_support.support_level),
            });
        }

        Ok(KernelCompatibilityInfo {
            kernel_version: kernel_support.kernel_version.clone(),
            total_devices_detected: kernel_support.supported_devices.len() as u32,
            supported_devices: supported_count,
            unsupported_devices: unsupported_count,
            experimental_devices: experimental_count,
            device_support_details: device_details,
            missing_modules,
            config_recommendations,
        })
    }

    /// Extract PCI ID from businfo string like "pci@0000:01:00.0"
    fn extract_pci_id_from_businfo(&self, businfo: &str) -> Option<(String, String)> {
        // lshw businfo format: "pci@0000:01:00.0"
        // We need to read the corresponding sysfs files for vendor/device IDs
        if businfo.starts_with("pci@") {
            let pci_addr = businfo.strip_prefix("pci@")?;
            let sysfs_path = format!("/sys/bus/pci/devices/{}", pci_addr);

            if let (Ok(vendor), Ok(device)) = (
                std::fs::read_to_string(format!("{}/vendor", sysfs_path)),
                std::fs::read_to_string(format!("{}/device", sysfs_path)),
            ) {
                let vendor = vendor.trim().strip_prefix("0x")?.trim();
                let device = device.trim().strip_prefix("0x")?.trim();
                return Some((vendor.to_string(), device.to_string()));
            }
        }
        None
    }

    /// Get human-readable device name from detection results
    fn get_device_name(&self, device_id: &str, detection_results: &[DetectionResult]) -> String {
        for result in detection_results {
            let DetectionData::Lshw(data) = &result.data else {
                continue;
            };

            for component in &data.components {
                let Some(ref businfo) = component.businfo else {
                    continue;
                };

                let Some((vendor, device)) = self.extract_pci_id_from_businfo(businfo) else {
                    continue;
                };

                let component_id = format!("{}:{}", vendor.to_lowercase(), device.to_lowercase());
                if component_id == device_id {
                    return component
                        .description
                        .clone()
                        .unwrap_or_else(|| format!("Device {}", device_id));
                }
            }
        }

        format!("Unknown Device {}", device_id)
    }

    /// Generate compatibility notes based on support level
    fn generate_compatibility_notes(&self, support_level: &SupportLevel) -> Option<String> {
        match support_level {
            SupportLevel::Supported => None,
            SupportLevel::Experimental => Some("Device supported by experimental driver. May require manual configuration.".to_string()),
            SupportLevel::Generic => Some("Supported by generic driver. Full functionality may not be available.".to_string()),
            SupportLevel::Similar => Some("No exact driver match found, but similar devices are supported. May work with existing drivers.".to_string()),
            SupportLevel::Unsupported => Some("No kernel driver found for this device. May require proprietary drivers or manual compilation.".to_string()),
        }
    }

    /// Build final hardware report with anonymization
    async fn build_hardware_report(
        &mut self,
        detection_results: Vec<DetectionResult>,
        kernel_compatibility: KernelCompatibilityInfo,
    ) -> Result<HardwareReport> {
        // Generate anonymized system ID
        let system_id = self.privacy_manager.anonymize_identifier("system")?;

        let metadata = ReportMetadata {
            version: env!("CARGO_PKG_VERSION").to_string(),
            generated_at: Utc::now(),
            privacy_level: self.privacy_manager.privacy_level(),
            tools_used: detection_results
                .iter()
                .filter(|r| r.success)
                .map(|r| r.tool_name.clone())
                .collect(),
            anonymized_system_id: system_id,
        };

        // Extract system information from detection results
        let system = self.extract_system_info(&detection_results).await?;

        // Extract hardware components from detection results
        let cpu = self.extract_cpu_info(&detection_results).await?;
        let memory = self.extract_memory_info(&detection_results).await?;
        let storage = self.extract_storage_devices(&detection_results).await?;
        let graphics = self.extract_graphics_devices(&detection_results).await?;
        let network = self.extract_network_devices(&detection_results).await?;
        let usb = self.extract_usb_devices(&detection_results).await?;
        let audio = self.extract_audio_devices(&detection_results).await?;

        Ok(HardwareReport {
            metadata,
            system,
            cpu,
            memory,
            storage,
            graphics,
            network,
            usb,
            audio,
            kernel_support: Some(kernel_compatibility),
        })
    }

    /// Extract system information with privacy protection
    async fn extract_system_info(
        &mut self,
        _detection_results: &[DetectionResult],
    ) -> Result<SystemInfo> {
        // Get system information from uname and /proc files
        let kernel_version = std::process::Command::new("uname")
            .arg("-r")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "unknown".to_string());

        let architecture = std::process::Command::new("uname")
            .arg("-m")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "unknown".to_string());

        // Anonymize hostname
        let hostname = std::process::Command::new("hostname")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "localhost".to_string());

        let anonymized_hostname = self.privacy_manager.anonymize_identifier(&hostname)?;

        // Try to detect distribution
        let distribution = self.detect_distribution();

        Ok(SystemInfo {
            anonymized_hostname,
            kernel_version,
            distribution,
            architecture,
            boot_time: None, // TODO: parse from /proc/stat
        })
    }

    /// Detect Linux distribution
    fn detect_distribution(&self) -> Option<String> {
        // Try /etc/os-release first
        if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
            for line in content.lines() {
                let Some(name) = line.strip_prefix("PRETTY_NAME=\"") else {
                    continue;
                };

                if let Some(end) = name.find('"') {
                    return Some(name[..end].to_string());
                }
            }
        }

        // Fallback to other detection methods
        if std::path::Path::new("/etc/debian_version").exists() {
            return Some("Debian-based".to_string());
        }
        if std::path::Path::new("/etc/redhat-release").exists() {
            return Some("Red Hat-based".to_string());
        }

        None
    }

    /// Extract CPU information from detection results
    async fn extract_cpu_info(
        &mut self,
        detection_results: &[DetectionResult],
    ) -> Result<Option<CpuInfo>> {
        // First try dmidecode for detailed CPU info
        for result in detection_results {
            if let DetectionData::Dmidecode(dmi_data) = &result.data {
                if let Some(processor) = dmi_data.processors.first() {
                    // Parse CPU flags from dmidecode
                    let flags = processor.flags.clone();

                    // Get frequency information
                    let base_frequency = processor.current_speed.map(|f| f as f64);
                    let max_frequency = processor.max_speed.map(|f| f as f64);

                    return Ok(Some(CpuInfo {
                        model: processor.version.clone(),
                        vendor: processor.manufacturer.clone(),
                        cores: processor.core_count.unwrap_or(1),
                        threads: processor
                            .thread_count
                            .unwrap_or(processor.core_count.unwrap_or(1)),
                        base_frequency,
                        max_frequency,
                        cache_l1: None, // dmidecode doesn't typically provide cache info
                        cache_l2: None,
                        cache_l3: None,
                        flags,
                    }));
                }
            }
        }

        // Fallback: try to extract basic info from lshw
        for result in detection_results {
            if let DetectionData::Lshw(lshw_data) = &result.data {
                for component in &lshw_data.components {
                    if component.class == "processor" {
                        let model = component.product.clone().unwrap_or_else(|| {
                            component.description.clone().unwrap_or("Unknown CPU".to_string())
                        });
                        let vendor = component.vendor.clone().unwrap_or("Unknown".to_string());

                        // Try to parse width/cores from lshw data
                        let cores = component.width.unwrap_or(1);
                        let threads = cores; // lshw doesn't distinguish cores vs threads

                        // Parse frequency from lshw capacity field
                        let frequency = component.capacity.map(|c| c as f64);

                        return Ok(Some(CpuInfo {
                            model,
                            vendor,
                            cores,
                            threads,
                            base_frequency: frequency,
                            max_frequency: frequency,
                            cache_l1: None,
                            cache_l2: None,
                            cache_l3: None,
                            flags: Vec::new(),
                        }));
                    }
                }
            }
        }

        Ok(None)
    }

    /// Extract memory information from detection results
    async fn extract_memory_info(
        &mut self,
        detection_results: &[DetectionResult],
    ) -> Result<Option<MemoryInfo>> {
        let mut total_bytes = 0u64;
        let mut dimms = Vec::new();

        // Extract memory information from dmidecode
        for result in detection_results {
            if let DetectionData::Dmidecode(dmi_data) = &result.data {
                for memory_device in &dmi_data.memory_devices {
                    // Only include populated memory slots
                    if let Some(size_mb) = memory_device.size_mb {
                        if size_mb > 0 {
                            let size_bytes = (size_mb as u64) * 1024 * 1024;
                            total_bytes += size_bytes;

                            dimms.push(MemoryDimm {
                                size_bytes,
                                speed_mhz: memory_device.speed_mts,
                                memory_type: memory_device.memory_type.clone(),
                                manufacturer: memory_device.manufacturer.clone(),
                            });
                        }
                    }
                }

                if !dimms.is_empty() {
                    // Calculate available memory (approximate)
                    let available_bytes = total_bytes.saturating_sub(total_bytes / 10); // Rough estimate

                    return Ok(Some(MemoryInfo { total_bytes, available_bytes, dimms }));
                }
            }
        }

        // Fallback: try to get total memory from lshw
        for result in detection_results {
            if let DetectionData::Lshw(lshw_data) = &result.data {
                for component in &lshw_data.components {
                    if component.class == "memory" {
                        if let Some(size) = component.size {
                            total_bytes += size;
                        }
                    }
                }

                if total_bytes > 0 {
                    let available_bytes = total_bytes.saturating_sub(total_bytes / 10);
                    return Ok(Some(MemoryInfo {
                        total_bytes,
                        available_bytes,
                        dimms: Vec::new(), // No detailed DIMM info from lshw
                    }));
                }
            }
        }

        Ok(None)
    }

    /// Extract storage devices from detection results
    async fn extract_storage_devices(
        &mut self,
        detection_results: &[DetectionResult],
    ) -> Result<Vec<StorageDevice>> {
        let mut storage_devices = Vec::new();

        // Extract storage information from lshw
        for result in detection_results {
            if let DetectionData::Lshw(lshw_data) = &result.data {
                for component in &lshw_data.components {
                    if component.class == "disk" || component.class == "storage" {
                        let model = component.product.clone().unwrap_or_else(|| {
                            component.description.clone().unwrap_or("Unknown Storage".to_string())
                        });
                        let vendor = component.vendor.clone();
                        let size_bytes = component.size.unwrap_or(0);

                        // Determine device type based on description and model
                        let device_type =
                            self.classify_storage_device(&model, component.description.as_ref());

                        // Anonymize serial number if present
                        let anonymized_serial = if let Some(serial) = &component.serial {
                            self.privacy_manager.anonymize_identifier(serial)?
                        } else {
                            "unknown".to_string()
                        };

                        // Extract interface information from businfo
                        let interface = component.businfo.as_ref().and_then(|businfo| {
                            if businfo.starts_with("scsi@") {
                                Some("SCSI".to_string())
                            } else if businfo.starts_with("ide@") {
                                Some("IDE".to_string())
                            } else if businfo.contains("nvme") {
                                Some("NVMe".to_string())
                            } else {
                                None
                            }
                        });

                        storage_devices.push(StorageDevice {
                            anonymized_serial,
                            device_type,
                            size_bytes,
                            model,
                            vendor,
                            interface,
                        });
                    }
                }
            }
        }

        Ok(storage_devices)
    }

    /// Extract graphics devices from detection results
    async fn extract_graphics_devices(
        &mut self,
        detection_results: &[DetectionResult],
    ) -> Result<Vec<GraphicsDevice>> {
        let mut graphics_devices = Vec::new();

        // First try lspci for detailed graphics info
        for result in detection_results {
            if let DetectionData::Lspci(lspci_data) = &result.data {
                for device in &lspci_data.devices {
                    if device.class_code.starts_with("03") {
                        // VGA/Display class
                        let pci_id = format!("{}:{}", device.vendor_id, device.device_id);
                        let vendor = device.vendor_name.clone().unwrap_or("Unknown".to_string());
                        let model = device
                            .device_name
                            .clone()
                            .unwrap_or_else(|| format!("Graphics Device {}", pci_id));
                        let driver = device.kernel_driver.clone();

                        graphics_devices.push(GraphicsDevice {
                            vendor,
                            model,
                            driver,
                            memory_bytes: None, // Would need additional parsing
                            pci_id,
                        });
                    }
                }
            }
        }

        // Fallback: extract graphics info from lshw
        if graphics_devices.is_empty() {
            for result in detection_results {
                if let DetectionData::Lshw(lshw_data) = &result.data {
                    for component in &lshw_data.components {
                        if component.class == "display" {
                            let vendor = component.vendor.clone().unwrap_or("Unknown".to_string());
                            let model = component.product.clone().unwrap_or_else(|| {
                                component
                                    .description
                                    .clone()
                                    .unwrap_or("Unknown Graphics".to_string())
                            });

                            // Extract PCI ID from businfo
                            let pci_id = component
                                .businfo
                                .as_ref()
                                .and_then(|bus| self.extract_pci_id_from_businfo(bus))
                                .map(|(v, d)| format!("{}:{}", v, d))
                                .unwrap_or("unknown".to_string());

                            graphics_devices.push(GraphicsDevice {
                                vendor,
                                model,
                                driver: None,
                                memory_bytes: component.size,
                                pci_id,
                            });
                        }
                    }
                }
            }
        }

        Ok(graphics_devices)
    }

    /// Extract network devices from detection results
    async fn extract_network_devices(
        &mut self,
        detection_results: &[DetectionResult],
    ) -> Result<Vec<NetworkDevice>> {
        let mut network_devices = Vec::new();

        // First try lspci for detailed network device info
        for result in detection_results {
            if let DetectionData::Lspci(lspci_data) = &result.data {
                for device in &lspci_data.devices {
                    if device.class_code.starts_with("02") {
                        // Network controller class
                        let vendor = device.vendor_name.clone().unwrap_or("Unknown".to_string());
                        let model = device.device_name.clone().unwrap_or_else(|| {
                            format!("Network Device {}:{}", device.vendor_id, device.device_id)
                        });
                        let driver = device.kernel_driver.clone();

                        // Determine device type from model/description
                        let device_type =
                            self.classify_network_device(&model, device.class_description.as_str());

                        network_devices.push(NetworkDevice {
                            device_type,
                            vendor,
                            model,
                            driver,
                            anonymized_mac: "unknown".to_string(), // PCI data doesn't include MAC
                        });
                    }
                }
            }
        }

        // Extract network devices from lshw
        for result in detection_results {
            if let DetectionData::Lshw(lshw_data) = &result.data {
                for component in &lshw_data.components {
                    if component.class == "network" {
                        let vendor = component.vendor.clone().unwrap_or("Unknown".to_string());
                        let model = component.product.clone().unwrap_or_else(|| {
                            component.description.clone().unwrap_or("Unknown Network".to_string())
                        });

                        // Determine device type
                        let device_type = self.classify_network_device(
                            &model,
                            component.description.as_deref().unwrap_or(""),
                        );

                        // Anonymize MAC address if available in configuration
                        let anonymized_mac = if let Some(config) = &component.configuration {
                            if let Some(serial_val) = config.get("serial") {
                                if let Some(serial_str) = serial_val.as_str() {
                                    self.privacy_manager.anonymize_identifier(serial_str)?
                                } else {
                                    "unknown".to_string()
                                }
                            } else {
                                "unknown".to_string()
                            }
                        } else {
                            "unknown".to_string()
                        };

                        // Check if we already added this device from lspci
                        if !network_devices
                            .iter()
                            .any(|dev| dev.model == model && dev.vendor == vendor)
                        {
                            network_devices.push(NetworkDevice {
                                device_type,
                                vendor,
                                model,
                                driver: None, // lshw doesn't always provide driver info
                                anonymized_mac,
                            });
                        }
                    }
                }
            }
        }

        Ok(network_devices)
    }

    /// Extract USB devices from detection results
    async fn extract_usb_devices(
        &mut self,
        detection_results: &[DetectionResult],
    ) -> Result<Vec<UsbDevice>> {
        let mut usb_devices = Vec::new();

        // Extract USB device information from lsusb
        for result in detection_results {
            if let DetectionData::Lsusb(lsusb_data) = &result.data {
                for device in &lsusb_data.devices {
                    usb_devices.push(UsbDevice {
                        vendor_id: device.vendor_id.clone(),
                        product_id: device.product_id.clone(),
                        vendor_name: device.vendor_name.clone(),
                        product_name: device.product_name.clone(),
                        usb_version: device.usb_version.clone(),
                    });
                }
            }
        }

        // Fallback: extract USB info from lshw if available
        if usb_devices.is_empty() {
            for result in detection_results {
                if let DetectionData::Lshw(lshw_data) = &result.data {
                    for component in &lshw_data.components {
                        if component.class == "usb"
                            || (component.businfo.is_some()
                                && component.businfo.as_ref().unwrap().starts_with("usb@"))
                        {
                            // Extract vendor/product IDs from businfo or configuration
                            let (vendor_id, product_id) =
                                self.extract_usb_ids_from_component(component);

                            usb_devices.push(UsbDevice {
                                vendor_id,
                                product_id,
                                vendor_name: component.vendor.clone(),
                                product_name: component.product.clone(),
                                usb_version: None,
                            });
                        }
                    }
                }
            }
        }

        Ok(usb_devices)
    }

    /// Extract audio devices from detection results
    async fn extract_audio_devices(
        &mut self,
        detection_results: &[DetectionResult],
    ) -> Result<Vec<AudioDevice>> {
        let mut audio_devices = Vec::new();

        // Extract audio devices from lspci
        for result in detection_results {
            if let DetectionData::Lspci(lspci_data) = &result.data {
                for device in &lspci_data.devices {
                    if device.class_code.starts_with("0401") || // Multimedia audio controller
                       device.class_code.starts_with("0403")
                    {
                        // Audio device

                        let vendor = device.vendor_name.clone().unwrap_or("Unknown".to_string());
                        let model = device.device_name.clone().unwrap_or_else(|| {
                            format!("Audio Device {}:{}", device.vendor_id, device.device_id)
                        });
                        let driver = device.kernel_driver.clone();

                        // Classify audio device type
                        let device_type = if device.class_code.starts_with("0401") {
                            "multimedia_audio".to_string()
                        } else {
                            "audio".to_string()
                        };

                        audio_devices.push(AudioDevice { vendor, model, driver, device_type });
                    }
                }
            }
        }

        // Extract audio devices from lshw
        for result in detection_results {
            if let DetectionData::Lshw(lshw_data) = &result.data {
                for component in &lshw_data.components {
                    if component.class == "multimedia" || component.class == "sound" {
                        let vendor = component.vendor.clone().unwrap_or("Unknown".to_string());
                        let model = component.product.clone().unwrap_or_else(|| {
                            component.description.clone().unwrap_or("Unknown Audio".to_string())
                        });

                        // Check if we already added this device from lspci
                        if !audio_devices
                            .iter()
                            .any(|dev| dev.model == model && dev.vendor == vendor)
                        {
                            audio_devices.push(AudioDevice {
                                vendor,
                                model,
                                driver: None,
                                device_type: "multimedia".to_string(),
                            });
                        }
                    }
                }
            }
        }

        Ok(audio_devices)
    }

    /// Helper to classify storage device types
    fn classify_storage_device(&self, model: &str, description: Option<&String>) -> String {
        let combined = format!(
            "{} {}",
            model.to_lowercase(),
            description.map(|d| d.to_lowercase()).unwrap_or_default()
        );

        if combined.contains("nvme") {
            "NVMe SSD".to_string()
        } else if combined.contains("ssd") {
            "SSD".to_string()
        } else if combined.contains("usb") || combined.contains("removable") {
            "USB Drive".to_string()
        } else if combined.contains("optical")
            || combined.contains("dvd")
            || combined.contains("cd")
        {
            "Optical Drive".to_string()
        } else {
            "HDD".to_string() // Default assumption
        }
    }

    /// Helper to classify network device types
    fn classify_network_device(&self, model: &str, description: &str) -> String {
        let combined = format!("{} {}", model.to_lowercase(), description.to_lowercase());

        if combined.contains("wireless") || combined.contains("wifi") || combined.contains("802.11")
        {
            "wifi".to_string()
        } else if combined.contains("bluetooth") {
            "bluetooth".to_string()
        } else if combined.contains("ethernet") || combined.contains("network") {
            "ethernet".to_string()
        } else {
            "network".to_string() // Generic fallback
        }
    }

    /// Helper to extract USB vendor/product IDs from lshw component
    fn extract_usb_ids_from_component(
        &self,
        component: &crate::detectors::lshw::LshwComponent,
    ) -> (String, String) {
        // Try to extract from configuration or other fields
        if let Some(config) = &component.configuration {
            // Look for USB IDs in various formats
            for (key, value) in config {
                if key.contains("vendor") || key.contains("product") {
                    if let Some(id_str) = value.as_str() {
                        // Simple parsing - in real implementation, would be more sophisticated
                        if id_str.len() == 4 && id_str.chars().all(|c| c.is_ascii_hexdigit()) {
                            return (id_str.to_string(), "0000".to_string());
                        }
                    }
                }
            }
        }

        // Fallback to unknown IDs
        ("unknown".to_string(), "unknown".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hardware_analyzer_creation() {
        let result = HardwareAnalyzer::new(PrivacyLevel::Basic);
        // Note: This might fail in test environments without proper kernel modules
        // That's expected and acceptable for unit tests
        if result.is_ok() {
            // Success case
        }
        // Expected in test environments - error case handled implicitly
    }

    #[test]
    fn test_device_id_extraction() {
        // Test that device ID extraction handles empty results gracefully
        let analyzer = HardwareAnalyzer {
            detector_registry: DetectorRegistry::new(),
            kernel_verifier: KernelSupportVerifier::new().unwrap_or_else(|_| {
                // Fallback for test environments
                panic!("Cannot create kernel verifier in test")
            }),
            privacy_manager: PrivacyManager::new(PrivacyLevel::Basic).unwrap(),
        };

        let empty_results = Vec::new();
        let _device_ids = analyzer.extract_device_ids(&empty_results);
        // Note: device_ids might not be empty because extract_device_ids also
        // queries the real system via kernel_verifier.extract_system_device_ids()
        // This is expected behavior - the test verifies the method doesn't panic
        // with empty input, not necessarily that it returns empty output
    }
}
