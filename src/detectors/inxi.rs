//! inxi hardware detection implementation

use super::{DetectionData, DetectionResult, HardwareDetector};
use crate::errors::{LxHwError, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Output;
use std::time::Duration;

/// Complete system information from inxi
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InxiData {
    pub system: Option<InxiSystem>,
    pub machine: Option<InxiMachine>,
    pub cpu: Option<InxiCpu>,
    pub graphics: Option<InxiGraphics>,
    pub audio: Option<InxiAudio>,
    pub network: Option<InxiNetwork>,
    pub bluetooth: Option<InxiBluetooth>,
    pub drives: Option<InxiDrives>,
    pub memory: Option<InxiMemory>,
    pub sensors: Option<InxiSensors>,
    pub summary: Option<InxiSummary>,
}

/// System information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiSystem {
    pub kernel: Option<String>,
    pub arch: Option<String>,
    pub bits: Option<String>,
    pub desktop: Option<String>,
    pub desktop_version: Option<String>,
    pub distro: Option<String>,
}

/// Machine information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiMachine {
    pub machine_type: Option<String>,
    pub system: Option<String>,
    pub product: Option<String>,
    pub version: Option<String>,
    pub serial: Option<String>,
    pub mobo: Option<String>,
    pub model: Option<String>,
    pub mobo_version: Option<String>,
    pub mobo_serial: Option<String>,
    pub uefi: Option<String>,
    pub uefi_version: Option<String>,
    pub uefi_date: Option<String>,
}

/// CPU information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiCpu {
    pub info: Option<String>,
    pub model: Option<String>,
    pub bits: Option<String>,
    pub cpu_type: Option<String>,
    pub cache: Option<String>,
    pub speed_avg: Option<String>,
    pub speed_min_max: Option<String>,
    pub cores: Vec<InxiCpuCore>,
}

/// Individual CPU core information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiCpuCore {
    pub core_id: u8,
    pub speed: String,
}

/// Graphics information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiGraphics {
    pub devices: Vec<InxiGpuDevice>,
    pub display: Option<InxiDisplay>,
    pub apis: Vec<InxiGraphicsApi>,
}

/// Graphics device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiGpuDevice {
    pub device_id: String,
    pub name: String,
    pub driver: Option<String>,
    pub driver_version: Option<String>,
    pub device_type: Option<String>, // GPU or USB camera
}

/// Display information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiDisplay {
    pub server: Option<String>,
    pub server_version: Option<String>,
    pub compositor: Option<String>,
    pub compositor_version: Option<String>,
    pub driver: Option<String>,
    pub gpu: Option<String>,
    pub resolution: Vec<String>,
}

/// Graphics API information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiGraphicsApi {
    pub api: String,
    pub version: Option<String>,
    pub compat_version: Option<String>,
    pub vendor: Option<String>,
    pub mesa_version: Option<String>,
    pub renderer: Option<String>,
}

/// Audio information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiAudio {
    pub devices: Vec<InxiAudioDevice>,
    pub server: Option<InxiAudioServer>,
}

/// Audio device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiAudioDevice {
    pub device_id: String,
    pub name: String,
    pub driver: Option<String>,
    pub device_type: Option<String>,
}

/// Audio server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiAudioServer {
    pub name: String,
    pub version: Option<String>,
    pub status: Option<String>,
}

/// Network information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiNetwork {
    pub devices: Vec<InxiNetworkDevice>,
    pub interfaces: Vec<InxiNetworkInterface>,
}

/// Network device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiNetworkDevice {
    pub device_id: String,
    pub name: String,
    pub driver: Option<String>,
}

/// Network interface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiNetworkInterface {
    pub name: String,
    pub state: Option<String>,
    pub speed: Option<String>,
    pub duplex: Option<String>,
    pub mac: Option<String>,
}

/// Bluetooth information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiBluetooth {
    pub device: Option<String>,
    pub driver: Option<String>,
    pub device_type: Option<String>,
    pub report: Option<String>,
    pub id: Option<String>,
    pub state: Option<String>,
    pub bt_version: Option<String>,
}

/// Storage drives information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiDrives {
    pub local_storage_total: Option<String>,
    pub local_storage_used: Option<String>,
    pub usage_percent: Option<String>,
    pub drives: Vec<InxiDrive>,
    pub partitions: Vec<InxiPartition>,
    pub swap: Vec<InxiSwap>,
}

/// Individual drive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiDrive {
    pub id: String,
    pub device: String,
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub size: String,
}

/// Partition information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiPartition {
    pub id: String,
    pub mount_point: String,
    pub size: String,
    pub used: String,
    pub usage_percent: String,
    pub filesystem: String,
    pub device: String,
}

