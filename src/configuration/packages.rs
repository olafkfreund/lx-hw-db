use std::collections::HashMap;
use serde_json::Value;
use crate::configuration::*;
use crate::hardware::HardwareReport;
use crate::errors::LxHwError;

pub struct PackageMapper {
    distribution_packages: HashMap<String, DistributionPackageMap>,
    hardware_package_mappings: HashMap<String, Vec<HardwarePackageMapping>>,
    firmware_packages: HashMap<String, FirmwarePackageInfo>,
}

#[derive(Debug, Clone)]
pub struct DistributionPackageMap {
    pub distribution_name: String,
    pub package_manager: PackageManager,
    pub install_command: String,
    pub update_command: String,
    pub search_command: String,
    pub repositories: Vec<PackageRepository>,
    pub package_mappings: HashMap<String, String>, // generic_name -> distribution_specific_name
}

#[derive(Debug, Clone)]
pub struct PackageRepository {
    pub name: String,
    pub url: String,
    pub enabled_by_default: bool,
    pub setup_command: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HardwarePackageMapping {
    pub hardware_pattern: HardwarePackagePattern,
    pub required_packages: Vec<String>,
    pub optional_packages: Vec<String>,
    pub firmware_packages: Vec<String>,
    pub configuration_packages: Vec<String>,
    pub post_install_commands: Vec<String>,
    pub package_priority: u8,
}

#[derive(Debug, Clone)]
pub struct HardwarePackagePattern {
    pub component_type: String,
    pub vendor_pattern: Option<String>,
    pub device_pattern: Option<String>,
    pub driver_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FirmwarePackageInfo {
    pub firmware_name: String,
    pub package_mappings: HashMap<String, String>, // distribution -> package_name
    pub firmware_files: Vec<String>,
    pub installation_path: String,
    pub license: String,
    pub description: String,
}

impl PackageMapper {
    pub fn new() -> Result<Self, LxHwError> {
        let mut mapper = Self {
            distribution_packages: HashMap::new(),
            hardware_package_mappings: HashMap::new(),
            firmware_packages: HashMap::new(),
        };

        mapper.initialize_distribution_packages()?;
        mapper.initialize_hardware_mappings()?;
        mapper.initialize_firmware_packages()?;

        Ok(mapper)
    }

    pub fn map_packages(&self, hardware: &HardwareReport, distribution: &str) -> Result<Vec<PackageInstallation>, LxHwError> {
        let mut installations = Vec::new();

        // Get distribution-specific package map
        let dist_map = self.distribution_packages.get(distribution)
            .or_else(|| {
                // Try case-insensitive lookup
                let dist_lower = distribution.to_lowercase();
                self.distribution_packages.iter()
                    .find(|(k, _)| k.to_lowercase().contains(&dist_lower) || dist_lower.contains(&k.to_lowercase()))
                    .map(|(_, v)| v)
            });

        if dist_map.is_none() {
            return Ok(installations); // No package mappings available for this distribution
        }

        let dist_map = dist_map.unwrap();

        // Map packages for CPU
        if let Some(cpu) = &hardware.cpu {
            installations.extend(self.map_cpu_packages_from_device(cpu, dist_map)?);
        }

        // Map packages for graphics
        for gpu in &hardware.graphics {
            installations.extend(self.map_gpu_packages_from_device(gpu, dist_map)?);
        }

        // Map packages for network devices
        for network in &hardware.network {
            installations.extend(self.map_network_packages_from_device(network, dist_map)?);
        }

        // Map packages for audio devices
        for audio in &hardware.audio {
            installations.extend(self.map_audio_packages_from_device(audio, dist_map)?);
        }

        // Map general system packages
        installations.extend(self.map_system_packages(hardware, dist_map)?);

        Ok(installations)
    }

