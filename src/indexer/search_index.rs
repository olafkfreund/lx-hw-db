//! API endpoint generation for static JSON files

use super::*;
use crate::errors::Result;
use serde_json::json;
use std::path::Path;

/// Builder for generating static API endpoint files
pub struct ApiBuilder<'a> {
    indices: &'a IndexCollection,
}

impl<'a> ApiBuilder<'a> {
    pub fn new(indices: &'a IndexCollection) -> Self {
        Self { indices }
    }

    /// Write all API endpoint files
    pub fn write_api_files(&self, api_dir: &Path) -> Result<()> {
        self.write_search_endpoints(api_dir)?;
        self.write_stats_endpoints(api_dir)?;
        self.write_recommendation_endpoints(api_dir)?;
        self.write_metadata_endpoints(api_dir)?;
        Ok(())
    }

    /// Write search-related API endpoints
    fn write_search_endpoints(&self, api_dir: &Path) -> Result<()> {
        let search_dir = api_dir.join("v1/search");
        std::fs::create_dir_all(&search_dir)?;

        // /api/v1/search/vendors.json - List all vendors with counts
        let vendors_data = json!({
            "version": "1.0",
            "generated": Utc::now().to_rfc3339(),
            "data": self.indices.by_vendor.iter().map(|(vendor, entry)| {
                json!({
                    "vendor": vendor,
                    "total_reports": entry.total_reports,
                    "compatibility_score": entry.compatibility_score,
                    "component_types": entry.components.keys().collect::<Vec<_>>()
                })
            }).collect::<Vec<_>>()
        });
        
        std::fs::write(
            search_dir.join("vendors.json"),
            serde_json::to_string_pretty(&vendors_data)?
        )?;

        // /api/v1/search/components.json - List all component types
        let components_data = json!({
            "version": "1.0", 
            "generated": Utc::now().to_rfc3339(),
            "data": self.indices.by_component.iter().map(|(comp_type, entry)| {
                json!({
                    "component_type": comp_type,
                    "total_reports": entry.total_reports,
                    "top_vendors": entry.vendors.iter()
                        .map(|(vendor, count)| json!({"vendor": vendor, "count": count}))
                        .collect::<Vec<_>>(),
                    "popular_models": entry.popular_models.iter().take(10).collect::<Vec<_>>()
                })
            }).collect::<Vec<_>>()
        });

        std::fs::write(
            search_dir.join("components.json"),
            serde_json::to_string_pretty(&components_data)?
        )?;

        // /api/v1/search/kernels.json - List kernel versions with stats
        let kernels_data = json!({
            "version": "1.0",
            "generated": Utc::now().to_rfc3339(),
            "data": self.indices.by_kernel.iter().map(|(kernel, entry)| {
                json!({
                    "kernel_version": kernel,
                    "total_reports": entry.total_reports,
                    "compatibility_stats": entry.compatibility_stats,
                    "problematic_hardware": entry.problematic_hardware.iter().take(5).collect::<Vec<_>>()
                })
            }).collect::<Vec<_>>()
        });

        std::fs::write(
            search_dir.join("kernels.json"),
            serde_json::to_string_pretty(&kernels_data)?
        )?;

        // /api/v1/search/distributions.json - List distributions
        let distributions_data = json!({
            "version": "1.0",
            "generated": Utc::now().to_rfc3339(),
            "data": self.indices.by_distribution.iter().map(|(distro, entry)| {
                json!({
                    "distribution": distro,
                    "total_reports": entry.total_reports,
                    "common_kernels": entry.common_kernels.iter().take(5).collect::<Vec<_>>(),
                    "top_vendors": entry.vendor_compatibility.iter()
                        .map(|(vendor, score)| json!({"vendor": vendor, "compatibility": score}))
                        .collect::<Vec<_>>()
                })
            }).collect::<Vec<_>>()
        });

        std::fs::write(
            search_dir.join("distributions.json"),
            serde_json::to_string_pretty(&distributions_data)?
        )?;

        Ok(())
    }

