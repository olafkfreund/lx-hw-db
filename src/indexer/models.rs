//! Data models and utilities for hardware indexing

use super::*;

/// Additional model implementations and utilities for indexing
impl CompatibilityStatus {
    /// Convert to numerical score for calculations
    pub fn to_score(&self) -> u8 {
        match self {
            CompatibilityStatus::Excellent => 100,
            CompatibilityStatus::Good => 85,
            CompatibilityStatus::Fair => 65,
            CompatibilityStatus::Poor => 35,
            CompatibilityStatus::Unknown => 50,
        }
    }

    /// Convert from numerical score
    pub fn from_score(score: u8) -> Self {
        match score {
            90..=100 => CompatibilityStatus::Excellent,
            75..=89 => CompatibilityStatus::Good,
            50..=74 => CompatibilityStatus::Fair,
            1..=49 => CompatibilityStatus::Poor,
            _ => CompatibilityStatus::Unknown,
        }
    }

    /// Get human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            CompatibilityStatus::Excellent => "All hardware working perfectly",
            CompatibilityStatus::Good => "Most hardware working with minor issues",
            CompatibilityStatus::Fair => "Some hardware working with moderate issues",
            CompatibilityStatus::Poor => "Major compatibility problems",
            CompatibilityStatus::Unknown => "Compatibility status unknown",
        }
    }
}

impl ConfidenceLevel {
    /// Convert to numerical confidence percentage
    pub fn to_percentage(&self) -> u8 {
        match self {
            ConfidenceLevel::High => 90,
            ConfidenceLevel::Medium => 70,
            ConfidenceLevel::Low => 40,
        }
    }

    /// Get human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            ConfidenceLevel::High => "High confidence (10+ reports)",
            ConfidenceLevel::Medium => "Medium confidence (3-9 reports)",
            ConfidenceLevel::Low => "Low confidence (1-2 reports)",
        }
    }
}

impl HardwareComponent {
    /// Get a unique identifier for this component
    pub fn get_identifier(&self) -> String {
        match (&self.vendor, &self.model) {
            (Some(vendor), Some(model)) => format!(
                "{}_{}",
                vendor.to_lowercase().replace(' ', "_"),
                model.to_lowercase().replace(' ', "_")
            ),
            (Some(vendor), None) => format!("{}_unknown", vendor.to_lowercase().replace(' ', "_")),
            (None, Some(model)) => format!("unknown_{}", model.to_lowercase().replace(' ', "_")),
            (None, None) => {
                format!("unknown_{}", self.component_type.to_lowercase().replace(' ', "_"))
            }
        }
    }

    /// Check if this component has driver information
    pub fn has_driver_info(&self) -> bool {
        self.driver.is_some() || self.driver_version.is_some()
    }

    /// Get display name for this component
    pub fn display_name(&self) -> String {
        match (&self.vendor, &self.model) {
            (Some(vendor), Some(model)) => format!("{} {}", vendor, model),
            (Some(vendor), None) => vendor.clone(),
            (None, Some(model)) => model.clone(),
            (None, None) => self.component_type.clone(),
        }
    }
}

impl PopularModel {
    /// Get recommendation level based on compatibility score
    pub fn recommendation_level(&self) -> &'static str {
        match self.avg_compatibility as u8 {
            90..=100 => "Highly Recommended",
            80..=89 => "Recommended",
            70..=79 => "Good Choice",
            60..=69 => "Acceptable",
            _ => "Consider Alternatives",
        }
    }

    /// Get confidence level based on report count
    pub fn confidence_level(&self) -> ConfidenceLevel {
        match self.report_count {
            1..=2 => ConfidenceLevel::Low,
            3..=9 => ConfidenceLevel::Medium,
            _ => ConfidenceLevel::High,
        }
    }
}

impl VendorEntry {
    /// Get total number of unique models for this vendor
    pub fn total_models(&self) -> usize {
        self.components.values().map(|models| models.len()).sum()
    }

