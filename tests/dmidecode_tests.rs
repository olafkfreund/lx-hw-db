//! Comprehensive unit tests for dmidecode detector

use lx_hw_detect::detectors::dmidecode::DmidecodeDetector;
use lx_hw_detect::detectors::{HardwareDetector, DetectionData};
use std::process::{Output, ExitStatus};

/// Mock dmidecode output for testing - represents real dmidecode output structure
const MOCK_DMIDECODE_OUTPUT: &str = r#"# dmidecode 3.3
Getting SMBIOS data from sysfs.
SMBIOS 3.0 present.
Table at 0x000F0480.

Handle 0x0000, DMI type 0, 26 bytes
BIOS Information
	Vendor: American Megatrends Inc.
	Version: F20
	Release Date: 05/21/2019
	Address: 0xF0000
	Runtime Size: 64 kB
	ROM Size: 16 MB
	Characteristics:
		PCI is supported
		BIOS is upgradeable
		BIOS shadowing is allowed
		Boot from CD is supported
		Selectable boot is supported
		BIOS ROM is socketed
		EDD is supported
		5.25"/1.2 MB floppy services are supported (int 13h)
		3.5"/720 kB floppy services are supported (int 13h)
		3.5"/2.88 MB floppy services are supported (int 13h)
		Print screen service is supported (int 5h)
		8042 keyboard services are supported (int 9h)
		Serial services are supported (int 14h)
		Printer services are supported (int 17h)
		ACPI is supported
		USB legacy is supported
		BIOS boot specification is supported
		Targeted content distribution is supported
		UEFI is supported
	BIOS Revision: 5.14

Handle 0x0001, DMI type 1, 27 bytes
System Information
	Manufacturer: Gigabyte Technology Co., Ltd.
	Product Name: B450M DS3H
	Version: -CF
	Serial Number: To be filled by O.E.M.
	UUID: 12345678-1234-5678-9012-123456789012
	Wake-up Type: Power Switch
	SKU Number: To be filled by O.E.M.
	Family: To be filled by O.E.M.

Handle 0x0002, DMI type 2, 15 bytes
Base Board Information
	Manufacturer: Gigabyte Technology Co., Ltd.
	Product Name: B450M DS3H-CF
	Version: x.x
	Serial Number: To be filled by O.E.M.
	Asset Tag: To be filled by O.E.M.
	Features:
		Board is a hosting board
		Board is replaceable
	Location In Chassis: To be filled by O.E.M.
	Chassis Handle: 0x0003
	Type: Motherboard
	Contained Object Handles: 0

Handle 0x0004, DMI type 4, 48 bytes
Processor Information
	Socket Designation: AM4
	Type: Central Processor
	Family: Zen
	Manufacturer: Advanced Micro Devices, Inc.
	ID: 10 0F 81 00 FF FB 8B 17
	Signature: Family 23, Model 17, Stepping 0
	Flags:
		FPU (Floating-point unit on-chip)
		VME (Virtual mode extension)
		DE (Debugging extension)
		PSE (Page size extension)
		TSC (Time stamp counter)
		MSR (Model specific registers)
		PAE (Physical address extension)
		MCE (Machine check exception)
		CX8 (CMPXCHG8 instruction supported)
		APIC (On-chip APIC hardware supported)
		SEP (Fast system call)
		MTRR (Memory type range registers)
		PGE (Page global enable)
		MCA (Machine check architecture)
		CMOV (Conditional move instruction supported)
		PAT (Page attribute table)
		PSE-36 (36-bit page size extension)
		CLFSH (CLFLUSH instruction supported)
		MMX (MMX technology supported)
		FXSR (FXSAVE and FXRSTOR instructions supported)
		SSE (Streaming SIMD extensions)
		SSE2 (Streaming SIMD extensions 2)
		HTT (Multi-threading)
	Version: AMD Ryzen 5 3600 6-Core Processor
	Voltage: 1.1 V
	External Clock: 100 MHz
	Max Speed: 3600 MHz
	Current Speed: 3600 MHz
	Status: Populated, Enabled
	Upgrade: Socket AM4
	L1 Cache Handle: 0x0005
	L2 Cache Handle: 0x0006
	L3 Cache Handle: 0x0007
	Serial Number: Unknown
	Asset Tag: Unknown
	Part Number: Unknown
	Core Count: 6
	Core Enabled: 6
	Thread Count: 12
	Characteristics:
		64-bit capable
		Multi-Core
		Hardware Thread
		Execute Protection
		Enhanced Virtualization
		Power/Performance Control

