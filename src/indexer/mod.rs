//! Hardware compatibility indexer for GitHub-native database
//!
//! This module processes hardware report JSON files and generates
//! search indices, compatibility matrices, and statistics for
//! client-side search and analysis.

pub mod analysis;
pub mod builder;
pub mod compatibility;
pub mod models;
pub mod search_index;
pub mod statistics;

use crate::errors::{LxHwError, Result};
use crate::hardware::HardwareReport;
use chrono::{DateTime, Utc};
use glob::glob;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// Re-export utility functions from models
pub use models::normalize_vendor_name;

/// Main hardware indexer that processes reports and builds indices
pub struct HardwareIndexer {
    /// All loaded hardware reports
    pub reports: Vec<IndexedReport>,
    /// Generated search indices
    pub indices: IndexCollection,
    /// Configuration for index generation
    config: IndexerConfig,
}

/// Configuration for the indexer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexerConfig {
    /// Input directory containing hardware reports
    pub reports_dir: PathBuf,
    /// Output directory for indices
    pub indices_dir: PathBuf,
    /// Output directory for API endpoints
    pub api_dir: PathBuf,
    /// Output directory for statistics
    pub stats_dir: PathBuf,
    /// Minimum number of reports for hardware to be included
    pub min_reports: usize,
    /// Enable verbose logging
    pub verbose: bool,
}

/// Hardware report with extracted metadata for indexing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedReport {
    /// Original report ID (filename without extension)
    pub id: String,
    /// File path relative to reports directory
    pub file_path: PathBuf,
    /// Extracted metadata from report
    pub metadata: ReportMetadata,
    /// Hardware components extracted from report
    pub components: Vec<HardwareComponent>,
    /// Compatibility information
    pub compatibility: CompatibilityInfo,
    /// When this report was processed
    pub indexed_at: DateTime<Utc>,
}

/// Metadata extracted from hardware report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMetadata {
    /// Anonymized system identifier
    pub system_id: String,
    /// Date when report was submitted
    pub submission_date: DateTime<Utc>,
    /// Kernel version
    pub kernel_version: String,
    /// Linux distribution
    pub distribution: String,
    /// System architecture
    pub architecture: String,
    /// Privacy level used
    pub privacy_level: String,
}

/// Hardware component extracted from report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareComponent {
    /// Component type (CPU, GPU, Storage, Network, etc.)
    pub component_type: String,
    /// Hardware vendor
    pub vendor: Option<String>,
    /// Product model
    pub model: Option<String>,
    /// Device ID (PCI ID, USB ID, etc.)
    pub device_id: Option<String>,
    /// Driver in use
    pub driver: Option<String>,
    /// Driver version
    pub driver_version: Option<String>,
    /// Additional properties specific to component type
    pub properties: HashMap<String, serde_json::Value>,
}

/// Compatibility status for a hardware report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityInfo {
    /// Overall compatibility status
    pub status: CompatibilityStatus,
    /// Individual component compatibility
    pub components: HashMap<String, ComponentCompatibility>,
    /// Known issues
    pub issues: Vec<String>,
    /// Available workarounds
    pub workarounds: Vec<String>,
    /// Compatibility confidence (0-100)
    pub confidence: u8,
}

/// Compatibility status levels
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum CompatibilityStatus {
    /// All hardware working perfectly
    Excellent,
    /// Most hardware working, minor issues
    Good,
    /// Some hardware working, moderate issues
    Fair,
    /// Major hardware compatibility problems
    Poor,
    /// Compatibility status unknown/untested
    Unknown,
}

/// Component-specific compatibility information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentCompatibility {
    /// Working status
    pub working: bool,
    /// Performance level (if applicable)
    pub performance: Option<String>,
    /// Specific issues with this component
    pub issues: Vec<String>,
    /// Workarounds for this component
    pub workarounds: Vec<String>,
}

/// Collection of all generated indices
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndexCollection {
    /// Index by hardware vendor
    pub by_vendor: VendorIndex,
    /// Index by component type
    pub by_component: ComponentIndex,
    /// Index by kernel version
    pub by_kernel: KernelIndex,
    /// Index by Linux distribution
    pub by_distribution: DistributionIndex,
    /// Full-text search terms
    pub search_terms: SearchTermsIndex,
    /// Hardware compatibility matrix
    pub compatibility_matrix: CompatibilityMatrix,
    /// Aggregated statistics
    pub statistics: Statistics,
}

/// Index organized by hardware vendor
pub type VendorIndex = HashMap<String, VendorEntry>;

/// Entry for a specific vendor in the index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorEntry {
    /// Total number of reports for this vendor
    pub total_reports: usize,
    /// Hardware components by type
    pub components: HashMap<String, Vec<String>>,
    /// Recent report IDs
    pub recent_reports: Vec<String>,
    /// Average compatibility score
    pub compatibility_score: f64,
    /// Last update timestamp
    pub last_updated: DateTime<Utc>,
}

/// Index organized by component type
pub type ComponentIndex = HashMap<String, ComponentEntry>;

