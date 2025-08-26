# Testing Specification

> Spec: Hardware Detection CLI Tool
> Created: 2025-08-26
> Version: 1.0.0

## Testing Strategy Overview

The testing strategy follows a comprehensive Test-Driven Development (TDD) approach with multiple testing layers to ensure reliability, privacy compliance, and cross-platform compatibility.

## Testing Pyramid

### Unit Tests (70% coverage target)
- **Scope:** Individual functions, modules, and components
- **Framework:** Rust's built-in `#[cfg(test)]` with `cargo test`
- **Mock Strategy:** Mock external tool execution and system calls
- **Coverage:** Privacy algorithms, parsing logic, data structures

### Integration Tests (20% coverage target)  
- **Scope:** Tool interaction, CLI interface, end-to-end workflows
- **Framework:** Rust integration tests in `tests/` directory
- **Environment:** Real and mocked system environments
- **Coverage:** Multi-tool detection, error handling, output generation

### System Tests (10% coverage target)
- **Scope:** Full CLI functionality across different platforms
- **Framework:** Shell scripts with `bats` testing framework
- **Environment:** Docker containers with different Linux distributions
- **Coverage:** Cross-platform compatibility, real hardware detection

## Unit Testing Specifications

### Privacy Module Tests
```rust
#[cfg(test)]
mod privacy_tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_hardware_id_anonymization() {
        let raw_id = "1234:5678";
        let salt = b"test_salt_2024";
        
        let anonymized1 = anonymize_hardware_id(raw_id, salt);
        let anonymized2 = anonymize_hardware_id(raw_id, salt);
        
        // Same input should produce same output
        assert_eq!(anonymized1, anonymized2);
        
        // Output should be properly formatted
        assert!(anonymized1.starts_with("hw_"));
        assert_eq!(anonymized1.len(), 35); // "hw_" + 32 hex chars
        
        // Different salt should produce different output
        let different_salt = b"different_salt";
        let anonymized3 = anonymize_hardware_id(raw_id, different_salt);
        assert_ne!(anonymized1, anonymized3);
    }

    #[test]
    fn test_salt_generation_periods() {
        let test_time = UNIX_EPOCH + Duration::from_secs(1693468800); // 2023-08-31 00:00:00
        
        let basic_salt = generate_salt(test_time, PrivacyLevel::Basic);
        let enhanced_salt = generate_salt(test_time, PrivacyLevel::Enhanced);
        let strict_salt = generate_salt(test_time, PrivacyLevel::Strict);
        
        // All salts should be different
        assert_ne!(basic_salt, enhanced_salt);
        assert_ne!(enhanced_salt, strict_salt);
        assert_ne!(basic_salt, strict_salt);
    }

    #[test]
    fn test_privacy_level_anonymization_scope() {
        let hardware_data = HardwareData {
            cpu_serial: "CPU123456789",
            gpu_serial: "GPU987654321", 
            mac_address: "00:11:22:33:44:55",
            bios_version: "BIOS v1.2.3",
        };
        
        let basic_anonymized = anonymize_hardware_data(&hardware_data, PrivacyLevel::Basic);
        let strict_anonymized = anonymize_hardware_data(&hardware_data, PrivacyLevel::Strict);
        
        // Basic should anonymize serials and MAC
        assert!(basic_anonymized.cpu_serial.starts_with("hw_"));
        assert!(basic_anonymized.mac_address.starts_with("hw_"));
        assert_eq!(basic_anonymized.bios_version, hardware_data.bios_version); // Not anonymized in basic
        
        // Strict should anonymize everything
        assert!(strict_anonymized.bios_version.starts_with("hw_"));
    }

    #[test]
    fn test_k_anonymity_validation() {
        let reports = vec![
            create_test_report("hw_aaa", "intel", "discrete"),
            create_test_report("hw_bbb", "intel", "discrete"), 
            create_test_report("hw_ccc", "amd", "integrated"),
        ];
        
        let k5_result = validate_k_anonymity(&reports, 5);
        assert!(!k5_result.is_valid); // Only 2 intel discrete, less than k=5
        
        let k2_result = validate_k_anonymity(&reports, 2);
        assert!(k2_result.is_valid); // 2 intel discrete meets k=2
    }
}
```