Handle 0x0011, DMI type 17, 84 bytes
Memory Device
	Array Handle: 0x0010
	Error Information Handle: Not Provided
	Total Width: 64 bits
	Data Width: 64 bits
	Size: 8192 MB
	Form Factor: DIMM
	Set: None
	Locator: DIMM 0
	Bank Locator: P0 CHANNEL A
	Type: DDR4
	Type Detail: Synchronous Unbuffered (Unregistered)
	Speed: 3200 MT/s
	Manufacturer: G.Skill
	Serial Number: 12345678
	Asset Tag: 9876543210987654321
	Part Number: F4-3200C16-8GVGB
	Rank: 1
	Configured Memory Speed: 3200 MT/s
	Minimum Voltage: Unknown
	Maximum Voltage: Unknown
	Configured Voltage: 1.350 V

Handle 0x0012, DMI type 17, 84 bytes
Memory Device
	Array Handle: 0x0010
	Error Information Handle: Not Provided
	Total Width: 64 bits
	Data Width: 64 bits
	Size: 8192 MB
	Form Factor: DIMM
	Set: None
	Locator: DIMM 1
	Bank Locator: P0 CHANNEL B
	Type: DDR4
	Type Detail: Synchronous Unbuffered (Unregistered)
	Speed: 3200 MT/s
	Manufacturer: G.Skill
	Serial Number: 87654321
	Asset Tag: 1234567890123456789
	Part Number: F4-3200C16-8GVGB
	Rank: 1
	Configured Memory Speed: 3200 MT/s
	Minimum Voltage: Unknown
	Maximum Voltage: Unknown
	Configured Voltage: 1.350 V
"#;

/// Minimal dmidecode output for privilege testing
const MOCK_DMIDECODE_NO_PRIVILEGES: &str = r#"# dmidecode 3.3
/dev/mem: Operation not permitted
"#;

/// Invalid dmidecode output for error handling testing
const MOCK_DMIDECODE_INVALID: &str = r#"dmidecode: command not found
"#;

#[tokio::test]
async fn test_dmidecode_detector_creation() {
    let detector = DmidecodeDetector::new();
    assert_eq!(detector.name(), "dmidecode");
}