/// Entry for a specific component type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentEntry {
    /// Total reports for this component type
    pub total_reports: usize,
    /// Vendors for this component type
    pub vendors: HashMap<String, usize>,
    /// Most popular models
    pub popular_models: Vec<PopularModel>,
    /// Compatibility distribution
    pub compatibility_distribution: HashMap<CompatibilityStatus, usize>,
}

/// Popular hardware model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopularModel {
    /// Vendor name
    pub vendor: String,
    /// Model name
    pub model: String,
    /// Number of reports
    pub report_count: usize,
    /// Average compatibility score
    pub avg_compatibility: f64,
}

/// Index organized by kernel version
pub type KernelIndex = HashMap<String, KernelEntry>;

/// Entry for a specific kernel version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelEntry {
    /// Total reports for this kernel
    pub total_reports: usize,
    /// Hardware compatibility stats
    pub compatibility_stats: HashMap<String, usize>,
    /// Most problematic hardware
    pub problematic_hardware: Vec<String>,
    /// Release date (if known)
    pub release_date: Option<DateTime<Utc>>,
}

/// Index organized by Linux distribution
pub type DistributionIndex = HashMap<String, DistributionEntry>;

/// Entry for a specific Linux distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionEntry {
    /// Total reports for this distribution
    pub total_reports: usize,
    /// Hardware compatibility by vendor
    pub vendor_compatibility: HashMap<String, f64>,
    /// Common kernel versions
    pub common_kernels: Vec<String>,
    /// Distribution-specific notes
    pub notes: Vec<String>,
}

/// Full-text search index
pub type SearchTermsIndex = HashMap<String, Vec<String>>;

/// Hardware compatibility scoring matrix
pub type CompatibilityMatrix = HashMap<String, HashMap<String, CompatibilityScore>>;

/// Compatibility score for hardware/kernel combination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityScore {
    /// Numerical compatibility score (0-100)
    pub score: u8,
    /// Driver information
    pub driver: Option<String>,
    /// Number of reports contributing to this score
    pub sample_size: usize,
    /// Confidence in this score
    pub confidence: ConfidenceLevel,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

/// Confidence level in compatibility scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfidenceLevel {
    High,   // 10+ reports
    Medium, // 3-9 reports
    Low,    // 1-2 reports
}

/// Aggregated database statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Statistics {
    /// Total number of hardware reports
    pub total_reports: usize,
    /// Total unique systems
    pub unique_systems: usize,
    /// Hardware vendors count
    pub total_vendors: usize,
    /// Component types count
    pub component_types: usize,
    /// Kernel versions count
    pub kernel_versions: usize,
    /// Linux distributions count
    pub distributions: usize,
    /// Overall compatibility distribution
    pub compatibility_overview: HashMap<CompatibilityStatus, usize>,
    /// Top hardware by report count
    pub top_hardware: Vec<PopularModel>,
    /// Database growth over time
    pub growth_stats: Vec<GrowthDataPoint>,
    /// Last index update
    pub last_updated: DateTime<Utc>,
}

/// Data point for growth statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthDataPoint {
    /// Date
    pub date: DateTime<Utc>,
    /// Total reports at this date
    pub total_reports: usize,
    /// New reports added
    pub new_reports: usize,
}

impl HardwareIndexer {
    /// Create new indexer with configuration
    pub fn new(config: IndexerConfig) -> Self {
        Self { reports: Vec::new(), indices: IndexCollection::default(), config }
    }

    /// Scan and load all hardware reports from directory
    pub fn scan_reports(&mut self) -> Result<()> {
        let pattern = format!("{}/**/*.json", self.config.reports_dir.display());
        let files: Vec<PathBuf> = glob(&pattern)
            .map_err(|e| LxHwError::ConfigError(format!("Invalid glob pattern: {}", e)))?
            .filter_map(|entry| entry.ok())
            .collect();

        if self.config.verbose {
            println!("Found {} hardware report files", files.len());
        }

        let mut loaded = 0;
        let mut errors = 0;

        for file_path in files {
            match self.load_report(&file_path) {
                Ok(report) => {
                    self.reports.push(report);
                    loaded += 1;
                }
                Err(e) => {
                    errors += 1;
                    eprintln!("Error loading {}: {}", file_path.display(), e);
                }
            }
        }

        if self.config.verbose {
            println!("Loaded {} reports successfully, {} errors", loaded, errors);
        }

        Ok(())
    }

    /// Load and parse a single hardware report
    fn load_report(&self, file_path: &Path) -> Result<IndexedReport> {
        let content = std::fs::read_to_string(file_path).map_err(LxHwError::IoError)?;

        let report: HardwareReport = serde_json::from_str(&content)
            .map_err(|e| LxHwError::SerializationError(format!("Failed to parse JSON: {}", e)))?;

        // Extract report ID from filename
        let id = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| LxHwError::ConfigError("Invalid filename".to_string()))?
            .to_string();

        // Extract relative path
        let relative_path =
            file_path.strip_prefix(&self.config.reports_dir).unwrap_or(file_path).to_path_buf();

