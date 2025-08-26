//! Privacy validation for hardware reports

use crate::hardware::{HardwareReport, PrivacyLevel};
use crate::validation::{ValidationConfig, ValidationError};
use regex::Regex;
use std::sync::OnceLock;

/// Validate privacy compliance and anonymization
pub fn validate_privacy_compliance(
    report: &HardwareReport,
    config: &ValidationConfig,
) -> Result<(), ValidationError> {
    // Check privacy level requirements
    validate_privacy_level_requirements(report, config)?;

    // Validate anonymization of sensitive fields
    validate_anonymized_fields(report)?;

    // Check for potential PII leaks
    validate_no_pii_leaks(report)?;

    // Validate anonymization strength
    validate_anonymization_strength(report)?;

    Ok(())
}

/// Validate that privacy level meets configuration requirements
fn validate_privacy_level_requirements(
    report: &HardwareReport,
    config: &ValidationConfig,
) -> Result<(), ValidationError> {
    if let Some(required_level) = config.privacy_level_required {
        let current_level = report.metadata.privacy_level;

        // Check if current privacy level is sufficient
        let level_hierarchy = |level: PrivacyLevel| -> u8 {
            match level {
                PrivacyLevel::Basic => 1,
                PrivacyLevel::Enhanced => 2,
                PrivacyLevel::Strict => 3,
            }
        };

        if level_hierarchy(current_level) < level_hierarchy(required_level) {
            return Err(ValidationError::PrivacyError {
                field: "metadata.privacy_level".to_string(),
                message: format!(
                    "Privacy level {:?} is insufficient. Required: {:?}",
                    current_level, required_level
                ),
            });
        }
    }

    Ok(())
}

/// Validate that sensitive fields are properly anonymized
fn validate_anonymized_fields(report: &HardwareReport) -> Result<(), ValidationError> {
    // Check system ID anonymization
    validate_anonymized_id(
        &report.metadata.anonymized_system_id,
        "metadata.anonymized_system_id",
        8, // Minimum length for anonymized IDs
    )?;

    // Check hostname anonymization
    validate_anonymized_id(&report.system.anonymized_hostname, "system.anonymized_hostname", 8)?;

    // Check storage serial numbers
    for (index, storage) in report.storage.iter().enumerate() {
        validate_anonymized_id(
            &storage.anonymized_serial,
            &format!("storage[{}].anonymized_serial", index),
            8,
        )?;
    }

    // Check network MAC addresses
    for (index, network) in report.network.iter().enumerate() {
        validate_mac_address_anonymization(
            &network.anonymized_mac,
            &format!("network[{}].anonymized_mac", index),
        )?;
    }

    Ok(())
}

/// Validate anonymized ID format and strength
fn validate_anonymized_id(
    id: &str,
    field_name: &str,
    min_length: usize,
) -> Result<(), ValidationError> {
    // Check minimum length
    if id.len() < min_length {
        return Err(ValidationError::PrivacyError {
            field: field_name.to_string(),
            message: format!(
                "Anonymized ID too short: {} characters (minimum: {})",
                id.len(),
                min_length
            ),
        });
    }

    // Check for obvious non-anonymized patterns
    let suspicious_patterns = [
        r"^localhost",
        r"^user[0-9]*$",
        r"^admin[0-9]*$",
        r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$", // IP address
        r"^[0-9]{12,}$",                         // Sequential numbers
    ];

    for pattern in &suspicious_patterns {
        if let Ok(regex) = Regex::new(pattern) {
            if regex.is_match(id) {
                return Err(ValidationError::PrivacyError {
                    field: field_name.to_string(),
                    message: format!("ID '{}' appears to contain non-anonymized data", id),
                });
            }
        }
    }

    // Check for sufficient entropy (not all same character)
    let unique_chars: std::collections::HashSet<char> = id.chars().collect();
    if unique_chars.len() < 3 {
        return Err(ValidationError::PrivacyError {
            field: field_name.to_string(),
            message: format!("Anonymized ID '{}' has insufficient entropy", id),
        });
    }

    Ok(())
}