    /// Write statistics API endpoints
    fn write_stats_endpoints(&self, api_dir: &Path) -> Result<()> {
        let stats_dir = api_dir.join("v1/stats");
        std::fs::create_dir_all(&stats_dir)?;

        // /api/v1/stats/overview.json - General database statistics
        let overview_data = json!({
            "version": "1.0",
            "generated": Utc::now().to_rfc3339(),
            "data": {
                "total_reports": self.indices.statistics.total_reports,
                "unique_systems": self.indices.statistics.unique_systems,
                "total_vendors": self.indices.statistics.total_vendors,
                "component_types": self.indices.statistics.component_types,
                "kernel_versions": self.indices.statistics.kernel_versions,
                "distributions": self.indices.statistics.distributions,
                "compatibility_overview": self.indices.statistics.compatibility_overview,
                "last_updated": self.indices.statistics.last_updated
            }
        });

        std::fs::write(
            stats_dir.join("overview.json"),
            serde_json::to_string_pretty(&overview_data)?
        )?;

        // /api/v1/stats/top-hardware.json - Most reported hardware
        let top_hardware_data = json!({
            "version": "1.0",
            "generated": Utc::now().to_rfc3339(),
            "data": {
                "top_hardware": self.indices.statistics.top_hardware.iter().take(25).map(|hw| {
                    json!({
                        "vendor": hw.vendor,
                        "model": hw.model,
                        "report_count": hw.report_count,
                        "avg_compatibility": hw.avg_compatibility
                    })
                }).collect::<Vec<_>>()
            }
        });

        std::fs::write(
            stats_dir.join("top-hardware.json"),
            serde_json::to_string_pretty(&top_hardware_data)?
        )?;

        // /api/v1/stats/trends.json - Growth and trend data
        let trends_data = json!({
            "version": "1.0",
            "generated": Utc::now().to_rfc3339(),
            "data": {
                "growth_stats": self.indices.statistics.growth_stats,
                "vendor_growth": self.build_vendor_trends(),
                "kernel_adoption": self.build_kernel_trends()
            }
        });

        std::fs::write(
            stats_dir.join("trends.json"),
            serde_json::to_string_pretty(&trends_data)?
        )?;

        Ok(())
    }

    /// Write recommendation API endpoints
    fn write_recommendation_endpoints(&self, api_dir: &Path) -> Result<()> {
        let rec_dir = api_dir.join("v1/recommendations");
        std::fs::create_dir_all(&rec_dir)?;

        // Create subdirectories for different recommendation types
        std::fs::create_dir_all(rec_dir.join("by-vendor"))?;
        std::fs::create_dir_all(rec_dir.join("by-component"))?;
        std::fs::create_dir_all(rec_dir.join("by-use-case"))?;

        // Generate vendor-specific recommendations
        self.write_vendor_recommendations(&rec_dir)?;
        
        // Generate component-specific recommendations
        self.write_component_recommendations(&rec_dir)?;

        // Generate use-case recommendations
        self.write_use_case_recommendations(&rec_dir)?;

        Ok(())
    }

    /// Write metadata API endpoints
    fn write_metadata_endpoints(&self, api_dir: &Path) -> Result<()> {
        let meta_dir = api_dir.join("v1");
        
        // /api/v1/index.json - API directory and version info
        let api_index = json!({
            "name": "Linux Hardware Compatibility Database API",
            "version": "1.0",
            "generated": Utc::now().to_rfc3339(),
            "endpoints": {
                "search": {
                    "vendors": "/api/v1/search/vendors.json",
                    "components": "/api/v1/search/components.json", 
                    "kernels": "/api/v1/search/kernels.json",
                    "distributions": "/api/v1/search/distributions.json"
                },
                "stats": {
                    "overview": "/api/v1/stats/overview.json",
                    "top_hardware": "/api/v1/stats/top-hardware.json",
                    "trends": "/api/v1/stats/trends.json"
                },
                "recommendations": {
                    "by_vendor": "/api/v1/recommendations/by-vendor/",
                    "by_component": "/api/v1/recommendations/by-component/",
                    "by_use_case": "/api/v1/recommendations/by-use-case/"
                }
            },
            "data_sources": {
                "raw_indices": "/indices/",
                "hardware_reports": "/hardware-reports/"
            },
            "rate_limits": "None (static files)",
            "documentation": "/web/api-docs/"
        });

        std::fs::write(
            meta_dir.join("index.json"),
            serde_json::to_string_pretty(&api_index)?
        )?;

        Ok(())
    }

