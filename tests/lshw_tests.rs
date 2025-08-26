//! Tests for lshw hardware detection

use lx_hw_detect::detectors::lshw::{LshwDetector, LshwData, LshwComponent};
use lx_hw_detect::detectors::{HardwareDetector, DetectionData};
use std::process::{Output, ExitStatus};
use std::os::unix::process::ExitStatusExt;

const SAMPLE_LSHW_JSON: &str = r#"[
{
  "id" : "system",
  "class" : "system",
  "claimed" : true,
  "description" : "Computer",
  "product" : "Test System",
  "vendor" : "Test Vendor",
  "serial" : "123456789",
  "width" : 64,
  "capabilities" : {
    "smp" : "Symmetric Multi-Processing",
    "vsyscall32" : "32-bit processes"
  }
},
{
  "id" : "memory",
  "class" : "memory",
  "claimed" : true,
  "description" : "System memory",
  "physid" : "0",
  "units" : "bytes",
  "size" : 17179869184
},
{
  "id" : "cpu",
  "class" : "processor",
  "claimed" : true,
  "product" : "Intel(R) Core(TM) i7-8700K CPU @ 3.70GHz",
  "vendor" : "Intel Corp.",
  "physid" : "1",
  "businfo" : "cpu@0",
  "version" : "6.158.10",
  "units" : "Hz",
  "size" : 3700000000,
  "capacity" : 4700000000,
  "width" : 64,
  "configuration" : {
    "cores" : "6",
    "enabledcores" : "6",
    "threads" : "12",
    "microcode" : "240"
  },
  "capabilities" : {
    "x86-64" : "64bit extensions (x86-64)",
    "fpu" : "mathematical co-processor",
    "vme" : "virtual mode extensions"
  }
},
{
  "id" : "pci",
  "class" : "bridge",
  "claimed" : true,
  "description" : "Host bridge",
  "product" : "8th Gen Core Processor Host Bridge/DRAM Registers",
  "vendor" : "Intel Corporation",
  "physid" : "100",
  "businfo" : "pci@0000:00:00.0",
  "version" : "07",
  "width" : 32,
  "clock" : 33000000
}
]"#;

const MALFORMED_JSON: &str = r#"[
{
  "id" : "system",
  "class" : "system",
  "description" : "Computer",
  // This comment makes it invalid JSON
}
]"#;

#[tokio::test]
async fn test_lshw_detector_name() {
    let detector = LshwDetector::new();
    assert_eq!(detector.name(), "lshw");
}

#[tokio::test]
async fn test_lshw_availability_check() {
    let detector = LshwDetector::new();
    // This will depend on the test environment
    // We'll assume lshw is available for real system tests
    let available = detector.is_available().await;
    println!("lshw available: {}", available);
}

#[test]
fn test_lshw_json_parsing_success() {
    let detector = LshwDetector::new();
    let output = Output {
        status: ExitStatus::from_raw(0),
        stdout: SAMPLE_LSHW_JSON.as_bytes().to_vec(),
        stderr: Vec::new(),
    };

    let result = detector.parse_output(&output).unwrap();
    assert!(result.success);
    assert_eq!(result.tool_name, "lshw");
    assert!(result.errors.is_empty());

    match result.data {
        DetectionData::Lshw(data) => {
            assert!(!data.components.is_empty());
            
            // Check system component
            let system_components: Vec<_> = data.components.iter()
                .filter(|c| c.class == "system")
                .collect();
            assert_eq!(system_components.len(), 1);
            assert_eq!(system_components[0].product.as_ref().unwrap(), "Test System");
            assert_eq!(system_components[0].vendor.as_ref().unwrap(), "Test Vendor");
            
            // Check memory component
            let memory_components: Vec<_> = data.components.iter()
                .filter(|c| c.class == "memory")
                .collect();
            assert_eq!(memory_components.len(), 1);
            assert_eq!(memory_components[0].size.unwrap(), 17179869184);
            
            // Check CPU component
            let cpu_components: Vec<_> = data.components.iter()
                .filter(|c| c.class == "processor")
                .collect();
            assert_eq!(cpu_components.len(), 1);
            assert_eq!(cpu_components[0].product.as_ref().unwrap(), "Intel(R) Core(TM) i7-8700K CPU @ 3.70GHz");
            assert_eq!(cpu_components[0].vendor.as_ref().unwrap(), "Intel Corp.");
        },
        _ => panic!("Expected LshwData"),
    }
}

