//! Tests for inxi hardware detection

use lx_hw_detect::detectors::inxi::{InxiDetector, InxiData, InxiSystem};
use lx_hw_detect::detectors::{HardwareDetector, DetectionData};
use std::process::{Output, ExitStatus};
use std::os::unix::process::ExitStatusExt;

const SAMPLE_INXI_OUTPUT: &str = r#"System:
  Kernel 6.16.0 arch x86_64 bits 64
  Desktop Hyprland v 0.50.1 Distro NixOS 25.11 (Xantusia)
Machine:
  Type Desktop System LENOVO product 30E1S6620H v ThinkStation P620 serial <superuser required>
  Mobo LENOVO model 1046 v NO DPK serial <superuser required> UEFI LENOVO v S07KT62A
    date 08/28/2024
CPU:
  Info 64-core model AMD Ryzen Threadripper PRO 3995WX bits 64 type MT MCP cache L2 32 MiB
  Speed (MHz) avg 2668 min/max 571/2701
Graphics:
  Device-1 Advanced Micro Devices [AMD/ATI] Navi 31 [Radeon RX 7900 XT/7900 XTX/7900 GRE/7900M]
    driver amdgpu v kernel
  Device-2 Elgato Systems GmbH Facecam Pro driver uvcvideo type USB
Bluetooth:
  Device-1 TP-Link Bluetooth USB Adapter driver btusb type USB
  Report hciconfig ID hci0 state up bt-v 5.1
Info:
  Memory total 224 GiB available 219.99 GiB used 53.94 GiB
  Processes 1741 Uptime 10d 2h 12m Client Unknown Client: node inxi 3.3.38
"#;

#[tokio::test]
async fn test_inxi_detector_name() {
    let detector = InxiDetector::new();
    assert_eq!(detector.name(), "inxi");
}

#[tokio::test]
async fn test_inxi_availability_check() {
    let detector = InxiDetector::new();
    let available = detector.is_available().await;
    println!("inxi available: {}", available);
}

#[test]
fn test_inxi_parsing_success() {
    let detector = InxiDetector::new();
    
    let output = Output {
        status: ExitStatus::from_raw(0),
        stdout: SAMPLE_INXI_OUTPUT.as_bytes().to_vec(),
        stderr: Vec::new(),
    };

    let result = detector.parse_output(&output).unwrap();
    assert!(result.success);
    assert_eq!(result.tool_name, "inxi");

    match result.data {
        DetectionData::Inxi(data) => {
            // Check system information
            assert!(data.system.is_some());
            let system = data.system.as_ref().unwrap();
            println!("Parsed kernel: {:?}", system.kernel);
            println!("Parsed arch: {:?}", system.arch);
            println!("Parsed distro: {:?}", system.distro);
            assert_eq!(system.kernel, Some("6.16.0".to_string()));
            assert_eq!(system.arch, Some("x86_64".to_string()));
            assert_eq!(system.bits, Some("64".to_string()));
            assert_eq!(system.desktop, Some("Hyprland".to_string()));
            assert_eq!(system.distro, Some("NixOS 25.11 (Xantusia)".to_string()));
            
            // Check machine information
            assert!(data.machine.is_some());
            let machine = data.machine.as_ref().unwrap();
            assert_eq!(machine.machine_type, Some("Desktop".to_string()));
            assert_eq!(machine.system, Some("LENOVO".to_string()));
            assert_eq!(machine.product, Some("30E1S6620H".to_string()));
            
            // Check CPU information
            assert!(data.cpu.is_some());
            let cpu = data.cpu.as_ref().unwrap();
            assert_eq!(cpu.info, Some("64-core".to_string()));
            assert_eq!(cpu.model, Some("AMD Ryzen Threadripper PRO 3995WX".to_string()));
            assert_eq!(cpu.bits, Some("64".to_string()));
            
            // Check memory information from Info section
            assert!(data.memory.is_some());
            let memory = data.memory.as_ref().unwrap();
            assert_eq!(memory.total, Some("224 GiB".to_string()));
            assert_eq!(memory.available, Some("219.99 GiB".to_string()));
            assert_eq!(memory.used, Some("53.94 GiB".to_string()));
            
            // Check bluetooth information
            assert!(data.bluetooth.is_some());
            let bluetooth = data.bluetooth.as_ref().unwrap();
            assert_eq!(bluetooth.device, Some("TP-Link Bluetooth USB Adapter".to_string()));
            assert_eq!(bluetooth.driver, Some("btusb".to_string()));
            assert_eq!(bluetooth.device_type, Some("USB".to_string()));
            
            // Check summary
            assert!(data.summary.is_some());
            let summary = data.summary.as_ref().unwrap();
            assert!(summary.sections_parsed >= 5);
            assert!(summary.privileged_execution);
        },
        _ => panic!("Expected InxiData"),
    }
}

