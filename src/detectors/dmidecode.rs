//! dmidecode hardware detection implementation
//! 
//! dmidecode reads BIOS/UEFI data structures (also called DMI or SMBIOS tables)
//! to extract hardware information about BIOS, system, baseboard, processor, and memory.
//! This provides complementary information to lshw, particularly for BIOS details
//! and memory module specifications.

use super::{HardwareDetector, DetectionResult, DetectionData};
use crate::errors::{Result, LxHwError};
use async_trait::async_trait;
use std::process::{Output, Command};
use std::collections::HashMap;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use log::{debug, warn, error};

/// Complete hardware information from dmidecode
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DmidecodeData {
    /// BIOS/UEFI information
    pub bios: Option<BiosInfo>,
    /// System information (motherboard/chassis)
    pub system: Option<SystemInfo>,
    /// Baseboard (motherboard) information
    pub baseboard: Option<BaseboardInfo>,
    /// Processor information
    pub processors: Vec<ProcessorInfo>,
    /// Memory devices (RAM modules)
    pub memory_devices: Vec<MemoryDevice>,
    /// Summary statistics and metadata
    pub summary: Option<DmidecodeSummary>,
}

/// BIOS/UEFI information from DMI type 0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiosInfo {
    pub vendor: String,
    pub version: String,
    pub release_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rom_size: Option<String>,
    pub characteristics: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bios_revision: Option<String>,
}

/// System information from DMI type 1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub manufacturer: String,
    pub product_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wake_up_type: Option<String>,
}

/// Baseboard information from DMI type 2  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseboardInfo {
    pub manufacturer: String,
    pub product_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_tag: Option<String>,
    pub features: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_in_chassis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub board_type: Option<String>,
}

/// Processor information from DMI type 4
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorInfo {
    pub socket_designation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    pub manufacturer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    pub flags: Vec<String>,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voltage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_clock: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_speed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_speed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_enabled: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_count: Option<u32>,
    pub characteristics: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
}

/// Memory device information from DMI type 17
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryDevice {
    pub locator: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_locator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_mb: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_factor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_mts: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_speed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_voltage: Option<String>,
}

/// Summary metadata for dmidecode detection run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DmidecodeSummary {
    pub total_memory_mb: u64,
    pub memory_slots_used: usize,
    pub processor_count: usize,
    pub privileged_execution: bool,
    pub bios_present: bool,
    pub system_info_present: bool,
    pub baseboard_present: bool,
    pub warnings: Vec<String>,
}

pub struct DmidecodeDetector;

impl Default for DmidecodeDetector {
    fn default() -> Self {
        Self
    }
}

impl DmidecodeDetector {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl HardwareDetector for DmidecodeDetector {
    fn name(&self) -> &'static str {
        "dmidecode"
    }

    async fn is_available(&self) -> bool {
        // Try multiple methods to check for dmidecode availability
        let which_result = Command::new("which")
            .arg("dmidecode")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false);
            
        if which_result {
            return true;
        }
        
        // Also check common installation paths
        for path in &["/usr/bin/dmidecode", "/usr/sbin/dmidecode", "/sbin/dmidecode"] {
            if std::path::Path::new(path).exists() {
                return true;
            }
        }
        
