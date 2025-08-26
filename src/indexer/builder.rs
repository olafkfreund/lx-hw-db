//! Index builder implementation for generating search indices from hardware reports

use super::*;
use crate::errors::Result;
use std::collections::{HashMap, HashSet};
use chrono::Utc;

/// Builder for generating all types of indices from hardware reports
pub struct IndexBuilder<'a> {
    config: &'a IndexerConfig,
    /// Vendor name normalization map
    vendor_aliases: HashMap<String, String>,
}

impl<'a> IndexBuilder<'a> {
    pub fn new(config: &'a IndexerConfig) -> Self {
        Self {
            config,
            vendor_aliases: Self::create_vendor_aliases(),
        }
    }

    /// Build complete index collection from reports
    pub fn build_indices(&self, reports: &[IndexedReport]) -> Result<IndexCollection> {
        if self.config.verbose {
            println!("Building indices from {} reports...", reports.len());
        }

        // Build each index type
        let indices = IndexCollection {
            by_vendor: self.build_vendor_index(reports)?,
            by_component: self.build_component_index(reports)?,
            by_kernel: self.build_kernel_index(reports)?,
            by_distribution: self.build_distribution_index(reports)?,
            search_terms: self.build_search_terms_index(reports)?,
            compatibility_matrix: self.build_compatibility_matrix(reports)?,
            statistics: self.build_statistics(reports)?,
        };

        if self.config.verbose {
            println!("All indices built successfully");
            self.print_index_summary(&indices);
        }

        Ok(indices)
    }

    /// Build vendor-organized index
    fn build_vendor_index(&self, reports: &[IndexedReport]) -> Result<VendorIndex> {
        if self.config.verbose {
            println!("Building vendor index...");
        }

        let mut vendor_index = HashMap::new();

        for report in reports {
            for component in &report.components {
                if let Some(vendor) = &component.vendor {
                    let normalized_vendor = self.normalize_vendor_name(vendor);
                    
                    let entry = vendor_index
                        .entry(normalized_vendor.clone())
                        .or_insert_with(|| VendorEntry {
                            total_reports: 0,
                            components: HashMap::new(),
                            recent_reports: Vec::new(),
                            compatibility_score: 0.0,
                            last_updated: Utc::now(),
                        });

                    entry.total_reports += 1;
                    entry.last_updated = Utc::now();

                    // Add component to vendor's component list
                    if let Some(model) = &component.model {
                        let component_list = entry
                            .components
                            .entry(component.component_type.clone())
                            .or_insert_with(Vec::new);
                        
                        if !component_list.contains(model) {
                            component_list.push(model.clone());
                        }
                    }

                    // Add to recent reports (keep last 10)
                    if !entry.recent_reports.contains(&report.id) {
                        entry.recent_reports.push(report.id.clone());
                        if entry.recent_reports.len() > 10 {
                            entry.recent_reports.remove(0);
                        }
                    }

                    // Update compatibility score
                    entry.compatibility_score = self.calculate_vendor_compatibility_score(
                        &normalized_vendor,
                        reports,
                    );
                }
            }
        }

        // Sort component lists for each vendor
        for entry in vendor_index.values_mut() {
            for component_list in entry.components.values_mut() {
                component_list.sort();
                component_list.dedup();
            }
        }

        if self.config.verbose {
            println!("   Found {} unique vendors", vendor_index.len());
        }

        Ok(vendor_index)
    }

