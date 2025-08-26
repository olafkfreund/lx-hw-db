//! Compatibility analysis and validation for hardware indices

use super::*;
use crate::errors::{Result, LxHwError};

/// Validator for ensuring index integrity and consistency
pub struct IndexValidator<'a> {
    indices: &'a IndexCollection,
}

impl<'a> IndexValidator<'a> {
    pub fn new(indices: &'a IndexCollection) -> Self {
        Self { indices }
    }

    /// Validate all indices for consistency and completeness
    pub fn validate(&self) -> Result<ValidationReport> {
        let mut report = ValidationReport::default();

        // Validate each index type
        report.vendor_validation = self.validate_vendor_index()?;
        report.component_validation = self.validate_component_index()?;
        report.kernel_validation = self.validate_kernel_index()?;
        report.distribution_validation = self.validate_distribution_index()?;
        report.search_validation = self.validate_search_index()?;
        report.matrix_validation = self.validate_compatibility_matrix()?;
        report.statistics_validation = self.validate_statistics()?;

        // Overall validation status
        report.overall_valid = report.all_validations_passed();
        
        if !report.overall_valid {
            return Err(LxHwError::ValidationError {
                message: "Index validation failed".to_string(),
            });
        }

        Ok(report)
    }

    /// Validate vendor index consistency
    fn validate_vendor_index(&self) -> Result<IndexValidation> {
        let mut validation = IndexValidation::new("vendor_index");

        for (vendor, entry) in &self.indices.by_vendor {
            // Check for empty vendor names
            if vendor.is_empty() {
                validation.add_error("Empty vendor name found");
            }

            // Check report count consistency
            if entry.total_reports == 0 {
                validation.add_warning(&format!("Vendor '{}' has zero reports", vendor));
            }

            // Check component list consistency
            for (component_type, models) in &entry.components {
                if models.is_empty() {
                    validation.add_warning(&format!(
                        "Vendor '{}' has empty model list for component '{}'",
                        vendor, component_type
                    ));
                }
            }

            // Check compatibility score range
            if entry.compatibility_score < 0.0 || entry.compatibility_score > 100.0 {
                validation.add_error(&format!(
                    "Invalid compatibility score {} for vendor '{}'",
                    entry.compatibility_score, vendor
                ));
            }

            // Check recent reports list
            if entry.recent_reports.len() > 10 {
                validation.add_warning(&format!(
                    "Vendor '{}' has too many recent reports ({})",
                    vendor, entry.recent_reports.len()
                ));
            }
        }

        validation.item_count = self.indices.by_vendor.len();
        Ok(validation)
    }

    /// Validate component index consistency
    fn validate_component_index(&self) -> Result<IndexValidation> {
        let mut validation = IndexValidation::new("component_index");
        
        let known_components = [
            "CPU", "GPU", "Storage", "Network", "Audio", "Bluetooth",
            "USB", "Camera", "Touchpad", "Keyboard", "Display"
        ];

        for (component_type, entry) in &self.indices.by_component {
            // Check for unknown component types
            if !known_components.contains(&component_type.as_str()) {
                validation.add_warning(&format!("Unknown component type: '{}'", component_type));
            }

            // Validate report count
            if entry.total_reports == 0 {
                validation.add_error(&format!("Component '{}' has zero reports", component_type));
            }

            // Validate vendor counts
            for (vendor, count) in &entry.vendors {
                if *count == 0 {
                    validation.add_warning(&format!(
                        "Zero count for vendor '{}' in component '{}'",
                        vendor, component_type
                    ));
                }
            }

            // Validate popular models
            for model in &entry.popular_models {
                if model.report_count == 0 {
                    validation.add_error(&format!(
                        "Popular model '{}' has zero reports", model.model
                    ));
                }
                
                if model.avg_compatibility < 0.0 || model.avg_compatibility > 100.0 {
                    validation.add_error(&format!(
                        "Invalid compatibility score {} for model '{}'",
                        model.avg_compatibility, model.model
                    ));
                }
            }
        }

        validation.item_count = self.indices.by_component.len();
        Ok(validation)
    }