### Hardware Detection Module Tests
```rust
#[cfg(test)]
mod detector_tests {
    use super::*;
    use mockall::predicate::*;
    
    #[test]
    fn test_lshw_json_parsing() {
        let mock_output = r#"
        {
            "id": "core",
            "class": "bus",
            "children": [
                {
                    "id": "cpu:0",
                    "class": "processor", 
                    "vendor": "Intel Corp.",
                    "product": "Intel(R) Core(TM) i7-8750H CPU @ 2.20GHz"
                }
            ]
        }
        "#;
        
        let mut mock_executor = MockCommandExecutor::new();
        mock_executor
            .expect_execute()
            .with(eq("lshw"), eq(vec!["-json", "-sanitize", "-quiet"]))
            .returning(|_, _| Ok(mock_output.to_string()));
            
        let detector = LshwDetector::new(mock_executor);
        let result = detector.detect().unwrap();
        
        assert_eq!(result.cpu.len(), 1);
        assert_eq!(result.cpu[0].vendor, "intel");
        assert!(result.cpu[0].model.contains("i7"));
    }

    #[test]
    fn test_dmidecode_parsing() {
        let mock_output = r#"
Handle 0x0001, DMI type 1, 27 bytes
System Information
    Manufacturer: Dell Inc.
    Product Name: XPS 15 9570
    Version: Not Specified
    Serial Number: ABCD1234
        "#;
        
        let mut mock_executor = MockCommandExecutor::new();
        mock_executor
            .expect_execute() 
            .returning(|_, _| Ok(mock_output.to_string()));
            
        let detector = DmidecodeDetector::new(mock_executor);
        let result = detector.detect().unwrap();
        
        assert_eq!(result.system.manufacturer, "dell");
        assert_eq!(result.system.product, "xps_15_9570");
        assert!(result.system.serial.starts_with("hw_")); // Should be anonymized
    }

    #[test]
    fn test_tool_timeout_handling() {
        let mut mock_executor = MockCommandExecutor::new();
        mock_executor
            .expect_execute()
            .returning(|_, _| {
                std::thread::sleep(std::time::Duration::from_secs(15));
                Ok("delayed output".to_string())
            });
            
        let detector = LshwDetector::new(mock_executor);
        let result = detector.detect_with_timeout(Duration::from_secs(5));
        
        assert!(matches!(result, Err(DetectionError::ToolTimeout { .. })));
    }

    #[test]
    fn test_tool_not_found_error() {
        let mut mock_executor = MockCommandExecutor::new();
        mock_executor
            .expect_execute()
            .returning(|_, _| Err(std::io::Error::new(std::io::ErrorKind::NotFound, "command not found")));
            
        let detector = LshwDetector::new(mock_executor);
        let result = detector.detect();
        
        assert!(matches!(result, Err(DetectionError::ToolNotFound { .. })));
    }
}
```

### Output Generation Module Tests
```rust
#[cfg(test)]
mod output_tests {
    use super::*;
    
    #[test]
    fn test_markdown_output_generation() {
        let hardware_report = create_test_hardware_report();
        let markdown_generator = MarkdownGenerator::new();
        
        let output = markdown_generator.generate(&hardware_report).unwrap();
        
        // Test YAML frontmatter
        assert!(output.starts_with("---\n"));
        assert!(output.contains("report_id:"));
        assert!(output.contains("privacy_level: enhanced"));
        
        // Test markdown content structure
        assert!(output.contains("# Hardware Compatibility Report"));
        assert!(output.contains("## System Overview"));
        assert!(output.contains("## Hardware Components"));
        
        // Test that sensitive data is anonymized
        assert!(!output.contains("real_serial_number"));
        assert!(output.contains("hw_")); // Anonymized IDs present
    }

    #[test]
    fn test_json_output_generation() {
        let hardware_report = create_test_hardware_report();
        let json_generator = JsonGenerator::new();
        
        let output = json_generator.generate(&hardware_report).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        
        assert_eq!(parsed["privacy_level"], "enhanced");
        assert!(parsed["system"]["kernel"].is_string());
        assert!(parsed["hardware_summary"]["cpu_count"].is_number());
    }

    #[test]  
    fn test_schema_validation() {
        let valid_report = create_valid_hardware_report();
        let invalid_report = create_invalid_hardware_report(); // Missing required fields
        
        let validator = SchemaValidator::new();
        
        assert!(validator.validate(&valid_report).is_ok());
        assert!(validator.validate(&invalid_report).is_err());
    }
}
```

