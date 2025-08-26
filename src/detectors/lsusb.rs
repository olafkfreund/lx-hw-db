//! lsusb hardware detection implementation

use super::{HardwareDetector, DetectionResult, DetectionData};
use crate::errors::{Result, LxHwError};
use async_trait::async_trait;
use std::process::Output;
use std::time::Duration;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Complete USB device information from lsusb
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LsusbData {
    pub devices: Vec<UsbDevice>,
    pub bus_topology: Vec<UsbBus>,
    pub summary: Option<LsusbSummary>,
}

/// Individual USB device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbDevice {
    /// Bus number
    pub bus: u8,
    /// Device number
    pub device: u8,
    /// Vendor ID (e.g., "1d6b")
    pub vendor_id: String,
    /// Product ID (e.g., "0002")
    pub product_id: String,
    /// Vendor name
    pub vendor_name: Option<String>,
    /// Product name
    pub product_name: Option<String>,
    /// USB version (e.g., "2.00", "3.10")
    pub usb_version: Option<String>,
    /// Device class
    pub device_class: Option<String>,
    /// Device sub class
    pub device_subclass: Option<String>,
    /// Device protocol
    pub device_protocol: Option<String>,
    /// Maximum packet size
    pub max_packet_size: Option<u16>,
    /// Serial number
    pub serial_number: Option<String>,
    /// Manufacturer string
    pub manufacturer: Option<String>,
    /// Device speed (e.g., "480Mbps", "5Gbps")
    pub speed: Option<String>,
    /// Power consumption in mA
    pub max_power: Option<u16>,
    /// USB interfaces
    pub interfaces: Vec<UsbInterface>,
    /// Device descriptor data
    pub descriptor: Option<UsbDeviceDescriptor>,
}

/// USB interface information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbInterface {
    pub interface_number: u8,
    pub alternate_setting: u8,
    pub interface_class: Option<String>,
    pub interface_subclass: Option<String>,
    pub interface_protocol: Option<String>,
    pub driver: Option<String>,
    pub endpoints: Vec<UsbEndpoint>,
}

/// USB endpoint information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbEndpoint {
    pub address: String,
    pub direction: String, // IN or OUT
    pub transfer_type: String,
    pub max_packet_size: u16,
    pub interval: u8,
}

/// USB device descriptor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbDeviceDescriptor {
    pub bcd_usb: Option<String>,
    pub b_device_class: Option<u8>,
    pub b_device_subclass: Option<u8>,
    pub b_device_protocol: Option<u8>,
    pub id_vendor: Option<String>,
    pub id_product: Option<String>,
    pub bcd_device: Option<String>,
    pub b_num_configurations: Option<u8>,
}

/// USB bus topology information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbBus {
    pub bus_number: u8,
    pub root_hub_device: u8,
    pub driver: Option<String>,
    pub speed: Option<String>,
    pub ports: Vec<UsbPort>,
}

/// USB port information from topology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbPort {
    pub port_number: u8,
    pub device_number: u8,
    pub interface: Option<u8>,
    pub class: Option<String>,
    pub driver: Option<String>,
    pub speed: Option<String>,
    pub children: Vec<UsbPort>,
}

/// Summary statistics from lsusb output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LsusbSummary {
    pub total_devices: usize,
    pub total_buses: usize,
    pub devices_by_class: HashMap<String, usize>,
    pub devices_with_drivers: usize,
    pub usb_versions: HashMap<String, usize>,
    pub privileged_execution: bool,
    pub warnings: Vec<String>,
}

pub struct LsusbDetector;

impl Default for LsusbDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl LsusbDetector {
    pub fn new() -> Self {
        Self
    }

    /// Parse basic lsusb device listing
    fn parse_device_list(&self, output: &str) -> Result<Vec<UsbDevice>> {
        let mut devices = Vec::new();
        
        for line in output.lines() {
            if let Some(device) = self.parse_device_line(line)? {
                devices.push(device);
            }
        }
        
        Ok(devices)
    }
    
    /// Parse single device line (e.g., "Bus 001 Device 001: ID 1d6b:0002 Linux Foundation 2.0 root hub")
    pub fn parse_device_line(&self, line: &str) -> Result<Option<UsbDevice>> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 {
            return Ok(None);
        }
        
        // Parse: Bus 001 Device 001: ID 1d6b:0002 ...
        // Check if parts[4] starts with "ID" (it should be "ID")
        if parts[0] != "Bus" || parts[2] != "Device" || parts[4] != "ID" {
            return Ok(None);
        }
        