/// Compiled MAC address regex for performance
static MAC_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_mac_regex() -> &'static Regex {
    MAC_REGEX.get_or_init(|| {
        Regex::new(r"^[0-9a-fA-F]{2}:[0-9a-fA-F]{2}:[0-9a-fA-F]{2}:[0-9a-fA-F]{2}:[0-9a-fA-F]{2}:[0-9a-fA-F]{2}$")
            .expect("MAC address regex should be valid")
    })
}

/// Validate MAC address anonymization
fn validate_mac_address_anonymization(mac: &str, field_name: &str) -> Result<(), ValidationError> {
    // Check MAC address format using cached regex
    if !get_mac_regex().is_match(mac) {
        return Err(ValidationError::PrivacyError {
            field: field_name.to_string(),
            message: format!("Invalid MAC address format: '{}'", mac),
        });
    }

    // Check for known vendor prefixes that shouldn't appear in anonymized MACs
    let mac_parts: Vec<&str> = mac.split(':').collect();
    if mac_parts.len() == 6 {
        let oui = format!("{}:{}", mac_parts[0], mac_parts[1]);

        // List of common vendor OUIs that should be anonymized
        let known_ouis = [
            "00:16", // Intel
            "00:1B", // Intel
            "00:50", // 3Com
            "08:00", // Various
            "AA:AA", // Obviously fake
            "FF:FF", // Obviously fake
        ];

        for known_oui in &known_ouis {
            if oui.eq_ignore_ascii_case(known_oui) {
                return Err(ValidationError::PrivacyError {
                    field: field_name.to_string(),
                    message: format!(
                        "MAC address '{}' contains known vendor OUI '{}'",
                        mac, known_oui
                    ),
                });
            }
        }
    }

    Ok(())
}