        false
    }

    async fn execute(&self) -> Result<Output> {
        debug!("Executing dmidecode hardware detection");
        
        let output = tokio::process::Command::new("dmidecode")
            .arg("-t") // Specify types to read
            .arg("system,baseboard,bios,processor,memory") // Focus on key hardware types
            .arg("-q") // Quiet mode - less verbose output
            .output()
            .await
            .map_err(|_e| LxHwError::SystemCommandError {
                command: "dmidecode -t system,baseboard,bios,processor,memory -q".to_string()
            })?;
            
        debug!("dmidecode execution completed with status: {}", output.status);
        
        // dmidecode may return non-zero exit status due to privilege issues but still provide data
        if !output.status.success() && output.stdout.is_empty() {
            // Check if it's a permission error
            let stderr_str = String::from_utf8_lossy(&output.stderr);
            if stderr_str.contains("Permission denied") || stderr_str.contains("Operation not permitted") {
                debug!("dmidecode requires elevated privileges");
            } else {
                return Err(LxHwError::DetectionError(
                    format!("dmidecode failed with exit code: {} and stderr: {}", 
                           output.status.code().unwrap_or(-1),
                           stderr_str)
                ));
            }
        }
        
        Ok(output)
    }

    fn parse_output(&self, output: &Output) -> Result<DetectionResult> {
        let mut errors = Vec::new();
        
        // Handle stderr warnings and errors
        if !output.stderr.is_empty() {
            let stderr_str = String::from_utf8_lossy(&output.stderr);
            for line in stderr_str.lines() {
                if !line.trim().is_empty() {
                    warn!("dmidecode stderr: {}", line);
                    errors.push(format!("Warning: {}", line));
                }
            }
        }
        
        // Parse stdout text
        if output.stdout.is_empty() {
            errors.push("Empty output from dmidecode".to_string());
            return Ok(DetectionResult {
                tool_name: self.name().to_string(),
                success: false,
                data: DetectionData::Dmidecode(Box::default()),
                errors,
            });
        }
        
        let stdout_str = String::from_utf8_lossy(&output.stdout);
        debug!("Parsing dmidecode text output ({} bytes)", stdout_str.len());
        
        match self.parse_dmidecode_text(&stdout_str) {
            Ok(mut dmidecode_data) => {
                debug!("Successfully parsed dmidecode output - bios: {}, system: {}, processors: {}, memory: {}", 
                       dmidecode_data.bios.is_some(),
                       dmidecode_data.system.is_some(),
                       dmidecode_data.processors.len(),
                       dmidecode_data.memory_devices.len());
                
                // Generate summary with error context for privilege detection
                dmidecode_data.summary = Some(self.generate_summary_with_errors(&dmidecode_data, &errors));
                
                Ok(DetectionResult {
                    tool_name: self.name().to_string(),
                    success: true,
                    data: DetectionData::Dmidecode(Box::new(dmidecode_data)),
                    errors,
                })
            }
            Err(e) => {
                error!("Failed to parse dmidecode text: {}", e);
                errors.push(format!("Text parsing failed: {}", e));
                Ok(DetectionResult {
                    tool_name: self.name().to_string(),
                    success: false,
                    data: DetectionData::Dmidecode(Box::default()),
                    errors,
                })
            }
        }
    }
    
    fn timeout(&self) -> Duration {
        // dmidecode is typically faster than lshw since it reads from /sys
        Duration::from_secs(15)
    }
}

impl DmidecodeDetector {
    /// Parse dmidecode text output into structured data
    fn parse_dmidecode_text(&self, text: &str) -> Result<DmidecodeData> {
        let mut dmidecode_data = DmidecodeData::default();
        let mut current_section = String::new();
        let mut current_handle = HashMap::new();
        
        let lines: Vec<&str> = text.lines().collect();
        let mut i = 0;
        
        while i < lines.len() {
            let line = lines[i];
            
            // Skip empty lines and comments (but don't trim yet to preserve tabs)
            if line.trim().is_empty() || line.starts_with('#') {
                i += 1;
                continue;
            }
            
            // Check for handle start
            if line.starts_with("Handle ") {
                // Process previous handle if it exists
                if !current_section.is_empty() {
                    self.process_handle(&current_section, &current_handle, &mut dmidecode_data);
                }
                
                // Reset for new handle
                current_handle.clear();
                current_section.clear();
                i += 1;
                continue;
            }
            
            // Check for section headers (e.g., "BIOS Information", "System Information", "Base Board Information", "Memory Device")
            // These are not indented and contain "Information" or are "Memory Device"
            if !line.starts_with('\t') && (line.contains("Information") || line == "Memory Device") {
                current_section = line.to_string();
                i += 1;
                continue;
            }
            
            // Parse key-value pairs (indented lines)
            if line.starts_with('\t') && line.contains(':') {
                let parts: Vec<&str> = line[1..].splitn(2, ':').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();
                    
                    // Handle multi-line values (characteristics, flags, etc.)
                    if value.is_empty() {
                        let mut multi_values = Vec::new();
                        i += 1;
                        
                        // Collect indented sub-items
                        while i < lines.len() && lines[i].starts_with("\t\t") {
                            let sub_value = lines[i][2..].trim();
                            if !sub_value.is_empty() {
                                multi_values.push(sub_value.to_string());
                            }
                            i += 1;
                        }
                        
                        current_handle.insert(key.to_string(), multi_values.join(","));
                        continue;
                    } else {
                        current_handle.insert(key.to_string(), value.to_string());
                    }
                }
            }
            
            i += 1;
        }
        
