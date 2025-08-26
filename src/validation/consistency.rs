//! Data consistency validation for hardware reports

use crate::hardware::HardwareReport;
use crate::validation::constants::MULTI_INSTANCE_USB_VENDORS;
use std::collections::HashSet;

/// Validate data consistency within a hardware report
pub fn validate_data_consistency(report: &HardwareReport) -> Result<Vec<String>, crate::validation::ValidationError> {
    let mut warnings = Vec::new();
    
    // 1. Cross-reference validation between different data sources
    validate_cross_references(report, &mut warnings)?;
    
    // 2. Tool consistency validation
    validate_tool_consistency(report, &mut warnings)?;
    
    // 3. Hardware count consistency
    validate_count_consistency(report, &mut warnings)?;
    
    // 4. Timestamp consistency
    validate_timestamp_consistency(report, &mut warnings)?;
    
    // 5. Data format consistency
    validate_format_consistency(report, &mut warnings)?;
    
    Ok(warnings)
}

/// Validate cross-references between different hardware components
fn validate_cross_references(
    report: &HardwareReport,
    warnings: &mut Vec<String>,
) -> Result<(), crate::validation::ValidationError> {
    // Check CPU and memory relationship
    if let (Some(cpu), Some(memory)) = (&report.cpu, &report.memory) {
        // Validate memory channels vs CPU support
        let dimm_count = memory.dimms.len();
        let expected_channels = match cpu.cores {
            1..=4 => 2,    // Dual channel
            5..=8 => 4,    // Quad channel
            9..=16 => 4,   // Quad channel (consumer) or more (server)
            17..=32 => 8,  // Octa channel (server)
            _ => 8,        // High-end server
        } as usize;
        
        if dimm_count > 0 && dimm_count != expected_channels && dimm_count % expected_channels != 0 {
            warnings.push(format!(
                "Unusual DIMM configuration: {} DIMMs for {}-core CPU (expected multiples of {})",
                dimm_count, cpu.cores, expected_channels
            ));
        }
        
        // Check memory speed vs CPU support
        let cpu_supports_high_speed = cpu.cores > 8 || 
            cpu.model.to_lowercase().contains("threadripper") ||
            cpu.model.to_lowercase().contains("xeon") ||
            cpu.model.to_lowercase().contains("epyc");
            
        if !cpu_supports_high_speed {
            for dimm in &memory.dimms {
                if let Some(speed) = dimm.speed_mhz {
                    if speed > 3600 {
                        warnings.push(format!(
                            "High memory speed ({}MHz) may not be fully supported by CPU: {}",
                            speed, cpu.model
                        ));
                        break;
                    }
                }
            }
        }
    }
    
    // Check graphics and system architecture consistency
    for graphics in &report.graphics {
        match report.system.architecture.as_str() {
            "aarch64" | "armv7l" => {
                if graphics.vendor.to_lowercase().contains("nvidia") ||
                   graphics.vendor.to_lowercase().contains("amd") {
                    warnings.push(format!(
                        "Discrete GPU '{}' on ARM architecture is unusual",
                        graphics.model
                    ));
                }
            }
            _ => {}
        }
    }
    
    // Check network device count consistency
    if report.network.len() > 10 {
        warnings.push(format!(
            "Unusually high number of network devices ({}). Verify detection accuracy",
            report.network.len()
        ));
    }
    
    // Check USB device relationships
    let usb_count = report.usb.len();
    let usb_hub_count = report.usb
        .iter()
        .filter(|usb| {
            usb.product_name
                .as_ref()
                .map_or(false, |name| name.to_lowercase().contains("hub"))
        })
        .count();
    
    if usb_count > 20 && usb_hub_count == 0 {
        warnings.push(
            "Many USB devices detected but no USB hubs. This may indicate detection issues".to_string()
        );
    }
    
    Ok(())
}

