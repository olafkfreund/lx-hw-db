//! Statistics generation and analysis for hardware compatibility data

use super::*;
use crate::errors::Result;
use std::collections::{HashMap, BTreeMap};
use chrono::{DateTime, Utc, Duration, Datelike};

/// Advanced statistics generator for hardware compatibility analysis
pub struct StatisticsGenerator<'a> {
    reports: &'a [IndexedReport],
}

impl<'a> StatisticsGenerator<'a> {
    pub fn new(reports: &'a [IndexedReport]) -> Self {
        Self { reports }
    }

    /// Generate comprehensive statistics from all reports
    pub fn generate_statistics(&self) -> Result<Statistics> {
        let mut stats = Statistics::default();
        
        stats.total_reports = self.reports.len();
        stats.last_updated = Utc::now();

        // Basic counts
        stats.unique_systems = self.count_unique_systems();
        stats.total_vendors = self.count_unique_vendors(); 
        stats.component_types = self.count_component_types();
        stats.kernel_versions = self.count_kernel_versions();
        stats.distributions = self.count_distributions();

        // Compatibility overview
        stats.compatibility_overview = self.build_compatibility_overview();

        // Top hardware
        stats.top_hardware = self.build_top_hardware_list();

        // Growth statistics
        stats.growth_stats = self.build_growth_statistics();

        Ok(stats)
    }

    /// Generate detailed vendor statistics
    pub fn generate_vendor_statistics(&self) -> Result<HashMap<String, VendorStatistics>> {
        let mut vendor_stats = HashMap::new();

        for report in self.reports {
            for component in &report.components {
                if let Some(vendor) = &component.vendor {
                    let normalized_vendor = normalize_vendor_name(vendor);
                    let stats = vendor_stats.entry(normalized_vendor.clone()).or_insert_with(|| {
                        VendorStatistics::new(&normalized_vendor)
                    });

                    stats.add_report(report, component);
                }
            }
        }

        // Calculate derived statistics
        for stats in vendor_stats.values_mut() {
            stats.finalize_calculations();
        }

        Ok(vendor_stats)
    }

    /// Generate kernel compatibility analysis
    pub fn generate_kernel_analysis(&self) -> Result<HashMap<String, KernelAnalysis>> {
        let mut kernel_analysis = HashMap::new();

        for report in self.reports {
            let kernel = &report.metadata.kernel_version;
            let analysis = kernel_analysis.entry(kernel.clone()).or_insert_with(|| {
                KernelAnalysis::new(kernel)
            });

            analysis.add_report(report);
        }

        // Finalize analysis
        for analysis in kernel_analysis.values_mut() {
            analysis.finalize_calculations();
        }

        Ok(kernel_analysis)
    }

    /// Generate distribution compatibility matrix
    pub fn generate_distribution_matrix(&self) -> Result<HashMap<String, DistributionMatrix>> {
        let mut dist_matrix = HashMap::new();

        for report in self.reports {
            let distribution = &report.metadata.distribution;
            let matrix = dist_matrix.entry(distribution.clone()).or_insert_with(|| {
                DistributionMatrix::new(distribution)
            });

            matrix.add_report(report);
        }

        Ok(dist_matrix)
    }

    /// Generate time-based trend analysis
    pub fn generate_trend_analysis(&self) -> Result<TrendAnalysis> {
        let mut trends = TrendAnalysis::default();

        // Group reports by month
        let mut monthly_data: BTreeMap<(i32, u32), MonthlyData> = BTreeMap::new();

        for report in self.reports {
            let date = report.metadata.submission_date;
            let key = (date.year(), date.month());
            
            let monthly = monthly_data.entry(key).or_insert_with(|| {
                MonthlyData::new(date.year(), date.month())
            });

            monthly.add_report(report);
        }

        // Convert to trend data
        trends.monthly_trends = monthly_data.into_iter().map(|(_, data)| data).collect();
        trends.overall_growth_rate = self.calculate_overall_growth_rate(&trends.monthly_trends);
        trends.vendor_trends = self.calculate_vendor_trends();
        trends.compatibility_trends = self.calculate_compatibility_trends();

        Ok(trends)
    }