    fn initialize_distribution_packages(&mut self) -> Result<(), LxHwError> {
        // Ubuntu/Debian (APT-based)
        let mut ubuntu_packages = HashMap::new();
        ubuntu_packages.insert("intel-microcode".to_string(), "intel-microcode".to_string());
        ubuntu_packages.insert("amd-microcode".to_string(), "amd64-microcode".to_string());
        ubuntu_packages.insert("nvidia-driver".to_string(), "nvidia-driver-525".to_string());
        ubuntu_packages.insert("mesa-drivers".to_string(), "mesa-vulkan-drivers".to_string());
        ubuntu_packages.insert("firmware-linux".to_string(), "linux-firmware".to_string());
        ubuntu_packages.insert("firmware-iwlwifi".to_string(), "linux-firmware".to_string());
        ubuntu_packages.insert("pulseaudio".to_string(), "pulseaudio".to_string());
        ubuntu_packages.insert("pipewire".to_string(), "pipewire".to_string());

        self.distribution_packages.insert("Ubuntu".to_string(), DistributionPackageMap {
            distribution_name: "Ubuntu".to_string(),
            package_manager: PackageManager::Apt,
            install_command: "apt update && apt install -y".to_string(),
            update_command: "apt update && apt upgrade -y".to_string(),
            search_command: "apt search".to_string(),
            repositories: vec![
                PackageRepository {
                    name: "universe".to_string(),
                    url: "http://archive.ubuntu.com/ubuntu".to_string(),
                    enabled_by_default: true,
                    setup_command: Some("add-apt-repository universe".to_string()),
                },
                PackageRepository {
                    name: "multiverse".to_string(),
                    url: "http://archive.ubuntu.com/ubuntu".to_string(),
                    enabled_by_default: false,
                    setup_command: Some("add-apt-repository multiverse".to_string()),
                },
            ],
            package_mappings: ubuntu_packages,
        });

        // Fedora (DNF-based)
        let mut fedora_packages = HashMap::new();
        fedora_packages.insert("intel-microcode".to_string(), "microcode_ctl".to_string());
        fedora_packages.insert("amd-microcode".to_string(), "microcode_ctl".to_string());
        fedora_packages.insert("nvidia-driver".to_string(), "akmod-nvidia".to_string());
        fedora_packages.insert("mesa-drivers".to_string(), "mesa-vulkan-drivers".to_string());
        fedora_packages.insert("firmware-linux".to_string(), "linux-firmware".to_string());
        fedora_packages.insert("firmware-iwlwifi".to_string(), "iwl*-firmware".to_string());
        fedora_packages.insert("pulseaudio".to_string(), "pulseaudio".to_string());
        fedora_packages.insert("pipewire".to_string(), "pipewire".to_string());

        self.distribution_packages.insert("Fedora".to_string(), DistributionPackageMap {
            distribution_name: "Fedora".to_string(),
            package_manager: PackageManager::Dnf,
            install_command: "dnf install -y".to_string(),
            update_command: "dnf update -y".to_string(),
            search_command: "dnf search".to_string(),
            repositories: vec![
                PackageRepository {
                    name: "rpmfusion-free".to_string(),
                    url: "https://download1.rpmfusion.org/free/fedora/".to_string(),
                    enabled_by_default: false,
                    setup_command: Some("dnf install -y https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm".to_string()),
                },
                PackageRepository {
                    name: "rpmfusion-nonfree".to_string(),
                    url: "https://download1.rpmfusion.org/nonfree/fedora/".to_string(),
                    enabled_by_default: false,
                    setup_command: Some("dnf install -y https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm".to_string()),
                },
            ],
            package_mappings: fedora_packages,
        });

        // Arch Linux (Pacman-based)
        let mut arch_packages = HashMap::new();
        arch_packages.insert("intel-microcode".to_string(), "intel-ucode".to_string());
        arch_packages.insert("amd-microcode".to_string(), "amd-ucode".to_string());
        arch_packages.insert("nvidia-driver".to_string(), "nvidia nvidia-settings".to_string());
        arch_packages.insert("mesa-drivers".to_string(), "mesa vulkan-radeon vulkan-intel".to_string());
        arch_packages.insert("firmware-linux".to_string(), "linux-firmware".to_string());
        arch_packages.insert("firmware-iwlwifi".to_string(), "linux-firmware".to_string());
        arch_packages.insert("pulseaudio".to_string(), "pulseaudio".to_string());
        arch_packages.insert("pipewire".to_string(), "pipewire".to_string());

        self.distribution_packages.insert("Arch Linux".to_string(), DistributionPackageMap {
            distribution_name: "Arch Linux".to_string(),
            package_manager: PackageManager::Pacman,
            install_command: "pacman -S --noconfirm".to_string(),
            update_command: "pacman -Syu --noconfirm".to_string(),
            search_command: "pacman -Ss".to_string(),
            repositories: vec![
                PackageRepository {
                    name: "multilib".to_string(),
                    url: "https://archlinux.org/packages/".to_string(),
                    enabled_by_default: false,
                    setup_command: Some("# Uncomment [multilib] section in /etc/pacman.conf".to_string()),
                },
            ],
            package_mappings: arch_packages,
        });

        // NixOS (Nix-based)
        let mut nixos_packages = HashMap::new();
        nixos_packages.insert("intel-microcode".to_string(), "intel-microcode".to_string());
        nixos_packages.insert("amd-microcode".to_string(), "amd-microcode".to_string());
        nixos_packages.insert("nvidia-driver".to_string(), "nvidia_x11".to_string());
        nixos_packages.insert("mesa-drivers".to_string(), "mesa".to_string());
        nixos_packages.insert("firmware-linux".to_string(), "linux-firmware".to_string());
        nixos_packages.insert("firmware-iwlwifi".to_string(), "linux-firmware".to_string());
        nixos_packages.insert("pulseaudio".to_string(), "pulseaudio".to_string());
        nixos_packages.insert("pipewire".to_string(), "pipewire".to_string());

        self.distribution_packages.insert("NixOS".to_string(), DistributionPackageMap {
            distribution_name: "NixOS".to_string(),
            package_manager: PackageManager::Nix,
            install_command: "nix-env -iA nixos.".to_string(),
            update_command: "nix-channel --update && nixos-rebuild switch".to_string(),
            search_command: "nix-env -qaP".to_string(),
            repositories: vec![
                PackageRepository {
                    name: "nixos-unstable".to_string(),
                    url: "https://nixos.org/channels/nixos-unstable".to_string(),
                    enabled_by_default: false,
                    setup_command: Some("nix-channel --add https://nixos.org/channels/nixos-unstable nixos-unstable".to_string()),
                },
            ],
            package_mappings: nixos_packages,
        });

        Ok(())
    }

