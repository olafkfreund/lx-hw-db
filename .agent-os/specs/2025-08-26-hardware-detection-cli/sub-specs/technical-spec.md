# Technical Specification

> Spec: Hardware Detection CLI Tool
> Created: 2025-08-26
> Version: 1.0.0

## Architecture Overview

### Core Components

```
lx-hw-detect
├── src/
│   ├── main.rs              # CLI entry point and argument parsing
│   ├── detectors/
│   │   ├── mod.rs           # Detector trait and registry
│   │   ├── lshw.rs          # lshw hardware detection
│   │   ├── dmidecode.rs     # BIOS/motherboard detection
│   │   ├── lspci.rs         # PCI devices detection
│   │   ├── lsusb.rs         # USB devices detection
│   │   └── inxi.rs          # System summary detection
│   ├── privacy/
│   │   ├── mod.rs           # Privacy anonymization interface
│   │   ├── hasher.rs        # SHA-512 and HMAC-SHA256 implementation
│   │   └── anonymizer.rs    # Hardware ID anonymization logic
│   ├── output/
│   │   ├── mod.rs           # Output formatting interface
│   │   ├── markdown.rs      # Markdown with YAML frontmatter
│   │   └── schema.rs        # Hardware schema validation
│   ├── hardware/
│   │   ├── mod.rs           # Hardware data structures
│   │   └── types.rs         # Hardware component types
│   └── errors/
│       └── mod.rs           # Error types and handling
├── tests/
│   ├── integration/         # End-to-end CLI tests
│   └── unit/               # Unit tests for components
└── examples/               # Example outputs and usage
```

## Hardware Detection Specifications

### Tool Integration Requirements

#### lshw (List Hardware)
- **Purpose:** Primary comprehensive hardware detection
- **Execution:** `lshw -json -sanitize -quiet`
- **Output Format:** JSON parsing for structured data extraction
- **Key Data:** CPU, memory, motherboard, storage, network interfaces
- **Error Handling:** Fallback to XML output if JSON unavailable
- **Privileges:** May require root for complete information

#### dmidecode (DMI Table Decoder)
- **Purpose:** BIOS, motherboard, and memory module details
- **Execution:** `dmidecode -t 0,1,2,3,4,16,17`
- **Types:** BIOS (0), System (1), Baseboard (2), Chassis (3), Processor (4), Memory Array (16), Memory Device (17)
- **Parsing:** Custom parser for DMI table format
- **Privileges:** Requires root access or sudo
- **Fallback:** Graceful degradation when unavailable

#### lspci (PCI Devices)
- **Purpose:** PCI/PCIe device enumeration with kernel driver mapping
- **Execution:** `lspci -k -v -nn`
- **Key Data:** GPU, network cards, sound cards, storage controllers
- **Driver Mapping:** Kernel driver and modules in use
- **Parsing:** PCI ID database integration for device names
- **No Privileges:** Runs without special permissions

#### lsusb (USB Devices)
- **Purpose:** USB device enumeration and hierarchy
- **Execution:** `lsusb -v -t`
- **Key Data:** USB controllers, hubs, connected devices
- **Parsing:** USB device tree structure
- **No Privileges:** Runs without special permissions
- **Optional:** Detailed device descriptors with appropriate permissions

#### inxi (System Information)
- **Purpose:** User-friendly system summary and verification
- **Execution:** `inxi -F -m -x -x -x`
- **Key Data:** System overview, verification of other tool outputs
- **Output Format:** Custom parsing of inxi's structured output
- **No Privileges:** Comprehensive information without root

### Privacy Implementation

#### Hash Algorithm Specifications
```rust
// Hardware ID anonymization
fn anonymize_hardware_id(raw_id: &str, salt: &[u8]) -> String {
    let key = hmac::Key::new(hmac::HMAC_SHA256, salt);
    let signature = hmac::sign(&key, raw_id.as_bytes());
    format!("hw_{}", hex::encode(&signature.as_ref()[..16])) // 32-char hex
}

// Time-rotating salt generation
fn generate_salt(timestamp: SystemTime, privacy_level: PrivacyLevel) -> Vec<u8> {
    let epoch_hours = timestamp.duration_since(UNIX_EPOCH)
        .unwrap().as_secs() / 3600;
    
    match privacy_level {
        PrivacyLevel::Basic => epoch_hours / 24,    // Daily rotation
        PrivacyLevel::Enhanced => epoch_hours / 12, // 12-hour rotation  
        PrivacyLevel::Strict => epoch_hours,        // Hourly rotation
    }.to_le_bytes().to_vec()
}
```

#### Anonymization Targets
- **PCI/USB Device IDs:** Vendor:Device ID pairs
- **Serial Numbers:** All device serial numbers
- **MAC Addresses:** Network interface hardware addresses
- **Unique Identifiers:** CPU, motherboard, memory module serials
- **BIOS/UEFI Information:** Version strings and build identifiers

#### Privacy Levels
1. **Basic:** Daily salt rotation, essential ID anonymization
2. **Enhanced:** 12-hour salt rotation, extended anonymization scope
3. **Strict:** Hourly salt rotation, maximum anonymization with k-anonymity

## Output Format Specification

