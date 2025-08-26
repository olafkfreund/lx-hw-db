# Hardware Database Schema

> Spec: Hardware Detection CLI Tool
> Created: 2025-08-26
> Version: 1.0.0

## Schema Overview

The hardware database uses YAML frontmatter combined with markdown content to create structured, human-readable hardware compatibility reports. This format enables both automated processing and community review.

## YAML Frontmatter Schema

### Complete Schema Definition

```yaml
---
# Report Metadata (Required)
report_id: string                    # Unique anonymized report identifier
generated_at: datetime               # ISO 8601 timestamp (UTC)
tool_version: string                 # CLI tool version that generated report
privacy_level: enum                  # basic|enhanced|strict
anonymization_salt_period: string   # Salt rotation period (24h|12h|1h)

# System Information (Required)
system:
  kernel: string                     # Kernel version (e.g., "6.5.0-generic")
  distribution: string               # Distribution name (lowercase)
  distribution_version: string       # Distribution version
  architecture: string               # Architecture (x86_64, aarch64, etc.)
  desktop_environment: string       # DE name or "none" for servers
  boot_mode: enum                    # uefi|bios|unknown
  secure_boot: boolean              # Secure boot status if detectable

# Hardware Summary (Required)  
hardware_summary:
  cpu_count: integer                # Number of CPU packages
  cpu_cores: integer                # Total physical cores
  cpu_threads: integer              # Total logical cores/threads
  total_memory_gb: integer          # Total system RAM in GB
  storage_devices: integer          # Count of storage devices
  gpu_count: integer                # Count of graphics devices
  network_interfaces: integer       # Count of network interfaces
  usb_devices: integer              # Count of USB devices
  pci_devices: integer              # Count of PCI/PCIe devices

# Compatibility Assessment (Required)
compatibility:
  overall_status: enum              # excellent|good|partial|poor|unknown
  working_devices: integer          # Count of fully functional devices
  partial_devices: integer          # Count of partially working devices  
  problem_devices: integer          # Count of non-functional devices
  untested_devices: integer         # Count of untested devices
  requires_proprietary_drivers: boolean
  requires_firmware: boolean
  kernel_parameters_needed: boolean

# Privacy and Anonymization (Required)
anonymization:
  method: string                    # Anonymization algorithm used
  salt_rotation: string             # Salt rotation frequency
  k_anonymity: integer              # K-anonymity level achieved
  anonymized_fields: array[string]  # List of anonymized field types
  
# Hardware Categories (Optional sections)
cpu:
  - anonymized_id: string           # Anonymized hardware identifier
    vendor: enum                    # intel|amd|arm|riscv|other
    family: string                  # CPU family/generation
    model: string                   # Model name (may be anonymized)
    cores: integer                  # Physical cores for this CPU
    threads: integer                # Logical threads for this CPU
    frequency_mhz: integer          # Base frequency in MHz
    cache_l1_kb: integer           # L1 cache size in KB
    cache_l2_kb: integer           # L2 cache size in KB  
    cache_l3_kb: integer           # L3 cache size in KB
    features: array[string]         # CPU features (avx, sse4, etc.)
    status: enum                    # excellent|good|partial|poor|unknown
    driver: string                  # Driver information
    notes: string                   # Compatibility notes

gpu:
  - anonymized_id: string
    vendor: enum                    # nvidia|amd|intel|other
    type: enum                      # discrete|integrated|virtual
    memory_gb: integer              # Video memory in GB
    driver_type: enum               # proprietary|open_source|firmware
    driver_name: string             # Specific driver name
    driver_version: string          # Driver version
    status: enum
    opengl_version: string          # OpenGL support version
    vulkan_support: boolean         # Vulkan API support
    compute_support: boolean        # CUDA/OpenCL support
    notes: string

memory:
  - anonymized_id: string
    type: enum                      # ddr4|ddr5|lpddr4|lpddr5|other
    size_gb: integer               # Memory module size in GB
    speed_mhz: integer             # Memory speed in MHz
    voltage: float                  # Operating voltage
    form_factor: enum               # dimm|sodimm|other
    ecc_support: boolean            # Error correction support
    status: enum
    notes: string

storage:
  - anonymized_id: string
    type: enum                      # ssd|hdd|nvme|emmc|other
    interface: enum                 # sata|nvme|usb|other
    size_gb: integer               # Storage capacity in GB
    model: string                   # Device model (anonymized)
    driver: string                  # Storage driver/controller
    status: enum
    smart_support: boolean          # SMART monitoring support
    trim_support: boolean           # TRIM/discard support
    notes: string

network:
  - anonymized_id: string
    type: enum                      # ethernet|wifi|bluetooth|other
    vendor: enum                    # intel|realtek|broadcom|other
    driver: string                  # Network driver name
    firmware_required: boolean      # External firmware needed
    status: enum
    speed_mbps: integer            # Link speed in Mbps
    wireless_standards: array[string] # WiFi standards (802.11ac, etc.)
    notes: string

audio:
  - anonymized_id: string
    type: enum                      # integrated|discrete|usb|other
    driver: string                  # Audio driver (ALSA/PulseAudio)
    status: enum
    channels: integer               # Audio channels supported
    sample_rate_hz: integer         # Maximum sample rate
    notes: string

input:
  - anonymized_id: string
    type: enum                      # keyboard|mouse|touchpad|touchscreen|other
    interface: enum                 # usb|ps2|i2c|other
    driver: string                  # Input driver
    status: enum
    features: array[string]         # Special features (multitouch, etc.)
    notes: string

usb_controllers:
  - anonymized_id: string
    version: enum                   # usb2|usb3|usb3.1|usb3.2|usb4
    ports: integer                  # Number of ports
    driver: string                  # USB controller driver
    status: enum
    notes: string

# BIOS/UEFI Information (Optional)
firmware:
  type: enum                        # bios|uefi|unknown
  vendor: string                    # BIOS vendor (may be anonymized)
  version: string                   # BIOS version (may be anonymized)
  release_date: string              # BIOS release date (YYYY-MM format)
  secure_boot_capable: boolean      # Secure boot support
  tpm_version: string               # TPM version if present
  virtualization_support: boolean   # Hardware virtualization support

# Power Management (Optional)
power:
  acpi_version: string              # ACPI specification version
  suspend_support: boolean          # Suspend to RAM support
  hibernate_support: boolean        # Suspend to disk support
  cpu_frequency_scaling: boolean    # CPU frequency scaling available
  gpu_power_management: boolean     # GPU power management support

# Virtualization (Optional, for VMs)
virtualization:
  hypervisor: enum                  # kvm|xen|vmware|virtualbox|hyperv|other
  guest_additions: boolean          # Guest additions/tools installed  
  paravirtualization: boolean       # Paravirt drivers in use
  hardware_passthrough: boolean     # GPU/device passthrough active

# Container Environment (Optional)
container:
  runtime: enum                     # docker|podman|lxc|other
  privileged: boolean               # Privileged container mode
  host_hardware_access: boolean     # Access to host hardware

# Testing Information (Optional)
testing:
  test_date: date                   # When compatibility was tested
  test_duration_days: integer       # How long system was tested
  workloads_tested: array[string]   # Types of workloads tested
  issues_encountered: array[string] # Any issues found during testing
  performance_notes: string         # Performance observations
---
```