    fn initialize_hardware_mappings(&mut self) -> Result<(), LxHwError> {
        // Intel CPU packages
        self.hardware_package_mappings.insert("intel_cpu".to_string(), vec![
            HardwarePackageMapping {
                hardware_pattern: HardwarePackagePattern {
                    component_type: "CPU".to_string(),
                    vendor_pattern: Some("Intel".to_string()),
                    device_pattern: None,
                    driver_name: None,
                },
                required_packages: vec!["intel-microcode".to_string()],
                optional_packages: vec!["thermald".to_string(), "intel-gpu-tools".to_string()],
                firmware_packages: vec!["intel-microcode".to_string()],
                configuration_packages: vec![],
                post_install_commands: vec![
                    "echo 'early-microcode' >> /etc/mkinitcpio.conf".to_string(),
                ],
                package_priority: 9,
            },
        ]);

        // AMD CPU packages
        self.hardware_package_mappings.insert("amd_cpu".to_string(), vec![
            HardwarePackageMapping {
                hardware_pattern: HardwarePackagePattern {
                    component_type: "CPU".to_string(),
                    vendor_pattern: Some("AMD".to_string()),
                    device_pattern: None,
                    driver_name: None,
                },
                required_packages: vec!["amd-microcode".to_string()],
                optional_packages: vec!["lm-sensors".to_string()],
                firmware_packages: vec!["amd-microcode".to_string()],
                configuration_packages: vec![],
                post_install_commands: vec![],
                package_priority: 9,
            },
        ]);

        // NVIDIA GPU packages
        self.hardware_package_mappings.insert("nvidia_gpu".to_string(), vec![
            HardwarePackageMapping {
                hardware_pattern: HardwarePackagePattern {
                    component_type: "GPU".to_string(),
                    vendor_pattern: Some("NVIDIA".to_string()),
                    device_pattern: None,
                    driver_name: Some("nvidia".to_string()),
                },
                required_packages: vec!["nvidia-driver".to_string()],
                optional_packages: vec!["nvidia-settings".to_string(), "nvidia-prime".to_string()],
                firmware_packages: vec![],
                configuration_packages: vec!["mesa-utils".to_string()],
                post_install_commands: vec![
                    "systemctl enable nvidia-persistenced".to_string(),
                    "nvidia-xconfig".to_string(),
                ],
                package_priority: 8,
            },
        ]);

        // AMD GPU packages
        self.hardware_package_mappings.insert("amd_gpu".to_string(), vec![
            HardwarePackageMapping {
                hardware_pattern: HardwarePackagePattern {
                    component_type: "GPU".to_string(),
                    vendor_pattern: Some("AMD".to_string()),
                    device_pattern: None,
                    driver_name: Some("amdgpu".to_string()),
                },
                required_packages: vec!["mesa-drivers".to_string()],
                optional_packages: vec!["radeontop".to_string(), "mesa-vdpau-drivers".to_string()],
                firmware_packages: vec!["firmware-linux".to_string()],
                configuration_packages: vec!["mesa-utils".to_string()],
                post_install_commands: vec![],
                package_priority: 8,
            },
        ]);

        // Intel wireless packages
        self.hardware_package_mappings.insert("intel_wifi".to_string(), vec![
            HardwarePackageMapping {
                hardware_pattern: HardwarePackagePattern {
                    component_type: "Network".to_string(),
                    vendor_pattern: Some("Intel".to_string()),
                    device_pattern: Some("Wireless".to_string()),
                    driver_name: Some("iwlwifi".to_string()),
                },
                required_packages: vec!["firmware-iwlwifi".to_string()],
                optional_packages: vec!["iw".to_string(), "wireless-tools".to_string()],
                firmware_packages: vec!["firmware-iwlwifi".to_string()],
                configuration_packages: vec!["network-manager".to_string()],
                post_install_commands: vec![
                    "systemctl enable NetworkManager".to_string(),
                ],
                package_priority: 7,
            },
        ]);

        Ok(())
    }