/// Validate consistency between tools used and data available
fn validate_tool_consistency(
    report: &HardwareReport,
    warnings: &mut Vec<String>,
) -> Result<(), crate::validation::ValidationError> {
    let tools_used: HashSet<_> = report.metadata.tools_used.iter().collect();
    
    // Check lshw tool consistency
    if tools_used.contains(&"lshw".to_string()) {
        let expected_lshw_data = !report.storage.is_empty() || 
                                !report.graphics.is_empty() || 
                                !report.network.is_empty() ||
                                report.cpu.is_some() ||
                                report.memory.is_some();
        
        if !expected_lshw_data {
            warnings.push(
                "lshw tool was used but no typical lshw data detected".to_string()
            );
        }
    }
    
    // Check dmidecode tool consistency
    if tools_used.contains(&"dmidecode".to_string()) {
        let has_dimm_details = report.memory.as_ref()
            .map_or(false, |mem| mem.dimms.iter()
                .any(|dimm| dimm.manufacturer.is_some() || dimm.memory_type.is_some()));
        
        if !has_dimm_details {
            warnings.push(
                "dmidecode tool was used but no detailed memory information detected".to_string()
            );
        }
    }
    
    // Check lspci tool consistency
    if tools_used.contains(&"lspci".to_string()) {
        let has_pci_devices = !report.graphics.is_empty() || !report.network.is_empty();
        
        if !has_pci_devices {
            warnings.push(
                "lspci tool was used but no PCI devices detected".to_string()
            );
        }
    }
    
    // Check lsusb tool consistency
    if tools_used.contains(&"lsusb".to_string()) && report.usb.is_empty() {
        warnings.push(
            "lsusb tool was used but no USB devices detected".to_string()
        );
    }
    
    // Check inxi tool consistency
    if tools_used.contains(&"inxi".to_string()) {
        // inxi should provide system overview information
        if report.system.distribution.is_none() {
            warnings.push(
                "inxi tool was used but no distribution information detected".to_string()
            );
        }
    }
    
    // Check for tools that should typically be used together
    if tools_used.contains(&"lshw".to_string()) && !tools_used.contains(&"dmidecode".to_string()) {
        warnings.push(
            "lshw used without dmidecode - consider using both for complete hardware detection".to_string()
        );
    }
    
    Ok(())
}

/// Validate consistency in hardware counts
fn validate_count_consistency(
    report: &HardwareReport,
    warnings: &mut Vec<String>,
) -> Result<(), crate::validation::ValidationError> {
    // Check for duplicate device entries
    validate_duplicate_devices(report, warnings)?;
    
    // Check expected device minimums for different system types
    validate_system_type_expectations(report, warnings)?;
    
    // Check for missing essential components
    validate_essential_components(report, warnings)?;
    
    Ok(())
}

/// Check for duplicate devices that might indicate detection errors
fn validate_duplicate_devices(
    report: &HardwareReport,
    warnings: &mut Vec<String>,
) -> Result<(), crate::validation::ValidationError> {
    // Check for duplicate storage devices
    let mut storage_signatures = HashSet::new();
    for storage in &report.storage {
        let signature = format!("{}_{}", storage.model, storage.size_bytes);
        if !storage_signatures.insert(signature.clone()) {
            warnings.push(format!(
                "Potential duplicate storage device: {} ({}GB)",
                storage.model,
                storage.size_bytes / 1_073_741_824
            ));
        }
    }
    
    // Check for duplicate graphics devices
    let mut graphics_signatures = HashSet::new();
    for graphics in &report.graphics {
        let signature = format!("{}_{}", graphics.vendor, graphics.model);
        if !graphics_signatures.insert(signature.clone()) {
            warnings.push(format!(
                "Potential duplicate graphics device: {} {}",
                graphics.vendor, graphics.model
            ));
        }
    }
    
    // Check for duplicate network devices (excluding virtual interfaces)
    let mut network_signatures = HashSet::new();
    for network in &report.network {
        if network.device_type != "loopback" {
            let signature = format!("{}_{}_{}", network.vendor, network.model, network.device_type);
            if !network_signatures.insert(signature.clone()) {
                warnings.push(format!(
                    "Potential duplicate network device: {} {} ({})",
                    network.vendor, network.model, network.device_type
                ));
            }
        }
    }
    
    // Check for duplicate USB devices
    let mut usb_signatures = HashSet::new();
    
    for usb in &report.usb {
        let signature = format!("{}:{}", usb.vendor_id, usb.product_id);
        if !usb_signatures.insert(signature) {
            // Multiple instances of the same USB device can be legitimate (e.g., multiple ports of a hub)
            // Only warn if it's not a common multi-instance device
            if !MULTI_INSTANCE_USB_VENDORS.contains(&usb.vendor_id.as_str()) {
                warnings.push(format!(
                    "Multiple instances of USB device: {}:{}",
                    usb.vendor_id, usb.product_id
                ));
            }
        }
    }
    
    Ok(())
}