## Integration Testing Specifications

### CLI Interface Tests
```rust
// tests/cli_integration_tests.rs
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_basic_detection_command() {
    let mut cmd = Command::cargo_bin("lx-hw-detect").unwrap();
    
    cmd.arg("--no-root-tools") // Avoid permission issues in CI
        .arg("--format").arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains("report_id"))
        .stdout(predicate::str::contains("hardware_summary"));
}

#[test]
fn test_privacy_level_options() {
    for level in &["basic", "enhanced", "strict"] {
        let mut cmd = Command::cargo_bin("lx-hw-detect").unwrap();
        
        cmd.arg("--privacy-level").arg(level)
            .arg("--no-root-tools")
            .arg("--format").arg("json")
            .assert()
            .success()
            .stdout(predicate::str::contains(&format!("\"privacy_level\": \"{}\"", level)));
    }
}

#[test]
fn test_tool_selection() {
    let mut cmd = Command::cargo_bin("lx-hw-detect").unwrap();
    
    cmd.arg("--tools").arg("lspci,lsusb")
        .arg("--format").arg("json")
        .assert()
        .success();
        
    // Should not contain lshw-specific data
    let output = cmd.output().unwrap();
    let output_str = String::from_utf8(output.stdout).unwrap();
    assert!(!output_str.contains("lshw_data"));
}

#[test] 
fn test_error_handling_tool_not_found() {
    let mut cmd = Command::cargo_bin("lx-hw-detect").unwrap();
    
    cmd.arg("--tools").arg("nonexistent_tool")
        .assert()
        .failure()
        .code(2) // ToolNotFound exit code
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn test_output_file_generation() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join("test-report.md");
    
    let mut cmd = Command::cargo_bin("lx-hw-detect").unwrap();
    
    cmd.arg("--output").arg(&output_path)
        .arg("--no-root-tools")
        .assert()
        .success();
        
    assert!(output_path.exists());
    let content = std::fs::read_to_string(&output_path).unwrap();
    assert!(content.starts_with("---")); // YAML frontmatter
    assert!(content.contains("# Hardware Compatibility Report"));
}

#[test]
fn test_configuration_file_loading() {
    let temp_dir = tempfile::tempdir().unwrap();
    let config_path = temp_dir.path().join("config.toml");
    
    std::fs::write(&config_path, r#"
[privacy]
level = "strict"

[output]  
format = "json"
    "#).unwrap();
    
    let mut cmd = Command::cargo_bin("lx-hw-detect").unwrap();
    
    cmd.env("LX_HW_DETECT_CONFIG", &config_path)
        .arg("--no-root-tools")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"privacy_level\": \"strict\""));
}
```

### Cross-Tool Integration Tests
```rust
#[test]
fn test_multi_tool_data_correlation() {
    // Test that data from different tools correlates correctly
    let mut cmd = Command::cargo_bin("lx-hw-detect").unwrap();
    
    let output = cmd
        .arg("--no-root-tools")
        .arg("--format").arg("json")
        .arg("--verbose")
        .output()
        .unwrap();
        
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    
    // Verify that PCI device count matches lspci data
    if let Some(pci_devices) = report["hardware_summary"]["pci_devices"].as_u64() {
        if let Some(gpu) = report["gpu"].as_array() {
            // GPU should be counted in PCI devices
            assert!(pci_devices >= gpu.len() as u64);
        }
    }
}

#[test]
fn test_parallel_vs_sequential_detection() {
    // Test that parallel and sequential execution produce same results
    let parallel_output = Command::cargo_bin("lx-hw-detect").unwrap()
        .arg("--parallel")
        .arg("--no-root-tools")
        .arg("--format").arg("json")
        .output()
        .unwrap();
        
    let sequential_output = Command::cargo_bin("lx-hw-detect").unwrap()
        .arg("--no-parallel") 
        .arg("--no-root-tools")
        .arg("--format").arg("json")
        .output()
        .unwrap();
        
    // Parse both outputs and compare key fields
    let parallel_data: serde_json::Value = serde_json::from_slice(&parallel_output.stdout).unwrap();
    let sequential_data: serde_json::Value = serde_json::from_slice(&sequential_output.stdout).unwrap();
    
    assert_eq!(
        parallel_data["hardware_summary"],
        sequential_data["hardware_summary"]
    );
}
```