    fn initialize_firmware_packages(&mut self) -> Result<(), LxHwError> {
        // Intel microcode firmware
        let mut intel_microcode_packages = HashMap::new();
        intel_microcode_packages.insert("Ubuntu".to_string(), "intel-microcode".to_string());
        intel_microcode_packages.insert("Debian".to_string(), "intel-microcode".to_string());
        intel_microcode_packages.insert("Fedora".to_string(), "microcode_ctl".to_string());
        intel_microcode_packages.insert("Arch Linux".to_string(), "intel-ucode".to_string());
        intel_microcode_packages.insert("NixOS".to_string(), "intel-microcode".to_string());

        self.firmware_packages.insert("intel-microcode".to_string(), FirmwarePackageInfo {
            firmware_name: "Intel CPU Microcode".to_string(),
            package_mappings: intel_microcode_packages,
            firmware_files: vec![
                "/lib/firmware/intel-ucode/*".to_string(),
                "/boot/intel-ucode.img".to_string(),
            ],
            installation_path: "/lib/firmware/intel-ucode/".to_string(),
            license: "Intel Proprietary".to_string(),
            description: "Intel CPU microcode updates for security and stability".to_string(),
        });

        // Intel WiFi firmware
        let mut intel_wifi_packages = HashMap::new();
        intel_wifi_packages.insert("Ubuntu".to_string(), "linux-firmware".to_string());
        intel_wifi_packages.insert("Debian".to_string(), "firmware-iwlwifi".to_string());
        intel_wifi_packages.insert("Fedora".to_string(), "iwl*-firmware".to_string());
        intel_wifi_packages.insert("Arch Linux".to_string(), "linux-firmware".to_string());
        intel_wifi_packages.insert("NixOS".to_string(), "linux-firmware".to_string());

        self.firmware_packages.insert("intel-wifi".to_string(), FirmwarePackageInfo {
            firmware_name: "Intel WiFi Firmware".to_string(),
            package_mappings: intel_wifi_packages,
            firmware_files: vec![
                "/lib/firmware/iwlwifi-*".to_string(),
            ],
            installation_path: "/lib/firmware/".to_string(),
            license: "Intel Proprietary".to_string(),
            description: "Intel WiFi adapter firmware files".to_string(),
        });

        // AMD GPU firmware
        let mut amd_gpu_packages = HashMap::new();
        amd_gpu_packages.insert("Ubuntu".to_string(), "linux-firmware".to_string());
        amd_gpu_packages.insert("Debian".to_string(), "firmware-amd-graphics".to_string());
        amd_gpu_packages.insert("Fedora".to_string(), "amd-gpu-firmware".to_string());
        amd_gpu_packages.insert("Arch Linux".to_string(), "linux-firmware".to_string());
        amd_gpu_packages.insert("NixOS".to_string(), "linux-firmware".to_string());

        self.firmware_packages.insert("amd-gpu".to_string(), FirmwarePackageInfo {
            firmware_name: "AMD GPU Firmware".to_string(),
            package_mappings: amd_gpu_packages,
            firmware_files: vec![
                "/lib/firmware/amdgpu/*".to_string(),
                "/lib/firmware/radeon/*".to_string(),
            ],
            installation_path: "/lib/firmware/amdgpu/".to_string(),
            license: "AMD Proprietary".to_string(),
            description: "AMD GPU firmware for amdgpu driver".to_string(),
        });

        Ok(())
    }