        // Process the last handle
        if !current_section.is_empty() {
            self.process_handle(&current_section, &current_handle, &mut dmidecode_data);
        }
        
        // Note: errors are passed separately in parse_output, summary is generated there
        
        Ok(dmidecode_data)
    }
    
    /// Process a parsed handle into the appropriate data structure
    fn process_handle(&self, section: &str, data: &HashMap<String, String>, dmidecode_data: &mut DmidecodeData) {
        match section {
            "BIOS Information" => {
                if let Some(bios) = self.parse_bios_info(data) {
                    dmidecode_data.bios = Some(bios);
                }
            }
            "System Information" => {
                if let Some(system) = self.parse_system_info(data) {
                    dmidecode_data.system = Some(system);
                }
            }
            "Base Board Information" => {
                if let Some(baseboard) = self.parse_baseboard_info(data) {
                    dmidecode_data.baseboard = Some(baseboard);
                }
            }
            "Processor Information" => {
                if let Some(processor) = self.parse_processor_info(data) {
                    dmidecode_data.processors.push(processor);
                }
            }
            "Memory Device" => {
                if let Some(memory) = self.parse_memory_device(data) {
                    dmidecode_data.memory_devices.push(memory);
                }
            }
            _ => {
                debug!("Skipping unknown section: {}", section);
            }
        }
    }
    
    /// Parse BIOS information
    fn parse_bios_info(&self, data: &HashMap<String, String>) -> Option<BiosInfo> {
        let vendor = data.get("Vendor")?.clone();
        let version = data.get("Version")?.clone();
        let release_date = data.get("Release Date")?.clone();
        
        let characteristics = data.get("Characteristics")
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default();
        
        Some(BiosInfo {
            vendor,
            version,
            release_date,
            address: data.get("Address").cloned(),
            runtime_size: data.get("Runtime Size").cloned(),
            rom_size: data.get("ROM Size").cloned(),
            characteristics,
            bios_revision: data.get("BIOS Revision").cloned(),
        })
    }
    
    /// Parse system information
    fn parse_system_info(&self, data: &HashMap<String, String>) -> Option<SystemInfo> {
        let manufacturer = data.get("Manufacturer")?.clone();
        let product_name = data.get("Product Name")?.clone();
        
        Some(SystemInfo {
            manufacturer,
            product_name,
            version: data.get("Version").cloned(),
            serial_number: data.get("Serial Number").cloned(),
            uuid: data.get("UUID").cloned(),
            sku_number: data.get("SKU Number").cloned(),
            family: data.get("Family").cloned(),
            wake_up_type: data.get("Wake-up Type").cloned(),
        })
    }
    
    /// Parse baseboard information
    fn parse_baseboard_info(&self, data: &HashMap<String, String>) -> Option<BaseboardInfo> {
        let manufacturer = data.get("Manufacturer")?.clone();
        let product_name = data.get("Product Name")?.clone();
        
        let features = data.get("Features")
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default();
        
        Some(BaseboardInfo {
            manufacturer,
            product_name,
            version: data.get("Version").cloned(),
            serial_number: data.get("Serial Number").cloned(),
            asset_tag: data.get("Asset Tag").cloned(),
            features,
            location_in_chassis: data.get("Location In Chassis").cloned(),
            board_type: data.get("Type").cloned(),
        })
    }
    
    /// Parse processor information
    fn parse_processor_info(&self, data: &HashMap<String, String>) -> Option<ProcessorInfo> {
        let socket_designation = data.get("Socket Designation")?.clone();
        let manufacturer = data.get("Manufacturer")?.clone();
        let version = data.get("Version")?.clone();
        
        let flags = data.get("Flags")
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default();
            
        let characteristics = data.get("Characteristics")
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default();
        
        Some(ProcessorInfo {
            socket_designation,
            processor_type: data.get("Type").cloned(),
            family: data.get("Family").cloned(),
            manufacturer,
            id: data.get("ID").cloned(),
            signature: data.get("Signature").cloned(),
            flags,
            version,
            voltage: data.get("Voltage").cloned(),
            external_clock: self.parse_frequency(data.get("External Clock")),
            max_speed: self.parse_frequency(data.get("Max Speed")),
            current_speed: self.parse_frequency(data.get("Current Speed")),
            status: data.get("Status").cloned(),
            upgrade: data.get("Upgrade").cloned(),
            core_count: self.parse_u32(data.get("Core Count")),
            core_enabled: self.parse_u32(data.get("Core Enabled")),
            thread_count: self.parse_u32(data.get("Thread Count")),
            characteristics,
            serial_number: data.get("Serial Number").cloned(),
            asset_tag: data.get("Asset Tag").cloned(),
            part_number: data.get("Part Number").cloned(),
        })
    }
    
    /// Parse memory device information
    fn parse_memory_device(&self, data: &HashMap<String, String>) -> Option<MemoryDevice> {
        let locator = data.get("Locator")?.clone();
        
        Some(MemoryDevice {
            locator,
            bank_locator: data.get("Bank Locator").cloned(),
            total_width: self.parse_u32(data.get("Total Width")),
            data_width: self.parse_u32(data.get("Data Width")),
            size_mb: self.parse_memory_size(data.get("Size")),
            form_factor: data.get("Form Factor").cloned(),
            memory_type: data.get("Type").cloned(),
            type_detail: data.get("Type Detail").cloned(),
            speed_mts: self.parse_frequency(data.get("Speed")),
            manufacturer: data.get("Manufacturer").cloned(),
            serial_number: data.get("Serial Number").cloned(),
            asset_tag: data.get("Asset Tag").cloned(),
            part_number: data.get("Part Number").cloned(),
            rank: self.parse_u32(data.get("Rank")),
            configured_speed: self.parse_frequency(data.get("Configured Memory Speed")),
            configured_voltage: data.get("Configured Voltage").cloned(),
        })
    }
    
    /// Helper to parse frequency values (removes MHz, MT/s units)
    fn parse_frequency(&self, value: Option<&String>) -> Option<u32> {
        value?.split_whitespace().next()?.parse().ok()
    }
    
    /// Helper to parse u32 values
    fn parse_u32(&self, value: Option<&String>) -> Option<u32> {
        value?.split_whitespace().next()?.parse().ok()
    }
    
    /// Helper to parse memory size values (converts MB to u32)
    fn parse_memory_size(&self, value: Option<&String>) -> Option<u32> {
        let size_str = value?;
        if size_str.contains("MB") {
            size_str.split_whitespace().next()?.parse().ok()
        } else if size_str.contains("GB") {
            size_str.split_whitespace().next()?.parse::<u32>().ok().map(|gb| gb * 1024)
        } else {
            None
        }
    }
    
    /// Generate summary statistics
    /// 
    /// Note: privileged_execution detection relies on checking from the parse_output level
    /// whether there were privilege-related errors in stderr
    fn generate_summary_with_errors(&self, data: &DmidecodeData, errors: &[String]) -> DmidecodeSummary {
        let total_memory_mb = data.memory_devices
            .iter()
            .filter_map(|mem| mem.size_mb)
            .map(|size| size as u64)
            .sum();
        
        let memory_slots_used = data.memory_devices
            .iter()
            .filter(|mem| mem.size_mb.is_some() && mem.size_mb.unwrap() > 0)
            .count();
        
        // Detect privileged execution by checking for privilege-related errors
        // OR if we have comprehensive data (both system and BIOS info)
        let has_privilege_errors = errors.iter().any(|e| 
            e.contains("Operation not permitted") || 
            e.contains("Permission denied") ||
            e.contains("/dev/mem")
        );
        
        let privileged_execution = !has_privilege_errors && data.system.is_some() && data.bios.is_some();
        
        let mut warnings = Vec::new();
        if !privileged_execution {
            warnings.push("Some hardware information may be missing due to insufficient privileges. Run as root for complete detection.".to_string());
        }
        
        DmidecodeSummary {
            total_memory_mb,
            memory_slots_used,
            processor_count: data.processors.len(),
            privileged_execution,
            bios_present: data.bios.is_some(),
            system_info_present: data.system.is_some(),
            baseboard_present: data.baseboard.is_some(),
            warnings,
        }
    }
}