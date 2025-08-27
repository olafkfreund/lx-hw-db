use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::configuration::*;
use crate::hardware::HardwareReport;
use crate::errors::LxHwError;
use crate::configuration::engine::ConfigurationEngineImpl;
use crate::configuration::dkms::DkmsManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemRecommendations {
    pub system_id: String,
    pub target_distribution: String,
    pub kernel_version: String,
    pub compatibility_score: f64,
    pub recommendations: Vec<Recommendation>,
    pub installation_script: String,
    pub configuration_files: HashMap<String, String>,
    pub warnings: Vec<String>,
    pub performance_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub category: RecommendationCategory,
    pub priority: Priority,
    pub title: String,
    pub description: String,
    pub implementation: Implementation,
    pub expected_outcome: String,
    pub risk_assessment: RiskAssessment,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RecommendationCategory {
    DriverInstallation,
    KernelParameters,
    PackageInstallation,
    DkmsModule,
    ConfigurationFile,
    PerformanceOptimization,
    SecurityConfiguration,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Critical = 4,
    High = 3,
    Medium = 2,
    Low = 1,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Implementation {
    pub implementation_type: ImplementationType,
    pub commands: Vec<String>,
    pub files_to_modify: Vec<FileModification>,
    pub verification_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplementationType {
    CommandExecution,
    FileModification,
    PackageInstallation,
    ServiceConfiguration,
    Combined,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileModification {
    pub file_path: String,
    pub modification_type: ModificationType,
    pub content: String,
    pub backup_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModificationType {
    Create,
    Append,
    Replace,
    InsertLine,
    ModifyParameter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub risk_level: RiskLevel,
    pub potential_issues: Vec<String>,
    pub rollback_instructions: Vec<String>,
    pub compatibility_notes: Vec<String>,
}

pub struct RecommendationEngine {
    config_engine: ConfigurationEngineImpl,
    dkms_manager: DkmsManager,
}

impl RecommendationEngine {
    pub fn new() -> Result<Self, LxHwError> {
        Ok(Self {
            config_engine: ConfigurationEngineImpl::new()?,
            dkms_manager: DkmsManager::new()?,
        })
    }

    pub fn generate_recommendations(&self, hardware: &HardwareReport, target_distribution: &str) -> Result<SystemRecommendations, LxHwError> {
        let configuration = self.config_engine.generate_configuration(hardware, target_distribution)?;
        let dkms_modules = self.dkms_manager.identify_required_modules(hardware)?;
        
        let mut recommendations = Vec::new();
        let mut warnings = Vec::new();
        let mut performance_notes = Vec::new();

        // Generate driver recommendations
        recommendations.extend(self.generate_driver_recommendations(&configuration.driver_recommendations)?);

        // Generate kernel parameter recommendations
        recommendations.extend(self.generate_kernel_parameter_recommendations(&configuration.kernel_parameters)?);

        // Generate package installation recommendations
        recommendations.extend(self.generate_package_recommendations(&configuration.package_installations)?);

        // Generate DKMS module recommendations
        recommendations.extend(self.generate_dkms_recommendations(&dkms_modules)?);

        // Generate performance optimization recommendations
        recommendations.extend(self.generate_performance_recommendations(&configuration.performance_optimizations)?);

        // Generate configuration file recommendations
        let config_file_recommendations = self.generate_configuration_file_recommendations(&configuration.configuration_files)?;
        recommendations.extend(config_file_recommendations);

        // Generate warnings and notes
        warnings.extend(self.analyze_potential_issues(&configuration, &dkms_modules)?);
        performance_notes.extend(self.generate_performance_notes(&configuration)?);

        // Sort recommendations by priority
        recommendations.sort_by(|a, b| b.priority.cmp(&a.priority));

        // Generate installation script
        let installation_script = self.generate_installation_script(&recommendations, target_distribution)?;

        // Generate configuration files content
        let configuration_files = self.generate_configuration_files_content(&configuration.configuration_files)?;

        Ok(SystemRecommendations {
            system_id: configuration.system_id,
            target_distribution: target_distribution.to_string(),
            kernel_version: configuration.kernel_version,
            compatibility_score: configuration.compatibility_score,
            recommendations,
            installation_script,
            configuration_files,
            warnings,
            performance_notes,
        })
    }

    fn generate_driver_recommendations(&self, driver_recommendations: &[DriverRecommendation]) -> Result<Vec<Recommendation>, LxHwError> {
        let mut recommendations = Vec::new();

        for driver_rec in driver_recommendations {
            let priority = match driver_rec.installation_priority {
                9..=10 => Priority::Critical,
                7..=8 => Priority::High,
                4..=6 => Priority::Medium,
                _ => Priority::Low,
            };
            let priority_clone = priority.clone();

            let implementation = match &driver_rec.driver_source {
                DriverSource::KernelBuiltin => Implementation {
                    implementation_type: ImplementationType::CommandExecution,
                    commands: vec![
                        format!("modprobe {}", driver_rec.kernel_modules.join(" ")),
                    ],
                    files_to_modify: vec![
                        FileModification {
                            file_path: "/etc/modules".to_string(),
                            modification_type: ModificationType::Append,
                            content: driver_rec.kernel_modules.join("\n"),
                            backup_required: true,
                        },
                    ],
                    verification_commands: vec![
                        format!("lsmod | grep -E '{}'", driver_rec.kernel_modules.join("|")),
                    ],
                },
                DriverSource::DistributionPackage { package_name } => Implementation {
                    implementation_type: ImplementationType::PackageInstallation,
                    commands: vec![
                        format!("# Install {} driver package", driver_rec.recommended_driver),
                        format!("# Package: {}", package_name),
                    ],
                    files_to_modify: vec![],
                    verification_commands: vec![
                        format!("dpkg -l | grep {} || rpm -q {} || pacman -Q {}", package_name, package_name, package_name),
                    ],
                },
                DriverSource::Dkms { module_name } => Implementation {
                    implementation_type: ImplementationType::CommandExecution,
                    commands: vec![
                        format!("# DKMS module installation will be handled separately"),
                        format!("# Module: {}", module_name),
                    ],
                    files_to_modify: vec![],
                    verification_commands: vec![
                        format!("dkms status | grep {}", module_name),
                    ],
                },
                DriverSource::ThirdParty { source_url } => Implementation {
                    implementation_type: ImplementationType::Combined,
                    commands: vec![
                        format!("# Download driver from: {}", source_url),
                        "# Follow manufacturer installation instructions".to_string(),
                    ],
                    files_to_modify: vec![],
                    verification_commands: vec![],
                },
            };

            recommendations.push(Recommendation {
                category: RecommendationCategory::DriverInstallation,
                priority: priority_clone,
                title: format!("Install {} driver for {}", driver_rec.recommended_driver, driver_rec.component_type),
                description: format!(
                    "Install the {} driver for your {} hardware. {}",
                    driver_rec.recommended_driver,
                    driver_rec.component_type,
                    driver_rec.compatibility_notes.as_ref().unwrap_or(&"".to_string())
                ),
                implementation,
                expected_outcome: format!("Proper hardware support for {}", driver_rec.component_type),
                risk_assessment: RiskAssessment {
                    risk_level: if priority == Priority::Critical { RiskLevel::Low } else { RiskLevel::Medium },
                    potential_issues: vec![
                        "Driver conflicts with existing drivers".to_string(),
                        "System instability if driver is incompatible".to_string(),
                    ],
                    rollback_instructions: vec![
                        format!("modprobe -r {}", driver_rec.kernel_modules.join(" ")),
                        "Reboot to previous kernel if issues persist".to_string(),
                    ],
                    compatibility_notes: driver_rec.alternative_drivers.iter()
                        .map(|alt| format!("Alternative: {}", alt))
                        .collect(),
                },
            });
        }

        Ok(recommendations)
    }

    fn generate_kernel_parameter_recommendations(&self, kernel_parameters: &[KernelParameter]) -> Result<Vec<Recommendation>, LxHwError> {
        let mut recommendations = Vec::new();

        // Group parameters by hardware target for better organization
        let mut grouped_params: HashMap<String, Vec<&KernelParameter>> = HashMap::new();
        
        for param in kernel_parameters {
            let target = param.hardware_target.as_ref()
                .unwrap_or(&"General".to_string())
                .clone();
            grouped_params.entry(target).or_insert_with(Vec::new).push(param);
        }

        for (target, params) in grouped_params {
            let parameter_strings: Vec<String> = params.iter()
                .map(|p| {
                    if let Some(value) = &p.value {
                        format!("{}={}", p.parameter, value)
                    } else {
                        p.parameter.clone()
                    }
                })
                .collect();

            recommendations.push(Recommendation {
                category: RecommendationCategory::KernelParameters,
                priority: Priority::High,
                title: format!("Configure kernel parameters for {}", target),
                description: format!(
                    "Add kernel boot parameters to optimize {} performance and compatibility. Parameters: {}",
                    target,
                    parameter_strings.join(", ")
                ),
                implementation: Implementation {
                    implementation_type: ImplementationType::FileModification,
                    commands: vec![],
                    files_to_modify: vec![
                        FileModification {
                            file_path: "/etc/default/grub".to_string(),
                            modification_type: ModificationType::ModifyParameter,
                            content: format!("GRUB_CMDLINE_LINUX_DEFAULT=\"$GRUB_CMDLINE_LINUX_DEFAULT {}\"", parameter_strings.join(" ")),
                            backup_required: true,
                        },
                    ],
                    verification_commands: vec![
                        "cat /proc/cmdline".to_string(),
                        "update-grub".to_string(),
                    ],
                },
                expected_outcome: format!("Optimized {} performance and stability", target),
                risk_assessment: RiskAssessment {
                    risk_level: RiskLevel::Medium,
                    potential_issues: vec![
                        "Incorrect parameters may prevent system boot".to_string(),
                        "Some parameters may conflict with hardware".to_string(),
                    ],
                    rollback_instructions: vec![
                        "Boot from GRUB advanced options with previous kernel parameters".to_string(),
                        "Edit /etc/default/grub to remove problematic parameters".to_string(),
                        "Run update-grub to apply changes".to_string(),
                    ],
                    compatibility_notes: params.iter()
                        .map(|p| p.purpose.clone())
                        .collect(),
                },
            });
        }

        Ok(recommendations)
    }

    fn generate_package_recommendations(&self, package_installations: &[PackageInstallation]) -> Result<Vec<Recommendation>, LxHwError> {
        let mut recommendations = Vec::new();

        for installation in package_installations {
            recommendations.push(Recommendation {
                category: RecommendationCategory::PackageInstallation,
                priority: Priority::High,
                title: format!("Install {}", installation.package_name),
                description: format!(
                    "{}: {}. Use command: {}",
                    installation.package_description,
                    "This package provides necessary drivers, firmware, and utilities for your hardware",
                    installation.installation_command
                ),
                implementation: Implementation {
                    implementation_type: ImplementationType::PackageInstallation,
                    commands: vec![
                        installation.installation_command.clone(),
                    ],
                    files_to_modify: vec![],
                    verification_commands: vec![
                        format!("which {} || dpkg -l {} || rpm -q {} || pacman -Q {}", 
                            installation.package_name, installation.package_name, 
                            installation.package_name, installation.package_name)
                    ],
                },
                expected_outcome: "Proper hardware support and functionality".to_string(),
                risk_assessment: RiskAssessment {
                    risk_level: RiskLevel::Low,
                    potential_issues: vec![
                        "Package conflicts with existing software".to_string(),
                        "Repository access issues".to_string(),
                    ],
                    rollback_instructions: vec![
                        format!("Remove package if needed: apt remove {} || dnf remove {} || pacman -R {}", 
                            installation.package_name, installation.package_name, installation.package_name
                        ),
                    ],
                    compatibility_notes: vec![],
                },
            });
        }

        Ok(recommendations)
    }

    fn generate_dkms_recommendations(&self, dkms_modules: &[DkmsModule]) -> Result<Vec<Recommendation>, LxHwError> {
        let mut recommendations = Vec::new();

        for module in dkms_modules {
            recommendations.push(Recommendation {
                category: RecommendationCategory::DkmsModule,
                priority: Priority::High,
                title: format!("Install DKMS module: {}", module.module_name),
                description: format!(
                    "Install and configure DKMS module {} version {} for automatic kernel module rebuilding.",
                    module.module_name, module.version
                ),
                implementation: Implementation {
                    implementation_type: ImplementationType::Combined,
                    commands: module.installation_steps.clone(),
                    files_to_modify: vec![],
                    verification_commands: vec![
                        format!("dkms status | grep {}", module.module_name),
                        format!("lsmod | grep {}", module.module_name),
                    ],
                },
                expected_outcome: format!("Automatic rebuilding of {} module on kernel updates", module.module_name),
                risk_assessment: RiskAssessment {
                    risk_level: RiskLevel::Medium,
                    potential_issues: vec![
                        "Compilation failures on kernel updates".to_string(),
                        "Module signing issues with secure boot".to_string(),
                    ],
                    rollback_instructions: vec![
                        format!("dkms remove {} -k $(uname -r)", module.module_name),
                        format!("dkms uninstall {}", module.module_name),
                    ],
                    compatibility_notes: module.kernel_versions.clone(),
                },
            });
        }

        Ok(recommendations)
    }

    fn generate_performance_recommendations(&self, optimizations: &[PerformanceOptimization]) -> Result<Vec<Recommendation>, LxHwError> {
        let mut recommendations = Vec::new();

        for optimization in optimizations {
            let file_modifications = optimization.configuration_changes.iter()
                .map(|change| FileModification {
                    file_path: change.file_path.clone(),
                    modification_type: ModificationType::ModifyParameter,
                    content: format!("{}={}", change.parameter, change.new_value),
                    backup_required: true,
                })
                .collect();

            recommendations.push(Recommendation {
                category: RecommendationCategory::PerformanceOptimization,
                priority: Priority::Medium,
                title: format!("Performance Optimization: {}", optimization.optimization_type),
                description: format!("{} - {}", optimization.description, optimization.expected_improvement),
                implementation: Implementation {
                    implementation_type: ImplementationType::FileModification,
                    commands: vec![],
                    files_to_modify: file_modifications,
                    verification_commands: vec![],
                },
                expected_outcome: optimization.expected_improvement.clone(),
                risk_assessment: RiskAssessment {
                    risk_level: optimization.risk_level.clone(),
                    potential_issues: vec![
                        "Performance changes may not suit all workloads".to_string(),
                        "Some optimizations may increase power consumption".to_string(),
                    ],
                    rollback_instructions: optimization.configuration_changes.iter()
                        .map(|change| format!("Restore {} to {}", 
                            change.parameter, 
                            change.old_value.as_ref().unwrap_or(&"default".to_string())
                        ))
                        .collect(),
                    compatibility_notes: vec![],
                },
            });
        }

        Ok(recommendations)
    }

    fn generate_configuration_file_recommendations(&self, config_files: &HashMap<String, ConfigurationFile>) -> Result<Vec<Recommendation>, LxHwError> {
        let mut recommendations = Vec::new();

        for (name, config_file) in config_files {
            recommendations.push(Recommendation {
                category: RecommendationCategory::ConfigurationFile,
                priority: Priority::Medium,
                title: format!("Configure {}", name),
                description: format!("Create or modify configuration file: {}", config_file.file_path),
                implementation: Implementation {
                    implementation_type: ImplementationType::FileModification,
                    commands: if config_file.backup_original {
                        vec![format!("cp {} {}.backup", config_file.file_path, config_file.file_path)]
                    } else {
                        vec![]
                    },
                    files_to_modify: vec![
                        FileModification {
                            file_path: config_file.file_path.clone(),
                            modification_type: ModificationType::Create,
                            content: config_file.content.clone(),
                            backup_required: config_file.backup_original,
                        },
                    ],
                    verification_commands: if let Some(validation) = &config_file.validation_command {
                        vec![validation.clone()]
                    } else {
                        vec![format!("test -f {}", config_file.file_path)]
                    },
                },
                expected_outcome: format!("Proper configuration for {}", name),
                risk_assessment: RiskAssessment {
                    risk_level: RiskLevel::Low,
                    potential_issues: vec![
                        "Configuration syntax errors".to_string(),
                        "Service restart required".to_string(),
                    ],
                    rollback_instructions: if config_file.backup_original {
                        vec![format!("cp {}.backup {}", config_file.file_path, config_file.file_path)]
                    } else {
                        vec![format!("Remove or edit {}", config_file.file_path)]
                    },
                    compatibility_notes: vec![],
                },
            });
        }

        Ok(recommendations)
    }

    fn analyze_potential_issues(&self, configuration: &Configuration, dkms_modules: &[DkmsModule]) -> Result<Vec<String>, LxHwError> {
        let mut warnings = Vec::new();

        // Check for NVIDIA-specific warnings
        for driver_rec in &configuration.driver_recommendations {
            if driver_rec.recommended_driver.contains("nvidia") {
                warnings.push("NVIDIA proprietary drivers may conflict with nouveau. Ensure nouveau is blacklisted.".to_string());
                warnings.push("Secure boot may need to be disabled for NVIDIA drivers to work properly.".to_string());
            }
        }

        // Check for DKMS warnings
        if !dkms_modules.is_empty() {
            warnings.push("DKMS modules require kernel headers and build tools. Ensure these are installed.".to_string());
            warnings.push("DKMS modules may need manual rebuilding after major kernel updates.".to_string());
        }

        // Check for compatibility score warnings
        if configuration.compatibility_score < 70.0 {
            warnings.push(format!(
                "Low compatibility score ({:.1}%). Some hardware may not work properly.",
                configuration.compatibility_score
            ));
        }

        Ok(warnings)
    }

    fn generate_performance_notes(&self, configuration: &Configuration) -> Result<Vec<String>, LxHwError> {
        let mut notes = Vec::new();

        // CPU performance notes
        if let Some(cpu) = &configuration.hardware_profile.cpu {
            if cpu.power_management.turbo_boost {
                notes.push("Turbo boost is enabled for better single-threaded performance".to_string());
            }
            
            notes.push(format!("Using {} CPU governor for optimal performance", cpu.power_management.cpu_governor));
        }

        // GPU performance notes
        for gpu in &configuration.hardware_profile.gpu {
            notes.push(format!("GPU performance profile set to: {}", gpu.performance_profile));
        }

        // Storage performance notes
        for storage in &configuration.hardware_profile.storage {
            if storage.device_type == "ssd" || storage.device_type == "nvme" {
                notes.push(format!("{} optimizations applied for better performance", storage.device_type.to_uppercase()));
            }
        }

        Ok(notes)
    }

    fn generate_installation_script(&self, recommendations: &[Recommendation], target_distribution: &str) -> Result<String, LxHwError> {
        let mut script = String::new();
        
        script.push_str("#!/bin/bash\n");
        script.push_str("# Hardware Configuration Installation Script\n");
        script.push_str(&format!("# Generated for: {}\n", target_distribution));
        script.push_str("# Generated by: lx-hw-db Configuration Engine\n\n");
        script.push_str("set -e\n\n");

        script.push_str("echo \"Starting hardware configuration installation...\"\n\n");

        // Group recommendations by category for logical execution order
        let mut categorized: HashMap<RecommendationCategory, Vec<&Recommendation>> = HashMap::new();
        
        for rec in recommendations {
            categorized.entry(rec.category.clone()).or_insert_with(Vec::new).push(rec);
        }

        // Execute in logical order
        let execution_order = vec![
            RecommendationCategory::PackageInstallation,
            RecommendationCategory::DkmsModule,
            RecommendationCategory::DriverInstallation,
            RecommendationCategory::ConfigurationFile,
            RecommendationCategory::KernelParameters,
            RecommendationCategory::PerformanceOptimization,
        ];

        for category in execution_order {
            if let Some(recommendations) = categorized.get(&category) {
                script.push_str(&format!("# {:?} recommendations\n", category));
                
                for rec in recommendations {
                    script.push_str(&format!("echo \"Implementing: {}\"\n", rec.title));
                    
                    for command in &rec.implementation.commands {
                        if !command.starts_with('#') {
                            script.push_str(&format!("{}\n", command));
                        } else {
                            script.push_str(&format!("{}\n", command));
                        }
                    }
                    
                    script.push_str("\n");
                }
            }
        }

        script.push_str("echo \"Hardware configuration installation completed!\"\n");
        script.push_str("echo \"Please reboot your system to ensure all changes take effect.\"\n");

        Ok(script)
    }

    fn generate_configuration_files_content(&self, config_files: &HashMap<String, ConfigurationFile>) -> Result<HashMap<String, String>, LxHwError> {
        let mut content_map = HashMap::new();
        
        for (name, config_file) in config_files {
            content_map.insert(format!("{} ({})", name, config_file.file_path), config_file.content.clone());
        }
        
        Ok(content_map)
    }
}