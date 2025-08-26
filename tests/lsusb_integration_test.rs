//! Integration test for lsusb with real system execution

use lx_hw_detect::detectors::lsusb::LsusbDetector;
use lx_hw_detect::detectors::{DetectionData, HardwareDetector};

#[tokio::test]
async fn test_lsusb_real_execution() {
    env_logger::try_init().ok(); // Initialize logging for debugging

    let detector = LsusbDetector::new();

    // Check if lsusb is available
    if !detector.is_available().await {
        eprintln!("lsusb not available on this system, skipping real execution test");
        return;
    }

    // Execute lsusb
    match detector.execute().await {
        Ok(output) => {
            println!("lsusb execution succeeded");

            // Parse the output
            match detector.parse_output(&output) {
                Ok(result) => {
                    println!(
                        "Detection result: tool={}, success={}, errors={:?}",
                        result.tool_name, result.success, result.errors
                    );

                    match result.data {
                        DetectionData::Lsusb(data) => {
                            println!("Found {} USB devices", data.devices.len());

                            if let Some(summary) = &data.summary {
                                println!("Total buses: {}", summary.total_buses);
                                println!("Devices by class:");
                                for (class, count) in &summary.devices_by_class {
                                    println!("  {}: {}", class, count);
                                }
                                println!(
                                    "Devices with drivers: {}/{}",
                                    summary.devices_with_drivers, summary.total_devices
                                );
                                println!("Privileged execution: {}", summary.privileged_execution);

                                if !summary.usb_versions.is_empty() {
                                    println!("USB versions:");
                                    for (version, count) in &summary.usb_versions {
                                        println!("  USB {}: {} devices", version, count);
                                    }
                                }

                                if !summary.warnings.is_empty() {
                                    println!("Warnings:");
                                    for warning in &summary.warnings {
                                        println!("  {}", warning);
                                    }
                                }
                            }

                            // Show some sample devices
                            for (i, device) in data.devices.iter().take(8).enumerate() {
                                println!(
                                    "Device {}: Bus {:03} Device {:03}: {}:{} - {:?}",
                                    i + 1,
                                    device.bus,
                                    device.device,
                                    device.vendor_id,
                                    device.product_id,
                                    device.vendor_name
                                );
                                if let Some(product) = &device.product_name {
                                    println!("  Product: {}", product);
                                }
                            }

                            // Show topology if available
                            if !data.bus_topology.is_empty() {
                                println!("\nUSB Bus Topology:");
                                for bus in &data.bus_topology {
                                    println!(
                                        "Bus {}: Device {} (Driver: {:?}, Speed: {:?})",
                                        bus.bus_number, bus.root_hub_device, bus.driver, bus.speed
                                    );
                                }
                            }
                        }
                        _ => panic!("Expected LsusbData"),
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse lsusb output: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("lsusb execution failed: {}", e);
        }
    }
}