    /// Build component type organized index
    fn build_component_index(&self, reports: &[IndexedReport]) -> Result<ComponentIndex> {
        if self.config.verbose {
            println!("Building component index...");
        }

        let mut component_index = HashMap::new();

        for report in reports {
            for component in &report.components {
                let entry = component_index
                    .entry(component.component_type.clone())
                    .or_insert_with(|| ComponentEntry {
                        total_reports: 0,
                        vendors: HashMap::new(),
                        popular_models: Vec::new(),
                        compatibility_distribution: HashMap::new(),
                    });

                entry.total_reports += 1;

                // Count vendors for this component type
                if let Some(vendor) = &component.vendor {
                    let normalized_vendor = self.normalize_vendor_name(vendor);
                    *entry.vendors.entry(normalized_vendor).or_insert(0) += 1;
                }

                // Update compatibility distribution
                let compat_status = self.get_component_compatibility_status(component, report);
                *entry.compatibility_distribution.entry(compat_status).or_insert(0) += 1;
            }
        }

        // Build popular models for each component type
        for (component_type, entry) in component_index.iter_mut() {
            entry.popular_models = self.build_popular_models_for_component(component_type, reports);
        }

        if self.config.verbose {
            println!("   Found {} component types", component_index.len());
        }

        Ok(component_index)
    }

    /// Build kernel version organized index
    fn build_kernel_index(&self, reports: &[IndexedReport]) -> Result<KernelIndex> {
        if self.config.verbose {
            println!("Building kernel index...");
        }

        let mut kernel_index = HashMap::new();

        for report in reports {
            let kernel_version = &report.metadata.kernel_version;
            
            let entry = kernel_index
                .entry(kernel_version.clone())
                .or_insert_with(|| KernelEntry {
                    total_reports: 0,
                    compatibility_stats: HashMap::new(),
                    problematic_hardware: Vec::new(),
                    release_date: None, // Would be populated from external data
                });

            entry.total_reports += 1;

            // Update compatibility statistics
            let status_str = match report.compatibility.status {
                CompatibilityStatus::Excellent => "excellent",
                CompatibilityStatus::Good => "good", 
                CompatibilityStatus::Fair => "fair",
                CompatibilityStatus::Poor => "poor",
                CompatibilityStatus::Unknown => "unknown",
            };
            *entry.compatibility_stats.entry(status_str.to_string()).or_insert(0) += 1;

            // Track problematic hardware
            if matches!(report.compatibility.status, CompatibilityStatus::Poor | CompatibilityStatus::Fair) {
                for component in &report.components {
                    if let (Some(vendor), Some(model)) = (&component.vendor, &component.model) {
                        let hw_id = format!("{} {}", vendor, model);
                        if !entry.problematic_hardware.contains(&hw_id) {
                            entry.problematic_hardware.push(hw_id);
                        }
                    }
                }
            }
        }

        if self.config.verbose {
            println!("   Found {} kernel versions", kernel_index.len());
        }

        Ok(kernel_index)
    }

    /// Build Linux distribution organized index
    fn build_distribution_index(&self, reports: &[IndexedReport]) -> Result<DistributionIndex> {
        if self.config.verbose {
            println!("Building distribution index...");
        }

        let mut dist_index = HashMap::new();

        for report in reports {
            let distribution = &report.metadata.distribution;
            
            let entry = dist_index
                .entry(distribution.clone())
                .or_insert_with(|| DistributionEntry {
                    total_reports: 0,
                    vendor_compatibility: HashMap::new(),
                    common_kernels: Vec::new(),
                    notes: Vec::new(),
                });

            entry.total_reports += 1;

            // Track kernel versions for this distribution
            let kernel = &report.metadata.kernel_version;
            if !entry.common_kernels.contains(kernel) {
                entry.common_kernels.push(kernel.clone());
            }

            // Update vendor compatibility scores
            for component in &report.components {
                if let Some(vendor) = &component.vendor {
                    let normalized_vendor = self.normalize_vendor_name(vendor);
                    let compat_score = self.component_compatibility_score(component, report);
                    
                    let current_score = entry.vendor_compatibility
                        .get(&normalized_vendor)
                        .unwrap_or(&0.0);
                    let new_score = (current_score + compat_score) / 2.0;
                    entry.vendor_compatibility.insert(normalized_vendor, new_score);
                }
            }
        }

        // Sort kernel lists and keep most common
        for entry in dist_index.values_mut() {
            entry.common_kernels.sort();
            entry.common_kernels.dedup();
            entry.common_kernels.truncate(10); // Keep top 10 most common kernels
        }

        if self.config.verbose {
            println!("   Found {} distributions", dist_index.len());
        }

        Ok(dist_index)
    }