    #[allow(dead_code)]
    fn map_cpu_packages(&self, cpu_info: &Value, dist_map: &DistributionPackageMap) -> Result<Vec<PackageInstallation>, LxHwError> {
        let mut installations = Vec::new();

        let vendor = cpu_info.get("vendor").and_then(|v| v.as_str()).unwrap_or("unknown");
        let vendor_lower = vendor.to_lowercase();

        if vendor_lower.contains("intel") {
            if let Some(mappings) = self.hardware_package_mappings.get("intel_cpu") {
                for mapping in mappings {
                    installations.push(self.create_package_installation(mapping, dist_map)?);
                }
            }
        } else if vendor_lower.contains("amd") {
            if let Some(mappings) = self.hardware_package_mappings.get("amd_cpu") {
                for mapping in mappings {
                    installations.push(self.create_package_installation(mapping, dist_map)?);
                }
            }
        }

        Ok(installations)
    }

    #[allow(dead_code)]
    fn map_gpu_packages(&self, gpu_info: &Value, dist_map: &DistributionPackageMap) -> Result<Vec<PackageInstallation>, LxHwError> {
        let mut installations = Vec::new();

        let vendor = gpu_info.get("vendor").and_then(|v| v.as_str()).unwrap_or("unknown");
        let vendor_lower = vendor.to_lowercase();

        if vendor_lower.contains("nvidia") {
            if let Some(mappings) = self.hardware_package_mappings.get("nvidia_gpu") {
                for mapping in mappings {
                    installations.push(self.create_package_installation(mapping, dist_map)?);
                }
            }
        } else if vendor_lower.contains("amd") || vendor_lower.contains("ati") {
            if let Some(mappings) = self.hardware_package_mappings.get("amd_gpu") {
                for mapping in mappings {
                    installations.push(self.create_package_installation(mapping, dist_map)?);
                }
            }
        }

        Ok(installations)
    }

    #[allow(dead_code)]
    fn map_network_packages(&self, network_info: &Value, dist_map: &DistributionPackageMap) -> Result<Vec<PackageInstallation>, LxHwError> {
        let mut installations = Vec::new();

        let vendor = network_info.get("vendor").and_then(|v| v.as_str()).unwrap_or("unknown");
        let product = network_info.get("product").and_then(|p| p.as_str()).unwrap_or("unknown");
        
        let vendor_lower = vendor.to_lowercase();
        let product_lower = product.to_lowercase();

        if vendor_lower.contains("intel") && (product_lower.contains("wireless") || product_lower.contains("wifi")) {
            if let Some(mappings) = self.hardware_package_mappings.get("intel_wifi") {
                for mapping in mappings {
                    installations.push(self.create_package_installation(mapping, dist_map)?);
                }
            }
        }

        Ok(installations)
    }