    /// Generate vendor-specific recommendation files
    fn write_vendor_recommendations(&self, rec_dir: &Path) -> Result<()> {
        for (vendor, entry) in &self.indices.by_vendor {
            let recommendations = self.generate_vendor_recommendations(vendor, entry);
            
            let vendor_filename = vendor.to_lowercase().replace(' ', '-') + ".json";
            let vendor_data = json!({
                "version": "1.0",
                "generated": Utc::now().to_rfc3339(),
                "vendor": vendor,
                "data": recommendations
            });

            std::fs::write(
                rec_dir.join("by-vendor").join(vendor_filename),
                serde_json::to_string_pretty(&vendor_data)?
            )?;
        }
        Ok(())
    }

    /// Generate component-specific recommendation files
    fn write_component_recommendations(&self, rec_dir: &Path) -> Result<()> {
        for (component_type, entry) in &self.indices.by_component {
            let recommendations = self.generate_component_recommendations(component_type, entry);
            
            let component_filename = component_type.to_lowercase().replace(' ', '-') + ".json";
            let component_data = json!({
                "version": "1.0",
                "generated": Utc::now().to_rfc3339(),
                "component_type": component_type,
                "data": recommendations
            });

            std::fs::write(
                rec_dir.join("by-component").join(component_filename),
                serde_json::to_string_pretty(&component_data)?
            )?;
        }
        Ok(())
    }

    /// Generate use-case specific recommendation files
    fn write_use_case_recommendations(&self, rec_dir: &Path) -> Result<()> {
        let use_cases = vec!["gaming", "development", "server", "general"];
        
        for use_case in use_cases {
            let recommendations = self.generate_use_case_recommendations(use_case);
            
            let use_case_data = json!({
                "version": "1.0",
                "generated": Utc::now().to_rfc3339(),
                "use_case": use_case,
                "data": recommendations
            });

            std::fs::write(
                rec_dir.join("by-use-case").join(format!("{}.json", use_case)),
                serde_json::to_string_pretty(&use_case_data)?
            )?;
        }
        Ok(())
    }

    /// Build vendor trend data
    fn build_vendor_trends(&self) -> serde_json::Value {
        let mut trends = Vec::new();
        
        for (vendor, entry) in &self.indices.by_vendor {
            trends.push(json!({
                "vendor": vendor,
                "total_reports": entry.total_reports,
                "compatibility_score": entry.compatibility_score,
                "component_diversity": entry.components.len()
            }));
        }

        // Sort by report count
        trends.sort_by(|a, b| {
            b["total_reports"].as_u64().unwrap_or(0)
                .cmp(&a["total_reports"].as_u64().unwrap_or(0))
        });

        json!(trends.into_iter().take(20).collect::<Vec<_>>())
    }

    /// Build kernel adoption trends
    fn build_kernel_trends(&self) -> serde_json::Value {
        let mut trends = Vec::new();
        
        for (kernel, entry) in &self.indices.by_kernel {
            let excellent_count = entry.compatibility_stats.get("excellent").unwrap_or(&0);
            let good_count = entry.compatibility_stats.get("good").unwrap_or(&0);
            let total_good = excellent_count + good_count;
            
            let compatibility_percentage = if entry.total_reports > 0 {
                (total_good * 100) / entry.total_reports
            } else {
                0
            };

            trends.push(json!({
                "kernel_version": kernel,
                "total_reports": entry.total_reports,
                "compatibility_percentage": compatibility_percentage,
                "compatibility_stats": entry.compatibility_stats
            }));
        }

        // Sort by kernel version (reverse order for latest first)
        trends.sort_by(|a, b| {
            b["kernel_version"].as_str().unwrap_or("")
                .cmp(a["kernel_version"].as_str().unwrap_or(""))
        });

        json!(trends.into_iter().take(15).collect::<Vec<_>>())
    }

    /// Generate recommendations for a specific vendor
    fn generate_vendor_recommendations(&self, vendor: &str, entry: &VendorEntry) -> serde_json::Value {
        json!({
            "vendor_overview": {
                "compatibility_score": entry.compatibility_score,
                "recommendation_level": self.get_recommendation_level(entry.compatibility_score),
                "total_reports": entry.total_reports,
                "component_types": entry.components.keys().collect::<Vec<_>>()
            },
            "best_components": self.get_best_components_for_vendor(vendor),
            "compatibility_notes": self.get_vendor_compatibility_notes(vendor),
            "alternatives": self.get_vendor_alternatives(vendor)
        })
    }