    /// Check if vendor has components of a specific type
    pub fn has_component_type(&self, component_type: &str) -> bool {
        self.components.contains_key(component_type)
    }

    /// Get recommendation category based on compatibility score
    pub fn recommendation_category(&self) -> &'static str {
        match self.compatibility_score as u8 {
            95..=100 => "Excellent",
            85..=94 => "Very Good",
            75..=84 => "Good",
            65..=74 => "Fair",
            _ => "Limited",
        }
    }
}

impl ComponentEntry {
    /// Get the most popular vendor for this component type
    pub fn most_popular_vendor(&self) -> Option<(&String, &usize)> {
        self.vendors.iter().max_by_key(|(_, count)| *count)
    }

    /// Get compatibility success rate (excellent + good / total)
    pub fn compatibility_success_rate(&self) -> f64 {
        let total: usize = self.compatibility_distribution.values().sum();
        if total == 0 {
            return 0.0;
        }

        let successful =
            self.compatibility_distribution.get(&CompatibilityStatus::Excellent).unwrap_or(&0)
                + self.compatibility_distribution.get(&CompatibilityStatus::Good).unwrap_or(&0);

        (successful as f64 / total as f64) * 100.0
    }
}

impl KernelEntry {
    /// Get compatibility success rate for this kernel
    pub fn compatibility_success_rate(&self) -> f64 {
        if self.total_reports == 0 {
            return 0.0;
        }

        let excellent = self.compatibility_stats.get("excellent").unwrap_or(&0);
        let good = self.compatibility_stats.get("good").unwrap_or(&0);
        let successful = excellent + good;

        (successful as f64 / self.total_reports as f64) * 100.0
    }

    /// Check if this is a long-term support kernel
    pub fn is_lts(&self) -> bool {
        // This would be determined by external kernel database
        // For now, use simple heuristics
        false // Placeholder
    }
}