#[tokio::test]
async fn test_parse_complete_dmidecode_output() {
    let detector = DmidecodeDetector::new();
    let output = create_mock_output(MOCK_DMIDECODE_OUTPUT, "", true);
    
    let result = detector.parse_output(&output).unwrap();
    
    assert!(result.success);
    assert_eq!(result.tool_name, "dmidecode");
    
    if let DetectionData::Dmidecode(data) = result.data {
        // Test BIOS information
        assert!(data.bios.is_some());
        let bios = data.bios.unwrap();
        assert_eq!(bios.vendor, "American Megatrends Inc.");
        assert_eq!(bios.version, "F20");
        assert_eq!(bios.release_date, "05/21/2019");
        assert!(bios.characteristics.contains(&"PCI is supported".to_string()));
        assert!(bios.characteristics.contains(&"UEFI is supported".to_string()));
        
        // Test System information
        assert!(data.system.is_some());
        let system = data.system.unwrap();
        assert_eq!(system.manufacturer, "Gigabyte Technology Co., Ltd.");
        assert_eq!(system.product_name, "B450M DS3H");
        assert_eq!(system.version, Some("-CF".to_string()));
        assert_eq!(system.uuid, Some("12345678-1234-5678-9012-123456789012".to_string()));
        
        // Test Baseboard information
        assert!(data.baseboard.is_some());
        let baseboard = data.baseboard.unwrap();
        assert_eq!(baseboard.manufacturer, "Gigabyte Technology Co., Ltd.");
        assert_eq!(baseboard.product_name, "B450M DS3H-CF");
        assert_eq!(baseboard.version, Some("x.x".to_string()));
        
        // Test Processor information
        assert!(!data.processors.is_empty());
        let processor = &data.processors[0];
        assert_eq!(processor.socket_designation, "AM4");
        assert_eq!(processor.manufacturer, "Advanced Micro Devices, Inc.");
        assert_eq!(processor.version, "AMD Ryzen 5 3600 6-Core Processor");
        assert_eq!(processor.core_count, Some(6));
        assert_eq!(processor.thread_count, Some(12));
        assert_eq!(processor.max_speed, Some(3600));
        assert_eq!(processor.current_speed, Some(3600));
        
        // Test Memory devices
        assert_eq!(data.memory_devices.len(), 2);
        
        let memory1 = &data.memory_devices[0];
        assert_eq!(memory1.locator, "DIMM 0");
        assert_eq!(memory1.size_mb, Some(8192));
        assert_eq!(memory1.memory_type, Some("DDR4".to_string()));
        assert_eq!(memory1.speed_mts, Some(3200));
        assert_eq!(memory1.manufacturer, Some("G.Skill".to_string()));
        assert_eq!(memory1.part_number, Some("F4-3200C16-8GVGB".to_string()));
        
        let memory2 = &data.memory_devices[1];
        assert_eq!(memory2.locator, "DIMM 1");
        assert_eq!(memory2.size_mb, Some(8192));
        
        // Test summary
        assert!(data.summary.is_some());
        let summary = data.summary.unwrap();
        assert_eq!(summary.total_memory_mb, 16384); // 2 x 8GB
        assert_eq!(summary.memory_slots_used, 2);
        assert!(summary.privileged_execution);  // Mock data is comprehensive, indicating privileged execution
        
    } else {
        panic!("Expected DmidecodeData but got different variant");
    }
}

#[tokio::test]
async fn test_parse_no_privileges_output() {
    let detector = DmidecodeDetector::new();
    let output = create_mock_output("", MOCK_DMIDECODE_NO_PRIVILEGES, false);
    
    let result = detector.parse_output(&output).unwrap();
    
    assert!(!result.success);
    assert_eq!(result.tool_name, "dmidecode");
    assert!(!result.errors.is_empty());
    // The error should contain the permission message from stderr  
    assert!(result.errors.iter().any(|e| e.contains("Operation not permitted")));
    
    if let DetectionData::Dmidecode(data) = result.data {
        assert!(data.bios.is_none());
        assert!(data.system.is_none());
        assert!(data.baseboard.is_none());
        assert!(data.processors.is_empty());
        assert!(data.memory_devices.is_empty());
    }
}

#[tokio::test]
async fn test_parse_empty_output() {
    let detector = DmidecodeDetector::new();
    let output = create_mock_output("", "", true);
    
    let result = detector.parse_output(&output).unwrap();
    
    assert!(!result.success);
    assert!(result.errors.contains(&"Empty output from dmidecode".to_string()));
}

#[tokio::test]
async fn test_parse_invalid_output() {
    let detector = DmidecodeDetector::new();
    let output = create_mock_output("", MOCK_DMIDECODE_INVALID, false);
    
    let result = detector.parse_output(&output).unwrap();
    
    assert!(!result.success);
    assert!(!result.errors.is_empty());
}

#[tokio::test]
async fn test_bios_characteristic_parsing() {
    let detector = DmidecodeDetector::new();
    let output = create_mock_output(MOCK_DMIDECODE_OUTPUT, "", true);
    
    let result = detector.parse_output(&output).unwrap();
    
    if let DetectionData::Dmidecode(data) = result.data {
        let bios = data.bios.unwrap();
        
        // Test that complex characteristics are correctly parsed
        assert!(bios.characteristics.contains(&"PCI is supported".to_string()));
        assert!(bios.characteristics.contains(&"BIOS is upgradeable".to_string()));
        assert!(bios.characteristics.contains(&"USB legacy is supported".to_string()));
        assert!(bios.characteristics.contains(&"UEFI is supported".to_string()));
        assert!(bios.characteristics.len() >= 10); // Should have many characteristics
    }
}

