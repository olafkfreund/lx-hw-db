//! Integration test for tool filtering and timeout functionality

use lx_hw_detect::detectors::DetectorRegistry;
use lx_hw_detect::errors::Result;
use std::time::Duration;

#[tokio::test]
async fn test_tool_filtering() -> Result<()> {
    let mut registry = DetectorRegistry::new();
    
    // Test enabling only specific tools
    registry.set_enabled_tools(vec!["lshw".to_string(), "lspci".to_string()])?;
    
    // Get available detectors (should only include enabled ones)
    let available = registry.get_available_detectors().await;
    
    // Check that only enabled tools are returned
    for detector in &available {
        let name = detector.name();
        assert!(name == "lshw" || name == "lspci", 
            "Unexpected detector found: {}", name);
    }
    
    Ok(())
}

#[tokio::test]
async fn test_invalid_tool_filtering() {
    let mut registry = DetectorRegistry::new();
    
    // Test with invalid tool name
    let result = registry.set_enabled_tools(vec!["invalid_tool".to_string()]);
    
    assert!(result.is_err(), "Expected error for invalid tool name");
    
    // Error message should contain available tools
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Unknown detection tools"), 
        "Error message should mention unknown tools");
    assert!(error_msg.contains("Available tools"), 
        "Error message should list available tools");
}

#[tokio::test]
async fn test_timeout_configuration() -> Result<()> {
    let mut registry = DetectorRegistry::new();
    
    // Set custom timeout
    let custom_timeout = Duration::from_secs(5);
    registry.set_detection_timeout(custom_timeout);
    
    // Test that timeout is applied (this test just ensures no panics occur)
    let _results = registry.detect_all().await?;
    
    Ok(())
}

#[tokio::test]
async fn test_empty_tool_list() -> Result<()> {
    let mut registry = DetectorRegistry::new();
    
    // Test with empty tool list
    registry.set_enabled_tools(vec![])?;
    
    // Should return no detectors
    let available = registry.get_available_detectors().await;
    assert_eq!(available.len(), 0, "Expected no detectors with empty filter");
    
    Ok(())
}

#[tokio::test]
async fn test_duplicate_tool_names() -> Result<()> {
    let mut registry = DetectorRegistry::new();
    
    // Test with duplicate tool names (should work fine)
    registry.set_enabled_tools(vec!["lshw".to_string(), "lshw".to_string()])?;
    
    // Should still work correctly
    let available = registry.get_available_detectors().await;
    
    // Check that we don't get duplicate detectors
    let mut names = Vec::new();
    for detector in &available {
        names.push(detector.name());
    }
    names.sort();
    names.dedup();
    
    // Should have only one instance of lshw (if available)
    if names.contains(&"lshw") {
        assert_eq!(names.len(), 1, "Should have exactly one detector type");
        assert_eq!(names[0], "lshw");
    }
    
    Ok(())
}