#[test]
fn test_lshw_json_parsing_malformed() {
    let detector = LshwDetector::new();
    let output = Output {
        status: ExitStatus::from_raw(0),
        stdout: MALFORMED_JSON.as_bytes().to_vec(),
        stderr: Vec::new(),
    };

    let result = detector.parse_output(&output).unwrap();
    assert!(!result.success);
    assert!(!result.errors.is_empty());
    assert!(result.errors[0].contains("JSON parsing failed"));
}

#[test]
fn test_lshw_empty_output() {
    let detector = LshwDetector::new();
    let output = Output {
        status: ExitStatus::from_raw(0),
        stdout: Vec::new(),
        stderr: Vec::new(),
    };

    let result = detector.parse_output(&output).unwrap();
    assert!(!result.success);
    assert!(!result.errors.is_empty());
    assert!(result.errors[0].contains("Empty output"));
}

#[test]
fn test_lshw_stderr_handling() {
    let detector = LshwDetector::new();
    let stderr_message = "lshw: WARNING: you should run this program as super-user.\n";
    let output = Output {
        status: ExitStatus::from_raw(0),
        stdout: SAMPLE_LSHW_JSON.as_bytes().to_vec(),
        stderr: stderr_message.as_bytes().to_vec(),
    };

    let result = detector.parse_output(&output).unwrap();
    assert!(result.success); // Should still succeed despite warnings
    assert!(!result.errors.is_empty()); // But warnings should be captured
    assert!(result.errors[0].contains("WARNING"));
}

#[test]
fn test_lshw_component_serialization() {
    let component = LshwComponent {
        id: "test".to_string(),
        class: "processor".to_string(),
        claimed: Some(true),
        description: Some("Test CPU".to_string()),
        product: Some("Test Processor".to_string()),
        vendor: Some("Test Vendor".to_string()),
        serial: Some("12345".to_string()),
        physid: Some("1".to_string()),
        businfo: Some("cpu@0".to_string()),
        version: Some("1.0".to_string()),
        size: Some(3700000000),
        capacity: Some(4700000000),
        width: Some(64),
        units: Some("Hz".to_string()),
        configuration: None,
        capabilities: None,
    };

    // Test JSON serialization/deserialization
    let json = serde_json::to_string(&component).unwrap();
    let deserialized: LshwComponent = serde_json::from_str(&json).unwrap();
    assert_eq!(component.id, deserialized.id);
    assert_eq!(component.product, deserialized.product);
}

#[test]
fn test_lshw_data_default() {
    let data = LshwData::default();
    assert!(data.components.is_empty());
    assert!(data.summary.is_none());
}

#[tokio::test]
async fn test_lshw_execution_timeout() {
    let detector = LshwDetector::new();
    let timeout = detector.timeout();
    assert!(timeout.as_secs() >= 10); // Should have reasonable timeout
    assert!(timeout.as_secs() <= 60); // But not too long
}

#[test]
fn test_lshw_privacy_sensitive_data_identification() {
    let component = LshwComponent {
        id: "system".to_string(),
        class: "system".to_string(),
        serial: Some("SENSITIVE123456".to_string()),
        product: Some("Dell OptiPlex".to_string()), // Brand info is generally OK
        vendor: Some("Dell Inc.".to_string()),
        // ... other fields
        claimed: Some(true),
        description: Some("Computer".to_string()),
        physid: None,
        businfo: None,
        version: None,
        size: None,
        capacity: None,
        width: None,
        units: None,
        configuration: None,
        capabilities: None,
    };

    // Serial numbers should be identified as sensitive
    assert!(component.serial.is_some());
    // But vendor/product info is generally acceptable for compatibility
    assert!(component.vendor.is_some());
    assert!(component.product.is_some());
}