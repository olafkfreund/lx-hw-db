//! JSON schema validation for hardware reports

use crate::hardware::HardwareReport;
use crate::validation::ValidationError;
use serde_json::Value;
use std::sync::OnceLock;

/// Hardware report JSON schema definition
const HARDWARE_REPORT_SCHEMA: &str = r#"
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Hardware Report",
  "description": "Linux hardware compatibility database report schema",
  "type": "object",
  "required": ["metadata", "system"],
  "properties": {
    "metadata": {
      "type": "object",
      "required": ["version", "generated_at", "privacy_level", "tools_used", "anonymized_system_id"],
      "properties": {
        "version": {
          "type": "string",
          "pattern": "^\\d+\\.\\d+\\.\\d+$",
          "description": "Report format version (semantic versioning)"
        },
        "generated_at": {
          "type": "string",
          "format": "date-time",
          "description": "UTC timestamp when report was generated"
        },
        "privacy_level": {
          "type": "string",
          "enum": ["Basic", "Enhanced", "Strict"],
          "description": "Privacy level used for anonymization"
        },
        "tools_used": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "minItems": 1,
          "description": "List of detection tools used"
        },
        "anonymized_system_id": {
          "type": "string",
          "minLength": 8,
          "description": "Anonymized system identifier"
        }
      }
    },
    "system": {
      "type": "object",
      "required": ["anonymized_hostname", "kernel_version", "architecture"],
      "properties": {
        "anonymized_hostname": {
          "type": "string",
          "minLength": 8,
          "description": "Anonymized hostname"
        },
        "kernel_version": {
          "type": "string",
          "pattern": "^\\d+\\.\\d+",
          "description": "Linux kernel version"
        },
        "distribution": {
          "type": ["string", "null"],
          "description": "Linux distribution name and version"
        },
        "architecture": {
          "type": "string",
          "enum": ["x86_64", "aarch64", "armv7l", "i686", "riscv64"],
          "description": "System architecture"
        },
        "boot_time": {
          "type": ["string", "null"],
          "format": "date-time",
          "description": "System boot time"
        }
      }
    },
    "cpu": {
      "type": ["object", "null"],
      "properties": {
        "model": {
          "type": "string",
          "minLength": 1,
          "description": "CPU model name"
        },
        "vendor": {
          "type": "string",
          "minLength": 1,
          "description": "CPU vendor"
        },
        "cores": {
          "type": "integer",
          "minimum": 1,
          "maximum": 256,
          "description": "Number of physical CPU cores"
        },
        "threads": {
          "type": "integer",
          "minimum": 1,
          "maximum": 512,
          "description": "Number of logical CPU threads"
        },
        "base_frequency": {
          "type": ["number", "null"],
          "minimum": 0.1,
          "maximum": 10.0,
          "description": "Base CPU frequency in GHz"
        },
        "max_frequency": {
          "type": ["number", "null"],
          "minimum": 0.1,
          "maximum": 15.0,
          "description": "Maximum CPU frequency in GHz"
        },
        "cache_l1": {
          "type": ["integer", "null"],
          "minimum": 0,
          "description": "L1 cache size in bytes"
        },
        "cache_l2": {
          "type": ["integer", "null"],
          "minimum": 0,
          "description": "L2 cache size in bytes"
        },
        "cache_l3": {
          "type": ["integer", "null"],
          "minimum": 0,
          "description": "L3 cache size in bytes"
        },
        "flags": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "description": "CPU flags and features"
        }
      },
      "required": ["model", "vendor", "cores", "threads"]
    },
    "memory": {
      "type": ["object", "null"],
      "properties": {
        "total_bytes": {
          "type": "integer",
          "minimum": 134217728,
          "maximum": 17592186044416,
          "description": "Total system memory in bytes (min 128MB, max 16TB)"
        },
        "available_bytes": {
          "type": "integer",
          "minimum": 0,
          "description": "Available memory in bytes"
        },
        "dimms": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "size_bytes": {
                "type": "integer",
                "minimum": 1048576,
                "description": "DIMM size in bytes (minimum 1MB)"
              },
              "speed_mhz": {
                "type": ["integer", "null"],
                "minimum": 100,
                "maximum": 8000,
                "description": "Memory speed in MHz"
              },
              "memory_type": {
                "type": ["string", "null"],
                "enum": [null, "DDR3", "DDR4", "DDR5", "DDR2", "LPDDR4", "LPDDR5"],
                "description": "Memory technology type"
              },
              "manufacturer": {
                "type": ["string", "null"],
                "description": "Memory module manufacturer"
              }
            },
            "required": ["size_bytes"]
          },
          "description": "Individual memory modules"
        }
      },
      "required": ["total_bytes", "available_bytes", "dimms"]
    },
    "storage": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "anonymized_serial": {
            "type": "string",
            "minLength": 8,
            "description": "Anonymized serial number"
          },
          "device_type": {
            "type": "string",
            "enum": ["HDD", "SSD", "NVMe", "eMMC", "SD Card", "USB", "CD-ROM", "DVD", "Blu-ray"],
            "description": "Storage device type"
          },
          "size_bytes": {
            "type": "integer",
            "minimum": 1048576,
            "description": "Storage capacity in bytes (minimum 1MB)"
          },
          "model": {
            "type": "string",
            "minLength": 1,
            "description": "Storage device model"
          },
          "vendor": {
            "type": ["string", "null"],
            "description": "Storage device vendor"
          },
          "interface": {
            "type": ["string", "null"],
            "enum": [null, "SATA", "NVMe", "PCIe", "USB", "IDE", "SCSI"],
            "description": "Storage interface type"
          }
        },
        "required": ["anonymized_serial", "device_type", "size_bytes", "model"]
      },
      "description": "Storage devices"
    },
    "graphics": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "vendor": {
            "type": "string",
            "minLength": 1,
            "description": "Graphics card vendor"
          },
          "model": {
            "type": "string",
            "minLength": 1,
            "description": "Graphics card model"
          },
          "driver": {
            "type": ["string", "null"],
            "description": "Graphics driver name"
          },
          "memory_bytes": {
            "type": ["integer", "null"],
            "minimum": 1048576,
            "description": "Graphics memory in bytes"
          },
          "pci_id": {
            "type": "string",
            "pattern": "^[0-9a-fA-F]{4}:[0-9a-fA-F]{4}$",
            "description": "PCI vendor:device ID"
          }
        },
        "required": ["vendor", "model", "pci_id"]
      },
      "description": "Graphics devices"
    },
    "network": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "device_type": {
            "type": "string",
            "enum": ["ethernet", "wifi", "bluetooth", "cellular", "loopback"],
            "description": "Network device type"
          },
          "vendor": {
            "type": "string",
            "minLength": 1,
            "description": "Network device vendor"
          },
          "model": {
            "type": "string",
            "minLength": 1,
            "description": "Network device model"
          },
          "driver": {
            "type": ["string", "null"],
            "description": "Network driver name"
          },
          "anonymized_mac": {
            "type": "string",
            "pattern": "^[0-9a-fA-F]{2}:[0-9a-fA-F]{2}:[0-9a-fA-F]{2}:[0-9a-fA-F]{2}:[0-9a-fA-F]{2}:[0-9a-fA-F]{2}$",
            "description": "Anonymized MAC address"
          }
        },
        "required": ["device_type", "vendor", "model", "anonymized_mac"]
      },
      "description": "Network devices"
    },
    "usb": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "vendor_id": {
            "type": "string",
            "pattern": "^[0-9a-fA-F]{4}$",
            "description": "USB vendor ID (4 hex digits)"
          },
          "product_id": {
            "type": "string",
            "pattern": "^[0-9a-fA-F]{4}$",
            "description": "USB product ID (4 hex digits)"
          },
          "vendor_name": {
            "type": ["string", "null"],
            "description": "USB vendor name"
          },
          "product_name": {
            "type": ["string", "null"],
            "description": "USB product name"
          },
          "usb_version": {
            "type": ["string", "null"],
            "enum": [null, "1.0", "1.1", "2.0", "3.0", "3.1", "3.2"],
            "description": "USB specification version"
          }
        },
        "required": ["vendor_id", "product_id"]
      },
      "description": "USB devices"
    },
    "audio": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "vendor": {
            "type": "string",
            "minLength": 1,
            "description": "Audio device vendor"
          },
          "model": {
            "type": "string",
            "minLength": 1,
            "description": "Audio device model"
          },
          "driver": {
            "type": ["string", "null"],
            "description": "Audio driver name"
          },
          "device_type": {
            "type": "string",
            "enum": ["playback", "capture", "duplex", "midi"],
            "description": "Audio device type"
          }
        },
        "required": ["vendor", "model", "device_type"]
      },
      "description": "Audio devices"
    },
    "kernel_support": {
      "type": ["object", "null"],
      "properties": {
        "kernel_version": {
          "type": "string",
          "pattern": "^\\d+\\.\\d+",
          "description": "Kernel version analyzed"
        },
        "total_devices_detected": {
          "type": "integer",
          "minimum": 0,
          "description": "Total number of devices detected"
        },
        "supported_devices": {
          "type": "integer",
          "minimum": 0,
          "description": "Number of supported devices"
        },
        "unsupported_devices": {
          "type": "integer",
          "minimum": 0,
          "description": "Number of unsupported devices"
        },
        "experimental_devices": {
          "type": "integer",
          "minimum": 0,
          "description": "Number of devices with experimental support"
        },
        "device_support_details": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "device_id": {
                "type": "string",
                "minLength": 1,
                "description": "Device identifier"
              },
              "device_name": {
                "type": "string",
                "minLength": 1,
                "description": "Human-readable device name"
              },
              "support_status": {
                "type": "string",
                "enum": ["supported", "experimental", "unsupported"],
                "description": "Device support status"
              },
              "driver_module": {
                "type": "string",
                "minLength": 1,
                "description": "Required kernel module"
              },
              "since_kernel_version": {
                "type": ["string", "null"],
                "pattern": "^\\d+\\.\\d+",
                "description": "Kernel version when support was added"
              },
              "config_dependencies": {
                "type": "array",
                "items": {
                  "type": "string"
                },
                "description": "Required kernel config options"
              },
              "notes": {
                "type": ["string", "null"],
                "description": "Additional notes about support"
              }
            },
            "required": ["device_id", "device_name", "support_status", "driver_module"]
          }
        },
        "missing_modules": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "description": "Missing kernel modules for detected devices"
        },
        "config_recommendations": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "description": "Recommended kernel configuration changes"
        }
      },
      "required": [
        "kernel_version",
        "total_devices_detected",
        "supported_devices",
        "unsupported_devices",
        "experimental_devices",
        "device_support_details",
        "missing_modules",
        "config_recommendations"
      ]
    }
  }
}
"#;