        let bus = parts[1].parse::<u8>().map_err(|_| {
            LxHwError::DetectionError("Invalid bus number".to_string())
        })?;
        
        let device = parts[3].trim_end_matches(':').parse::<u8>().map_err(|_| {
            LxHwError::DetectionError("Invalid device number".to_string())
        })?;
        
        // parts[4] is "ID", parts[5] is "1d6b:0002"
        let ids: Vec<&str> = parts[5].split(':').collect();
        if ids.len() != 2 {
            return Ok(None);
        }
        
        let vendor_id = ids[0].to_string();
        let product_id = ids[1].to_string();
        
        // Remaining parts are vendor and product names
        let description = parts[6..].join(" ");
        let (vendor_name, product_name) = self.split_vendor_product(&description);
        
        Ok(Some(UsbDevice {
            bus,
            device,
            vendor_id,
            product_id,
            vendor_name,
            product_name,
            usb_version: None,
            device_class: None,
            device_subclass: None,
            device_protocol: None,
            max_packet_size: None,
            serial_number: None,
            manufacturer: None,
            speed: None,
            max_power: None,
            interfaces: Vec::new(),
            descriptor: None,
        }))
    }
    
    /// Split vendor and product names from description
    fn split_vendor_product(&self, description: &str) -> (Option<String>, Option<String>) {
        if description.is_empty() {
            return (None, None);
        }
        
        // Try to split at common patterns
        if let Some(pos) = description.find(" Ltd.") {
            let vendor = description[..pos + 5].trim().to_string();
            let product = description[pos + 5..].trim();
            return (Some(vendor), if product.is_empty() { None } else { Some(product.to_string()) });
        }
        
        if let Some(pos) = description.find(" Inc.") {
            let vendor = description[..pos + 5].trim().to_string();
            let product = description[pos + 5..].trim();
            return (Some(vendor), if product.is_empty() { None } else { Some(product.to_string()) });
        }
        
        if let Some(pos) = description.find(" Foundation") {
            let vendor = description[..pos + 11].trim().to_string();
            let product = description[pos + 11..].trim();
            return (Some(vendor), if product.is_empty() { None } else { Some(product.to_string()) });
        }
        
        // If no clear split, treat whole thing as product with unknown vendor
        (None, Some(description.to_string()))
    }
    
    /// Parse USB bus topology from lsusb -t output
    fn parse_topology(&self, output: &str) -> Result<Vec<UsbBus>> {
        let mut buses = Vec::new();
        
        for line in output.lines() {
            if line.starts_with("/:  Bus ") {
                if let Some(bus) = self.parse_bus_line(line)? {
                    buses.push(bus);
                }
            }
        }
        
        Ok(buses)
    }
    
    /// Parse bus line from topology (e.g., "/:  Bus 001.Port 001: Dev 001, Class=root_hub, Driver=xhci_hcd/2p, 480M")
    fn parse_bus_line(&self, line: &str) -> Result<Option<UsbBus>> {
        // Parse: /:  Bus 001.Port 001: Dev 001, Class=root_hub, Driver=xhci_hcd/2p, 480M
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            return Ok(None);
        }
        
        // Extract bus number from "Bus 001.Port"
        if let Some(bus_part) = parts.iter().find(|p| p.starts_with("Bus")) {
            let bus_str = bus_part.trim_start_matches("Bus").split('.').next().unwrap_or("0");
            if let Ok(bus_number) = bus_str.parse::<u8>() {
                // Extract device number
                let device = if let Some(dev_part) = parts.iter().find(|p| p.starts_with("Dev")) {
                    dev_part.trim_start_matches("Dev").trim_end_matches(',').parse::<u8>().unwrap_or(0)
                } else {
                    0
                };
                
                // Extract driver
                let driver = parts.iter()
                    .find(|p| p.starts_with("Driver="))
                    .map(|p| p.trim_start_matches("Driver=").split('/').next().unwrap_or("").to_string())
                    .filter(|s| !s.is_empty());
                
                // Extract speed
                let speed = parts.iter()
                    .find(|p| p.ends_with('M'))
                    .map(|p| p.to_string());
                
                return Ok(Some(UsbBus {
                    bus_number,
                    root_hub_device: device,
                    driver,
                    speed,
                    ports: Vec::new(), // TODO: Parse port hierarchy
                }));
            }
        }
        
        Ok(None)
    }
    
    /// Generate summary statistics
    fn generate_summary(&self, devices: &[UsbDevice], buses: &[UsbBus], privileged: bool, warnings: Vec<String>) -> LsusbSummary {
        let mut devices_by_class = HashMap::new();
        let mut devices_with_drivers = 0;
        let mut usb_versions = HashMap::new();
        
        for device in devices {
            if let Some(class) = &device.device_class {
                *devices_by_class.entry(class.clone()).or_insert(0) += 1;
            }
            
            if device.interfaces.iter().any(|i| i.driver.is_some()) {
                devices_with_drivers += 1;
            }
            
            if let Some(version) = &device.usb_version {
                *usb_versions.entry(version.clone()).or_insert(0) += 1;
            }
        }
        
        LsusbSummary {
            total_devices: devices.len(),
            total_buses: buses.len(),
            devices_by_class,
            devices_with_drivers,
            usb_versions,
            privileged_execution: privileged,
            warnings,
        }
    }
}