/// Swap information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiSwap {
    pub id: String,
    pub swap_type: String,
    pub size: String,
    pub used: String,
    pub usage_percent: String,
    pub device: String,
}

/// Memory information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiMemory {
    pub total: Option<String>,
    pub available: Option<String>,
    pub used: Option<String>,
    pub usage_percent: Option<String>,
}

/// Sensors information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiSensors {
    pub cpu_temp: Option<String>,
    pub mobo_temp: Option<String>,
    pub gpu_temps: Vec<InxiGpuTemp>,
    pub fan_speeds: Vec<InxiFanSpeed>,
}

/// GPU temperature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiGpuTemp {
    pub gpu: String,
    pub temp: String,
}

/// Fan speed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiFanSpeed {
    pub fan: String,
    pub rpm: Option<String>,
}

/// Summary statistics from inxi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InxiSummary {
    pub sections_parsed: usize,
    pub inxi_version: Option<String>,
    pub uptime: Option<String>,
    pub processes: Option<String>,
    pub privileged_execution: bool,
    pub warnings: Vec<String>,
}

pub struct InxiDetector;

impl Default for InxiDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl InxiDetector {
    pub fn new() -> Self {
        Self
    }

    /// Parse inxi output into structured data
    pub fn parse_inxi_output(&self, output: &str) -> Result<InxiData> {
        let mut data = InxiData::default();
        let mut current_section = String::new();
        let mut section_content = Vec::new();

        for line in output.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Check if this is a new section (starts without indent)
            if !line.starts_with(' ') && line.contains(':') {
                // Process previous section
                if !current_section.is_empty() && !section_content.is_empty() {
                    self.parse_section(&mut data, &current_section, &section_content)?;
                }

                // Start new section
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                current_section = parts[0].trim().to_string();
                section_content.clear();
                if parts.len() > 1 {
                    section_content.push(parts[1].trim().to_string());
                }
            } else {
                // Continuation of current section
                section_content.push(trimmed.to_string());
            }
        }

        // Process final section
        if !current_section.is_empty() && !section_content.is_empty() {
            self.parse_section(&mut data, &current_section, &section_content)?;
        }