    /// Validate kernel index consistency
    fn validate_kernel_index(&self) -> Result<IndexValidation> {
        let mut validation = IndexValidation::new("kernel_index");

        for (kernel_version, entry) in &self.indices.by_kernel {
            // Validate kernel version format
            if !self.is_valid_kernel_version(kernel_version) {
                validation.add_warning(&format!("Unusual kernel version format: '{}'", kernel_version));
            }

            // Check report counts
            if entry.total_reports == 0 {
                validation.add_error(&format!("Kernel '{}' has zero reports", kernel_version));
            }

            // Validate compatibility stats sum
            let total_compat: usize = entry.compatibility_stats.values().sum();
            if total_compat != entry.total_reports {
                validation.add_error(&format!(
                    "Compatibility stats sum ({}) doesn't match total reports ({}) for kernel '{}'",
                    total_compat, entry.total_reports, kernel_version
                ));
            }

            // Check for required compatibility categories
            let required_categories = vec!["excellent", "good", "fair", "poor", "unknown"];
            for category in required_categories {
                if !entry.compatibility_stats.contains_key(category) {
                    validation.add_warning(&format!(
                        "Missing compatibility category '{}' for kernel '{}'",
                        category, kernel_version
                    ));
                }
            }
        }

        validation.item_count = self.indices.by_kernel.len();
        Ok(validation)
    }

    /// Validate distribution index consistency
    fn validate_distribution_index(&self) -> Result<IndexValidation> {
        let mut validation = IndexValidation::new("distribution_index");

        for (distribution, entry) in &self.indices.by_distribution {
            // Check for empty distribution names
            if distribution.is_empty() {
                validation.add_error("Empty distribution name found");
            }

            // Validate report count
            if entry.total_reports == 0 {
                validation.add_error(&format!("Distribution '{}' has zero reports", distribution));
            }

            // Check compatibility scores
            for (vendor, score) in &entry.vendor_compatibility {
                if *score < 0.0 || *score > 100.0 {
                    validation.add_error(&format!(
                        "Invalid compatibility score {} for vendor '{}' in distribution '{}'",
                        score, vendor, distribution
                    ));
                }
            }

            // Check kernel list
            if entry.common_kernels.is_empty() {
                validation.add_warning(&format!("Distribution '{}' has no common kernels", distribution));
            }
        }

        validation.item_count = self.indices.by_distribution.len();
        Ok(validation)
    }

    /// Validate search terms index
    fn validate_search_index(&self) -> Result<IndexValidation> {
        let mut validation = IndexValidation::new("search_index");
        let mut total_terms = 0;
        let mut total_mappings = 0;

        for (term, report_ids) in &self.indices.search_terms {
            total_terms += 1;
            total_mappings += report_ids.len();

            // Check for empty terms
            if term.is_empty() {
                validation.add_error("Empty search term found");
            }

            // Check for very short terms (might be noise)
            if term.len() == 1 {
                validation.add_warning(&format!("Very short search term: '{}'", term));
            }

            // Check for empty report lists
            if report_ids.is_empty() {
                validation.add_error(&format!("Search term '{}' has empty report list", term));
            }

            // Check for duplicate report IDs
            let mut sorted_ids = report_ids.clone();
            sorted_ids.sort();
            sorted_ids.dedup();
            if sorted_ids.len() != report_ids.len() {
                validation.add_warning(&format!("Search term '{}' has duplicate report IDs", term));
            }
        }

        validation.item_count = total_terms;
        validation.add_info(&format!("Total search term mappings: {}", total_mappings));

        Ok(validation)
    }