/// Cached parsed schema for performance
static PARSED_SCHEMA: OnceLock<Value> = OnceLock::new();

/// Get the parsed schema, initializing it if necessary
fn get_schema() -> &'static Value {
    PARSED_SCHEMA.get_or_init(|| {
        serde_json::from_str(HARDWARE_REPORT_SCHEMA)
            .expect("Hardware report schema should be valid JSON")
    })
}

/// Validate a hardware report against the JSON schema
pub fn validate_report_schema(report: &HardwareReport) -> Result<(), ValidationError> {
    // Convert report to JSON for schema validation
    let report_json = serde_json::to_value(report).map_err(|e| ValidationError::SchemaError {
        message: format!("Failed to serialize report for validation: {}", e),
    })?;

    // Use cached parsed schema
    let schema = get_schema();

    // Basic validation - check required fields
    validate_required_fields(&report_json, schema)?;

    // Validate field constraints
    validate_field_constraints(&report_json, schema)?;

    Ok(())
}

/// Validate required fields are present
fn validate_required_fields(data: &Value, _schema: &Value) -> Result<(), ValidationError> {
    // Simplified validation - just check that top-level required fields exist
    let data_obj = data.as_object().ok_or_else(|| ValidationError::SchemaError {
        message: "Root must be an object".to_string(),
    })?;

    // Check for essential top-level fields
    if !data_obj.contains_key("metadata") {
        return Err(ValidationError::SchemaError {
            message: "Required field 'metadata' is missing".to_string(),
        });
    }

    if !data_obj.contains_key("system") {
        return Err(ValidationError::SchemaError {
            message: "Required field 'system' is missing".to_string(),
        });
    }

    // Check metadata fields
    if let Some(metadata) = data_obj.get("metadata").and_then(|m| m.as_object()) {
        let metadata_required =
            ["version", "generated_at", "privacy_level", "tools_used", "anonymized_system_id"];
        for field in &metadata_required {
            if !metadata.contains_key(*field) {
                return Err(ValidationError::SchemaError {
                    message: format!("Required field 'metadata.{}' is missing", field),
                });
            }
        }
    }

    // Check system fields
    if let Some(system) = data_obj.get("system").and_then(|s| s.as_object()) {
        let system_required = ["anonymized_hostname", "kernel_version", "architecture"];
        for field in &system_required {
            if !system.contains_key(*field) {
                return Err(ValidationError::SchemaError {
                    message: format!("Required field 'system.{}' is missing", field),
                });
            }
        }
    }

    Ok(())
}