## System Testing Specifications

### Cross-Platform Testing Matrix
```yaml
# .github/workflows/system-tests.yml
strategy:
  matrix:
    distribution:
      - ubuntu:20.04
      - ubuntu:22.04  
      - ubuntu:24.04
      - fedora:38
      - fedora:39
      - archlinux:latest
      - opensuse/leap:15.5
      - debian:11
      - debian:12
    architecture:
      - x86_64
      - aarch64
```

### Docker-Based System Tests
```bash
# tests/system/test_ubuntu.bats
#!/usr/bin/env bats

setup() {
    # Run in Ubuntu container
    docker run --rm -v $(pwd):/workspace ubuntu:22.04 bash -c "
        apt-get update && 
        apt-get install -y lshw dmidecode pciutils usbutils inxi &&
        cd /workspace
    "
}

@test "basic detection works on Ubuntu 22.04" {
    run docker run --rm -v $(pwd):/workspace ubuntu:22.04 /workspace/target/release/lx-hw-detect --no-root-tools --format json
    [ "$status" -eq 0 ]
    [[ "$output" == *"hardware_summary"* ]]
}

@test "privacy levels work correctly" {
    for level in basic enhanced strict; do
        run docker run --rm -v $(pwd):/workspace ubuntu:22.04 /workspace/target/release/lx-hw-detect --privacy-level $level --no-root-tools --format json
        [ "$status" -eq 0 ]
        [[ "$output" == *"\"privacy_level\": \"$level\""* ]]
    done
}

@test "handles missing tools gracefully" {
    # Test with minimal tool set
    run docker run --rm -v $(pwd):/workspace ubuntu:22.04 bash -c "
        apt-get update && apt-get install -y pciutils &&
        /workspace/target/release/lx-hw-detect --format json
    "
    [ "$status" -eq 0 ]
    [[ "$output" == *"hardware_summary"* ]]
}
```

### Performance Testing
```bash
# tests/performance/benchmark.bats
#!/usr/bin/env bats

@test "detection completes within timeout" {
    # Should complete within 30 seconds
    timeout 30 ./target/release/lx-hw-detect --no-root-tools >/dev/null
    [ $? -eq 0 ]
}

@test "parallel execution is faster than sequential" {
    start_parallel=$(date +%s%N)
    ./target/release/lx-hw-detect --parallel --no-root-tools >/dev/null
    end_parallel=$(date +%s%N)
    parallel_time=$((($end_parallel - $start_parallel) / 1000000))
    
    start_sequential=$(date +%s%N)  
    ./target/release/lx-hw-detect --no-parallel --no-root-tools >/dev/null
    end_sequential=$(date +%s%N)
    sequential_time=$((($end_sequential - $start_sequential) / 1000000))
    
    # Parallel should be at least 20% faster
    threshold=$((sequential_time * 80 / 100))
    [ $parallel_time -lt $threshold ]
}

@test "memory usage stays under limit" {
    # Monitor memory usage during execution
    /usr/bin/time -v ./target/release/lx-hw-detect --no-root-tools >/dev/null 2> memory_output.txt
    max_memory=$(grep "Maximum resident set size" memory_output.txt | awk '{print $6}')
    
    # Should use less than 50MB (50000 KB)
    [ $max_memory -lt 50000 ]
}
```

## Privacy and Security Testing