    /// Build full-text search terms index
    fn build_search_terms_index(&self, reports: &[IndexedReport]) -> Result<SearchTermsIndex> {
        if self.config.verbose {
            println!("Building search terms index...");
        }

        let mut search_index = HashMap::new();

        for report in reports {
            let mut terms = HashSet::new();

            // Extract search terms from components
            for component in &report.components {
                // Add vendor terms
                if let Some(vendor) = &component.vendor {
                    self.add_search_terms(&mut terms, vendor);
                }

                // Add model terms
                if let Some(model) = &component.model {
                    self.add_search_terms(&mut terms, model);
                }

                // Add component type
                self.add_search_terms(&mut terms, &component.component_type);

                // Add driver name
                if let Some(driver) = &component.driver {
                    self.add_search_terms(&mut terms, driver);
                }
            }

            // Add distribution terms
            self.add_search_terms(&mut terms, &report.metadata.distribution);

            // Add kernel version terms
            self.add_search_terms(&mut terms, &report.metadata.kernel_version);

            // Add terms to index
            for term in terms {
                search_index
                    .entry(term)
                    .or_insert_with(Vec::new)
                    .push(report.id.clone());
            }
        }

        // Deduplicate and sort report lists
        for report_list in search_index.values_mut() {
            report_list.sort();
            report_list.dedup();
        }

        if self.config.verbose {
            println!("   Built index with {} search terms", search_index.len());
        }

        Ok(search_index)
    }

    /// Build hardware compatibility scoring matrix
    fn build_compatibility_matrix(&self, reports: &[IndexedReport]) -> Result<CompatibilityMatrix> {
        if self.config.verbose {
            println!("Building compatibility matrix...");
        }

        let mut matrix = HashMap::new();

        for report in reports {
            for component in &report.components {
                if let (Some(vendor), Some(model)) = (&component.vendor, &component.model) {
                    let hw_key = format!("{} {}", self.normalize_vendor_name(vendor), model);
                    let kernel_key = format!("{}_{}", 
                        report.metadata.kernel_version,
                        report.metadata.distribution
                    );

                    let hardware_entry = matrix.entry(hw_key).or_insert_with(HashMap::new);
                    
                    let score_entry = hardware_entry.entry(kernel_key).or_insert_with(|| {
                        CompatibilityScore {
                            score: 0,
                            driver: component.driver.clone(),
                            sample_size: 0,
                            confidence: ConfidenceLevel::Low,
                            last_updated: Utc::now(),
                        }
                    });

                    // Update score based on compatibility
                    let component_score = self.component_compatibility_score(component, report) as u8;
                    let total_samples = score_entry.sample_size + 1;
                    
                    // Calculate weighted average
                    score_entry.score = (((score_entry.score as usize * score_entry.sample_size) + component_score as usize) / total_samples) as u8;
                    score_entry.sample_size = total_samples;
                    score_entry.last_updated = Utc::now();
                    
                    // Update confidence level
                    score_entry.confidence = match total_samples {
                        1..=2 => ConfidenceLevel::Low,
                        3..=9 => ConfidenceLevel::Medium,
                        _ => ConfidenceLevel::High,
                    };

                    // Update driver if more recent
                    if component.driver.is_some() {
                        score_entry.driver = component.driver.clone();
                    }
                }
            }
        }

        if self.config.verbose {
            let total_combinations = matrix.values().map(|v| v.len()).sum::<usize>();
            println!("   Built matrix with {} hardware/kernel combinations", total_combinations);
        }

        Ok(matrix)
    }

