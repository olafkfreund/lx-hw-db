//! lspci hardware detection implementation

use super::{HardwareDetector, DetectionResult, DetectionData};
use crate::errors::{Result, LxHwError};
use async_trait::async_trait;
use std::process::Output;
use std::time::Duration;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Complete PCI device information from lspci
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LspciData {
    pub devices: Vec<PciDevice>,
    pub summary: Option<LspciSummary>,
}

/// Individual PCI device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PciDevice {
    /// PCI address (e.g., "00:01.0")
    pub address: String,
    /// Device class code (e.g., "0600")
    pub class_code: String,
    /// Device class description (e.g., "Host bridge")
    pub class_description: String,
    /// Vendor ID (e.g., "1022")
    pub vendor_id: String,
    /// Device ID (e.g., "1480")
    pub device_id: String,
    /// Vendor name (e.g., "Advanced Micro Devices, Inc. [AMD]")
    pub vendor_name: Option<String>,
    /// Device name/product
    pub device_name: Option<String>,
    /// Subsystem information
    pub subsystem: Option<String>,
    /// Device flags and capabilities
    pub flags: Vec<String>,
    /// Kernel driver in use
    pub kernel_driver: Option<String>,
    /// Kernel modules available
    pub kernel_modules: Vec<String>,
    /// Bus information
    pub bus_info: Option<PciBusInfo>,
    /// Memory ranges
    pub memory_regions: Vec<String>,
    /// I/O ports
    pub io_ports: Vec<String>,
    /// IRQ information
    pub irq: Option<u32>,
    /// IOMMU group
    pub iommu_group: Option<u32>,
    /// Revision
    pub revision: Option<String>,
}

/// PCI bus topology information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PciBusInfo {
    pub primary: Option<u8>,
    pub secondary: Option<u8>,
    pub subordinate: Option<u8>,
    pub sec_latency: Option<u8>,
}

/// Summary statistics from lspci output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspciSummary {
    pub total_devices: usize,
    pub devices_by_class: HashMap<String, usize>,
    pub devices_with_drivers: usize,
    pub privileged_execution: bool,
    pub warnings: Vec<String>,
}

type NumericData = (String, String, String, Option<String>);

pub struct LspciDetector;

impl Default for LspciDetector {
    fn default() -> Self {
        Self
    }
}

impl LspciDetector {
    pub fn new() -> Self {
        Self
    }

    /// Parse verbose lspci output into structured data
    fn parse_verbose_output(&self, output: &str) -> Result<Vec<PciDevice>> {
        let mut devices = Vec::new();
        let mut current_device: Option<PciDevice> = None;
        
        for line in output.lines() {
            let line = line.trim_end();
            
            if line.is_empty() {
                if let Some(device) = current_device.take() {
                    devices.push(device);
                }
                continue;
            }
            
            if !line.starts_with('\t') {
                // New device line
                if let Some(device) = current_device.take() {
                    devices.push(device);
                }
                
                if let Some(device) = self.parse_device_header(line)? {
                    current_device = Some(device);
                }
            } else {
                // Device property line
                if let Some(ref mut device) = current_device {
                    self.parse_device_property(device, line.trim_start())?;
                }
            }
        }
        
        // Don't forget the last device
        if let Some(device) = current_device {
            devices.push(device);
        }
        
        Ok(devices)
    }
    
    /// Parse device header line (e.g., "00:01.0 Host bridge: AMD [1022:1480]")
    fn parse_device_header(&self, line: &str) -> Result<Option<PciDevice>> {
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() < 2 {
            return Ok(None);
        }
        
        let address = parts[0].to_string();
        let rest = parts[1];
        
        // Parse class and description
        let class_desc_parts: Vec<&str> = rest.splitn(2, ": ").collect();
        if class_desc_parts.len() < 2 {
            return Ok(None);
        }
        
        let class_description = class_desc_parts[0].to_string();
        let device_info = class_desc_parts[1];
        
        // Extract vendor and device names
        let (vendor_name, device_name) = if device_info.contains('[') && device_info.contains(']') {
            let bracket_start = device_info.find('[').unwrap();
            let vendor_device = &device_info[..bracket_start].trim();
            (Some(vendor_device.to_string()), None)
        } else {
            (Some(device_info.to_string()), None)
        };
        
        Ok(Some(PciDevice {
            address,
            class_code: String::new(), // Will be filled from numeric output
            class_description,
            vendor_id: String::new(),
            device_id: String::new(),
            vendor_name,
            device_name,
            subsystem: None,
            flags: Vec::new(),
            kernel_driver: None,
            kernel_modules: Vec::new(),
            bus_info: None,
            memory_regions: Vec::new(),
            io_ports: Vec::new(),
            irq: None,
            iommu_group: None,
            revision: None,
        }))
    }
    