/// Validate system type expectations
fn validate_system_type_expectations(
    report: &HardwareReport,
    warnings: &mut Vec<String>,
) -> Result<(), crate::validation::ValidationError> {
    // Determine system type from available information
    let is_server = report.cpu.as_ref().map_or(false, |cpu| {
        cpu.cores > 16 || 
        cpu.model.to_lowercase().contains("xeon") ||
        cpu.model.to_lowercase().contains("epyc") ||
        cpu.model.to_lowercase().contains("threadripper pro")
    });
    
    let is_workstation = report.cpu.as_ref().map_or(false, |cpu| {
        cpu.cores > 8 || cpu.model.to_lowercase().contains("threadripper")
    }) || report.memory.as_ref().map_or(false, |mem| {
        mem.total_bytes > 34_359_738_368 // > 32GB
    });
    
    let is_laptop = report.cpu.as_ref().map_or(false, |cpu| {
        cpu.model.to_lowercase().contains("mobile") ||
        cpu.model.to_lowercase().contains("h") || // Intel mobile suffix
        cpu.model.to_lowercase().contains("u") || // Intel ultrabook suffix
        cpu.model.to_lowercase().contains("hs")   // AMD mobile suffix
    });
    
    // Server expectations
    if is_server {
        if report.storage.len() < 2 {
            warnings.push("Server system with only one storage device is unusual".to_string());
        }
        
        if report.network.len() < 2 {
            warnings.push("Server system typically has multiple network interfaces".to_string());
        }
        
        // Servers often have multiple CPUs, but our current model doesn't track this
        // Could be enhanced in the future
    }
    
    // Workstation expectations
    if is_workstation && !is_server {
        if report.graphics.is_empty() {
            warnings.push("Workstation system without dedicated graphics is unusual".to_string());
        }
    }
    
    // Laptop expectations
    if is_laptop {
        if report.storage.len() > 3 {
            warnings.push("Laptop with many storage devices is unusual".to_string());
        }
        
        let has_wifi = report.network.iter().any(|net| net.device_type == "wifi");
        if !has_wifi {
            warnings.push("Laptop system without WiFi device is unusual".to_string());
        }
    }
    
    Ok(())
}

/// Validate presence of essential components
fn validate_essential_components(
    report: &HardwareReport,
    warnings: &mut Vec<String>,
) -> Result<(), crate::validation::ValidationError> {
    // Every system should have CPU information
    if report.cpu.is_none() {
        warnings.push("No CPU information detected - this is highly unusual".to_string());
    }
    
    // Every system should have memory information
    if report.memory.is_none() {
        warnings.push("No memory information detected - this is highly unusual".to_string());
    }
    
    // Every system should have at least one storage device (except live systems)
    if report.storage.is_empty() {
        warnings.push("No storage devices detected - verify this is not a live/diskless system".to_string());
    }
    
    // Most systems should have graphics capability
    if report.graphics.is_empty() {
        // Check if this might be a headless server
        let is_headless_server = report.cpu.as_ref().map_or(false, |cpu| {
            cpu.cores > 8 && (
                cpu.model.to_lowercase().contains("xeon") ||
                cpu.model.to_lowercase().contains("epyc")
            )
        });
        
        if !is_headless_server {
            warnings.push("No graphics devices detected - this is unusual for desktop systems".to_string());
        }
    }
    
    Ok(())
}

