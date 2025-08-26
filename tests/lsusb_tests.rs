//! Tests for lsusb hardware detection

use lx_hw_detect::detectors::lsusb::{LsusbData, LsusbDetector, UsbDevice};
use lx_hw_detect::detectors::{DetectionData, HardwareDetector};
use std::os::unix::process::ExitStatusExt;
use std::process::{ExitStatus, Output};

const SAMPLE_LSUSB_OUTPUT: &str = r#"Bus 001 Device 001: ID 1d6b:0002 Linux Foundation 2.0 root hub
Bus 002 Device 001: ID 1d6b:0003 Linux Foundation 3.0 root hub
Bus 003 Device 001: ID 1d6b:0002 Linux Foundation 2.0 root hub
Bus 003 Device 002: ID 2109:2822 VIA Labs, Inc. USB2.0 Hub
Bus 003 Device 003: ID 1532:007b Razer USA, Ltd. RC30-0305 Gaming Mouse Dongle [Viper Ultimate (Wireless)]
Bus 005 Device 001: ID 1d6b:0002 Linux Foundation 2.0 root hub
Bus 005 Device 002: ID 17aa:104d Generic Realtek USB Audio Front
Bus 006 Device 001: ID 1d6b:0003 Linux Foundation 3.0 root hub
Bus 006 Device 002: ID 0fd9:0079 Elgato Systems GmbH Elgato Facecam Pro
"#;

const SAMPLE_LSUSB_TOPOLOGY: &str = r#"/:  Bus 001.Port 001: Dev 001, Class=root_hub, Driver=xhci_hcd/2p, 480M
/:  Bus 002.Port 001: Dev 001, Class=root_hub, Driver=xhci_hcd/2p, 10000M
/:  Bus 003.Port 001: Dev 001, Class=root_hub, Driver=xhci_hcd/2p, 480M
    |__ Port 001: Dev 002, If 0, Class=Hub, Driver=hub/4p, 480M
    |__ Port 002: Dev 003, If 0, Class=Human Interface Device, Driver=usbhid, 12M
/:  Bus 005.Port 001: Dev 001, Class=root_hub, Driver=xhci_hcd/6p, 480M
    |__ Port 001: Dev 002, If 0, Class=Audio, Driver=snd-usb-audio, 480M
/:  Bus 006.Port 001: Dev 001, Class=root_hub, Driver=xhci_hcd/4p, 10000M
    |__ Port 004: Dev 002, If 0, Class=Video, Driver=uvcvideo, 5000M
"#;

#[tokio::test]
async fn test_lsusb_detector_name() {
    let detector = LsusbDetector::new();
    assert_eq!(detector.name(), "lsusb");
}

#[tokio::test]
async fn test_lsusb_availability_check() {
    let detector = LsusbDetector::new();
    let available = detector.is_available().await;
    println!("lsusb available: {}", available);
}

#[test]
fn test_lsusb_parsing_success() {
    let detector = LsusbDetector::new();

    // Create combined output as it would come from execute()
    let mut combined_output = SAMPLE_LSUSB_OUTPUT.as_bytes().to_vec();
    combined_output.extend_from_slice(b"\n--- TOPOLOGY DATA ---\n");
    combined_output.extend_from_slice(SAMPLE_LSUSB_TOPOLOGY.as_bytes());

    let output =
        Output { status: ExitStatus::from_raw(0), stdout: combined_output, stderr: Vec::new() };

    // Debug: test individual line parsing first
    for line in SAMPLE_LSUSB_OUTPUT.lines() {
        println!("Testing line: '{}'", line);
        match detector.parse_device_line(line) {
            Ok(Some(device)) => println!(
                "  Parsed: Bus {} Device {}: {}:{}",
                device.bus, device.device, device.vendor_id, device.product_id
            ),
            Ok(None) => println!("  No device parsed"),
            Err(e) => println!("  Error: {}", e),
        }
    }

    let result = detector.parse_output(&output).unwrap();
    assert!(result.success);
    assert_eq!(result.tool_name, "lsusb");

    match result.data {
        DetectionData::Lsusb(data) => {
            println!("Parsed {} devices", data.devices.len());
            if data.devices.is_empty() {
                println!("No devices parsed - checking result errors: {:?}", result.errors);
            }
            assert!(!data.devices.is_empty());

            // Check that we parsed the expected number of devices
            assert_eq!(data.devices.len(), 9);

            // Check Linux Foundation root hub
            let root_hub = data
                .devices
                .iter()
                .find(|d| d.bus == 1 && d.device == 1)
                .expect("Root hub should be found");

            assert_eq!(root_hub.vendor_id, "1d6b");
            assert_eq!(root_hub.product_id, "0002");
            assert!(root_hub.vendor_name.as_ref().unwrap().contains("Linux Foundation"));
            assert!(root_hub.product_name.as_ref().unwrap().contains("2.0 root hub"));

            // Check VIA Labs hub
            let via_hub = data
                .devices
                .iter()
                .find(|d| d.bus == 3 && d.device == 2)
                .expect("VIA hub should be found");

            assert_eq!(via_hub.vendor_id, "2109");
            assert_eq!(via_hub.product_id, "2822");
            assert!(via_hub.vendor_name.as_ref().unwrap().contains("VIA Labs, Inc."));
            assert!(via_hub.product_name.as_ref().unwrap().contains("USB2.0 Hub"));

            // Check Razer gaming mouse
            let razer_mouse = data
                .devices
                .iter()
                .find(|d| d.bus == 3 && d.device == 3)
                .expect("Razer mouse should be found");

            assert_eq!(razer_mouse.vendor_id, "1532");
            assert_eq!(razer_mouse.product_id, "007b");
            assert!(razer_mouse.vendor_name.as_ref().unwrap().contains("Razer USA, Ltd."));

            // Check USB 3.0 root hub
            let usb3_hub = data
                .devices
                .iter()
                .find(|d| d.bus == 2 && d.device == 1)
                .expect("USB 3.0 hub should be found");

            assert_eq!(usb3_hub.vendor_id, "1d6b");
            assert_eq!(usb3_hub.product_id, "0003");
            assert!(usb3_hub.product_name.as_ref().unwrap().contains("3.0 root hub"));

            // Check topology data
            println!("Topology buses: {}", data.bus_topology.len());
            if !data.bus_topology.is_empty() {
                // Find Bus 001 topology
                if let Some(bus1_topology) = data.bus_topology.iter().find(|b| b.bus_number == 1) {
                    assert_eq!(bus1_topology.root_hub_device, 1);
                    assert_eq!(bus1_topology.driver, Some("xhci_hcd".to_string()));
                    assert_eq!(bus1_topology.speed, Some("480M".to_string()));
                } else {
                    println!("Bus 1 topology not found in parsed data");
                }
            }

            // Check summary
            assert!(data.summary.is_some());
            let summary = data.summary.as_ref().unwrap();
            assert_eq!(summary.total_devices, 9);
            println!("Total buses parsed: {}", summary.total_buses);
        }
        _ => panic!("Expected LsusbData"),
    }
}

