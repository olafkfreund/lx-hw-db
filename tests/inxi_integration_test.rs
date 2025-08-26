//! Integration test for inxi with real system execution

use lx_hw_detect::detectors::inxi::InxiDetector;
use lx_hw_detect::detectors::{HardwareDetector, DetectionData};

#[tokio::test]
async fn test_inxi_real_execution() {
    env_logger::try_init().ok(); // Initialize logging for debugging
    
    let detector = InxiDetector::new();
    
    // Check if inxi is available
    if !detector.is_available().await {
        eprintln!("inxi not available on this system, skipping real execution test");
        return;
    }
    
    // Execute inxi
    match detector.execute().await {
        Ok(output) => {
            println!("inxi execution succeeded");
            
            // Parse the output
            match detector.parse_output(&output) {
                Ok(result) => {
                    println!("Detection result: tool={}, success={}, errors={:?}", 
                           result.tool_name, result.success, result.errors);
                    
                    match result.data {
                        DetectionData::Inxi(data) => {
                            if let Some(summary) = &data.summary {
                                println!("Sections parsed: {}", summary.sections_parsed);
                                println!("Privileged execution: {}", summary.privileged_execution);
                                
                                if !summary.warnings.is_empty() {
                                    println!("Warnings:");
                                    for warning in &summary.warnings {
                                        println!("  {}", warning);
                                    }
                                }
                            }
                            
                            // Show system information
                            if let Some(system) = &data.system {
                                println!("\nSystem Information:");
                                if let Some(kernel) = &system.kernel {
                                    println!("  Kernel: {}", kernel);
                                }
                                if let Some(arch) = &system.arch {
                                    println!("  Architecture: {}", arch);
                                }
                                if let Some(bits) = &system.bits {
                                    println!("  Bits: {}", bits);
                                }
                                if let Some(desktop) = &system.desktop {
                                    println!("  Desktop: {}", desktop);
                                    if let Some(version) = &system.desktop_version {
                                        println!("  Desktop Version: {}", version);
                                    }
                                }
                                if let Some(distro) = &system.distro {
                                    println!("  Distribution: {}", distro);
                                }
                            }
                            
                            // Show machine information
                            if let Some(machine) = &data.machine {
                                println!("\nMachine Information:");
                                if let Some(machine_type) = &machine.machine_type {
                                    println!("  Type: {}", machine_type);
                                }
                                if let Some(system) = &machine.system {
                                    println!("  System: {}", system);
                                }
                                if let Some(product) = &machine.product {
                                    println!("  Product: {}", product);
                                }
                            }
                            
                            // Show CPU information
                            if let Some(cpu) = &data.cpu {
                                println!("\nCPU Information:");
                                if let Some(info) = &cpu.info {
                                    println!("  Info: {}", info);
                                }
                                if let Some(model) = &cpu.model {
                                    println!("  Model: {}", model);
                                }
                                if let Some(bits) = &cpu.bits {
                                    println!("  Bits: {}", bits);
                                }
                                if let Some(cache) = &cpu.cache {
                                    println!("  Cache: {}", cache);
                                }
                            }
                            
                            // Show memory information
                            if let Some(memory) = &data.memory {
                                println!("\nMemory Information:");
                                if let Some(total) = &memory.total {
                                    println!("  Total: {}", total);
                                }
                                if let Some(available) = &memory.available {
                                    println!("  Available: {}", available);
                                }
                                if let Some(used) = &memory.used {
                                    println!("  Used: {}", used);
                                }
                            }
                            
                            // Show bluetooth information
                            if let Some(bluetooth) = &data.bluetooth {
                                println!("\nBluetooth Information:");
                                if let Some(device) = &bluetooth.device {
                                    println!("  Device: {}", device);
                                }
                                if let Some(driver) = &bluetooth.driver {
                                    println!("  Driver: {}", driver);
                                }
                                if let Some(state) = &bluetooth.state {
                                    println!("  State: {}", state);
                                }
                            }
                        }
                        _ => panic!("Expected InxiData"),
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse inxi output: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("inxi execution failed: {}", e);
        }
    }
}