/// Validate timestamp consistency
fn validate_timestamp_consistency(
    report: &HardwareReport,
    warnings: &mut Vec<String>,
) -> Result<(), crate::validation::ValidationError> {
    use chrono::{Duration, Utc};
    
    let generated_at = report.metadata.generated_at;
    let now = Utc::now();
    
    // Check if generation timestamp is reasonable
    if generated_at > now + Duration::minutes(5) {
        warnings.push("Report generated in the future - check system clock".to_string());
    }
    
    if generated_at < now - Duration::days(30) {
        warnings.push("Report is over 30 days old - may not reflect current system state".to_string());
    }
    
    // Check boot time vs generation time consistency
    if let Some(boot_time) = report.system.boot_time {
        if boot_time > generated_at {
            warnings.push("System boot time is after report generation - check timestamps".to_string());
        }
        
        let uptime = generated_at - boot_time;
        if uptime.num_seconds() < 60 {
            warnings.push("System uptime less than 1 minute - hardware detection may be incomplete".to_string());
        }
        
        if uptime.num_days() > 365 {
            warnings.push("System uptime over 1 year - consider reboot for security updates".to_string());
        }
    }
    
    Ok(())
}

/// Validate data format consistency
fn validate_format_consistency(
    report: &HardwareReport,
    warnings: &mut Vec<String>,
) -> Result<(), crate::validation::ValidationError> {
    // Check consistent units and formats across similar fields
    validate_memory_units(report, warnings)?;
    validate_frequency_units(report, warnings)?;
    validate_identifier_formats(report, warnings)?;
    
    Ok(())
}

/// Validate memory unit consistency
fn validate_memory_units(
    report: &HardwareReport,
    warnings: &mut Vec<String>,
) -> Result<(), crate::validation::ValidationError> {
    if let Some(memory) = &report.memory {
        let total_bytes = memory.total_bytes;
        let available_bytes = memory.available_bytes;
        
        // Check if values are in reasonable ranges
        if total_bytes < 1_048_576 { // Less than 1MB
            warnings.push("Total memory less than 1MB - check unit conversion".to_string());
        }
        
        if available_bytes > total_bytes {
            return Err(crate::validation::ValidationError::ConsistencyError {
                field: "memory.available_bytes".to_string(),
                message: "Available memory exceeds total memory".to_string(),
            });
        }
        
        // Check DIMM size consistency
        let total_dimm_size: u64 = memory.dimms.iter().map(|d| d.size_bytes).sum();
        if total_dimm_size > 0 {
            let diff_percent = if total_bytes > total_dimm_size {
                ((total_bytes - total_dimm_size) as f64 / total_bytes as f64) * 100.0
            } else {
                ((total_dimm_size - total_bytes) as f64 / total_bytes as f64) * 100.0
            };
            
            if diff_percent > 15.0 {
                warnings.push(format!(
                    "Total DIMM size differs from system memory by {:.1}% - check detection accuracy",
                    diff_percent
                ));
            }
        }
    }
    
    Ok(())
}

/// Validate frequency unit consistency
fn validate_frequency_units(
    report: &HardwareReport,
    warnings: &mut Vec<String>,
) -> Result<(), crate::validation::ValidationError> {
    if let Some(cpu) = &report.cpu {
        // Check CPU frequency ranges
        if let Some(base_freq) = cpu.base_frequency {
            if base_freq > 10.0 {
                warnings.push("CPU base frequency over 10GHz - check unit conversion".to_string());
            }
            if base_freq < 0.5 {
                warnings.push("CPU base frequency under 0.5GHz - check unit conversion".to_string());
            }
        }
        
        if let Some(max_freq) = cpu.max_frequency {
            if max_freq > 15.0 {
                warnings.push("CPU max frequency over 15GHz - check unit conversion".to_string());
            }
        }
    }
    
    // Check memory frequency consistency
    if let Some(memory) = &report.memory {
        for dimm in &memory.dimms {
            if let Some(speed) = dimm.speed_mhz {
                if speed > 10000 {
                    warnings.push(format!(
                        "Memory speed {}MHz is unusually high - check unit conversion",
                        speed
                    ));
                }
                if speed < 100 {
                    warnings.push(format!(
                        "Memory speed {}MHz is unusually low - check unit conversion",
                        speed
                    ));
                }
            }
        }
    }
    
    Ok(())
}