## Field Validation Rules

### Required Fields
All reports MUST include:
- `report_id`, `generated_at`, `tool_version`, `privacy_level`
- Complete `system` section
- Complete `hardware_summary` section  
- Complete `compatibility` section
- Complete `anonymization` section

### Data Type Constraints
```yaml
# String length limits
report_id: 8-64 characters, alphanumeric plus underscore
tool_version: semantic versioning pattern (x.y.z)
system.kernel: max 50 characters
system.distribution: max 30 characters, lowercase
system.architecture: max 20 characters

# Numeric constraints  
hardware_summary.total_memory_gb: 1-2048
hardware_summary.*_count: 0-999
cpu.cores: 1-256
cpu.threads: 1-512
cpu.frequency_mhz: 100-10000

# Enum constraints
privacy_level: ["basic", "enhanced", "strict"]
compatibility.overall_status: ["excellent", "good", "partial", "poor", "unknown"]
system.boot_mode: ["uefi", "bios", "unknown"]
```

### Anonymization Requirements
Fields subject to anonymization based on privacy level:

**Basic Level:**
- All `anonymized_id` fields
- Hardware serial numbers
- MAC addresses
- Device-specific version strings

**Enhanced Level:**
- Basic level fields plus:
- BIOS vendor and version details
- Specific model numbers
- Network device names