    /// Validate compatibility matrix
    fn validate_compatibility_matrix(&self) -> Result<IndexValidation> {
        let mut validation = IndexValidation::new("compatibility_matrix");
        let mut total_combinations = 0;

        for (hardware, kernel_map) in &self.indices.compatibility_matrix {
            if hardware.is_empty() {
                validation.add_error("Empty hardware identifier found in compatibility matrix");
            }

            for (kernel_combo, score) in kernel_map {
                total_combinations += 1;

                if kernel_combo.is_empty() {
                    validation.add_error("Empty kernel combination found");
                }

                // Validate score range
                if score.score > 100 {
                    validation.add_error(&format!(
                        "Invalid compatibility score {} for '{}'->'{}'",
                        score.score, hardware, kernel_combo
                    ));
                }

                // Validate sample size
                if score.sample_size == 0 {
                    validation.add_error(&format!(
                        "Zero sample size for '{}'->'{}'",
                        hardware, kernel_combo
                    ));
                }

                // Check confidence level consistency
                let expected_confidence = match score.sample_size {
                    1..=2 => ConfidenceLevel::Low,
                    3..=9 => ConfidenceLevel::Medium,
                    _ => ConfidenceLevel::High,
                };

                if !matches!(
                    (&score.confidence, &expected_confidence),
                    (ConfidenceLevel::Low, ConfidenceLevel::Low) |
                    (ConfidenceLevel::Medium, ConfidenceLevel::Medium) |
                    (ConfidenceLevel::High, ConfidenceLevel::High)
                ) {
                    validation.add_warning(&format!(
                        "Confidence level mismatch for '{}'->'{}'",
                        hardware, kernel_combo
                    ));
                }
            }
        }

        validation.item_count = total_combinations;
        Ok(validation)
    }

    /// Validate statistics consistency
    fn validate_statistics(&self) -> Result<IndexValidation> {
        let mut validation = IndexValidation::new("statistics");

        let stats = &self.indices.statistics;

        // Basic sanity checks
        if stats.total_reports == 0 {
            validation.add_error("Statistics show zero total reports");
        }

        if stats.unique_systems == 0 {
            validation.add_error("Statistics show zero unique systems");
        }

        if stats.unique_systems > stats.total_reports {
            validation.add_error("More unique systems than total reports");
        }

        if stats.total_vendors == 0 {
            validation.add_error("Statistics show zero vendors");
        }

        // Cross-reference with other indices
        if stats.total_vendors != self.indices.by_vendor.len() {
            validation.add_warning(&format!(
                "Vendor count mismatch: stats={}, index={}",
                stats.total_vendors, self.indices.by_vendor.len()
            ));
        }

        if stats.component_types != self.indices.by_component.len() {
            validation.add_warning(&format!(
                "Component type count mismatch: stats={}, index={}",
                stats.component_types, self.indices.by_component.len()
            ));
        }

        // Validate compatibility overview
        let total_compat: usize = stats.compatibility_overview.values().sum();
        if total_compat != stats.total_reports {
            validation.add_error(&format!(
                "Compatibility overview sum ({}) doesn't match total reports ({})",
                total_compat, stats.total_reports
            ));
        }

        // Validate top hardware list
        for (i, hardware) in stats.top_hardware.iter().enumerate() {
            if hardware.report_count == 0 {
                validation.add_error(&format!("Top hardware #{} has zero reports", i + 1));
            }
            
            if hardware.avg_compatibility < 0.0 || hardware.avg_compatibility > 100.0 {
                validation.add_error(&format!(
                    "Invalid compatibility score {} for top hardware '{}'",
                    hardware.avg_compatibility, hardware.model
                ));
            }
        }

        validation.item_count = 1; // Single statistics object
        Ok(validation)
    }

    /// Check if kernel version has valid format
    fn is_valid_kernel_version(&self, version: &str) -> bool {
        // Basic kernel version pattern: X.Y.Z or X.Y.Z-suffix
        let parts: Vec<&str> = version.split('.').collect();
        parts.len() >= 2 && parts.iter().all(|part| {
            part.chars().take_while(|c| c.is_ascii_digit()).count() > 0
        })
    }
}