### YAML Frontmatter Schema
```yaml
---
# Hardware Report Metadata
report_id: "hw_a1b2c3d4e5f6"
generated_at: "2025-08-26T10:30:00Z"
tool_version: "1.0.0"
privacy_level: "enhanced"
anonymization_salt_period: "12h"

# System Information
system:
  kernel: "6.5.0-generic"
  distribution: "ubuntu"
  distribution_version: "24.04"
  architecture: "x86_64"
  desktop_environment: "gnome"

# Hardware Summary
hardware_summary:
  cpu_count: 1
  total_memory_gb: 16
  storage_devices: 2
  gpu_count: 1
  network_interfaces: 2
  usb_devices: 8

# Compatibility Status
compatibility:
  overall_status: "excellent"  # excellent/good/partial/poor/unknown
  problem_devices: []
  working_devices: 12
  untested_devices: 1

# Anonymization Info  
anonymization:
  method: "hmac_sha256"
  salt_rotation: "12h"
  k_anonymity: 5
  anonymized_fields: ["device_ids", "serials", "mac_addresses"]
---
```

### Markdown Content Structure
```markdown
# Hardware Compatibility Report

## System Overview
- **Kernel:** 6.5.0-generic
- **Distribution:** Ubuntu 24.04 LTS  
- **Architecture:** x86_64
- **Desktop:** GNOME Shell 46.0

## Hardware Components

### CPU
- **Model:** [Anonymized: cpu_a1b2c3d4e5]
- **Cores:** 8 cores / 16 threads
- **Status:** ✅ Excellent
- **Driver:** Built-in kernel support

### GPU  
- **Model:** [Anonymized: gpu_f6e5d4c3b2a1]
- **Type:** Discrete Graphics
- **Status:** ✅ Excellent  
- **Driver:** nvidia-driver-535

[Additional hardware sections...]

## Compatibility Notes
- All hardware components detected and functioning properly
- No driver issues or compatibility warnings
- Performance optimizations available through kernel parameters

## Technical Details
<details>
<summary>Detailed Hardware Information</summary>

```json
{
  "cpu": {
    "anonymized_id": "cpu_a1b2c3d4e5",
    "vendor": "amd",
    "family": "zen4",
    "cores": 8,
    "threads": 16
  }
}
```
</details>
```

## CLI Interface Design

### Command Structure
```bash
# Basic usage
lx-hw-detect

# Privacy level selection
lx-hw-detect --privacy-level basic|enhanced|strict

# Output format options  
lx-hw-detect --format markdown|json|yaml

# Output file specification
lx-hw-detect --output /path/to/report.md

# Verbose logging
lx-hw-detect --verbose

# Specific tool selection
lx-hw-detect --tools lshw,dmidecode,lspci

# Skip privilege-requiring tools
lx-hw-detect --no-root-tools
```

### Configuration File Support
```toml
# ~/.config/lx-hw-detect/config.toml
[privacy]
level = "enhanced"
salt_rotation = "12h"

[output]  
format = "markdown"
include_technical_details = true

[detection]
timeout_seconds = 30
tools = ["lshw", "dmidecode", "lspci", "lsusb", "inxi"]
require_root = false
```

## Error Handling Requirements

### Tool Availability Errors
```rust
#[derive(Debug, Error)]
enum DetectionError {
    #[error("Tool not found: {tool}. Install with: {install_command}")]
    ToolNotFound { tool: String, install_command: String },
    
    #[error("Insufficient privileges for {tool}. Run with sudo or install {alternative}")]
    InsufficientPrivileges { tool: String, alternative: String },
    
    #[error("Tool {tool} timed out after {timeout}s")]
    ToolTimeout { tool: String, timeout: u64 },
    
    #[error("Parse error in {tool} output: {details}")]
    ParseError { tool: String, details: String },
}
```

### Graceful Degradation Strategy
1. **Missing Tools:** Continue with available tools, warn about missing data
2. **Permission Errors:** Skip privileged tools, suggest alternatives
3. **Parse Failures:** Log errors, continue with partial data
4. **Timeout Handling:** Cancel long-running tools, continue with others

## Performance Requirements

### Execution Time Targets
- **Complete Detection:** < 30 seconds on typical hardware
- **Tool Timeouts:** 10 seconds per tool maximum
- **Memory Usage:** < 50MB peak memory consumption
- **Binary Size:** < 10MB single executable

### Optimization Strategies
- **Parallel Execution:** Run independent tools concurrently
- **Lazy Loading:** Only parse outputs when needed
- **Streaming:** Process tool outputs as they arrive
- **Caching:** Cache tool availability checks

## Testing Requirements

### Integration Testing
- **Tool Mocking:** Mock system tool outputs for consistent testing
- **Privacy Verification:** Ensure no sensitive data leaks in outputs
- **Format Validation:** Verify YAML frontmatter and markdown structure
- **Error Scenarios:** Test all error conditions and recovery paths

### Platform Testing
- **Distributions:** Ubuntu, Fedora, Arch, openSUSE, Debian
- **Hardware Variants:** Different CPU architectures, GPU vendors
- **Permission Levels:** Root, sudo, user-only execution
- **Tool Availability:** Various combinations of installed/missing tools