        Ok(data)
    }

    /// Parse individual section content
    fn parse_section(&self, data: &mut InxiData, section: &str, content: &[String]) -> Result<()> {
        match section {
            "System" => {
                data.system = Some(self.parse_system_section(content)?);
            }
            "Machine" => {
                data.machine = Some(self.parse_machine_section(content)?);
            }
            "CPU" => {
                data.cpu = Some(self.parse_cpu_section(content)?);
            }
            "Graphics" => {
                data.graphics = Some(self.parse_graphics_section(content)?);
            }
            "Audio" => {
                data.audio = Some(self.parse_audio_section(content)?);
            }
            "Network" => {
                data.network = Some(self.parse_network_section(content)?);
            }
            "Bluetooth" => {
                data.bluetooth = Some(self.parse_bluetooth_section(content)?);
            }
            "Drives" => {
                data.drives = Some(self.parse_drives_section(content)?);
            }
            "Memory" => {
                data.memory = Some(self.parse_memory_section(content)?);
            }
            "Sensors" => {
                data.sensors = Some(self.parse_sensors_section(content)?);
            }
            "Info" => {
                // This often contains memory and system info
                if let Some(memory) = self.parse_info_memory(content)? {
                    data.memory = Some(memory);
                }
            }
            _ => {
                // Unknown section, skip
            }
        }
        Ok(())
    }

    /// Parse key-value pairs from section content
    fn parse_key_values(&self, content: &[String]) -> HashMap<String, String> {
        let mut kv = HashMap::new();
        let full_text = content.join(" ");

        // Use regex-like approach to extract specific patterns
        self.extract_specific_patterns(&mut kv, &full_text);

        kv
    }

    /// Extract specific known patterns from inxi output
    #[allow(clippy::excessive_nesting)]
    fn extract_specific_patterns(&self, kv: &mut HashMap<String, String>, text: &str) {
        // Pattern: "Kernel 6.16.0" or "12Kernel 6.16.0"
        if let Some(start) = text.find("Kernel ") {
            let after_kernel = &text[start + 7..];
            if let Some(end) = after_kernel.find(' ') {
                let kernel_version = after_kernel[..end].trim();
                if !kernel_version.is_empty() && kernel_version.contains('.') {
                    kv.insert("Kernel".to_string(), kernel_version.to_string());
                }
            }
        }

        // Pattern: "arch x86_64"
        if let Some(start) = text.find("arch ") {
            let after_arch = &text[start + 5..];
            if let Some(end) = after_arch.find(' ') {
                let arch = after_arch[..end].trim();
                if !arch.is_empty() {
                    kv.insert("arch".to_string(), arch.to_string());
                }
            }
        }

        // Pattern: "bits 64"
        if let Some(start) = text.find("bits ") {
            let after_bits = &text[start + 5..];
            if let Some(end) = after_bits.find(' ') {
                let bits = after_bits[..end].trim();
                if !bits.is_empty() && bits.chars().all(char::is_numeric) {
                    kv.insert("bits".to_string(), bits.to_string());
                }
            }
        }

        // Pattern: "Desktop Hyprland"
        if let Some(start) = text.find("Desktop ") {
            let after_desktop = &text[start + 8..];
            if let Some(end) = after_desktop.find(' ') {
                let desktop = after_desktop[..end].trim();
                if !desktop.is_empty() {
                    kv.insert("Desktop".to_string(), desktop.to_string());
                }
            }
        }

        // Pattern: "Distro NixOS 25.11 (Xantusia)"
        if let Some(start) = text.find("Distro ") {
            let after_distro = &text[start + 7..];
            // Look for patterns like "NixOS 25.11 (Xantusia)" or similar
            if let Some(nixos_start) = after_distro.find("NixOS") {
                let from_nixos = &after_distro[nixos_start..];
                if let Some(paren_end) = from_nixos.find(')') {
                    let distro = from_nixos[..paren_end + 1].trim();
                    kv.insert("Distro".to_string(), distro.to_string());
                }
            }
        }

        // Pattern: "Type Desktop"
        if let Some(start) = text.find("Type ") {
            let after_type = &text[start + 5..];
            if let Some(end) = after_type.find(' ') {
                let machine_type = after_type[..end].trim();
                if !machine_type.is_empty() {
                    kv.insert("Type".to_string(), machine_type.to_string());
                }
            }
        }

        // Pattern: "System LENOVO"
        if let Some(start) = text.find("System ") {
            let after_system = &text[start + 7..];
            if let Some(end) = after_system.find(' ') {
                let system = after_system[..end].trim();
                if !system.is_empty() {
                    kv.insert("System".to_string(), system.to_string());
                }
            }
        }

        // Pattern: "product 30E1S6620H"
        if let Some(start) = text.find("product ") {
            let after_product = &text[start + 8..];
            if let Some(end) = after_product.find(' ') {
                let product = after_product[..end].trim();
                if !product.is_empty() {
                    kv.insert("product".to_string(), product.to_string());
                }
            }
        }

        // Pattern: "model AMD Ryzen Threadripper PRO 3995WX"
        if let Some(start) = text.find("model ") {
            let after_model = &text[start + 6..];
            // Find the next key to know where the model name ends
            let end_pos = after_model
                .find(" bits ")
                .or_else(|| after_model.find(" type "))
                .or_else(|| after_model.find(" cache "))
                .unwrap_or(after_model.len().min(100));

            let model = after_model[..end_pos].trim();
            if !model.is_empty() {
                kv.insert("model".to_string(), model.to_string());
            }
        }

        // Pattern: "Info 64-core"
        if let Some(start) = text.find("Info ") {
            let after_info = &text[start + 5..];
            if let Some(end) = after_info.find(' ') {
                let info = after_info[..end].trim();
                if !info.is_empty() {
                    kv.insert("Info".to_string(), info.to_string());
                }
            }
        }

        // Memory patterns from Info section
        if let Some(start) = text.find("Memory ") {
            let after_memory = &text[start + 7..];

            // Pattern: "total 224 GiB"
            if let Some(total_start) = after_memory.find("total ") {
                let after_total = &after_memory[total_start + 6..];
                if let Some(available_pos) = after_total.find(" available") {
                    let total = after_total[..available_pos].trim();
                    if !total.is_empty() {
                        kv.insert("total".to_string(), total.to_string());
                    }
                }
            }

            // Pattern: "available 219.99 GiB"
            if let Some(available_start) = after_memory.find("available ") {
                let after_available = &after_memory[available_start + 10..];
                if let Some(used_pos) = after_available.find(" used") {
                    let available = after_available[..used_pos].trim();
                    if !available.is_empty() {
                        kv.insert("available".to_string(), available.to_string());
                    }
                }
            }

            // Pattern: "used 53.94 GiB"
            if let Some(used_start) = after_memory.find("used ") {
                let after_used = &after_memory[used_start + 5..];
                // Take the next two words (value and unit)
                let used_parts: Vec<&str> = after_used.split_whitespace().take(2).collect();
                if used_parts.len() >= 2 {
                    let used = format!("{} {}", used_parts[0], used_parts[1]);
                    kv.insert("used".to_string(), used);
                } else if used_parts.len() == 1 && !used_parts[0].is_empty() {
                    kv.insert("used".to_string(), used_parts[0].to_string());
                }
            }
        }

        // Bluetooth patterns
        if let Some(start) = text.find("Device-1 ") {
            let after_device = &text[start + 9..];
            if let Some(driver_pos) = after_device.find(" driver") {
                let device_name = after_device[..driver_pos].trim();
                if !device_name.is_empty() {
                    kv.insert("Device-1".to_string(), device_name.to_string());
                }
            }
        }

        if let Some(start) = text.find("driver ") {
            let after_driver = &text[start + 7..];
            if let Some(end) = after_driver.find(' ') {
                let driver = after_driver[..end].trim();
                if !driver.is_empty() {
                    kv.insert("driver".to_string(), driver.to_string());
                }
            }
        }

        if let Some(start) = text.find("type ") {
            let after_type = &text[start + 5..];
            if let Some(end) = after_type.find(' ') {
                let device_type = after_type[..end].trim();
                if !device_type.is_empty() {
                    kv.insert("type".to_string(), device_type.to_string());
                }
            }
        }
    }

    /// Parse system section
    fn parse_system_section(&self, content: &[String]) -> Result<InxiSystem> {
        let kv = self.parse_key_values(content);
        Ok(InxiSystem {
            kernel: kv.get("Kernel").cloned(),
            arch: kv.get("arch").cloned(),
            bits: kv.get("bits").cloned(),
            desktop: kv.get("Desktop").cloned(),
            desktop_version: kv.get("v").cloned(),
            distro: kv.get("Distro").cloned(),
        })
    }

    /// Parse machine section
    fn parse_machine_section(&self, content: &[String]) -> Result<InxiMachine> {
        let kv = self.parse_key_values(content);
        Ok(InxiMachine {
            machine_type: kv.get("Type").cloned(),
            system: kv.get("System").cloned(),
            product: kv.get("product").cloned(),
            version: kv.get("v").cloned(),
            serial: kv.get("serial").cloned(),
            mobo: kv.get("Mobo").cloned(),
            model: kv.get("model").cloned(),
            mobo_version: kv.get("v").cloned(),
            mobo_serial: kv.get("serial").cloned(),
            uefi: kv.get("UEFI").cloned(),
            uefi_version: kv.get("v").cloned(),
            uefi_date: kv.get("date").cloned(),
        })
    }

    /// Parse CPU section
    fn parse_cpu_section(&self, content: &[String]) -> Result<InxiCpu> {
        let kv = self.parse_key_values(content);
        Ok(InxiCpu {
            info: kv.get("Info").cloned(),
            model: kv.get("model").cloned(),
            bits: kv.get("bits").cloned(),
            cpu_type: kv.get("type").cloned(),
            cache: kv.get("cache").cloned(),
            speed_avg: kv.get("avg").cloned(),
            speed_min_max: kv.get("min/max").cloned(),
            cores: Vec::new(), // TODO: Parse individual core data
        })
    }

    /// Parse graphics section
    fn parse_graphics_section(&self, _content: &[String]) -> Result<InxiGraphics> {
        Ok(InxiGraphics { devices: Vec::new(), display: None, apis: Vec::new() })
    }

    /// Parse audio section
    fn parse_audio_section(&self, _content: &[String]) -> Result<InxiAudio> {
        Ok(InxiAudio { devices: Vec::new(), server: None })
    }

    /// Parse network section
    fn parse_network_section(&self, _content: &[String]) -> Result<InxiNetwork> {
        Ok(InxiNetwork { devices: Vec::new(), interfaces: Vec::new() })
    }

    /// Parse bluetooth section
    fn parse_bluetooth_section(&self, content: &[String]) -> Result<InxiBluetooth> {
        let kv = self.parse_key_values(content);
        Ok(InxiBluetooth {
            device: kv.get("Device-1").cloned(),
            driver: kv.get("driver").cloned(),
            device_type: kv.get("type").cloned(),
            report: kv.get("Report").cloned(),
            id: kv.get("ID").cloned(),
            state: kv.get("state").cloned(),
            bt_version: kv.get("bt-v").cloned(),
        })
    }

    /// Parse drives section
    fn parse_drives_section(&self, _content: &[String]) -> Result<InxiDrives> {
        Ok(InxiDrives {
            local_storage_total: None,
            local_storage_used: None,
            usage_percent: None,
            drives: Vec::new(),
            partitions: Vec::new(),
            swap: Vec::new(),
        })
    }

    /// Parse memory from info section
    fn parse_info_memory(&self, content: &[String]) -> Result<Option<InxiMemory>> {
        let kv = self.parse_key_values(content);
        if kv.contains_key("Memory") || kv.contains_key("total") {
            Ok(Some(InxiMemory {
                total: kv.get("total").cloned(),
                available: kv.get("available").cloned(),
                used: kv.get("used").cloned(),
                usage_percent: None,
            }))
        } else {
            Ok(None)
        }
    }

    /// Parse memory section
    fn parse_memory_section(&self, content: &[String]) -> Result<InxiMemory> {
        let kv = self.parse_key_values(content);
        Ok(InxiMemory {
            total: kv.get("total").cloned(),
            available: kv.get("available").cloned(),
            used: kv.get("used").cloned(),
            usage_percent: None,
        })
    }

    /// Parse sensors section
    fn parse_sensors_section(&self, _content: &[String]) -> Result<InxiSensors> {
        Ok(InxiSensors {
            cpu_temp: None,
            mobo_temp: None,
            gpu_temps: Vec::new(),
            fan_speeds: Vec::new(),
        })
    }

    /// Generate summary statistics
    fn generate_summary(
        &self,
        data: &InxiData,
        privileged: bool,
        warnings: Vec<String>,
    ) -> InxiSummary {
        let mut sections_parsed = 0;

        if data.system.is_some() {
            sections_parsed += 1;
        }
        if data.machine.is_some() {
            sections_parsed += 1;
        }
        if data.cpu.is_some() {
            sections_parsed += 1;
        }
        if data.graphics.is_some() {
            sections_parsed += 1;
        }
        if data.audio.is_some() {
            sections_parsed += 1;
        }
        if data.network.is_some() {
            sections_parsed += 1;
        }
        if data.bluetooth.is_some() {
            sections_parsed += 1;
        }
        if data.drives.is_some() {
            sections_parsed += 1;
        }
        if data.memory.is_some() {
            sections_parsed += 1;
        }
        if data.sensors.is_some() {
            sections_parsed += 1;
        }

        InxiSummary {
            sections_parsed,
            inxi_version: None,
            uptime: None,
            processes: None,
            privileged_execution: privileged,
            warnings,
        }
    }
}