    /// Build aggregated statistics
    fn build_statistics(&self, reports: &[IndexedReport]) -> Result<Statistics> {
        if self.config.verbose {
            println!("Building statistics...");
        }

        let mut stats = Statistics {
            total_reports: reports.len(),
            last_updated: Utc::now(),
            ..Default::default()
        };

        // Count unique systems
        let mut unique_systems = HashSet::new();
        let mut all_vendors = HashSet::new();
        let mut all_component_types = HashSet::new();
        let mut all_kernels = HashSet::new();
        let mut all_distributions = HashSet::new();

        for report in reports {
            unique_systems.insert(&report.metadata.system_id);
            all_kernels.insert(&report.metadata.kernel_version);
            all_distributions.insert(&report.metadata.distribution);

            // Count compatibility status
            *stats.compatibility_overview.entry(report.compatibility.status.clone()).or_insert(0) += 1;

            for component in &report.components {
                all_component_types.insert(&component.component_type);
                if let Some(vendor) = &component.vendor {
                    all_vendors.insert(self.normalize_vendor_name(vendor));
                }
            }
        }

        stats.unique_systems = unique_systems.len();
        stats.total_vendors = all_vendors.len();
        stats.component_types = all_component_types.len();
        stats.kernel_versions = all_kernels.len();
        stats.distributions = all_distributions.len();

        // Build top hardware list
        stats.top_hardware = self.build_top_hardware_list(reports);

        // Build growth statistics (simplified)
        stats.growth_stats = self.build_growth_statistics(reports);

        if self.config.verbose {
            println!("   Statistics: {} unique systems, {} vendors, {} distributions",
                stats.unique_systems, stats.total_vendors, stats.distributions);
        }

        Ok(stats)
    }

    /// Helper methods
    /// Create vendor name normalization aliases
    fn create_vendor_aliases() -> HashMap<String, String> {
        let mut aliases = HashMap::new();
        
        // Common vendor name variations
        aliases.insert("Advanced Micro Devices, Inc.".to_string(), "AMD".to_string());
        aliases.insert("Advanced Micro Devices [AMD]".to_string(), "AMD".to_string());
        aliases.insert("Intel Corporation".to_string(), "Intel".to_string());
        aliases.insert("Intel Corp.".to_string(), "Intel".to_string());
        aliases.insert("NVIDIA Corporation".to_string(), "NVIDIA".to_string());
        aliases.insert("NVIDIA Corp.".to_string(), "NVIDIA".to_string());
        aliases.insert("Realtek Semiconductor Co., Ltd.".to_string(), "Realtek".to_string());
        aliases.insert("Broadcom Limited".to_string(), "Broadcom".to_string());
        aliases.insert("Broadcom Inc.".to_string(), "Broadcom".to_string());

        aliases
    }

    /// Normalize vendor name using aliases
    fn normalize_vendor_name(&self, vendor: &str) -> String {
        self.vendor_aliases
            .get(vendor)
            .unwrap_or(&vendor.to_string())
            .clone()
    }

    /// Add search terms from a text string
    fn add_search_terms(&self, terms: &mut HashSet<String>, text: &str) {
        // Split on common delimiters and clean terms
        for word in text.split(&[' ', '-', '_', '.', '(', ')', '[', ']']) {
            let clean_word = word.trim().to_lowercase();
            if clean_word.len() >= 2 && !clean_word.chars().all(char::is_numeric) {
                terms.insert(clean_word);
            }
        }
    }

    /// Calculate compatibility score for a component
    fn component_compatibility_score(&self, _component: &HardwareComponent, report: &IndexedReport) -> f64 {
        match report.compatibility.status {
            CompatibilityStatus::Excellent => 100.0,
            CompatibilityStatus::Good => 85.0,
            CompatibilityStatus::Fair => 65.0,
            CompatibilityStatus::Poor => 35.0,
            CompatibilityStatus::Unknown => 50.0,
        }
    }

    /// Get compatibility status for a component
    fn get_component_compatibility_status(&self, _component: &HardwareComponent, report: &IndexedReport) -> CompatibilityStatus {
        // For now, use overall report status
        // In a real implementation, this would analyze component-specific data
        report.compatibility.status.clone()
    }

    /// Calculate vendor compatibility score
    fn calculate_vendor_compatibility_score(&self, vendor: &str, reports: &[IndexedReport]) -> f64 {
        let mut total_score = 0.0;
        let mut count = 0;

        for report in reports {
            for component in &report.components {
                if let Some(comp_vendor) = &component.vendor {
                    if self.normalize_vendor_name(comp_vendor) == *vendor {
                        total_score += self.component_compatibility_score(component, report);
                        count += 1;
                    }
                }
            }
        }

        if count > 0 {
            total_score / count as f64
        } else {
            0.0
        }
    }