    #[allow(dead_code)]
    fn map_audio_packages(&self, _audio_info: &Value, dist_map: &DistributionPackageMap) -> Result<Vec<PackageInstallation>, LxHwError> {
        let mut installations = Vec::new();

        // Most modern Linux distributions use PipeWire or PulseAudio
        let audio_packages = vec!["pulseaudio".to_string(), "alsa-utils".to_string()];
        let mapped_packages = self.map_package_names(&audio_packages, dist_map);

        if !mapped_packages.is_empty() {
            installations.push(PackageInstallation {
                package_name: "pulseaudio-alsa-utils".to_string(),
                package_description: "Audio system packages for PulseAudio and ALSA".to_string(),
                package_category: PackageCategory::System,
                installation_reason: InstallationReason::HardwareSupport,
                installation_command: format!("{} {}", 
                    dist_map.install_command,
                    mapped_packages.join(" ")
                ),
                post_install_commands: vec![
                    "systemctl --user enable pulseaudio".to_string(),
                ],
                dependencies: vec![],
            });
        }

        Ok(installations)
    }

    fn map_system_packages(&self, _hardware: &HardwareReport, dist_map: &DistributionPackageMap) -> Result<Vec<PackageInstallation>, LxHwError> {
        let mut installations = Vec::new();

        // Essential system packages for hardware management
        let system_packages = vec![
            "lshw".to_string(),
            "pciutils".to_string(),
            "usbutils".to_string(),
            "dmidecode".to_string(),
        ];

        let mapped_packages = self.map_package_names(&system_packages, dist_map);

        if !mapped_packages.is_empty() {
            installations.push(PackageInstallation {
                package_name: "system-packages".to_string(),
                package_description: "Essential system packages".to_string(),
                package_category: PackageCategory::System,
                installation_reason: InstallationReason::HardwareSupport,
                installation_command: format!("{} {}", 
                    dist_map.install_command,
                    mapped_packages.join(" ")
                ),
                post_install_commands: vec![],
                dependencies: vec![],
            });
        }

        Ok(installations)
    }

    #[allow(dead_code)]
    fn create_package_installation(&self, mapping: &HardwarePackageMapping, dist_map: &DistributionPackageMap) -> Result<PackageInstallation, LxHwError> {
        let all_packages = [
            mapping.required_packages.clone(),
            mapping.optional_packages.clone(),
            mapping.firmware_packages.clone(),
            mapping.configuration_packages.clone(),
        ].concat();

        let mapped_packages = self.map_package_names(&all_packages, dist_map);

        Ok(PackageInstallation {
            package_name: "hardware-packages".to_string(),
            package_description: "Hardware-specific packages".to_string(),
            package_category: PackageCategory::Driver,
            installation_reason: InstallationReason::HardwareSupport,
            installation_command: format!("{} {}", 
                dist_map.install_command,
                mapped_packages.join(" ")
            ),
            post_install_commands: mapping.post_install_commands.clone(),
            dependencies: mapping.required_packages.clone(),
        })
    }

    fn map_package_names(&self, generic_packages: &[String], dist_map: &DistributionPackageMap) -> Vec<String> {
        generic_packages.iter()
            .filter_map(|pkg| {
                dist_map.package_mappings.get(pkg)
                    .map(|mapped| mapped.clone())
                    .or_else(|| Some(pkg.clone())) // Fallback to original name
            })
            .collect()
    }

