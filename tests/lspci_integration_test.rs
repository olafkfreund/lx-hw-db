//! Integration test for lspci with real system execution

use lx_hw_detect::detectors::lspci::LspciDetector;
use lx_hw_detect::detectors::{HardwareDetector, DetectionData};

#[tokio::test]
async fn test_lspci_real_execution() {
    env_logger::try_init().ok(); // Initialize logging for debugging
    
    let detector = LspciDetector::new();
    
    // Check if lspci is available
    if !detector.is_available().await {
        eprintln!("lspci not available on this system, skipping real execution test");
        return;
    }
    
    // Execute lspci
    match detector.execute().await {
        Ok(output) => {
            println!("lspci execution succeeded");
            
            // Parse the output
            match detector.parse_output(&output) {
                Ok(result) => {
                    println!("Detection result: tool={}, success={}, errors={:?}", 
                           result.tool_name, result.success, result.errors);
                    
                    match result.data {
                        DetectionData::Lspci(data) => {
                            println!("Found {} PCI devices", data.devices.len());
                            
                            if let Some(summary) = &data.summary {
                                println!("Devices by class:");
                                for (class, count) in &summary.devices_by_class {
                                    println!("  {}: {}", class, count);
                                }
                                println!("Devices with drivers: {}/{}", 
                                       summary.devices_with_drivers, summary.total_devices);
                                println!("Privileged execution: {}", summary.privileged_execution);
                                
                                if !summary.warnings.is_empty() {
                                    println!("Warnings:");
                                    for warning in &summary.warnings {
                                        println!("  {}", warning);
                                    }
                                }
                            }
                            
                            // Show some sample devices
                            for (i, device) in data.devices.iter().take(5).enumerate() {
                                println!("Device {}: {} - {} ({}:{})", 
                                       i + 1, 
                                       device.address, 
                                       device.class_description,
                                       device.vendor_id,
                                       device.device_id);
                                if let Some(driver) = &device.kernel_driver {
                                    println!("  Driver: {}", driver);
                                }
                                if device.iommu_group.is_some() {
                                    println!("  IOMMU group: {}", device.iommu_group.unwrap());
                                }
                            }
                        }
                        _ => panic!("Expected LspciData"),
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse lspci output: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("lspci execution failed: {}", e);
        }
    }
}