    /// Private helper methods

    fn count_unique_systems(&self) -> usize {
        let mut systems = std::collections::HashSet::new();
        for report in self.reports {
            systems.insert(&report.metadata.system_id);
        }
        systems.len()
    }

    fn count_unique_vendors(&self) -> usize {
        let mut vendors = std::collections::HashSet::new();
        for report in self.reports {
            for component in &report.components {
                if let Some(vendor) = &component.vendor {
                    vendors.insert(normalize_vendor_name(vendor));
                }
            }
        }
        vendors.len()
    }

    fn count_component_types(&self) -> usize {
        let mut types = std::collections::HashSet::new();
        for report in self.reports {
            for component in &report.components {
                types.insert(&component.component_type);
            }
        }
        types.len()
    }

    fn count_kernel_versions(&self) -> usize {
        let mut kernels = std::collections::HashSet::new();
        for report in self.reports {
            kernels.insert(&report.metadata.kernel_version);
        }
        kernels.len()
    }

    fn count_distributions(&self) -> usize {
        let mut distros = std::collections::HashSet::new();
        for report in self.reports {
            distros.insert(&report.metadata.distribution);
        }
        distros.len()
    }

    fn build_compatibility_overview(&self) -> HashMap<CompatibilityStatus, usize> {
        let mut overview = HashMap::new();

        for report in self.reports {
            *overview.entry(report.compatibility.status.clone()).or_insert(0) += 1;
        }

        overview
    }

    fn build_top_hardware_list(&self) -> Vec<PopularModel> {
        let mut hardware_counts: HashMap<(String, String), (usize, f64)> = HashMap::new();

        for report in self.reports {
            for component in &report.components {
                if let (Some(vendor), Some(model)) = (&component.vendor, &component.model) {
                    let key = (normalize_vendor_name(vendor), model.clone());
                    let compat_score = report.compatibility.status.to_score() as f64;
                    
                    let (count, avg_score) = hardware_counts.entry(key).or_insert((0, 0.0));
                    *count += 1;
                    *avg_score = (*avg_score * (*count - 1) as f64 + compat_score) / *count as f64;
                }
            }
        }

        let mut top_hardware: Vec<PopularModel> = hardware_counts
            .into_iter()
            .map(|((vendor, model), (count, avg_score))| PopularModel {
                vendor,
                model,
                report_count: count,
                avg_compatibility: avg_score,
            })
            .collect();

        top_hardware.sort_by(|a, b| b.report_count.cmp(&a.report_count));
        top_hardware.truncate(50);
        top_hardware
    }

    fn build_growth_statistics(&self) -> Vec<GrowthDataPoint> {
        // Group reports by month and calculate cumulative growth
        let mut monthly_counts: BTreeMap<(i32, u32), usize> = BTreeMap::new();

        for report in self.reports {
            let date = report.metadata.submission_date;
            let key = (date.year(), date.month());
            *monthly_counts.entry(key).or_insert(0) += 1;
        }

        let mut growth_stats = Vec::new();
        let mut cumulative_total = 0;

        for ((year, month), new_reports) in monthly_counts {
            cumulative_total += new_reports;
            
            // Create first day of month for the date
            let date = chrono::Utc.ymd_opt(year, month, 1)
                .and_then(|date| date.and_hms_opt(0, 0, 0))
                .unwrap_or_else(Utc::now);

            growth_stats.push(GrowthDataPoint {
                date,
                total_reports: cumulative_total,
                new_reports,
            });
        }

        growth_stats
    }

