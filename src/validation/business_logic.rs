//! Business logic validation for hardware reports

use crate::hardware::HardwareReport;
use crate::validation::{ValidationError, ValidationConfig};

/// Validate business logic constraints for a hardware report
pub fn validate_business_rules(
    report: &HardwareReport, 
    config: &ValidationConfig
) -> Result<Vec<String>, ValidationError> {
    let mut warnings = Vec::new();
    
    // 1. Hardware Count Validation
    validate_hardware_counts(report, config, &mut warnings)?;
    
    // 2. Hardware Compatibility Validation
    validate_hardware_compatibility(report, &mut warnings)?;
    
    // 3. System Coherence Validation
    validate_system_coherence(report, &mut warnings)?;
    
    // 4. Resource Utilization Validation
    validate_resource_utilization(report, &mut warnings)?;
    
    // 5. Performance Expectations Validation
    validate_performance_expectations(report, &mut warnings)?;
    
    Ok(warnings)
}

/// Validate hardware device counts against expectations
fn validate_hardware_counts(
    report: &HardwareReport,
    config: &ValidationConfig,
    warnings: &mut Vec<String>,
) -> Result<(), ValidationError> {
    let total_devices = [
        report.storage.len(),
        report.graphics.len(),
        report.network.len(),
        report.usb.len(),
        report.audio.len(),
        usize::from(report.cpu.is_some()),
        usize::from(report.memory.is_some()),
    ].iter().sum::<usize>();
    
    // Check minimum device count if configured
    if let Some(min_count) = config.minimum_device_count {
        if total_devices < min_count as usize {
            return Err(ValidationError::BusinessLogicError {
                field: "total_devices".to_string(),
                message: format!(
                    "Only {} devices detected, expected at least {}",
                    total_devices, min_count
                ),
            });
        }
    }
    
    // Check for suspiciously low device counts
    if total_devices < 3 {
        warnings.push(format!(
            "Very few devices detected ({}). This may indicate detection issues",
            total_devices
        ));
    }
    
    // Validate specific device expectations
    if report.cpu.is_none() && config.strict_mode {
        return Err(ValidationError::BusinessLogicError {
            field: "cpu".to_string(),
            message: "CPU information is required in strict mode".to_string(),
        });
    }
    
    if report.memory.is_none() && config.strict_mode {
        return Err(ValidationError::BusinessLogicError {
            field: "memory".to_string(),
            message: "Memory information is required in strict mode".to_string(),
        });
    }
    
    // Check for expected device types using iterator patterns
    let device_checks = [
        (report.graphics.is_empty(), "No graphics devices detected. This is unusual for most systems"),
        (report.network.is_empty(), "No network devices detected. This is unusual for most systems"),
        (report.storage.is_empty(), "No storage devices detected. This is highly unusual"),
    ];
    
    warnings.extend(
        device_checks
            .iter()
            .filter_map(|(is_empty, message)| {
                if *is_empty {
                    Some(message.to_string())
                } else {
                    None
                }
            })
    );
    
    Ok(())
}