    /// Parse device property lines (subsystem, flags, etc.)
    fn parse_device_property(&self, device: &mut PciDevice, line: &str) -> Result<()> {
        if let Some(subsystem) = line.strip_prefix("Subsystem: ") {
            device.subsystem = Some(subsystem.to_string());
        } else if let Some(flags_str) = line.strip_prefix("Flags: ") {
            device.flags = flags_str.split(", ").map(|s| s.to_string()).collect();
            
            // Extract IRQ from flags
            for flag in &device.flags {
                if let Some(irq_str) = flag.strip_prefix("IRQ ") {
                    if let Ok(irq) = irq_str.parse::<u32>() {
                        device.irq = Some(irq);
                    }
                }
                if let Some(group_str) = flag.strip_prefix("IOMMU group ") {
                    if let Ok(group) = group_str.parse::<u32>() {
                        device.iommu_group = Some(group);
                    }
                }
            }
        } else if let Some(bus_str) = line.strip_prefix("Bus: ") {
            device.bus_info = self.parse_bus_info(bus_str);
        } else if line.starts_with("I/O behind bridge: ") || line.starts_with("I/O ports at ") {
            device.io_ports.push(line.to_string());
        } else if line.starts_with("Memory behind bridge: ") || line.starts_with("Memory at ") {
            device.memory_regions.push(line.to_string());
        } else if let Some(driver_str) = line.strip_prefix("Kernel driver in use: ") {
            device.kernel_driver = Some(driver_str.to_string());
        } else if let Some(modules_str) = line.strip_prefix("Kernel modules: ") {
            device.kernel_modules = modules_str.split(", ").map(|s| s.to_string()).collect();
        }
        
        Ok(())
    }
    
    /// Parse bus information (e.g., "primary=00, secondary=01, subordinate=01, sec-latency=0")
    fn parse_bus_info(&self, bus_line: &str) -> Option<PciBusInfo> {
        let mut bus_info = PciBusInfo {
            primary: None,
            secondary: None,
            subordinate: None,
            sec_latency: None,
        };
        
        for part in bus_line.split(", ") {
            let key_value: Vec<&str> = part.splitn(2, '=').collect();
            if key_value.len() == 2 {
                let value = key_value[1].parse::<u8>().ok();
                match key_value[0] {
                    "primary" => bus_info.primary = value,
                    "secondary" => bus_info.secondary = value,
                    "subordinate" => bus_info.subordinate = value,
                    "sec-latency" => bus_info.sec_latency = value,
                    _ => {}
                }
            }
        }
        
        Some(bus_info)
    }
    
    /// Parse numeric lspci output to get vendor/device IDs and class codes
    fn parse_numeric_output(&self, output: &str) -> Result<HashMap<String, NumericData>> {
        let mut numeric_data = HashMap::new();
        
        for line in output.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let address = parts[0];
                let class_code = parts[1].trim_end_matches(':');
                let vendor_device = parts[2];
                let revision = parts.get(3).map(|s| s.trim_start_matches("(rev ").trim_end_matches(')').to_string());
                
                if let Some(colon_pos) = vendor_device.find(':') {
                    let vendor_id = &vendor_device[..colon_pos];
                    let device_id = &vendor_device[colon_pos + 1..];
                    
                    numeric_data.insert(
                        address.to_string(),
                        (class_code.to_string(), vendor_id.to_string(), device_id.to_string(), revision)
                    );
                }
            }
        }
        
        Ok(numeric_data)
    }
    
    /// Generate summary statistics
    fn generate_summary(&self, devices: &[PciDevice], privileged: bool, warnings: Vec<String>) -> LspciSummary {
        let mut devices_by_class = HashMap::new();
        let mut devices_with_drivers = 0;
        
        for device in devices {
            *devices_by_class.entry(device.class_description.clone()).or_insert(0) += 1;
            
            if device.kernel_driver.is_some() {
                devices_with_drivers += 1;
            }
        }
        
        LspciSummary {
            total_devices: devices.len(),
            devices_by_class,
            devices_with_drivers,
            privileged_execution: privileged,
            warnings,
        }
    }
}

