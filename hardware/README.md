# Hardware Compatibility Database

This directory contains hardware compatibility data organized by component type. Each hardware entry uses standardized markdown with YAML frontmatter containing structured metadata for compatibility status, specifications, and tested configurations.

## Directory Structure

```
hardware/
├── cpu/
│   ├── intel/          # Intel processors
│   └── amd/            # AMD processors
├── gpu/
│   ├── nvidia/         # NVIDIA graphics cards
│   ├── amd/            # AMD graphics cards
│   └── intel/          # Intel graphics
├── motherboard/        # Motherboards and chipsets
├── network/            # Network interfaces and WiFi cards
└── peripherals/        # USB devices, audio, input devices
```

## Data Format

Each hardware entry follows this structure:

```yaml
---
name: "Hardware Name"
category: "cpu|gpu|motherboard|network|peripherals"
manufacturer: "Vendor"
compatibility_status: "full|partial|limited|none"
last_tested: "YYYY-MM-DD"
specs:
  # Hardware-specific specifications
compatibility:
  linux:
    kernel_version: "5.15+"
    status: "full|partial|limited|none"
    notes: "Detailed compatibility information"
tested_configurations:
  - motherboard: "Board Model"
    status: "working|issues|broken"
---

# Hardware compatibility details and user reports
```

## Contributing

This directory will be populated through community submissions via the automated hardware detection tool and manual contributions. See [CONTRIBUTING.md](../CONTRIBUTING.md) for submission guidelines.

## Privacy

All hardware identifiers in this database are cryptographically anonymized using HMAC-SHA256 with time-rotating salts to protect user privacy while maintaining statistical utility.