    /// Build popular models for a component type
    fn build_popular_models_for_component(&self, component_type: &str, reports: &[IndexedReport]) -> Vec<PopularModel> {
        let mut model_counts: HashMap<(String, String), usize> = HashMap::new();
        let mut model_scores: HashMap<(String, String), f64> = HashMap::new();

        for report in reports {
            for component in &report.components {
                if component.component_type == component_type {
                    if let (Some(vendor), Some(model)) = (&component.vendor, &component.model) {
                        let key = (self.normalize_vendor_name(vendor), model.clone());
                        *model_counts.entry(key.clone()).or_insert(0) += 1;
                        
                        let current_score = model_scores.get(&key).unwrap_or(&0.0);
                        let component_score = self.component_compatibility_score(component, report);
                        model_scores.insert(key, (current_score + component_score) / 2.0);
                    }
                }
            }
        }

        let mut popular_models: Vec<PopularModel> = model_counts
            .into_iter()
            .filter(|(_, count)| *count >= self.config.min_reports)
            .map(|((vendor, model), count)| PopularModel {
                vendor: vendor.clone(),
                model: model.clone(),
                report_count: count,
                avg_compatibility: *model_scores.get(&(vendor, model)).unwrap_or(&0.0),
            })
            .collect();

        // Sort by report count, then by compatibility score
        popular_models.sort_by(|a, b| {
            b.report_count.cmp(&a.report_count)
                .then_with(|| b.avg_compatibility.partial_cmp(&a.avg_compatibility).unwrap_or(std::cmp::Ordering::Equal))
        });

        popular_models.truncate(20); // Keep top 20
        popular_models
    }

    /// Build top hardware list across all categories
    fn build_top_hardware_list(&self, reports: &[IndexedReport]) -> Vec<PopularModel> {
        let mut all_hardware: HashMap<(String, String), (usize, f64)> = HashMap::new();

        for report in reports {
            for component in &report.components {
                if let (Some(vendor), Some(model)) = (&component.vendor, &component.model) {
                    let key = (self.normalize_vendor_name(vendor), model.clone());
                    let (count, score) = all_hardware.entry(key).or_insert((0, 0.0));
                    *count += 1;
                    *score = (*score + self.component_compatibility_score(component, report)) / 2.0;
                }
            }
        }

        let mut top_hardware: Vec<PopularModel> = all_hardware
            .into_iter()
            .map(|((vendor, model), (count, score))| PopularModel {
                vendor,
                model,
                report_count: count,
                avg_compatibility: score,
            })
            .collect();

        top_hardware.sort_by(|a, b| b.report_count.cmp(&a.report_count));
        top_hardware.truncate(50);
        top_hardware
    }

    /// Build growth statistics over time
    fn build_growth_statistics(&self, reports: &[IndexedReport]) -> Vec<GrowthDataPoint> {
        // Simplified growth stats - in real implementation would analyze dates
        vec![GrowthDataPoint {
            date: Utc::now(),
            total_reports: reports.len(),
            new_reports: reports.len(),
        }]
    }

    /// Print summary of built indices
    fn print_index_summary(&self, indices: &IndexCollection) {
        println!("\nIndex Summary:");
        println!("   Vendors: {}", indices.by_vendor.len());
        println!("   Component Types: {}", indices.by_component.len());
        println!("   Kernel Versions: {}", indices.by_kernel.len());
        println!("   Distributions: {}", indices.by_distribution.len());
        println!("   Search Terms: {}", indices.search_terms.len());
        println!("   Hardware/Kernel Combinations: {}", 
            indices.compatibility_matrix.values().map(|v| v.len()).sum::<usize>());
        println!("   Total Reports: {}", indices.statistics.total_reports);
        println!("   Unique Systems: {}", indices.statistics.unique_systems);
    }
}