    /// Generate recommendations for a component type
    fn generate_component_recommendations(&self, component_type: &str, entry: &ComponentEntry) -> serde_json::Value {
        json!({
            "component_overview": {
                "total_reports": entry.total_reports,
                "top_vendors": entry.vendors.iter()
                    .map(|(v, c)| json!({"vendor": v, "count": c}))
                    .collect::<Vec<_>>(),
                "compatibility_distribution": entry.compatibility_distribution
            },
            "recommended_models": entry.popular_models.iter()
                .filter(|m| m.avg_compatibility > 80.0)
                .take(10)
                .collect::<Vec<_>>(),
            "budget_options": self.get_budget_options_for_component(component_type),
            "performance_leaders": self.get_performance_leaders_for_component(component_type)
        })
    }

    /// Generate use-case specific recommendations  
    fn generate_use_case_recommendations(&self, use_case: &str) -> serde_json::Value {
        match use_case {
            "gaming" => json!({
                "recommended_gpus": self.get_gaming_gpu_recommendations(),
                "recommended_cpus": self.get_gaming_cpu_recommendations(),
                "compatibility_notes": [
                    "NVIDIA GPUs generally have better gaming performance on Linux",
                    "AMD GPUs have excellent open-source driver support",
                    "Consider kernel version for latest hardware support"
                ]
            }),
            "development" => json!({
                "recommended_hardware": self.get_development_hardware_recommendations(),
                "compatibility_notes": [
                    "Intel hardware typically has excellent Linux support",
                    "Focus on stable, well-supported components",
                    "Consider multiple monitors and USB devices"
                ]
            }),
            "server" => json!({
                "recommended_hardware": self.get_server_hardware_recommendations(),
                "compatibility_notes": [
                    "Prioritize stability and long-term support",
                    "ECC memory support important for servers",
                    "Network hardware compatibility critical"
                ]
            }),
            _ => json!({
                "top_compatible_hardware": self.indices.statistics.top_hardware.iter()
                    .filter(|hw| hw.avg_compatibility > 85.0)
                    .take(20)
                    .collect::<Vec<_>>()
            })
        }
    }

    /// Helper methods for recommendations

    fn get_recommendation_level(&self, score: f64) -> &'static str {
        match score {
            s if s >= 90.0 => "Excellent",
            s if s >= 80.0 => "Good", 
            s if s >= 70.0 => "Fair",
            _ => "Caution"
        }
    }

    fn get_best_components_for_vendor(&self, _vendor: &str) -> Vec<serde_json::Value> {
        // Placeholder - would analyze compatibility matrix
        vec![]
    }

    fn get_vendor_compatibility_notes(&self, _vendor: &str) -> Vec<&'static str> {
        // Placeholder - would provide vendor-specific guidance
        vec![]
    }

    fn get_vendor_alternatives(&self, _vendor: &str) -> Vec<serde_json::Value> {
        // Placeholder - would suggest alternative vendors
        vec![]
    }

    fn get_budget_options_for_component(&self, _component_type: &str) -> Vec<serde_json::Value> {
        // Placeholder - would filter by price/performance
        vec![]
    }

    fn get_performance_leaders_for_component(&self, _component_type: &str) -> Vec<serde_json::Value> {
        // Placeholder - would identify top performers
        vec![]
    }

    fn get_gaming_gpu_recommendations(&self) -> Vec<serde_json::Value> {
        // Placeholder - would filter GPUs suitable for gaming
        vec![]
    }

    fn get_gaming_cpu_recommendations(&self) -> Vec<serde_json::Value> {
        // Placeholder - would filter CPUs suitable for gaming
        vec![]
    }

    fn get_development_hardware_recommendations(&self) -> Vec<serde_json::Value> {
        // Placeholder - would recommend development-friendly hardware
        vec![]
    }

    fn get_server_hardware_recommendations(&self) -> Vec<serde_json::Value> {
        // Placeholder - would recommend server-grade hardware
        vec![]
    }
}