/// Validate identifier format consistency
fn validate_identifier_formats(
    report: &HardwareReport,
    warnings: &mut Vec<String>,
) -> Result<(), crate::validation::ValidationError> {
    // Check USB ID format consistency
    for usb in &report.usb {
        if usb.vendor_id.len() != 4 {
            warnings.push(format!(
                "USB vendor ID '{}' is not 4 characters - check format",
                usb.vendor_id
            ));
        }
        if usb.product_id.len() != 4 {
            warnings.push(format!(
                "USB product ID '{}' is not 4 characters - check format",
                usb.product_id
            ));
        }
    }
    
    // Check PCI ID format consistency
    for graphics in &report.graphics {
        if !graphics.pci_id.contains(':') {
            warnings.push(format!(
                "PCI ID '{}' does not contain expected ':' separator",
                graphics.pci_id
            ));
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::{ReportMetadata, SystemInfo, CpuInfo, MemoryInfo, MemoryDimm, PrivacyLevel};
    use chrono::Utc;
    
    fn create_consistent_report() -> HardwareReport {
        HardwareReport {
            metadata: ReportMetadata {
                version: "1.0.0".to_string(),
                generated_at: Utc::now(),
                privacy_level: PrivacyLevel::Basic,
                tools_used: vec!["lshw".to_string(), "dmidecode".to_string()],
                anonymized_system_id: "test_id_123456".to_string(),
            },
            system: SystemInfo {
                anonymized_hostname: "test_host_456789".to_string(),
                kernel_version: "6.16.0".to_string(),
                distribution: Some("NixOS 25.11".to_string()),
                architecture: "x86_64".to_string(),
                boot_time: Some(Utc::now() - chrono::Duration::hours(2)),
            },
            cpu: Some(CpuInfo {
                model: "AMD Ryzen 7 5800X".to_string(),
                vendor: "AMD".to_string(),
                cores: 8,
                threads: 16,
                base_frequency: Some(3.8),
                max_frequency: Some(4.7),
                cache_l1: Some(262144),
                cache_l2: Some(4194304),
                cache_l3: Some(33554432),
                flags: vec!["fpu".to_string(), "vme".to_string()],
            }),
            memory: Some(MemoryInfo {
                total_bytes: 34359738368, // 32GB
                available_bytes: 17179869184, // 16GB available
                dimms: vec![
                    MemoryDimm {
                        size_bytes: 17179869184, // 16GB
                        speed_mhz: Some(3200),
                        memory_type: Some("DDR4".to_string()),
                        manufacturer: Some("Corsair".to_string()),
                    },
                    MemoryDimm {
                        size_bytes: 17179869184, // 16GB
                        speed_mhz: Some(3200),
                        memory_type: Some("DDR4".to_string()),
                        manufacturer: Some("Corsair".to_string()),
                    },
                ],
            }),
            storage: Vec::new(),
            graphics: Vec::new(),
            network: Vec::new(),
            usb: Vec::new(),
            audio: Vec::new(),
            kernel_support: None,
        }
    }
    
    #[test]
    fn test_consistent_report() {
        let report = create_consistent_report();
        let result = validate_data_consistency(&report);
        
        assert!(result.is_ok());
        // May have warnings but should not error
    }
    
    #[test]
    fn test_inconsistent_memory() {
        let mut report = create_consistent_report();
        if let Some(ref mut memory) = report.memory {
            memory.available_bytes = memory.total_bytes + 1; // More than total
        }
        
        let result = validate_data_consistency(&report);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_cpu_thread_consistency() {
        let mut report = create_consistent_report();
        if let Some(ref mut cpu) = report.cpu {
            cpu.threads = 4; // Less than cores (8)
        }
        
        let result = validate_data_consistency(&report);
        // This should generate warnings but not fail
        assert!(result.is_ok());
        let warnings = result.unwrap();
        assert!(!warnings.is_empty());
    }
}