/// Compiled PII detection patterns for performance
static PII_PATTERNS: OnceLock<Vec<(Regex, &'static str)>> = OnceLock::new();

fn get_pii_patterns() -> &'static Vec<(Regex, &'static str)> {
    PII_PATTERNS.get_or_init(|| {
        [
            (r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b", "email address"),
            (r"\b(?:\d{1,3}\.){3}\d{1,3}\b", "IP address"),
            (r"\b\d{3}-\d{2}-\d{4}\b", "SSN pattern"),
            (r"\b(username|password|secret|key|token)[\s:=]+\S+", "credential"),
        ]
        .into_iter()
        .map(|(pattern, desc)| (Regex::new(pattern).expect("Valid regex"), desc))
        .collect()
    })
}

/// Check for potential PII (Personally Identifiable Information) leaks
fn validate_no_pii_leaks(report: &HardwareReport) -> Result<(), ValidationError> {
    use crate::validation::constants::HARDWARE_VENDORS;

    // Check all string fields in the report for PII
    let all_strings = collect_all_strings(report);

    for text in &all_strings {
        // Skip known hardware vendors
        if HARDWARE_VENDORS.iter().any(|vendor| text.contains(vendor)) {
            continue;
        }

        // Check against compiled regex patterns
        for (regex, description) in get_pii_patterns().iter() {
            if regex.is_match(text) {
                return Err(ValidationError::PrivacyError {
                    field: "various".to_string(),
                    message: format!(
                        "Potential PII detected ({}): '{}'",
                        description,
                        text.chars().take(50).collect::<String>()
                    ),
                });
            }
        }
    }

    Ok(())
}

/// Validate anonymization strength based on privacy level
fn validate_anonymization_strength(report: &HardwareReport) -> Result<(), ValidationError> {
    match report.metadata.privacy_level {
        PrivacyLevel::Basic => {
            // Basic privacy - just check that anonymization exists
            validate_basic_anonymization(report)?;
        }
        PrivacyLevel::Enhanced => {
            // Enhanced privacy - stronger anonymization requirements
            validate_enhanced_anonymization(report)?;
        }
        PrivacyLevel::Strict => {
            // Strict privacy - maximum anonymization
            validate_strict_anonymization(report)?;
        }
    }

    Ok(())
}

/// Validate basic anonymization requirements
fn validate_basic_anonymization(report: &HardwareReport) -> Result<(), ValidationError> {
    // System ID should be at least 8 characters
    if report.metadata.anonymized_system_id.len() < 8 {
        return Err(ValidationError::PrivacyError {
            field: "metadata.anonymized_system_id".to_string(),
            message: "Basic privacy requires at least 8-character system ID".to_string(),
        });
    }

    // Hostname should be anonymized
    if report.system.anonymized_hostname.len() < 8 {
        return Err(ValidationError::PrivacyError {
            field: "system.anonymized_hostname".to_string(),
            message: "Basic privacy requires at least 8-character hostname".to_string(),
        });
    }

    Ok(())
}

/// Validate enhanced anonymization requirements
fn validate_enhanced_anonymization(report: &HardwareReport) -> Result<(), ValidationError> {
    // Enhanced privacy requires longer anonymized IDs
    if report.metadata.anonymized_system_id.len() < 12 {
        return Err(ValidationError::PrivacyError {
            field: "metadata.anonymized_system_id".to_string(),
            message: "Enhanced privacy requires at least 12-character system ID".to_string(),
        });
    }

    // Check that hardware details are sufficiently generic
    if let Some(cpu) = &report.cpu {
        // CPU model should not contain specific stepping or revision info
        let specific_patterns = [r"\bstepping\b", r"\brevision\b", r"\bES\b", r"\bQS\b"];
        for pattern in &specific_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if regex.is_match(&cpu.model.to_lowercase()) {
                    return Err(ValidationError::PrivacyError {
                        field: "cpu.model".to_string(),
                        message: "Enhanced privacy should not include specific CPU stepping/revision info".to_string(),
                    });
                }
            }
        }
    }

    Ok(())
}

/// Validate strict anonymization requirements
fn validate_strict_anonymization(report: &HardwareReport) -> Result<(), ValidationError> {
    // Strict privacy requires maximum anonymization
    if report.metadata.anonymized_system_id.len() < 16 {
        return Err(ValidationError::PrivacyError {
            field: "metadata.anonymized_system_id".to_string(),
            message: "Strict privacy requires at least 16-character system ID".to_string(),
        });
    }

    // In strict mode, even generic hardware info should be limited
    if let Some(cpu) = &report.cpu {
        // CPU model should be generalized
        if cpu.model.len() > 50 {
            return Err(ValidationError::PrivacyError {
                field: "cpu.model".to_string(),
                message: "Strict privacy requires shorter, more generic CPU model names"
                    .to_string(),
            });
        }
    }

    // Check that no serial numbers or specific identifiers remain
    for (index, storage) in report.storage.iter().enumerate() {
        if storage.anonymized_serial.len() < 16 {
            return Err(ValidationError::PrivacyError {
                field: format!("storage[{}].anonymized_serial", index),
                message: "Strict privacy requires at least 16-character storage serial".to_string(),
            });
        }
    }

    Ok(())
}