    fn calculate_overall_growth_rate(&self, monthly_trends: &[MonthlyData]) -> f64 {
        if monthly_trends.len() < 2 {
            return 0.0;
        }

        let first = &monthly_trends[0];
        let last = &monthly_trends[monthly_trends.len() - 1];
        
        if first.total_reports == 0 {
            return 0.0;
        }

        ((last.total_reports as f64 - first.total_reports as f64) / first.total_reports as f64) * 100.0
    }

    fn calculate_vendor_trends(&self) -> Vec<VendorTrend> {
        // Simplified vendor trend calculation
        let mut vendor_counts: HashMap<String, usize> = HashMap::new();
        
        for report in self.reports {
            for component in &report.components {
                if let Some(vendor) = &component.vendor {
                    let normalized = normalize_vendor_name(vendor);
                    *vendor_counts.entry(normalized).or_insert(0) += 1;
                }
            }
        }

        let mut trends: Vec<VendorTrend> = vendor_counts
            .into_iter()
            .map(|(vendor, count)| VendorTrend {
                vendor,
                report_count: count,
                growth_rate: 0.0, // Would calculate from historical data
                market_share: 0.0, // Would calculate from total reports
            })
            .collect();

        trends.sort_by(|a, b| b.report_count.cmp(&a.report_count));
        trends.truncate(20);

        // Calculate market share
        let total_reports = trends.iter().map(|t| t.report_count).sum::<usize>() as f64;
        for trend in &mut trends {
            trend.market_share = (trend.report_count as f64 / total_reports) * 100.0;
        }

        trends
    }

    fn calculate_compatibility_trends(&self) -> Vec<CompatibilityTrend> {
        // Simplified compatibility trend calculation
        let mut trends = Vec::new();

        for status in &[
            CompatibilityStatus::Excellent,
            CompatibilityStatus::Good,
            CompatibilityStatus::Fair,
            CompatibilityStatus::Poor,
            CompatibilityStatus::Unknown,
        ] {
            let count = self.reports.iter()
                .filter(|r| r.compatibility.status == *status)
                .count();

            trends.push(CompatibilityTrend {
                status: status.clone(),
                report_count: count,
                percentage: (count as f64 / self.reports.len() as f64) * 100.0,
                trend_direction: TrendDirection::Stable, // Would calculate from historical data
            });
        }

        trends
    }
}

/// Detailed vendor statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorStatistics {
    pub vendor: String,
    pub total_reports: usize,
    pub unique_models: std::collections::HashSet<String>,
    pub component_types: std::collections::HashSet<String>,
    pub compatibility_scores: Vec<f64>,
    pub avg_compatibility: f64,
    pub market_share: f64,
    pub top_models: Vec<PopularModel>,
}

impl VendorStatistics {
    fn new(vendor: &str) -> Self {
        Self {
            vendor: vendor.to_string(),
            total_reports: 0,
            unique_models: std::collections::HashSet::new(),
            component_types: std::collections::HashSet::new(),
            compatibility_scores: Vec::new(),
            avg_compatibility: 0.0,
            market_share: 0.0,
            top_models: Vec::new(),
        }
    }

    fn add_report(&mut self, report: &IndexedReport, component: &HardwareComponent) {
        self.total_reports += 1;
        
        if let Some(model) = &component.model {
            self.unique_models.insert(model.clone());
        }
        
        self.component_types.insert(component.component_type.clone());
        self.compatibility_scores.push(report.compatibility.status.to_score() as f64);
    }

    fn finalize_calculations(&mut self) {
        if !self.compatibility_scores.is_empty() {
            self.avg_compatibility = self.compatibility_scores.iter().sum::<f64>() / self.compatibility_scores.len() as f64;
        }
    }
}

/// Kernel version analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelAnalysis {
    pub kernel_version: String,
    pub total_reports: usize,
    pub compatibility_breakdown: HashMap<CompatibilityStatus, usize>,
    pub problematic_vendors: Vec<String>,
    pub success_rate: f64,
    pub adoption_trend: TrendDirection,
}