/// Validate hardware compatibility and realistic combinations
fn validate_hardware_compatibility(
    report: &HardwareReport,
    warnings: &mut Vec<String>,
) -> Result<(), ValidationError> {
    // Validate CPU/Memory combinations
    if let (Some(cpu), Some(memory)) = (&report.cpu, &report.memory) {
        // Check for realistic CPU core to memory ratios
        let memory_gb = memory.total_bytes / 1_073_741_824; // Convert to GB
        let cores = cpu.cores as u64;
        
        // Extremely low memory for high-end CPUs
        if cores >= 16 && memory_gb < 8 {
            warnings.push(format!(
                "{} CPU cores with only {}GB RAM is unusual - may indicate memory detection issues",
                cores, memory_gb
            ));
        }
        
        // Extremely high memory for low-end CPUs
        if cores <= 2 && memory_gb > 64 {
            warnings.push(format!(
                "{} CPU cores with {}GB RAM is an unusual configuration",
                cores, memory_gb
            ));
        }
        
        // Check CPU threads vs cores ratio
        if cpu.threads < cpu.cores {
            return Err(ValidationError::BusinessLogicError {
                field: "cpu.threads".to_string(),
                message: format!(
                    "CPU threads ({}) cannot be less than cores ({})",
                    cpu.threads, cpu.cores
                ),
            });
        }
        
        // Warn about unusual thread/core ratios
        let thread_ratio = cpu.threads as f64 / cpu.cores as f64;
        if thread_ratio > 2.5 {
            warnings.push(format!(
                "Unusual CPU thread/core ratio: {:.1} (threads: {}, cores: {})",
                thread_ratio, cpu.threads, cpu.cores
            ));
        }
        
        // Check frequency constraints
        if let (Some(base_freq), Some(max_freq)) = (cpu.base_frequency, cpu.max_frequency) {
            if max_freq < base_freq {
                return Err(ValidationError::BusinessLogicError {
                    field: "cpu.max_frequency".to_string(),
                    message: format!(
                        "Maximum frequency ({:.2}GHz) cannot be less than base frequency ({:.2}GHz)",
                        max_freq, base_freq
                    ),
                });
            }
            
            // Warn about unusual frequency ranges
            let freq_ratio = max_freq / base_freq;
            if freq_ratio > 3.0 {
                warnings.push(format!(
                    "Unusual CPU frequency range: {:.2}GHz to {:.2}GHz (ratio: {:.1}x)",
                    base_freq, max_freq, freq_ratio
                ));
            }
        }
    }
    
    // Validate Memory DIMM configurations
    if let Some(memory) = &report.memory {
        let mut total_dimm_size = 0u64;
        for dimm in &memory.dimms {
            total_dimm_size += dimm.size_bytes;
            
            // Check for realistic DIMM sizes
            let dimm_gb = dimm.size_bytes / 1_073_741_824;
            if dimm_gb > 0 && dimm_gb < 1 {
                warnings.push(format!(
                    "Very small DIMM detected: {}MB. This may indicate detection errors",
                    dimm.size_bytes / 1_048_576
                ));
            }
            
            if dimm_gb > 128 {
                warnings.push(format!(
                    "Very large DIMM detected: {}GB. This is unusual for consumer systems",
                    dimm_gb
                ));
            }
            
            // Check memory speed constraints
            if let Some(speed) = dimm.speed_mhz {
                if !(400..=8000).contains(&speed) {
                    warnings.push(format!(
                        "Unusual memory speed: {}MHz. Expected range: 400-8000MHz",
                        speed
                    ));
                }
            }
        }
        
        // Check if total DIMM size matches system memory
        let size_diff = total_dimm_size.abs_diff(memory.total_bytes);
        
        let diff_percent = (size_diff as f64 / memory.total_bytes as f64) * 100.0;
        if diff_percent > 10.0 {
            warnings.push(format!(
                "DIMM total size differs significantly from system memory: {:.1}% difference",
                diff_percent
            ));
        }
    }
    
    // Validate Storage configurations
    for storage in &report.storage {
        let size_gb = storage.size_bytes / 1_073_741_824;
        
        // Check for realistic storage sizes
        if size_gb < 1 {
            warnings.push(format!(
                "Very small storage device: {}MB ({})",
                storage.size_bytes / 1_048_576, storage.model
            ));
        }
        
        if size_gb > 20_000 {
            warnings.push(format!(
                "Very large storage device: {}GB ({}). Verify this is correct",
                size_gb, storage.model
            ));
        }
        
        // Validate interface/type combinations
        match (storage.device_type.as_str(), storage.interface.as_deref()) {
            ("NVMe", Some(interface)) if interface != "NVMe" && interface != "PCIe" => {
                warnings.push(format!(
                    "NVMe device '{}' with non-NVMe interface '{}' is unusual",
                    storage.model, interface
                ));
            },
            ("SSD", Some("IDE")) => {
                warnings.push(format!(
                    "SSD '{}' with IDE interface is very unusual",
                    storage.model
                ));
            },
            _ => {}
        }
    }
    
    Ok(())
}

/// Validate system coherence and consistency
fn validate_system_coherence(
    report: &HardwareReport,
    warnings: &mut Vec<String>,
) -> Result<(), ValidationError> {
    // Check architecture consistency
    let arch = &report.system.architecture;
    
    // Validate CPU architecture alignment
    if let Some(cpu) = &report.cpu {
        match arch.as_str() {
            "x86_64" => {
                if cpu.vendor != "Intel" && cpu.vendor != "AMD" && !cpu.vendor.contains("Virtual") {
                    warnings.push(format!(
                        "x86_64 architecture with unexpected CPU vendor: {}",
                        cpu.vendor
                    ));
                }
            },
            "aarch64" | "armv7l" => {
                if !cpu.vendor.to_lowercase().contains("arm") && 
                   !cpu.vendor.to_lowercase().contains("qualcomm") &&
                   !cpu.vendor.to_lowercase().contains("apple") {
                    warnings.push(format!(
                        "ARM architecture with unexpected CPU vendor: {}",
                        cpu.vendor
                    ));
                }
            },
            _ => {}
        }
    }
    
    // Validate kernel version format
    let kernel_parts: Vec<&str> = report.system.kernel_version.split('.').collect();
    if kernel_parts.len() < 2 {
        return Err(ValidationError::BusinessLogicError {
            field: "system.kernel_version".to_string(),
            message: format!(
                "Invalid kernel version format: {}",
                report.system.kernel_version
            ),
        });
    }
    
    // Check for reasonable kernel version
    if let Ok(major) = kernel_parts[0].parse::<u32>() {
        if !(2..=10).contains(&major) {
            warnings.push(format!(
                "Unusual kernel major version: {}. Expected 2-10",
                major
            ));
        }
    }
    
    // Validate tool usage consistency
    if report.metadata.tools_used.is_empty() {
        return Err(ValidationError::BusinessLogicError {
            field: "metadata.tools_used".to_string(),
            message: "No detection tools specified".to_string(),
        });
    }
    
    // Check if tools match detected data
    let has_lshw = report.metadata.tools_used.contains(&"lshw".to_string());
    let has_rich_data = !report.storage.is_empty() || !report.graphics.is_empty();
    
    if !has_lshw && has_rich_data {
        warnings.push(
            "Rich hardware data detected without lshw tool - verify tool detection".to_string()
        );
    }
    
    Ok(())
}