    // New methods that work with structured hardware types
    fn map_cpu_packages_from_device(&self, cpu: &crate::hardware::CpuInfo, dist_map: &DistributionPackageMap) -> Result<Vec<PackageInstallation>, LxHwError> {
        let mut installations = Vec::new();
        let vendor = cpu.vendor.to_lowercase();

        // CPU microcode updates
        if vendor.contains("intel") {
            installations.push(PackageInstallation {
                package_name: "intel-microcode".to_string(),
                package_description: "Intel CPU microcode updates".to_string(),
                package_category: PackageCategory::Firmware,
                installation_reason: InstallationReason::HardwareSupport,
                installation_command: format!("{} intel-microcode", dist_map.install_command),
                post_install_commands: vec![],
                dependencies: vec![],
            });
        } else if vendor.contains("amd") {
            installations.push(PackageInstallation {
                package_name: "amd64-microcode".to_string(),
                package_description: "AMD CPU microcode updates".to_string(),
                package_category: PackageCategory::Firmware,
                installation_reason: InstallationReason::HardwareSupport,
                installation_command: format!("{} amd64-microcode", dist_map.install_command),
                post_install_commands: vec![],
                dependencies: vec![],
            });
        }

        Ok(installations)
    }

    fn map_gpu_packages_from_device(&self, gpu: &crate::hardware::GraphicsDevice, dist_map: &DistributionPackageMap) -> Result<Vec<PackageInstallation>, LxHwError> {
        let mut installations = Vec::new();
        let vendor = gpu.vendor.to_lowercase();

        if vendor.contains("nvidia") {
            installations.push(PackageInstallation {
                package_name: "nvidia-driver".to_string(),
                package_description: "NVIDIA proprietary graphics driver".to_string(),
                package_category: PackageCategory::Driver,
                installation_reason: InstallationReason::HardwareSupport,
                installation_command: format!("{} nvidia-driver", dist_map.install_command),
                post_install_commands: vec!["nvidia-xconfig".to_string()],
                dependencies: vec!["linux-headers".to_string()],
            });
        } else if vendor.contains("amd") {
            installations.push(PackageInstallation {
                package_name: "mesa-vulkan-drivers".to_string(),
                package_description: "Mesa Vulkan drivers for AMD graphics".to_string(),
                package_category: PackageCategory::Driver,
                installation_reason: InstallationReason::HardwareSupport,
                installation_command: format!("{} mesa-vulkan-drivers", dist_map.install_command),
                post_install_commands: vec![],
                dependencies: vec![],
            });
        }

        Ok(installations)
    }

    fn map_network_packages_from_device(&self, network: &crate::hardware::NetworkDevice, dist_map: &DistributionPackageMap) -> Result<Vec<PackageInstallation>, LxHwError> {
        let mut installations = Vec::new();
        let vendor = network.vendor.to_lowercase();
        let device_type = &network.device_type;

        if device_type == "wifi" || device_type == "wireless" {
            // Wireless firmware packages
            if vendor.contains("intel") {
                installations.push(PackageInstallation {
                    package_name: "firmware-iwlwifi".to_string(),
                    package_description: "Intel wireless firmware".to_string(),
                    package_category: PackageCategory::Firmware,
                    installation_reason: InstallationReason::HardwareSupport,
                    installation_command: format!("{} firmware-iwlwifi", dist_map.install_command),
                    post_install_commands: vec!["modprobe -r iwlwifi && modprobe iwlwifi".to_string()],
                    dependencies: vec![],
                });
            } else if vendor.contains("broadcom") {
                installations.push(PackageInstallation {
                    package_name: "firmware-brcm80211".to_string(),
                    package_description: "Broadcom wireless firmware".to_string(),
                    package_category: PackageCategory::Firmware,
                    installation_reason: InstallationReason::HardwareSupport,
                    installation_command: format!("{} firmware-brcm80211", dist_map.install_command),
                    post_install_commands: vec![],
                    dependencies: vec![],
                });
            }
        }

        Ok(installations)
    }

    fn map_audio_packages_from_device(&self, _audio: &crate::hardware::AudioDevice, dist_map: &DistributionPackageMap) -> Result<Vec<PackageInstallation>, LxHwError> {
        let mut installations = Vec::new();

        // Basic audio support
        installations.push(PackageInstallation {
            package_name: "alsa-utils".to_string(),
            package_description: "ALSA sound utilities".to_string(),
            package_category: PackageCategory::Utility,
            installation_reason: InstallationReason::HardwareSupport,
            installation_command: format!("{} alsa-utils", dist_map.install_command),
            post_install_commands: vec!["alsactl init".to_string()],
            dependencies: vec![],
        });

        Ok(installations)
    }
}