impl DistributionEntry {
    /// Get the most compatible vendor for this distribution
    pub fn best_vendor(&self) -> Option<(&String, &f64)> {
        self.vendor_compatibility.iter().max_by(|(_, score_a), (_, score_b)| {
            score_a.partial_cmp(score_b).unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Get average compatibility score across all vendors
    pub fn average_compatibility(&self) -> f64 {
        if self.vendor_compatibility.is_empty() {
            return 0.0;
        }

        let total: f64 = self.vendor_compatibility.values().sum();
        total / self.vendor_compatibility.len() as f64
    }
}

impl Statistics {
    /// Calculate database health score (0-100)
    pub fn health_score(&self) -> u8 {
        if self.total_reports == 0 {
            return 0;
        }

        // Calculate based on various factors
        let mut score = 0.0;

        // Report volume (max 30 points)
        let volume_score = match self.total_reports {
            0..=100 => self.total_reports as f64 * 0.3,
            101..=1000 => 30.0 + (self.total_reports - 100) as f64 * 0.02,
            _ => 50.0,
        };
        score += volume_score.min(30.0);

        // Diversity (max 25 points)
        let diversity_score = (self.total_vendors as f64).min(50.0) * 0.5;
        score += diversity_score.min(25.0);

        // Compatibility distribution (max 25 points)
        let total_compat: usize = self.compatibility_overview.values().sum();
        if total_compat > 0 {
            let excellent =
                self.compatibility_overview.get(&CompatibilityStatus::Excellent).unwrap_or(&0);
            let good = self.compatibility_overview.get(&CompatibilityStatus::Good).unwrap_or(&0);
            let compat_rate = (*excellent + *good) as f64 / total_compat as f64;
            score += compat_rate * 25.0;
        }

        // Recency (max 20 points) - would use actual date analysis
        score += 15.0; // Placeholder

        score as u8
    }

    /// Get the most reported component type
    pub fn most_reported_component_type(&self) -> Option<String> {
        // This would be calculated from component statistics
        // Placeholder for now
        None
    }

    /// Get database growth trend
    pub fn growth_trend(&self) -> GrowthTrend {
        if self.growth_stats.len() < 2 {
            return GrowthTrend::Stable;
        }

        // Simple trend analysis
        let recent = &self.growth_stats[self.growth_stats.len() - 1];
        let previous = &self.growth_stats[self.growth_stats.len() - 2];

        let growth_rate = if previous.total_reports > 0 {
            ((recent.total_reports as f64 - previous.total_reports as f64)
                / previous.total_reports as f64)
                * 100.0
        } else {
            0.0
        };

        match growth_rate {
            r if r > 10.0 => GrowthTrend::FastGrowth,
            r if r > 2.0 => GrowthTrend::Moderate,
            r if r > -2.0 => GrowthTrend::Stable,
            _ => GrowthTrend::Declining,
        }
    }
}

/// Database growth trend categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GrowthTrend {
    FastGrowth,
    Moderate,
    Stable,
    Declining,
}

impl GrowthTrend {
    pub fn description(&self) -> &'static str {
        match self {
            GrowthTrend::FastGrowth => "Rapidly growing database",
            GrowthTrend::Moderate => "Steady growth",
            GrowthTrend::Stable => "Stable submission rate",
            GrowthTrend::Declining => "Declining submissions",
        }
    }
}

impl GrowthDataPoint {
    /// Calculate growth rate compared to previous data point
    pub fn growth_rate(&self, previous: &GrowthDataPoint) -> f64 {
        if previous.total_reports == 0 {
            return 0.0;
        }

        ((self.total_reports as f64 - previous.total_reports as f64)
            / previous.total_reports as f64)
            * 100.0
    }
}

/// Utility functions for data processing
/// Normalize hardware vendor names to canonical forms
pub fn normalize_vendor_name(vendor: &str) -> String {
    match vendor.to_lowercase().as_str() {
        name if name.contains("advanced micro devices") || name.contains("amd") => {
            "AMD".to_string()
        }
        name if name.contains("intel") => "Intel".to_string(),
        name if name.contains("nvidia") => "NVIDIA".to_string(),
        name if name.contains("realtek") => "Realtek".to_string(),
        name if name.contains("broadcom") => "Broadcom".to_string(),
        name if name.contains("qualcomm") => "Qualcomm".to_string(),
        name if name.contains("microsoft") => "Microsoft".to_string(),
        name if name.contains("apple") => "Apple".to_string(),
        _ => vendor.to_string(), // Keep original if no match
    }
}

/// Extract search keywords from text
pub fn extract_keywords(text: &str) -> Vec<String> {
    text.split(&[' ', '-', '_', '.', '(', ')', '[', ']', ',', ';'])
        .map(|word| word.trim().to_lowercase())
        .filter(|word| word.len() >= 2 && !word.chars().all(char::is_numeric))
        .filter(|word| !is_stop_word(word))
        .collect()
}

/// Check if word is a common stop word that shouldn't be indexed
fn is_stop_word(word: &str) -> bool {
    const STOP_WORDS: &[&str] = &[
        "the",
        "and",
        "or",
        "but",
        "in",
        "on",
        "at",
        "to",
        "for",
        "of",
        "with",
        "by",
        "inc",
        "ltd",
        "corp",
        "corporation",
        "company",
        "co",
        "llc",
    ];

    STOP_WORDS.contains(&word)
}

/// Calculate similarity between two strings (simple implementation)
pub fn string_similarity(a: &str, b: &str) -> f64 {
    if a == b {
        return 1.0;
    }

    let a_lower = a.to_lowercase();
    let b_lower = b.to_lowercase();

    if a_lower == b_lower {
        return 0.95;
    }

    // Simple substring matching
    if a_lower.contains(&b_lower) || b_lower.contains(&a_lower) {
        return 0.8;
    }

    // Levenshtein distance would be better here
    0.0
}

/// Truncate text to specified length with ellipsis
pub fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len.saturating_sub(3)])
    }
}
