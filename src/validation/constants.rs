//! Constants for validation system
//!
//! This module contains all the constants used throughout the validation system
//! for better maintainability and performance.

/// Confidence score impact factors
pub mod confidence {
    /// Impact of schema validation errors on confidence (severe)
    pub const SCHEMA_ERROR_IMPACT: f64 = 0.5;

    /// Impact of business logic validation errors on confidence
    pub const BUSINESS_LOGIC_ERROR_IMPACT: f64 = 0.8;

    /// Impact of privacy validation errors on confidence (serious)
    pub const PRIVACY_ERROR_IMPACT: f64 = 0.7;

    /// Impact of consistency validation errors on confidence
    pub const CONSISTENCY_ERROR_IMPACT: f64 = 0.9;

    /// Impact of compatibility validation errors on confidence
    pub const COMPATIBILITY_ERROR_IMPACT: f64 = 0.85;

    /// Reduction per warning (maximum impact)
    pub const WARNING_IMPACT_PER_ITEM: f64 = 0.05;
}

/// Memory and storage size constants
pub mod sizes {
    /// Minimum memory size in bytes (128MB)
    pub const MIN_MEMORY_BYTES: u64 = 134_217_728;

    /// Maximum memory size in bytes (16TB)
    pub const MAX_MEMORY_BYTES: u64 = 17_592_186_044_416;

    /// Minimum storage size in bytes (1MB)
    pub const MIN_STORAGE_BYTES: u64 = 1_048_576;

    /// Minimum DIMM size in bytes (1MB)
    pub const MIN_DIMM_BYTES: u64 = 1_048_576;

    /// Bytes per GB for conversions
    pub const BYTES_PER_GB: u64 = 1_073_741_824;

    /// Bytes per MB for conversions
    pub const BYTES_PER_MB: u64 = 1_048_576;

    /// Bytes per TB for conversions
    pub const BYTES_PER_TB: u64 = 1_099_511_627_776;
}

/// Hardware limits and thresholds
pub mod limits {
    /// Maximum CPU cores for validation
    pub const MAX_CPU_CORES: u32 = 256;

    /// Maximum CPU threads for validation
    pub const MAX_CPU_THREADS: u32 = 512;

    /// Maximum CPU frequency in GHz
    pub const MAX_CPU_FREQUENCY: f64 = 15.0;

    /// Minimum CPU frequency in GHz
    pub const MIN_CPU_FREQUENCY: f64 = 0.1;

    /// Maximum memory speed in MHz
    pub const MAX_MEMORY_SPEED: u32 = 8000;

    /// Minimum memory speed in MHz
    pub const MIN_MEMORY_SPEED: u32 = 100;

    /// Maximum DIMM size in GB (for warnings)
    pub const MAX_DIMM_SIZE_GB: u64 = 128;

    /// Maximum storage size in GB (for warnings)
    pub const MAX_STORAGE_SIZE_GB: u64 = 20_000;

    /// Maximum network device count (warning threshold)
    pub const MAX_NETWORK_DEVICES: usize = 10;

    /// Maximum USB device count before hub check
    pub const MAX_USB_DEVICES_NO_HUB: usize = 20;

    /// Minimum device count for non-trivial systems
    pub const MIN_TOTAL_DEVICES: usize = 3;
}

/// Privacy validation constants
pub mod privacy {
    /// Minimum length for anonymized IDs at basic privacy level
    pub const MIN_BASIC_ID_LENGTH: usize = 8;

    /// Minimum length for anonymized IDs at enhanced privacy level
    pub const MIN_ENHANCED_ID_LENGTH: usize = 12;

    /// Minimum length for anonymized IDs at strict privacy level
    pub const MIN_STRICT_ID_LENGTH: usize = 16;

    /// Minimum entropy (unique characters) for anonymized IDs
    pub const MIN_ID_ENTROPY: usize = 3;

    /// Maximum CPU model name length for strict privacy
    pub const MAX_STRICT_CPU_MODEL_LENGTH: usize = 50;
}

/// System architecture constants
pub mod architectures {
    /// Supported CPU architectures
    pub const VALID_ARCHITECTURES: &[&str] = &["x86_64", "aarch64", "armv7l", "i686", "riscv64"];

    /// x86 CPU vendors
    pub const X86_CPU_VENDORS: &[&str] = &["Intel", "AMD"];

    /// ARM CPU vendors (partial matches)
    pub const ARM_CPU_VENDOR_PATTERNS: &[&str] = &["arm", "qualcomm", "apple"];
}

/// Time-based validation constants
pub mod time {
    /// Maximum report age in days before warning
    pub const MAX_REPORT_AGE_DAYS: i64 = 30;