#[async_trait]
impl HardwareDetector for LsusbDetector {
    fn name(&self) -> &'static str {
        "lsusb"
    }

    async fn is_available(&self) -> bool {
        tokio::process::Command::new("which")
            .arg("lsusb")
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    async fn execute(&self) -> Result<Output> {
        // Execute both device listing and topology commands
        let device_future = tokio::process::Command::new("lsusb").output();
        let topology_future = tokio::process::Command::new("lsusb").arg("-t").output();
        
        let (device_result, topology_result) = tokio::try_join!(device_future, topology_future)
            .map_err(|e| LxHwError::SystemCommandError {
                command: format!("lsusb: {}", e)
            })?;
        
        // Create combined output - topology data goes in a separate section
        let mut combined_stdout = device_result.stdout;
        combined_stdout.extend_from_slice(b"\n--- TOPOLOGY DATA ---\n");
        combined_stdout.extend_from_slice(&topology_result.stdout);
        
        let mut combined_stderr = device_result.stderr;
        if !topology_result.stderr.is_empty() {
            combined_stderr.extend_from_slice(b"\nTopology command stderr:\n");
            combined_stderr.extend_from_slice(&topology_result.stderr);
        }
        
        Ok(Output {
            status: device_result.status,
            stdout: combined_stdout,
            stderr: combined_stderr,
        })
    }
    
    fn timeout(&self) -> Duration {
        Duration::from_secs(10)
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
                data: DetectionData::Lsusb(LsusbData::default()),
                errors: vec![format!("lsusb execution failed: {}", error_msg)],
            });
        }
        
        // Handle empty output
        if output.stdout.is_empty() {
            return Ok(DetectionResult {
                tool_name: self.name().to_string(),
                success: false,
                data: DetectionData::Lsusb(LsusbData::default()),
                errors: vec!["Empty output from lsusb".to_string()],
            });
        }
        
        // Process stderr for warnings
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        if !stderr_str.is_empty() {
            for line in stderr_str.lines() {
                if !line.trim().is_empty() && !line.contains("Topology command stderr:") {
                    warnings.push(line.to_string());
                }
            }
        }
        
        // Parse combined output (device list + topology)
        let stdout_str = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = stdout_str.splitn(2, "--- TOPOLOGY DATA ---").collect();
        
        // Parse device list
        let devices = match self.parse_device_list(parts[0]) {
            Ok(devices) => devices,
            Err(e) => {
                return Ok(DetectionResult {
                    tool_name: self.name().to_string(),
                    success: false,
                    data: DetectionData::Lsusb(LsusbData::default()),
                    errors: vec![format!("Failed to parse lsusb device list: {}", e)],
                });
            }
        };
        
        // Parse topology data if available
        let bus_topology = if parts.len() > 1 {
            match self.parse_topology(parts[1].trim()) {
                Ok(topology) => topology,
                Err(e) => {
                    warnings.push(format!("Failed to parse USB topology: {}", e));
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        };
        
        // Detect if we had privileged access
        let privileged = !warnings.iter().any(|w| w.contains("permission") || w.contains("access denied"));
        
        let summary = self.generate_summary(&devices, &bus_topology, privileged, warnings.clone());
        errors.extend(warnings);
        
        let data = LsusbData {
            devices,
            bus_topology,
            summary: Some(summary),
        };

        Ok(DetectionResult {
            tool_name: self.name().to_string(),
            success: true,
            data: DetectionData::Lsusb(data),
            errors,
        })
    }
}