### Anonymization Validation Tests
```rust
#[test]
fn test_no_sensitive_data_leakage() {
    let output = Command::cargo_bin("lx-hw-detect").unwrap()
        .arg("--privacy-level").arg("strict")
        .arg("--format").arg("json")
        .output()
        .unwrap();
        
    let output_str = String::from_utf8(output.stdout).unwrap();
    
    // Ensure no real MAC addresses (00:11:22:33:44:55 pattern)
    let mac_regex = regex::Regex::new(r"([0-9a-fA-F]{2}[:-]){5}([0-9a-fA-F]{2})").unwrap();
    assert!(!mac_regex.is_match(&output_str));
    
    // Ensure no obvious serial numbers (alphanumeric strings > 8 chars)
    let serial_regex = regex::Regex::new(r"\b[A-Z0-9]{8,}\b").unwrap();
    for match_ in serial_regex.find_iter(&output_str) {
        // Should be anonymized (start with hw_)
        assert!(match_.as_str().starts_with("hw_"));
    }
}

#[test]
fn test_differential_privacy_noise() {
    // Run detection multiple times with same hardware
    let mut results = Vec::new();
    
    for _ in 0..10 {
        let output = Command::cargo_bin("lx-hw-detect").unwrap()
            .arg("--privacy-level").arg("strict")
            .arg("--format").arg("json")
            .output()
            .unwrap();
            
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        results.push(report);
    }
    
    // With differential privacy, some numeric values should vary slightly
    let memory_values: Vec<u64> = results.iter()
        .map(|r| r["hardware_summary"]["total_memory_gb"].as_u64().unwrap())
        .collect();
        
    // Should have some variation due to noise injection
    let unique_values: std::collections::HashSet<_> = memory_values.iter().collect();
    assert!(unique_values.len() > 1, "Differential privacy should introduce variation");
}
```

## Test Data Management

### Mock Data Generation
```rust
// tests/common/mock_data.rs
pub fn create_mock_lshw_output() -> &'static str {
    include_str!("../fixtures/lshw_sample.json")
}

pub fn create_mock_dmidecode_output() -> &'static str {
    include_str!("../fixtures/dmidecode_sample.txt")  
}

pub fn create_test_hardware_configurations() -> Vec<HardwareConfig> {
    vec![
        HardwareConfig {
            name: "gaming_desktop",
            cpu: "intel_i7_8700k",
            gpu: "nvidia_rtx_3080",
            memory_gb: 32,
        },
        HardwareConfig {
            name: "business_laptop", 
            cpu: "amd_ryzen_5_5600u",
            gpu: "amd_integrated",
            memory_gb: 16,
        },
        // Additional configurations...
    ]
}
```

### Fixture Management
```
tests/
├── fixtures/
│   ├── lshw_sample.json
│   ├── dmidecode_sample.txt
│   ├── lspci_sample.txt
│   ├── lsusb_sample.txt
│   └── inxi_sample.txt
├── golden/
│   ├── expected_report_basic.md
│   ├── expected_report_enhanced.md
│   └── expected_report_strict.md
└── system/
    ├── ubuntu.bats
    ├── fedora.bats
    └── arch.bats
```

## Continuous Integration Testing

### GitHub Actions Workflow
```yaml
# .github/workflows/tests.yml
name: Tests

on: [push, pull_request]

jobs:
  unit-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run unit tests
        run: cargo test --lib
        
  integration-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install system tools
        run: sudo apt-get install -y lshw dmidecode pciutils usbutils inxi
      - name: Run integration tests  
        run: cargo test --test '*'
        
  system-tests:
    strategy:
      matrix:
        os: [ubuntu-20.04, ubuntu-22.04, ubuntu-24.04]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - name: Install bats
        run: sudo apt-get install -y bats
      - name: Run system tests
        run: bats tests/system/
        
  cross-platform:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        target: [x86_64-unknown-linux-gnu, aarch64-unknown-linux-gnu]
    steps:
      - uses: actions/checkout@v4
      - name: Install cross
        run: cargo install cross
      - name: Build for target
        run: cross build --target ${{ matrix.target }}
      - name: Test with Docker
        run: cross test --target ${{ matrix.target }}
```

This comprehensive testing specification ensures the hardware detection CLI tool meets quality, privacy, and compatibility requirements across diverse Linux environments.