/// Complete validation report for all indices
#[derive(Debug, Default)]
pub struct ValidationReport {
    pub overall_valid: bool,
    pub vendor_validation: IndexValidation,
    pub component_validation: IndexValidation,
    pub kernel_validation: IndexValidation,
    pub distribution_validation: IndexValidation,
    pub search_validation: IndexValidation,
    pub matrix_validation: IndexValidation,
    pub statistics_validation: IndexValidation,
}

impl ValidationReport {
    /// Check if all individual validations passed
    pub fn all_validations_passed(&self) -> bool {
        self.vendor_validation.is_valid() &&
        self.component_validation.is_valid() &&
        self.kernel_validation.is_valid() &&
        self.distribution_validation.is_valid() &&
        self.search_validation.is_valid() &&
        self.matrix_validation.is_valid() &&
        self.statistics_validation.is_valid()
    }

    /// Get summary of all validation issues
    pub fn get_summary(&self) -> ValidationSummary {
        let mut summary = ValidationSummary::default();
        
        let validations = vec![
            &self.vendor_validation,
            &self.component_validation,
            &self.kernel_validation,
            &self.distribution_validation,
            &self.search_validation,
            &self.matrix_validation,
            &self.statistics_validation,
        ];

        for validation in validations {
            summary.total_errors += validation.errors.len();
            summary.total_warnings += validation.warnings.len();
            summary.total_items += validation.item_count;
        }

        summary.validation_passed = self.overall_valid;
        summary
    }

    /// Print validation report to console
    pub fn print_report(&self) {
        println!("\n🔍 Index Validation Report");
        println!("========================");

        let validations = vec![
            ("Vendor Index", &self.vendor_validation),
            ("Component Index", &self.component_validation),
            ("Kernel Index", &self.kernel_validation),
            ("Distribution Index", &self.distribution_validation),
            ("Search Index", &self.search_validation),
            ("Compatibility Matrix", &self.matrix_validation),
            ("Statistics", &self.statistics_validation),
        ];

        for (name, validation) in validations {
            let status = if validation.is_valid() { "✅" } else { "❌" };
            println!("{} {}: {} items", status, name, validation.item_count);
            
            if !validation.errors.is_empty() {
                for error in &validation.errors {
                    println!("   ❌ {}", error);
                }
            }
            
            if !validation.warnings.is_empty() {
                for warning in &validation.warnings {
                    println!("   ⚠️  {}", warning);
                }
            }

            if !validation.info.is_empty() {
                for info in &validation.info {
                    println!("   ℹ️  {}", info);
                }
            }
        }

        let summary = self.get_summary();
        println!("\nSummary:");
        println!("  Total Items: {}", summary.total_items);
        println!("  Errors: {}", summary.total_errors);
        println!("  Warnings: {}", summary.total_warnings);
        println!("  Overall Status: {}", if self.overall_valid { "✅ PASSED" } else { "❌ FAILED" });
    }
}

/// Validation results for a single index type
#[derive(Debug, Default)]
pub struct IndexValidation {
    pub index_name: String,
    pub item_count: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub info: Vec<String>,
}

impl IndexValidation {
    pub fn new(name: &str) -> Self {
        Self {
            index_name: name.to_string(),
            item_count: 0,
            errors: Vec::new(),
            warnings: Vec::new(),
            info: Vec::new(),
        }
    }

    pub fn add_error(&mut self, message: &str) {
        self.errors.push(message.to_string());
    }

    pub fn add_warning(&mut self, message: &str) {
        self.warnings.push(message.to_string());
    }

    pub fn add_info(&mut self, message: &str) {
        self.info.push(message.to_string());
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
}

/// Summary statistics for validation report
#[derive(Debug, Default)]
pub struct ValidationSummary {
    pub validation_passed: bool,
    pub total_items: usize,
    pub total_errors: usize,
    pub total_warnings: usize,
}