#[async_trait]
impl HardwareDetector for InxiDetector {
    fn name(&self) -> &'static str {
        "inxi"
    }

    async fn is_available(&self) -> bool {
        tokio::process::Command::new("which")
            .arg("inxi")
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    async fn execute(&self) -> Result<Output> {
        let output = tokio::process::Command::new("inxi")
            .arg("-F")  // Full system info
            .output()
            .await
            .map_err(|e| LxHwError::SystemCommandError {
                command: format!("inxi: {}", e)
            })?;

        Ok(output)
    }

    fn timeout(&self) -> Duration {
        Duration::from_secs(20)
    }

    fn parse_output(&self, output: &Output) -> Result<DetectionResult> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check for execution errors
        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Ok(DetectionResult {
                tool_name: self.name().to_string(),
                success: false,
                data: DetectionData::Inxi(Box::default()),
                errors: vec![format!("inxi execution failed: {}", error_msg)],
            });
        }

        // Handle empty output
        if output.stdout.is_empty() {
            return Ok(DetectionResult {
                tool_name: self.name().to_string(),
                success: false,
                data: DetectionData::Inxi(Box::default()),
                errors: vec!["Empty output from inxi".to_string()],
            });
        }

        // Process stderr for warnings
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        if !stderr_str.is_empty() {
            for line in stderr_str.lines() {
                if !line.trim().is_empty() {
                    warnings.push(line.to_string());
                }
            }
        }

        // Parse inxi output
        let stdout_str = String::from_utf8_lossy(&output.stdout);
        let mut data = match self.parse_inxi_output(&stdout_str) {
            Ok(data) => data,
            Err(e) => {
                return Ok(DetectionResult {
                    tool_name: self.name().to_string(),
                    success: false,
                    data: DetectionData::Inxi(Box::default()),
                    errors: vec![format!("Failed to parse inxi output: {}", e)],
                });
            }
        };

        // Detect if we had privileged access
        let privileged =
            !warnings.iter().any(|w| w.contains("superuser required") || w.contains("permission"));

        let summary = self.generate_summary(&data, privileged, warnings.clone());
        data.summary = Some(summary);
        errors.extend(warnings);

        Ok(DetectionResult {
            tool_name: self.name().to_string(),
            success: true,
            data: DetectionData::Inxi(Box::new(data)),
            errors,
        })
    }
}