**Strict Level:**  
- Enhanced level fields plus:
- CPU model specifics
- Detailed hardware specifications
- Manufacturing dates and codes

## Schema Evolution

### Version Compatibility
- **Schema Version:** Embedded in `tool_version` field
- **Backward Compatibility:** New optional fields only
- **Migration Path:** Automated schema upgrading for older reports
- **Deprecation Policy:** 2-version deprecation cycle for removed fields

### Extensibility Points
- **Custom Fields:** Prefixed with `x_` for experimental features
- **Vendor Extensions:** Namespaced vendor-specific data
- **Plugin Data:** Reserved `plugins` section for tool extensions

## Example Report Structure

### Minimal Valid Report
```yaml
---
report_id: "hw_a1b2c3d4"
generated_at: "2025-08-26T10:30:00Z"
tool_version: "1.0.0"
privacy_level: "basic"
anonymization_salt_period: "24h"

system:
  kernel: "6.5.0-generic"
  distribution: "ubuntu"  
  distribution_version: "24.04"
  architecture: "x86_64"
  desktop_environment: "gnome"
  boot_mode: "uefi"
  secure_boot: true

hardware_summary:
  cpu_count: 1
  cpu_cores: 8
  cpu_threads: 16
  total_memory_gb: 16
  storage_devices: 1
  gpu_count: 1
  network_interfaces: 1
  usb_devices: 4
  pci_devices: 12

compatibility:
  overall_status: "excellent"
  working_devices: 18
  partial_devices: 0
  problem_devices: 0
  untested_devices: 0
  requires_proprietary_drivers: false
  requires_firmware: false
  kernel_parameters_needed: false

anonymization:
  method: "hmac_sha256"
  salt_rotation: "24h"
  k_anonymity: 5
  anonymized_fields: ["device_ids", "serials"]
---

# Hardware Compatibility Report
[Markdown content follows...]
```

## Validation Implementation

### Schema Validation
```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
struct HardwareReport {
    report_id: String,
    generated_at: DateTime<Utc>,
    tool_version: String,
    privacy_level: PrivacyLevel,
    anonymization_salt_period: String,
    system: SystemInfo,
    hardware_summary: HardwareSummary,
    compatibility: CompatibilityInfo,
    anonymization: AnonymizationInfo,
    // Optional hardware sections
    cpu: Option<Vec<CpuInfo>>,
    gpu: Option<Vec<GpuInfo>>,
    // ... other optional sections
}

#[derive(Debug, Serialize, Deserialize)]
enum PrivacyLevel {
    Basic,
    Enhanced, 
    Strict,
}
```

### Validation Rules
1. **Required Field Validation:** Ensure all mandatory fields present
2. **Type Validation:** Verify correct data types for all fields
3. **Range Validation:** Check numeric values within acceptable ranges
4. **Enum Validation:** Verify enum values match allowed options
5. **Anonymization Validation:** Confirm sensitive data properly anonymized
6. **Cross-Field Validation:** Verify logical consistency between fields

This schema provides the foundation for structured hardware data collection while ensuring privacy compliance and community collaboration through standardized formats.