#[test]
fn test_inxi_empty_output() {
    let detector = InxiDetector::new();
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
fn test_inxi_stderr_handling() {
    let detector = InxiDetector::new();
    
    let stderr_message = "inxi: Some warning message\n";
    let output = Output {
        status: ExitStatus::from_raw(0),
        stdout: SAMPLE_INXI_OUTPUT.as_bytes().to_vec(),
        stderr: stderr_message.as_bytes().to_vec(),
    };

    let result = detector.parse_output(&output).unwrap();
    assert!(result.success); // Should still succeed
    assert!(!result.errors.is_empty()); // But warnings should be captured
}

#[test]
fn test_inxi_system_serialization() {
    let system = InxiSystem {
        kernel: Some("6.16.0".to_string()),
        arch: Some("x86_64".to_string()),
        bits: Some("64".to_string()),
        desktop: Some("Hyprland".to_string()),
        desktop_version: Some("0.50.1".to_string()),
        distro: Some("NixOS 25.11".to_string()),
    };

    // Test JSON serialization/deserialization
    let json = serde_json::to_string(&system).unwrap();
    let deserialized: InxiSystem = serde_json::from_str(&json).unwrap();
    assert_eq!(system.kernel, deserialized.kernel);
    assert_eq!(system.arch, deserialized.arch);
    assert_eq!(system.desktop, deserialized.desktop);
}

#[test]
fn test_inxi_data_default() {
    let data = InxiData::default();
    assert!(data.system.is_none());
    assert!(data.machine.is_none());
    assert!(data.cpu.is_none());
    assert!(data.graphics.is_none());
    assert!(data.audio.is_none());
    assert!(data.network.is_none());
    assert!(data.bluetooth.is_none());
    assert!(data.drives.is_none());
    assert!(data.memory.is_none());
    assert!(data.sensors.is_none());
    assert!(data.summary.is_none());
}

#[tokio::test]
async fn test_inxi_execution_timeout() {
    let detector = InxiDetector::new();
    let timeout = detector.timeout();
    assert!(timeout.as_secs() >= 15);
    assert!(timeout.as_secs() <= 30);
}

#[test]
fn test_inxi_section_parsing() {
    let detector = InxiDetector::new();
    
    // Test section detection and parsing
    let test_output = r#"System:
  Kernel 6.16.0 arch x86_64
CPU:
  Info 64-core model AMD Threadripper
"#;
    
    let result = detector.parse_inxi_output(test_output).unwrap();
    
    // Should have parsed system and CPU sections
    assert!(result.system.is_some());
    assert!(result.cpu.is_some());
    
    let system = result.system.as_ref().unwrap();
    assert!(system.kernel.is_some());
    println!("System parsed: {:?}", system);
    
    let cpu = result.cpu.as_ref().unwrap();
    assert!(cpu.info.is_some() || cpu.model.is_some()); // At least some CPU info should be parsed
    println!("CPU parsed: {:?}", cpu);
}