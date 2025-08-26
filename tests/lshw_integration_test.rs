//! Integration test for lshw with real system execution

use lx_hw_detect::detectors::lshw::LshwDetector;
use lx_hw_detect::detectors::{HardwareDetector, DetectionData};

#[tokio::test]
async fn test_lshw_real_execution() {
    env_logger::try_init().ok(); // Initialize logging for debugging
    
    let detector = LshwDetector::new();
    
    // Check if lshw is available
    if !detector.is_available().await {
        eprintln!("lshw not available on this system, skipping real execution test");
        return;
    }
    
    // Execute lshw
    match detector.execute().await {
        Ok(output) => {
            println!("lshw execution succeeded");
            
            // Parse the output
            match detector.parse_output(&output) {
                Ok(result) => {
                    println!("Detection result: tool={}, success={}, errors={:?}", 
                           result.tool_name, result.success, result.errors);
                    
                    match result.data {
                        DetectionData::Lshw(data) => {
                            println!("Found {} hardware components", data.components.len());
                            
                            if let Some(summary) = &data.summary {
                                println!("Components by class:");
                                for (class, count) in &summary.components_by_class {
                                    println!("  {}: {}", class, count);
                                }
                                println!("Privileged execution: {}", summary.privileged_execution);
                                
                                if !summary.warnings.is_empty() {
                                    println!("Warnings:");
                                    for warning in &summary.warnings {
                                        println!("  {}", warning);
                                    }
                                }
                            }
                            
                            // Show some sample components
                            for (i, component) in data.components.iter().take(3).enumerate() {
                                println!("Component {}: class={}, id={}, description={:?}", 
                                       i + 1, component.class, component.id, component.description);
                            }
                        }
                        _ => panic!("Expected LshwData"),
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse lshw output: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("lshw execution failed: {}", e);
        }
    }
}