impl KernelAnalysis {
    fn new(kernel: &str) -> Self {
        Self {
            kernel_version: kernel.to_string(),
            total_reports: 0,
            compatibility_breakdown: HashMap::new(),
            problematic_vendors: Vec::new(),
            success_rate: 0.0,
            adoption_trend: TrendDirection::Stable,
        }
    }

    fn add_report(&mut self, report: &IndexedReport) {
        self.total_reports += 1;
        *self.compatibility_breakdown.entry(report.compatibility.status.clone()).or_insert(0) += 1;
    }

    fn finalize_calculations(&mut self) {
        if self.total_reports > 0 {
            let excellent = self.compatibility_breakdown.get(&CompatibilityStatus::Excellent).unwrap_or(&0);
            let good = self.compatibility_breakdown.get(&CompatibilityStatus::Good).unwrap_or(&0);
            self.success_rate = ((*excellent + *good) as f64 / self.total_reports as f64) * 100.0;
        }
    }
}

/// Distribution compatibility matrix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionMatrix {
    pub distribution: String,
    pub total_reports: usize,
    pub vendor_compatibility: HashMap<String, f64>,
    pub kernel_compatibility: HashMap<String, f64>,
    pub overall_score: f64,
}

impl DistributionMatrix {
    fn new(distribution: &str) -> Self {
        Self {
            distribution: distribution.to_string(),
            total_reports: 0,
            vendor_compatibility: HashMap::new(),
            kernel_compatibility: HashMap::new(),
            overall_score: 0.0,
        }
    }

    fn add_report(&mut self, report: &IndexedReport) {
        self.total_reports += 1;
        
        let compat_score = report.compatibility.status.to_score() as f64;
        
        // Track kernel compatibility
        let kernel_score = self.kernel_compatibility
            .entry(report.metadata.kernel_version.clone())
            .or_insert(0.0);
        *kernel_score = (*kernel_score + compat_score) / 2.0;

        // Track vendor compatibility
        for component in &report.components {
            if let Some(vendor) = &component.vendor {
                let normalized_vendor = normalize_vendor_name(vendor);
                let vendor_score = self.vendor_compatibility
                    .entry(normalized_vendor)
                    .or_insert(0.0);
                *vendor_score = (*vendor_score + compat_score) / 2.0;
            }
        }
    }
}

/// Time-based trend analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrendAnalysis {
    pub monthly_trends: Vec<MonthlyData>,
    pub overall_growth_rate: f64,
    pub vendor_trends: Vec<VendorTrend>,
    pub compatibility_trends: Vec<CompatibilityTrend>,
}

/// Monthly data point for trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyData {
    pub year: i32,
    pub month: u32,
    pub total_reports: usize,
    pub new_reports: usize,
    pub unique_systems: usize,
    pub avg_compatibility: f64,
}

impl MonthlyData {
    fn new(year: i32, month: u32) -> Self {
        Self {
            year,
            month,
            total_reports: 0,
            new_reports: 0,
            unique_systems: 0,
            avg_compatibility: 0.0,
        }
    }

    fn add_report(&mut self, _report: &IndexedReport) {
        self.new_reports += 1;
        // Would update other metrics based on report data
    }
}

/// Vendor trend information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorTrend {
    pub vendor: String,
    pub report_count: usize,
    pub growth_rate: f64,
    pub market_share: f64,
}

/// Compatibility trend information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityTrend {
    pub status: CompatibilityStatus,
    pub report_count: usize,
    pub percentage: f64,
    pub trend_direction: TrendDirection,
}

/// Trend direction indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Increasing,
    Stable,
    Decreasing,
}

impl TrendDirection {
    pub fn description(&self) -> &'static str {
        match self {
            TrendDirection::Increasing => "Trending upward",
            TrendDirection::Stable => "Stable trend",
            TrendDirection::Decreasing => "Trending downward",
        }
    }
}