/// Validate resource utilization patterns
fn validate_resource_utilization(
    report: &HardwareReport,
    warnings: &mut Vec<String>,
) -> Result<(), ValidationError> {
    // Check memory utilization
    if let Some(memory) = &report.memory {
        if memory.available_bytes > memory.total_bytes {
            return Err(ValidationError::BusinessLogicError {
                field: "memory.available_bytes".to_string(),
                message: "Available memory cannot exceed total memory".to_string(),
            });
        }
        
        let used_bytes = memory.total_bytes - memory.available_bytes;
        let usage_percent = (used_bytes as f64 / memory.total_bytes as f64) * 100.0;
        
        if usage_percent > 95.0 {
            warnings.push(format!(
                "Very high memory usage: {:.1}%. This may affect detection accuracy",
                usage_percent
            ));
        }
        
        if usage_percent < 5.0 {
            warnings.push(format!(
                "Very low memory usage: {:.1}%. This is unusual for active systems",
                usage_percent
            ));
        }
    }
    
    Ok(())
}

/// Validate performance expectations based on hardware
fn validate_performance_expectations(
    report: &HardwareReport,
    warnings: &mut Vec<String>,
) -> Result<(), ValidationError> {
    // Check for performance bottlenecks
    if let Some(cpu) = &report.cpu {
        // Check for systems with many cores but low memory
        if let Some(memory) = &report.memory {
            let memory_gb = memory.total_bytes / 1_073_741_824;
            let memory_per_core = memory_gb as f64 / cpu.cores as f64;
            
            if memory_per_core < 1.0 {
                warnings.push(format!(
                    "Low memory per CPU core: {:.1}GB per core. This may limit performance",
                    memory_per_core
                ));
            }
        }
        
        // Check for high-end CPU with slow storage
        if cpu.cores >= 8 {
            let has_fast_storage = report.storage.iter().any(|s| {
                s.device_type == "NVMe" || s.device_type == "SSD"
            });
            
            if !has_fast_storage && !report.storage.is_empty() {
                warnings.push(
                    "High-end CPU with only slow storage. Consider SSD upgrade for better performance".to_string()
                );
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::{ReportMetadata, SystemInfo, CpuInfo, MemoryInfo, MemoryDimm, PrivacyLevel};
    use chrono::Utc;
    
    fn create_test_report_with_data() -> HardwareReport {
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
                boot_time: Some(Utc::now()),
            },
            cpu: Some(CpuInfo {
                model: "AMD Ryzen 9 5950X".to_string(),
                vendor: "AMD".to_string(),
                cores: 16,
                threads: 32,
                base_frequency: Some(3.4),
                max_frequency: Some(4.9),
                cache_l1: Some(524288),
                cache_l2: Some(8388608),
                cache_l3: Some(67108864),
                flags: vec!["fpu".to_string(), "vme".to_string(), "de".to_string()],
            }),
            memory: Some(MemoryInfo {
                total_bytes: 68719476736, // 64GB
                available_bytes: 34359738368, // 32GB available
                dimms: vec![
                    MemoryDimm {
                        size_bytes: 34359738368, // 32GB
                        speed_mhz: Some(3200),
                        memory_type: Some("DDR4".to_string()),
                        manufacturer: Some("Corsair".to_string()),
                    },
                    MemoryDimm {
                        size_bytes: 34359738368, // 32GB
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
    fn test_valid_business_logic() {
        let report = create_test_report_with_data();
        let config = ValidationConfig::default();
        
        let result = validate_business_rules(&report, &config);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_invalid_cpu_thread_count() {
        let mut report = create_test_report_with_data();
        if let Some(ref mut cpu) = report.cpu {
            cpu.threads = 8; // Less than cores (16)
        }
        
        let config = ValidationConfig::default();
        let result = validate_business_rules(&report, &config);
        
        assert!(result.is_err());
        if let Err(ValidationError::BusinessLogicError { field, .. }) = result {
            assert_eq!(field, "cpu.threads");
        }
    }
    
    #[test]
    fn test_invalid_memory_availability() {
        let mut report = create_test_report_with_data();
        if let Some(ref mut memory) = report.memory {
            memory.available_bytes = memory.total_bytes + 1; // More than total
        }
        
        let config = ValidationConfig::default();
        let result = validate_business_rules(&report, &config);
        
        assert!(result.is_err());
        if let Err(ValidationError::BusinessLogicError { field, .. }) = result {
            assert_eq!(field, "memory.available_bytes");
        }
    }
    
    #[test]
    fn test_strict_mode_requirements() {
        let mut report = create_test_report_with_data();
        report.cpu = None; // Remove CPU data
        
        let config = ValidationConfig {
            strict_mode: true,
            ..ValidationConfig::default()
        };
        
        let result = validate_business_rules(&report, &config);
        assert!(result.is_err());
    }
}