/// Collect all string values from the report for PII scanning
fn collect_all_strings(report: &HardwareReport) -> Vec<String> {
    let mut strings = Vec::new();

    // System info
    strings.push(report.system.anonymized_hostname.clone());
    strings.push(report.system.kernel_version.clone());
    strings.push(report.system.architecture.clone());
    if let Some(ref distro) = report.system.distribution {
        strings.push(distro.clone());
    }

    // CPU info
    if let Some(ref cpu) = report.cpu {
        strings.push(cpu.model.clone());
        strings.push(cpu.vendor.clone());
        strings.extend(cpu.flags.clone());
    }

    // Memory info
    if let Some(ref memory) = report.memory {
        for dimm in &memory.dimms {
            if let Some(ref manufacturer) = dimm.manufacturer {
                strings.push(manufacturer.clone());
            }
            if let Some(ref memory_type) = dimm.memory_type {
                strings.push(memory_type.clone());
            }
        }
    }

    // Storage info
    for storage in &report.storage {
        strings.push(storage.model.clone());
        if let Some(ref vendor) = storage.vendor {
            strings.push(vendor.clone());
        }
    }

    // Graphics info
    for graphics in &report.graphics {
        strings.push(graphics.vendor.clone());
        strings.push(graphics.model.clone());
        if let Some(ref driver) = graphics.driver {
            strings.push(driver.clone());
        }
    }

    // Network info
    for network in &report.network {
        strings.push(network.vendor.clone());
        strings.push(network.model.clone());
        if let Some(ref driver) = network.driver {
            strings.push(driver.clone());
        }
    }

    strings
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::{NetworkDevice, ReportMetadata, SystemInfo};
    use chrono::Utc;

    fn create_test_report_with_privacy(privacy_level: PrivacyLevel) -> HardwareReport {
        HardwareReport {
            metadata: ReportMetadata {
                version: "1.0.0".to_string(),
                generated_at: Utc::now(),
                privacy_level,
                tools_used: vec!["lshw".to_string()],
                anonymized_system_id: "abcd1234efgh5678".to_string(), // 16 chars
            },
            system: SystemInfo {
                anonymized_hostname: "host_abcd1234efgh".to_string(), // 16 chars
                kernel_version: "6.16.0".to_string(),
                distribution: Some("NixOS 25.11".to_string()),
                architecture: "x86_64".to_string(),
                boot_time: Some(Utc::now()),
            },
            cpu: None,
            memory: None,
            storage: Vec::new(),
            graphics: Vec::new(),
            network: vec![NetworkDevice {
                device_type: "ethernet".to_string(),
                vendor: "Intel Corporation".to_string(),
                model: "I225-V Gigabit Network Connection".to_string(),
                driver: Some("igc".to_string()),
                anonymized_mac: "12:34:56:78:9a:bc".to_string(),
            }],
            usb: Vec::new(),
            audio: Vec::new(),
            kernel_support: None,
        }
    }

    #[test]
    fn test_valid_basic_privacy() {
        let report = create_test_report_with_privacy(PrivacyLevel::Basic);
        let config = ValidationConfig::default();

        assert!(validate_privacy_compliance(&report, &config).is_ok());
    }

    #[test]
    fn test_invalid_short_system_id() {
        let mut report = create_test_report_with_privacy(PrivacyLevel::Basic);
        report.metadata.anonymized_system_id = "short".to_string(); // Too short

        let config = ValidationConfig::default();
        let result = validate_privacy_compliance(&report, &config);

        assert!(result.is_err());
        if let Err(ValidationError::PrivacyError { field, .. }) = result {
            assert_eq!(field, "metadata.anonymized_system_id");
        }
    }

    #[test]
    fn test_privacy_level_requirement() {
        let report = create_test_report_with_privacy(PrivacyLevel::Basic);
        let config = ValidationConfig {
            privacy_level_required: Some(PrivacyLevel::Enhanced),
            ..ValidationConfig::default()
        };

        let result = validate_privacy_compliance(&report, &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_mac_address() {
        let mut report = create_test_report_with_privacy(PrivacyLevel::Basic);
        report.network[0].anonymized_mac = "invalid_mac".to_string();

        let config = ValidationConfig::default();
        let result = validate_privacy_compliance(&report, &config);

        assert!(result.is_err());
    }

    #[test]
    fn test_strict_privacy_requirements() {
        let report = create_test_report_with_privacy(PrivacyLevel::Strict);
        let config = ValidationConfig::default();

        assert!(validate_privacy_compliance(&report, &config).is_ok());
    }
}