        // Extract metadata and components
        let metadata = self.extract_metadata(&report)?;
        let components = self.extract_components(&report)?;
        let compatibility = self.analyze_compatibility(&report, &components)?;

        Ok(IndexedReport {
            id,
            file_path: relative_path,
            metadata,
            components,
            compatibility,
            indexed_at: Utc::now(),
        })
    }

    /// Extract metadata from hardware report
    fn extract_metadata(&self, _report: &HardwareReport) -> Result<ReportMetadata> {
        // This would extract metadata from the actual report structure
        // For now, return a placeholder that matches expected fields
        Ok(ReportMetadata {
            system_id: "placeholder".to_string(),
            submission_date: Utc::now(),
            kernel_version: "6.16.0".to_string(),
            distribution: "Unknown".to_string(),
            architecture: "x86_64".to_string(),
            privacy_level: "Enhanced".to_string(),
        })
    }

    /// Extract hardware components from report
    fn extract_components(&self, _report: &HardwareReport) -> Result<Vec<HardwareComponent>> {
        // This would parse the actual hardware data from the report
        // For now, return a placeholder
        Ok(Vec::new())
    }

    /// Analyze compatibility from report and components
    fn analyze_compatibility(
        &self,
        _report: &HardwareReport,
        _components: &[HardwareComponent],
    ) -> Result<CompatibilityInfo> {
        // This would analyze the hardware and determine compatibility
        // For now, return a placeholder
        Ok(CompatibilityInfo {
            status: CompatibilityStatus::Unknown,
            components: HashMap::new(),
            issues: Vec::new(),
            workarounds: Vec::new(),
            confidence: 50,
        })
    }

    /// Build all indices from loaded reports
    pub fn build_indices(&mut self) -> Result<()> {
        if self.config.verbose {
            println!("Building indices from {} reports...", self.reports.len());
        }

        let builder = builder::IndexBuilder::new(&self.config);
        self.indices = builder.build_indices(&self.reports)?;

        if self.config.verbose {
            println!("Index generation completed");
        }

        Ok(())
    }

    /// Write all indices to disk
    pub fn write_indices(&self) -> Result<()> {
        self.write_index_files()?;
        self.write_api_endpoints()?;
        self.write_statistics()?;

        if self.config.verbose {
            println!("All indices written successfully");
        }

        Ok(())
    }

    /// Write index JSON files
    fn write_index_files(&self) -> Result<()> {
        std::fs::create_dir_all(&self.config.indices_dir)?;

        // Write each index type
        self.write_json_file(
            &self.config.indices_dir.join("by-vendor.json"),
            &self.indices.by_vendor,
        )?;

        self.write_json_file(
            &self.config.indices_dir.join("by-component.json"),
            &self.indices.by_component,
        )?;

        self.write_json_file(
            &self.config.indices_dir.join("by-kernel.json"),
            &self.indices.by_kernel,
        )?;

        self.write_json_file(
            &self.config.indices_dir.join("by-distribution.json"),
            &self.indices.by_distribution,
        )?;

        self.write_json_file(
            &self.config.indices_dir.join("search-terms.json"),
            &self.indices.search_terms,
        )?;

        self.write_json_file(
            &self.config.indices_dir.join("compatibility-matrix.json"),
            &self.indices.compatibility_matrix,
        )?;

        Ok(())
    }

    /// Write API endpoint files
    fn write_api_endpoints(&self) -> Result<()> {
        std::fs::create_dir_all(&self.config.api_dir)?;

        // Create API structure similar to REST endpoints
        let api_builder = search_index::ApiBuilder::new(&self.indices);
        api_builder.write_api_files(&self.config.api_dir)?;

        Ok(())
    }

    /// Write statistics files
    fn write_statistics(&self) -> Result<()> {
        std::fs::create_dir_all(&self.config.stats_dir)?;

        self.write_json_file(
            &self.config.stats_dir.join("overview.json"),
            &self.indices.statistics,
        )?;

        Ok(())
    }

    /// Helper to write JSON file with pretty formatting
    fn write_json_file<T: Serialize>(&self, path: &Path, data: &T) -> Result<()> {
        let json = serde_json::to_string_pretty(data)
            .map_err(|e| LxHwError::SerializationError(e.to_string()))?;

        std::fs::write(path, json)?;

        if self.config.verbose {
            println!("Wrote {}", path.display());
        }

        Ok(())
    }

    /// Validate generated indices
    pub fn validate_indices(&self) -> Result<()> {
        // Validate index consistency and completeness
        let validator = compatibility::IndexValidator::new(&self.indices);
        let validation_report = validator.validate()?;

        if self.config.verbose {
            validation_report.print_report();
            println!("Index validation completed successfully");
        }

        Ok(())
    }
}

impl Default for IndexerConfig {
    fn default() -> Self {
        Self {
            reports_dir: PathBuf::from("hardware-reports"),
            indices_dir: PathBuf::from("indices"),
            api_dir: PathBuf::from("api"),
            stats_dir: PathBuf::from("statistics"),
            min_reports: 1,
            verbose: false,
        }
    }
}