/// Validate field types and constraints
fn validate_field_constraints(data: &Value, _schema: &Value) -> Result<(), ValidationError> {
    // Validate metadata version format
    if let Some(metadata) = data.get("metadata") {
        if let Some(version) = metadata.get("version").and_then(|v| v.as_str()) {
            if version.matches('.').count() < 2 {
                return Err(ValidationError::SchemaError {
                    message: "Version must follow semantic versioning format (x.y.z)".to_string(),
                });
            }
        }

        if let Some(tools_used) = metadata.get("tools_used").and_then(|t| t.as_array()) {
            if tools_used.is_empty() {
                return Err(ValidationError::SchemaError {
                    message: "At least one detection tool must be specified".to_string(),
                });
            }
        }
    }

    // Validate system information
    if let Some(system) = data.get("system") {
        if let Some(arch) = system.get("architecture").and_then(|a| a.as_str()) {
            let valid_archs = ["x86_64", "aarch64", "armv7l", "i686", "riscv64"];
            if !valid_archs.contains(&arch) {
                return Err(ValidationError::SchemaError {
                    message: format!(
                        "Invalid architecture '{}'. Must be one of: {}",
                        arch,
                        valid_archs.join(", ")
                    ),
                });
            }
        }
    }

    // Validate CPU constraints
    if let Some(cpu) = data.get("cpu") {
        if !cpu.is_null() {
            if let Some(cores) = cpu.get("cores").and_then(|c| c.as_u64()) {
                if cores == 0 || cores > 256 {
                    return Err(ValidationError::SchemaError {
                        message: "CPU cores must be between 1 and 256".to_string(),
                    });
                }
            }

            if let Some(threads) = cpu.get("threads").and_then(|t| t.as_u64()) {
                if threads == 0 || threads > 512 {
                    return Err(ValidationError::SchemaError {
                        message: "CPU threads must be between 1 and 512".to_string(),
                    });
                }
            }
        }
    }

    // Validate memory constraints
    if let Some(memory) = data.get("memory") {
        if !memory.is_null() {
            if let Some(total_bytes) = memory.get("total_bytes").and_then(|t| t.as_u64()) {
                let min_memory = 134_217_728; // 128MB
                let max_memory = 17_592_186_044_416; // 16TB
                if total_bytes < min_memory || total_bytes > max_memory {
                    return Err(ValidationError::SchemaError {
                        message: format!(
                            "Total memory must be between {}MB and {}TB",
                            min_memory / 1048576,
                            max_memory / 1099511627776
                        ),
                    });
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::{PrivacyLevel, ReportMetadata, SystemInfo};
    use chrono::Utc;

    fn create_test_report() -> HardwareReport {
        HardwareReport {
            metadata: ReportMetadata {
                version: "1.0.0".to_string(),
                generated_at: Utc::now(),
                privacy_level: PrivacyLevel::Basic,
                tools_used: vec!["lshw".to_string()],
                anonymized_system_id: "test_id_123456".to_string(),
            },
            system: SystemInfo {
                anonymized_hostname: "test_host_456789".to_string(),
                kernel_version: "6.16.0".to_string(),
                distribution: Some("NixOS 25.11".to_string()),
                architecture: "x86_64".to_string(),
                boot_time: Some(Utc::now()),
            },
            cpu: None,
            memory: None,
            storage: Vec::new(),
            graphics: Vec::new(),
            network: Vec::new(),
            usb: Vec::new(),
            audio: Vec::new(),
            kernel_support: None,
        }
    }

    #[test]
    fn test_valid_report_schema() {
        let report = create_test_report();
        assert!(validate_report_schema(&report).is_ok());
    }

    #[test]
    fn test_invalid_architecture() {
        let mut report = create_test_report();
        report.system.architecture = "invalid_arch".to_string();

        let result = validate_report_schema(&report);
        assert!(result.is_err());

        if let Err(ValidationError::SchemaError { message }) = result {
            assert!(message.contains("Invalid architecture"));
        }
    }

    #[test]
    fn test_invalid_version_format() {
        let mut report = create_test_report();
        report.metadata.version = "1.0".to_string(); // Missing patch version

        let result = validate_report_schema(&report);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_tools_used() {
        let mut report = create_test_report();
        report.metadata.tools_used = Vec::new();

        let result = validate_report_schema(&report);
        assert!(result.is_err());

        if let Err(ValidationError::SchemaError { message }) = result {
            assert!(message.contains("At least one detection tool"));
        }
    }
}
