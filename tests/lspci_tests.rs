//! Tests for lspci hardware detection

use lx_hw_detect::detectors::lspci::{LspciData, LspciDetector, PciDevice};
use lx_hw_detect::detectors::{DetectionData, HardwareDetector};
use std::os::unix::process::ExitStatusExt;
use std::process::{ExitStatus, Output};

const SAMPLE_LSPCI_VERBOSE: &str = r#"00:00.0 Host bridge: Advanced Micro Devices, Inc. [AMD] Starship/Matisse Root Complex
	Subsystem: Lenovo ThinkStation P620
	Flags: fast devsel, IOMMU group 0

00:01.0 Host bridge: Advanced Micro Devices, Inc. [AMD] Starship/Matisse PCIe Dummy Host Bridge
	Flags: fast devsel, IOMMU group 1

00:01.1 PCI bridge: Advanced Micro Devices, Inc. [AMD] Starship/Matisse GPP Bridge (prog-if 00 [Normal decode])
	Subsystem: Advanced Micro Devices, Inc. [AMD] Device 1453
	Flags: bus master, fast devsel, latency 0, IRQ 46, IOMMU group 2
	Bus: primary=00, secondary=01, subordinate=01, sec-latency=0
	I/O behind bridge: [disabled] [32-bit]
	Memory behind bridge: f4400000-f48fffff [size=5M] [32-bit]
	Prefetchable memory behind bridge: [disabled] [64-bit]
	Capabilities: <access denied>
	Kernel driver in use: pcieport

00:02.0 Host bridge: Advanced Micro Devices, Inc. [AMD] Starship/Matisse PCIe Dummy Host Bridge
	Flags: fast devsel, IOMMU group 3

01:00.0 Non-Volatile memory controller: Samsung Electronics Co Ltd NVMe SSD Controller SM981/PM981/PM983
	Subsystem: Samsung Electronics Co Ltd SSD 970 EVO Plus 1TB
	Flags: bus master, fast devsel, latency 0, IRQ 47, NUMA node 0, IOMMU group 4
	Memory at f4400000 (64-bit, non-prefetchable) [size=16K]
	Capabilities: <access denied>
	Kernel driver in use: nvme
	Kernel modules: nvme
"#;

const SAMPLE_LSPCI_NUMERIC: &str = r#"00:00.0 0600: 1022:1480
00:01.0 0600: 1022:1482
00:01.1 0604: 1022:1483
00:02.0 0600: 1022:1482
01:00.0 0108: 144d:a808
"#;

#[tokio::test]
async fn test_lspci_detector_name() {
    let detector = LspciDetector::new();
    assert_eq!(detector.name(), "lspci");
}

#[tokio::test]
async fn test_lspci_availability_check() {
    let detector = LspciDetector::new();
    let available = detector.is_available().await;
    println!("lspci available: {}", available);
}