#[async_trait]
impl HardwareDetector for LspciDetector {
    fn name(&self) -> &'static str {
        "lspci"
    }

    async fn is_available(&self) -> bool {
        tokio::process::Command::new("which")
            .arg("lspci")
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    async fn execute(&self) -> Result<Output> {
        // Execute both verbose and numeric commands and combine results
        let verbose_future = tokio::process::Command::new("lspci")
            .arg("-v")  // verbose output
            .arg("-k")  // show kernel drivers
            .output();
            
        let numeric_future = tokio::process::Command::new("lspci")
            .arg("-n")  // numeric IDs
            .output();
        
        let (verbose_result, numeric_result) = tokio::try_join!(verbose_future, numeric_future)
            .map_err(|e| LxHwError::SystemCommandError {
                command: format!("lspci: {}", e)
            })?;
        
        // Create combined output - we'll put numeric data in stderr for parsing
        let mut combined_stdout = verbose_result.stdout;
        combined_stdout.extend_from_slice(b"\n--- NUMERIC DATA ---\n");
        combined_stdout.extend_from_slice(&numeric_result.stdout);
        
        let mut combined_stderr = verbose_result.stderr;
        if !numeric_result.stderr.is_empty() {
            combined_stderr.extend_from_slice(b"\nNumeric command stderr:\n");
            combined_stderr.extend_from_slice(&numeric_result.stderr);
        }
        
        Ok(Output {
            status: verbose_result.status,
            stdout: combined_stdout,
            stderr: combined_stderr,
        })
    }
    
    fn timeout(&self) -> Duration {
        Duration::from_secs(15)
    }

    fn parse_output(&self, output: &Output) -> Result<DetectionResult> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        
        // Check for execution errors
        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Ok(DetectionResult {
                tool_name: self.name().to_string(),
                success: false,
                data: DetectionData::Lspci(LspciData::default()),
                errors: vec![format!("lspci execution failed: {}", error_msg)],
            });
        }
        
        // Handle empty output
        if output.stdout.is_empty() {
            return Ok(DetectionResult {
                tool_name: self.name().to_string(),
                success: false,
                data: DetectionData::Lspci(LspciData::default()),
                errors: vec!["Empty output from lspci".to_string()],
            });
        }
        
        // Process stderr for warnings
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        if !stderr_str.is_empty() {
            for line in stderr_str.lines() {
                if !line.trim().is_empty() {
                    warnings.push(line.to_string());
                }
            }
        }
        
        // Parse combined output (verbose + numeric)
        let stdout_str = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = stdout_str.splitn(2, "--- NUMERIC DATA ---").collect();
        
        // Parse verbose output
        let mut devices = match self.parse_verbose_output(parts[0]) {
            Ok(devices) => devices,
            Err(e) => {
                return Ok(DetectionResult {
                    tool_name: self.name().to_string(),
                    success: false,
                    data: DetectionData::Lspci(LspciData::default()),
                    errors: vec![format!("Failed to parse lspci verbose output: {}", e)],
                });
            }
        };
        
        // Parse numeric data if available
        if parts.len() > 1 {
            match self.parse_numeric_output(parts[1].trim()) {
                Ok(numeric_data) => {
                    // Merge numeric data with verbose data
                    for device in &mut devices {
                        if let Some((class_code, vendor_id, device_id, revision)) = numeric_data.get(&device.address) {
                            device.class_code = class_code.clone();
                            device.vendor_id = vendor_id.clone();
                            device.device_id = device_id.clone();
                            device.revision = revision.clone();
                        }
                    }
                }
                Err(e) => {
                    warnings.push(format!("Failed to parse numeric lspci output: {}", e));
                }
            }
        }
        
        // Detect if we had privileged access
        let privileged = !warnings.iter().any(|w| w.contains("access denied") || w.contains("permission"));
        
        let summary = self.generate_summary(&devices, privileged, warnings.clone());
        errors.extend(warnings);
        
        let data = LspciData {
            devices,
            summary: Some(summary),
        };

        Ok(DetectionResult {
            tool_name: self.name().to_string(),
            success: true,
            data: DetectionData::Lspci(data),
            errors,
        })
    }
}