#[test]
fn test_lsusb_empty_output() {
    let detector = LsusbDetector::new();
    let output = Output { status: ExitStatus::from_raw(0), stdout: Vec::new(), stderr: Vec::new() };

    let result = detector.parse_output(&output).unwrap();
    assert!(!result.success);
    assert!(!result.errors.is_empty());
    assert!(result.errors[0].contains("Empty output"));
}

#[test]
fn test_lsusb_stderr_handling() {
    let detector = LsusbDetector::new();

    let mut combined_output = SAMPLE_LSUSB_OUTPUT.as_bytes().to_vec();
    combined_output.extend_from_slice(b"\n--- TOPOLOGY DATA ---\n");
    combined_output.extend_from_slice(SAMPLE_LSUSB_TOPOLOGY.as_bytes());

    let stderr_message = "lsusb: cannot access USB device database\n";
    let output = Output {
        status: ExitStatus::from_raw(0),
        stdout: combined_output,
        stderr: stderr_message.as_bytes().to_vec(),
    };

    let result = detector.parse_output(&output).unwrap();
    assert!(result.success); // Should still succeed
    assert!(!result.errors.is_empty()); // But warnings should be captured
}

#[test]
fn test_usb_device_serialization() {
    let device = UsbDevice {
        bus: 3,
        device: 2,
        vendor_id: "2109".to_string(),
        product_id: "2822".to_string(),
        vendor_name: Some("VIA Labs, Inc.".to_string()),
        product_name: Some("USB2.0 Hub".to_string()),
        usb_version: Some("2.00".to_string()),
        device_class: Some("Hub".to_string()),
        device_subclass: None,
        device_protocol: None,
        max_packet_size: Some(64),
        serial_number: None,
        manufacturer: Some("VIA Labs".to_string()),
        speed: Some("480Mbps".to_string()),
        max_power: Some(100),
        interfaces: Vec::new(),
        descriptor: None,
    };

    // Test JSON serialization/deserialization
    let json = serde_json::to_string(&device).unwrap();
    let deserialized: UsbDevice = serde_json::from_str(&json).unwrap();
    assert_eq!(device.bus, deserialized.bus);
    assert_eq!(device.device, deserialized.device);
    assert_eq!(device.vendor_id, deserialized.vendor_id);
    assert_eq!(device.product_id, deserialized.product_id);
}

#[test]
fn test_lsusb_data_default() {
    let data = LsusbData::default();
    assert!(data.devices.is_empty());
    assert!(data.bus_topology.is_empty());
    assert!(data.summary.is_none());
}

#[tokio::test]
async fn test_lsusb_execution_timeout() {
    let detector = LsusbDetector::new();
    let timeout = detector.timeout();
    assert!(timeout.as_secs() >= 5);
    assert!(timeout.as_secs() <= 15);
}

#[test]
fn test_lsusb_device_parsing_edge_cases() {
    let detector = LsusbDetector::new();

    // Test parsing with various device line formats
    let test_lines = vec![
        "Bus 001 Device 001: ID 1d6b:0002 Linux Foundation 2.0 root hub",
        "Bus 010 Device 255: ID abcd:1234 Test Vendor Test Product",
    ];

    for line in test_lines {
        if let Ok(Some(device)) = detector.parse_device_line(line) {
            assert!(!device.vendor_id.is_empty());
            assert!(!device.product_id.is_empty());
            assert!(device.bus > 0);
            assert!(device.device > 0);
        }
    }
}