#[test]
fn test_lspci_parsing_success() {
    let detector = LspciDetector::new();

    // Create combined output as it would come from execute()
    let mut combined_output = SAMPLE_LSPCI_VERBOSE.as_bytes().to_vec();
    combined_output.extend_from_slice(b"\n--- NUMERIC DATA ---\n");
    combined_output.extend_from_slice(SAMPLE_LSPCI_NUMERIC.as_bytes());

    let output =
        Output { status: ExitStatus::from_raw(0), stdout: combined_output, stderr: Vec::new() };

    let result = detector.parse_output(&output).unwrap();
    assert!(result.success);
    assert_eq!(result.tool_name, "lspci");

    match result.data {
        DetectionData::Lspci(data) => {
            assert!(!data.devices.is_empty());

            // Check that we parsed some devices
            assert!(data.devices.len() >= 5);

            // Check root complex device
            let root_complex = data
                .devices
                .iter()
                .find(|d| d.address == "00:00.0")
                .expect("Root complex should be found");

            println!("Root complex: {:?}", root_complex);
            assert_eq!(root_complex.class_description, "Host bridge");
            assert_eq!(root_complex.vendor_id, "1022");
            assert_eq!(root_complex.device_id, "1480");
            assert_eq!(root_complex.class_code, "0600");
            assert!(root_complex.vendor_name.is_some());
            if let Some(vendor) = &root_complex.vendor_name {
                assert!(vendor.contains("AMD") || vendor.contains("Advanced Micro Devices"));
            }
            assert_eq!(root_complex.iommu_group, Some(0));

            // Check PCIe bridge
            let pcie_bridge = data
                .devices
                .iter()
                .find(|d| d.address == "00:01.1")
                .expect("PCIe bridge should be found");

            assert_eq!(pcie_bridge.class_description, "PCI bridge");
            assert_eq!(pcie_bridge.kernel_driver, Some("pcieport".to_string()));
            assert_eq!(pcie_bridge.irq, Some(46));
            assert_eq!(pcie_bridge.iommu_group, Some(2));
            assert!(pcie_bridge.bus_info.is_some());

            let bus_info = pcie_bridge.bus_info.as_ref().unwrap();
            assert_eq!(bus_info.primary, Some(0));
            assert_eq!(bus_info.secondary, Some(1));
            assert_eq!(bus_info.subordinate, Some(1));
            assert_eq!(bus_info.sec_latency, Some(0));

            // Check NVMe device
            let nvme_device = data
                .devices
                .iter()
                .find(|d| d.address == "01:00.0")
                .expect("NVMe device should be found");

            assert_eq!(nvme_device.class_description, "Non-Volatile memory controller");
            assert_eq!(nvme_device.vendor_id, "144d");
            assert_eq!(nvme_device.device_id, "a808");
            assert_eq!(nvme_device.class_code, "0108");
            assert_eq!(nvme_device.kernel_driver, Some("nvme".to_string()));
            assert_eq!(nvme_device.kernel_modules, vec!["nvme"]);
            assert!(!nvme_device.memory_regions.is_empty());

            // Check summary
            assert!(data.summary.is_some());
            let summary = data.summary.as_ref().unwrap();
            assert!(summary.total_devices >= 5);
            assert!(summary.devices_by_class.contains_key("Host bridge"));
            assert!(summary.devices_by_class.contains_key("PCI bridge"));
            assert!(summary.devices_with_drivers >= 2); // pcieport and nvme
        }
        _ => panic!("Expected LspciData"),
    }
}

#[test]
fn test_lspci_empty_output() {
    let detector = LspciDetector::new();
    let output = Output { status: ExitStatus::from_raw(0), stdout: Vec::new(), stderr: Vec::new() };

    let result = detector.parse_output(&output).unwrap();
    assert!(!result.success);
    assert!(!result.errors.is_empty());
    assert!(result.errors[0].contains("Empty output"));
}

#[test]
fn test_lspci_stderr_handling() {
    let detector = LspciDetector::new();

    let mut combined_output = SAMPLE_LSPCI_VERBOSE.as_bytes().to_vec();
    combined_output.extend_from_slice(b"\n--- NUMERIC DATA ---\n");
    combined_output.extend_from_slice(SAMPLE_LSPCI_NUMERIC.as_bytes());

    let stderr_message =
        "pcilib: Cannot open /proc/bus/pci\nlspci: Cannot find any working access method.\n";
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
fn test_pci_device_serialization() {
    let device = PciDevice {
        address: "00:01.0".to_string(),
        class_code: "0600".to_string(),
        class_description: "Host bridge".to_string(),
        vendor_id: "1022".to_string(),
        device_id: "1480".to_string(),
        vendor_name: Some("AMD".to_string()),
        device_name: Some("Root Complex".to_string()),
        subsystem: Some("Lenovo ThinkStation".to_string()),
        flags: vec!["fast devsel".to_string(), "IOMMU group 1".to_string()],
        kernel_driver: Some("pcieport".to_string()),
        kernel_modules: vec!["pci_bridge".to_string()],
        bus_info: None,
        memory_regions: Vec::new(),
        io_ports: Vec::new(),
        irq: Some(46),
        iommu_group: Some(1),
        revision: Some("01".to_string()),
    };

    // Test JSON serialization/deserialization
    let json = serde_json::to_string(&device).unwrap();
    let deserialized: PciDevice = serde_json::from_str(&json).unwrap();
    assert_eq!(device.address, deserialized.address);
    assert_eq!(device.vendor_id, deserialized.vendor_id);
    assert_eq!(device.device_id, deserialized.device_id);
}

#[test]
fn test_lspci_data_default() {
    let data = LspciData::default();
    assert!(data.devices.is_empty());
    assert!(data.summary.is_none());
}

#[tokio::test]
async fn test_lspci_execution_timeout() {
    let detector = LspciDetector::new();
    let timeout = detector.timeout();
    assert!(timeout.as_secs() >= 10);
    assert!(timeout.as_secs() <= 30);
}