#[tokio::test]
async fn test_processor_flags_parsing() {
    let detector = DmidecodeDetector::new();
    let output = create_mock_output(MOCK_DMIDECODE_OUTPUT, "", true);
    
    let result = detector.parse_output(&output).unwrap();
    
    if let DetectionData::Dmidecode(data) = result.data {
        let processor = &data.processors[0];
        
        // Test that processor flags are correctly parsed
        assert!(processor.flags.contains(&"FPU (Floating-point unit on-chip)".to_string()));
        assert!(processor.flags.contains(&"VME (Virtual mode extension)".to_string()));
        assert!(processor.flags.contains(&"SSE (Streaming SIMD extensions)".to_string()));
        assert!(processor.flags.contains(&"SSE2 (Streaming SIMD extensions 2)".to_string()));
        assert!(processor.flags.len() >= 10); // Should have many flags
    }
}

#[tokio::test]
async fn test_memory_detail_parsing() {
    let detector = DmidecodeDetector::new();
    let output = create_mock_output(MOCK_DMIDECODE_OUTPUT, "", true);
    
    let result = detector.parse_output(&output).unwrap();
    
    if let DetectionData::Dmidecode(data) = result.data {
        let memory1 = &data.memory_devices[0];
        
        // Test detailed memory parsing
        assert_eq!(memory1.total_width, Some(64));
        assert_eq!(memory1.data_width, Some(64));
        assert_eq!(memory1.form_factor, Some("DIMM".to_string()));
        assert_eq!(memory1.bank_locator, Some("P0 CHANNEL A".to_string()));
        assert_eq!(memory1.type_detail, Some("Synchronous Unbuffered (Unregistered)".to_string()));
        assert_eq!(memory1.configured_speed, Some(3200));
        assert_eq!(memory1.rank, Some(1));
    }
}

#[tokio::test]
async fn test_uuid_privacy_consideration() {
    let detector = DmidecodeDetector::new();
    let output = create_mock_output(MOCK_DMIDECODE_OUTPUT, "", true);
    
    let result = detector.parse_output(&output).unwrap();
    
    if let DetectionData::Dmidecode(data) = result.data {
        let system = data.system.unwrap();
        
        // UUID should be captured for potential anonymization
        assert_eq!(system.uuid, Some("12345678-1234-5678-9012-123456789012".to_string()));
        
        // Check that serial numbers are captured (for anonymization)
        assert!(data.memory_devices[0].serial_number.is_some());
        assert!(data.memory_devices[1].serial_number.is_some());
    }
}

#[tokio::test]
async fn test_summary_statistics() {
    let detector = DmidecodeDetector::new();
    let output = create_mock_output(MOCK_DMIDECODE_OUTPUT, "", true);
    
    let result = detector.parse_output(&output).unwrap();
    
    if let DetectionData::Dmidecode(data) = result.data {
        let summary = data.summary.unwrap();
        
        // Test calculated summary statistics
        assert_eq!(summary.total_memory_mb, 16384); // 2x 8GB
        assert_eq!(summary.memory_slots_used, 2);
        assert_eq!(summary.processor_count, 1);
        assert!(summary.bios_present);
        assert!(summary.system_info_present);
        assert!(summary.baseboard_present);
    }
}

#[tokio::test]
async fn test_dmidecode_detector_timeout() {
    let detector = DmidecodeDetector::new();
    let timeout = detector.timeout();
    assert_eq!(timeout, std::time::Duration::from_secs(15)); // Should be faster than lshw
}

/// Helper function to create mock process output
fn create_mock_output(stdout: &str, stderr: &str, success: bool) -> Output {
    Output {
        status: if success {
            ExitStatus::default() // Success status
        } else {
            // Create a non-successful status - this is system-dependent
            #[cfg(unix)]
            {
                use std::os::unix::process::ExitStatusExt;
                ExitStatus::from_raw(256) // Exit code 1
            }
            #[cfg(not(unix))]
            {
                ExitStatus::default() // Fallback for non-Unix systems
            }
        },
        stdout: stdout.as_bytes().to_vec(),
        stderr: stderr.as_bytes().to_vec(),
    }
}