    /// Maximum system uptime in days before suggestion
    pub const MAX_UPTIME_DAYS: i64 = 365;

    /// Minimum system uptime in seconds for stable detection
    pub const MIN_UPTIME_SECONDS: i64 = 60;

    /// Maximum future time tolerance in minutes
    pub const MAX_FUTURE_TIME_MINUTES: i64 = 5;
}

/// Hardware vendor whitelist for PII detection
pub const HARDWARE_VENDORS: &[&str] = &[
    // Major CPU/GPU vendors
    "Intel",
    "Intel Corporation",
    "Advanced Micro Devices",
    "AMD",
    "NVIDIA",
    "NVIDIA Corporation",
    "Qualcomm",
    "Apple",
    "Arm",
    "ARM",
    "MediaTek",
    "Broadcom",
    "Marvell",
    "Texas Instruments",
    // Memory vendors
    "Samsung",
    "SK Hynix",
    "Micron",
    "Corsair",
    "Crucial",
    "Kingston",
    "G.Skill",
    "Team Group",
    "Patriot",
    "ADATA",
    "PNY",
    "Mushkin",
    "Ballistix",
    "HyperX",
    // Storage vendors
    "Western Digital",
    "WD",
    "Seagate",
    "Toshiba",
    "SanDisk",
    "Crucial",
    "Intel",
    "Samsung",
    "Micron",
    "ADATA",
    "Kingston",
    "PNY",
    "Mushkin",
    "Corsair",
    "Sabrent",
    "WD_BLACK",
    // Network vendors
    "Realtek",
    "Intel",
    "Broadcom",
    "Qualcomm",
    "TP-Link",
    "Netgear",
    "D-Link",
    "ASUS",
    "Linksys",
    "Cisco",
    "Ubiquiti",
    "Mikrotik",
    // Audio vendors
    "Creative",
    "Creative Labs",
    "Realtek",
    "VIA",
    "C-Media",
    "ESS",
    "Cirrus Logic",
    "Yamaha",
    "Roland",
    "Focusrite",
    "PreSonus",
    "Behringer",
    // System vendors
    "Dell",
    "HP",
    "Hewlett-Packard",
    "Lenovo",
    "IBM",
    "Acer",
    "ASUS",
    "MSI",
    "Gigabyte",
    "ASRock",
    "Supermicro",
    "Tyan",
    "Biostar",
    "ECS",
    "Foxconn",
    "Shuttle",
    "Zotac",
    // Peripheral vendors
    "Logitech",
    "Razer",
    "Corsair",
    "SteelSeries",
    "HyperX",
    "Roccat",
    "Cooler Master",
    "Thermaltake",
    "NZXT",
    "be quiet!",
    "Fractal Design",
    "Phanteks",
    // Legacy vendors
    "3dfx",
    "S3",
    "Matrox",
    "ATI",
    "VIA",
    "SiS",
    "Cyrix",
    "Trident",
    "Tseng Labs",
    // Specialized vendors
    "Elgato",
    "Elgato Systems",
    "Hauppauge",
    "BlackMagic",
    "AVerMedia",
    "Magewell",
    "Linux Foundation",
    "VIA Labs",
    "Genesys Logic",
    "ASMedia",
    "JMicron",
    "Etron",
];

/// USB vendor IDs that commonly appear multiple times (hubs, root hubs)
pub const MULTI_INSTANCE_USB_VENDORS: &[&str] = &[
    "1d6b", // Linux Foundation (USB hubs, root hubs)
    "8087", // Intel Corp. (internal hubs)
    "05e3", // Genesys Logic (USB hubs)
    "0424", // Microchip Technology (USB hubs)
];

/// Storage device types
pub const STORAGE_DEVICE_TYPES: &[&str] =
    &["HDD", "SSD", "NVMe", "eMMC", "SD Card", "USB", "CD-ROM", "DVD", "Blu-ray"];

/// Network device types
pub const NETWORK_DEVICE_TYPES: &[&str] =
    &["ethernet", "wifi", "bluetooth", "cellular", "loopback"];

/// Audio device types
pub const AUDIO_DEVICE_TYPES: &[&str] = &["playback", "capture", "duplex", "midi"];

/// Memory technology types
pub const MEMORY_TYPES: &[&str] = &["DDR2", "DDR3", "DDR4", "DDR5", "LPDDR4", "LPDDR5"];

/// USB specification versions
pub const USB_VERSIONS: &[&str] = &["1.0", "1.1", "2.0", "3.0", "3.1", "3.2"];

/// Storage interface types
pub const STORAGE_INTERFACES: &[&str] = &["SATA", "NVMe", "PCIe", "USB", "IDE", "SCSI"];
