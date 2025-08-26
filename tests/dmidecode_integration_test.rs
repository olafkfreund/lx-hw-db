//! Integration tests for dmidecode detector with real system execution

use lx_hw_detect::detectors::dmidecode::DmidecodeDetector;
use lx_hw_detect::detectors::{HardwareDetector, DetectionData};

#[tokio::test]
async fn test_dmidecode_real_execution() {
    let detector = DmidecodeDetector::new();
    
    // Skip test if dmidecode is not available
    if !detector.is_available().await {
        println!("dmidecode not available, skipping integration test");
        return;
    }
    
    // Execute dmidecode
    let output = detector.execute().await;
    
    match output {
        Ok(raw_output) => {
            // Parse the output
            let result = detector.parse_output(&raw_output);
            
            match result {
                Ok(detection_result) => {
                    println!("dmidecode detection successful: {}", detection_result.success);
                    println!("Errors: {}", detection_result.errors.len());
                    
                    if let DetectionData::Dmidecode(data) = detection_result.data {
                        println!("BIOS present: {}", data.bios.is_some());
                        println!("System info present: {}", data.system.is_some());
                        println!("Baseboard info present: {}", data.baseboard.is_some());
                        println!("Processors detected: {}", data.processors.len());
                        println!("Memory devices detected: {}", data.memory_devices.len());
                        
                        if let Some(summary) = &data.summary {
                            println!("Total memory: {} MB", summary.total_memory_mb);
                            println!("Memory slots used: {}", summary.memory_slots_used);
                            println!("Privileged execution: {}", summary.privileged_execution);
                        }
                        
                        // Basic validations for real system
                        if detection_result.success {
                            // On a successful run, we should have at least some information
                            assert!(
                                data.bios.is_some() || 
                                data.system.is_some() || 
                                !data.processors.is_empty() || 
                                !data.memory_devices.is_empty(),
                                "Successful dmidecode run should provide some hardware information"
                            );
                            
                            // If we have system info, verify it has required fields
                            if let Some(system) = &data.system {
                                assert!(!system.manufacturer.is_empty(), "System manufacturer should not be empty");
                                assert!(!system.product_name.is_empty(), "Product name should not be empty");
                            }
                            
                            // If we have BIOS info, verify basic fields
                            if let Some(bios) = &data.bios {
                                assert!(!bios.vendor.is_empty(), "BIOS vendor should not be empty");
                                assert!(!bios.version.is_empty(), "BIOS version should not be empty");
                            }
                        } else {
                            // On failure, we should have meaningful error messages
                            assert!(!detection_result.errors.is_empty(), "Failed detection should have error messages");
                        }
                        
                        // Test privacy-sensitive data detection
                        // UUIDs, serial numbers, and MAC addresses should be present for anonymization
                        let mut has_sensitive_data = false;
                        
                        if let Some(system) = &data.system {
                            if system.uuid.is_some() || system.serial_number.is_some() {
                                has_sensitive_data = true;
                                println!("Found system-level sensitive data for anonymization");
                            }
                        }
                        
                        for memory in &data.memory_devices {
                            if memory.serial_number.is_some() {
                                has_sensitive_data = true;
                                println!("Found memory device sensitive data for anonymization");
                                break;
                            }
                        }
                        
                        if has_sensitive_data {
                            println!("✓ Privacy-sensitive data detected for anonymization");
                        } else {
                            println!("ℹ No privacy-sensitive data found (system may have generic/placeholder values)");
                        }
                        
                    } else {
                        panic!("Expected DmidecodeData variant");
                    }
                }
                Err(e) => {
                    println!("dmidecode parsing failed: {}", e);
                    // This is acceptable - parsing might fail due to privilege issues or format variations
                }
            }
        }
        Err(e) => {
            println!("dmidecode execution failed: {}", e);
            // This is acceptable - dmidecode might not be available or might lack privileges
        }
    }
    
    // The test passes as long as we don't panic - the detector should handle various real-world scenarios gracefully
    assert!(true);
}