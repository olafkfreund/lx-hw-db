# Linux Hardware Database (lx-hw-db)

A privacy-first, community-driven Linux hardware compatibility database that leverages GitHub's infrastructure for transparent governance and distributed collaboration.

## Warning: AI-Assisted Development

This project is being developed with assistance from Claude AI (Anthropic's AI assistant). While human oversight ensures quality and decision-making, some code, documentation, and architectural decisions have been generated or influenced by artificial intelligence. All AI contributions are reviewed and validated by human developers before integration.

## Project Goal

The Linux Hardware Database aims to solve the persistent problem of hardware compatibility uncertainty in Linux systems by creating a comprehensive, community-maintained database that provides:

- **Hardware Compatibility Information**: Detailed compatibility reports for Linux hardware across different kernel versions and distributions
- **Privacy-Preserving Data Collection**: Comprehensive anonymization using cryptographic hashing and differential privacy techniques
- **Actionable Configuration Recommendations**: Automated generation of kernel parameters, driver configurations, and system optimizations
- **Community-Driven Validation**: Transparent peer review and expert validation through GitHub's collaborative workflows

## What We're Building

### Core Components

**Hardware Detection Tool (`lx-hw-detect`)**
A Rust-based command-line tool that collects hardware information using multiple Linux utilities (lshw, dmidecode, lspci, lsusb, inxi) while implementing comprehensive privacy protection. The tool provides real-time kernel compatibility analysis, kernel source code searching, and generates actionable hardware upgrade recommendations. It produces standardized compatibility reports that can be contributed to the community database.

**Key Commands:**
- `detect`: Comprehensive hardware scanning with kernel compatibility analysis
- `analyze`: Focused kernel compatibility verification with upgrade recommendations  
- `check`: Tool availability verification and system readiness assessment

**GitHub-Based Database**
Hardware compatibility data is stored as markdown files with YAML frontmatter in a hierarchical GitHub repository structure. This approach provides natural version control, distributed redundancy, and transparent governance through pull requests and peer review.

**Automated Processing Pipeline**
GitHub Actions workflows handle submission validation, duplicate detection, schema compliance checking, and automated index generation for search functionality.

**Search and Discovery Interface**
Multiple search interfaces including static site generation, client-side search capabilities, and programmatic API access enable users to find compatible hardware and configuration recommendations.

### Privacy-First Architecture

The system implements multiple layers of privacy protection:

- **Cryptographic Anonymization**: All hardware identifiers undergo HMAC-SHA256 hashing with time-rotating salts
- **Differential Privacy**: Statistical noise injection prevents individual system identification while preserving data utility
- **K-Anonymity**: Hardware configurations must appear multiple times in the dataset to prevent unique identification
- **Data Minimization**: Only essential hardware information is collected, with no personal files, network configurations, or user credentials

Users can select from three privacy levels:
- **Basic**: Standard anonymization with 24-hour salt rotation
- **Enhanced**: Additional generalization with 12-hour salt rotation  
- **Strict**: Maximum privacy with 1-hour salt rotation and aggressive anonymization

### Federated Architecture

The system supports distributed collaboration through a federated network:

- **Primary Repository**: Authoritative source for public hardware compatibility data
- **Specialized Repositories**: Hardware category or use-case specific databases
- **Organization Repositories**: Private databases that can contribute anonymized data to the public repository
- **Trust Hierarchies**: Configurable weighting based on source credibility and validation

## Technology Stack

- **Hardware Detection**: Rust (single binary, maximum performance and safety)
- **Data Processing**: Go (concurrent processing, simple deployment)
- **Web API**: Python with FastAPI (rapid development, rich ecosystem)
- **Frontend**: TypeScript with React (modern user experience)
- **Storage**: Git-based with optional PostgreSQL for advanced queries
- **Deployment**: GitHub Actions, GitHub Pages, CDN integration

## Kernel Compatibility Analysis

The system provides advanced kernel compatibility analysis that goes beyond simple hardware detection:

### Real-Time Compatibility Verification
- **Module Resolution**: Maps detected hardware to kernel modules using `/lib/modules/*/modules.alias`
- **Support Classification**: Categorizes devices as supported, experimental, generic, or unsupported
- **Driver Information**: Provides kernel module names, configuration dependencies, and version requirements
- **Missing Module Detection**: Identifies hardware lacking kernel drivers

### Kernel Source Analysis
- **Direct Source Queries**: Searches official Linux kernel repositories via GitHub API
- **MODULE_DEVICE_TABLE Parsing**: Extracts hardware support information directly from driver source code
- **Version History Tracking**: Determines when hardware support was introduced in specific kernel versions
- **Experimental Driver Detection**: Identifies staging or experimental drivers for newer hardware

### Upgrade Recommendations
- **Distribution-Specific Commands**: Provides tailored kernel upgrade instructions for major Linux distributions
- **Success Probability**: Estimates likelihood of improved hardware support after kernel upgrades
- **Configuration Guidance**: Suggests kernel configuration options and module parameters
- **Risk Assessment**: Categorizes upgrade complexity and potential system impact

## Hardware Detection Architecture

The hardware detection system uses a modular architecture with multiple specialized detectors:

### lshw Detector (Complete)
- **Data Source**: JSON output from `lshw -json -quiet -sanitize`
- **Capabilities**: Comprehensive hardware tree with PCI, USB, memory, storage, and network devices  
- **Privilege Handling**: Warns about missing privileges but continues with available data
- **Privacy Features**: Automatically identifies serial numbers, MAC addresses for anonymization
- **Performance**: 30-second timeout, efficient JSON parsing with serde

### dmidecode Detector (Complete)
- **Data Source**: Text output from `dmidecode -t system,baseboard,bios,processor,memory`
- **Capabilities**: BIOS information, motherboard details, memory modules, processor specifications
- **Privilege Handling**: Detects `/dev/mem` access issues, gracefully handles unprivileged execution
- **Privacy Features**: Captures UUIDs, serial numbers, asset tags for anonymization pipeline
- **Performance**: 15-second timeout, custom text parser for DMI/SMBIOS data structures
- **Data Coverage**: 
  - **BIOS**: Vendor, version, release date, characteristics, revision
  - **System**: Manufacturer, product name, UUID, serial number, SKU
  - **Baseboard**: Manufacturer, product name, version, serial number, features
  - **Processor**: Socket, manufacturer, version, cores, threads, speed, flags
  - **Memory**: DIMMs with size, type, speed, manufacturer, part numbers

### Kernel Compatibility Detector (Complete)
- **Data Source**: `/lib/modules/*/modules.alias` and sysfs filesystem (`/sys/bus/pci/devices/*`)
- **Capabilities**: Real-time kernel compatibility verification for detected hardware
- **Support Analysis**: Identifies supported, unsupported, experimental, and generic driver support
- **Device Extraction**: Direct PCI device enumeration from sysfs with vendor:device ID mapping
- **Module Resolution**: Maps hardware IDs to kernel modules using modules.alias database
- **Upgrade Recommendations**: Distribution-specific kernel upgrade suggestions with estimated success rates
- **Performance**: Concurrent device analysis with intelligent fallback strategies

### Kernel Source Detector (Complete)
- **Data Source**: Official Linux kernel Git repositories (GitHub API integration)
- **Capabilities**: Direct kernel source code analysis for hardware support verification
- **Search Scope**: MODULE_DEVICE_TABLE definitions, driver source code, Kconfig entries
- **Repository Support**: Both remote GitHub API queries and local kernel repository analysis
- **Version Tracking**: Kernel version history for hardware support introduction
- **Experimental Detection**: Identifies experimental or staging drivers for unsupported hardware
- **Performance**: Efficient API queries with rate limiting and caching strategies

### Planned Detectors
- **lspci**: PCI device enumeration with extended kernel driver mapping
- **lsusb**: USB device detection with vendor/product identification  
- **inxi**: User-friendly system summaries and additional hardware insights

## Current Status

**Phase 1: Foundation Complete**
- Rust CLI tool with comprehensive argument parsing
- Privacy-preserving anonymization architecture
- Modular project structure ready for hardware detection implementation
- Configuration management and error handling systems

**Phase 2: Hardware Detection Complete**
- **lshw detector**: Complete JSON-based hardware information extraction
- **dmidecode detector**: Complete BIOS, motherboard, and memory detection with privilege handling
- **Kernel compatibility detector**: Real-time kernel module compatibility verification
- **Kernel source detector**: Direct Linux kernel source code analysis via GitHub API
- **Hardware analyzer**: Unified detection pipeline with privacy-preserving anonymization
- **CLI integration**: Full command-line interface with multiple output formats (YAML, JSON, Markdown)
- **Comprehensive testing**: Real-system validation with 31+ detected devices and kernel analysis
- **Privacy-sensitive data identification**: Automatic detection of serial numbers, UUIDs, and other identifiers for anonymization
- **lspci detector**: Planned - Enhanced PCI device detection with extended kernel driver mapping
- **lsusb detector**: Planned - USB device enumeration
- **inxi detector**: Planned - User-friendly system summary information

**Next: Phase 3 - Report Generation & Submission**
- Report generation and validation
- Community submission workflows
- GitHub integration for automated database updates

## Getting Started

### Prerequisites

- Rust toolchain (1.70+)
- Linux system with standard hardware detection tools

### Installation

```bash
git clone https://github.com/your-org/lx-hw-db.git
cd lx-hw-db
cargo build --release
```

### Usage

```bash
# Generate default configuration
./target/release/lx-hw-detect config init

# Check available detection tools  
./target/release/lx-hw-detect check

# Detect hardware with kernel compatibility analysis
./target/release/lx-hw-detect detect --privacy enhanced --format markdown

# Analyze kernel compatibility for current system
./target/release/lx-hw-detect analyze --recommendations

# Analyze specific device with kernel source search
./target/release/lx-hw-detect analyze --device 8086:1234 --kernel-source --recommendations

# Use local kernel repository for faster analysis
./target/release/lx-hw-detect analyze --kernel-repo /path/to/linux --kernel-source
```

## Contributing

This project follows community-driven development principles:

1. **Hardware Submissions**: Contribute compatibility reports through guided templates
2. **Code Contributions**: Follow standard GitHub workflow with pull requests
3. **Documentation**: Help improve user guides and technical documentation
4. **Validation**: Peer review of hardware submissions and code changes

All contributions are subject to the project's privacy standards and quality control processes.

## Privacy Commitment

This project is designed with privacy as a fundamental principle, not an afterthought:

- No tracking of individual users or systems
- All hardware identifiers are cryptographically anonymized
- Users maintain complete control over their privacy level
- Transparent data collection practices with no hidden information gathering
- GDPR compliant data handling and user rights

## License

This project is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0). This ensures that any network service using this code must also provide source code to users.

## Acknowledgments

This project builds upon the success of existing hardware compatibility efforts and aims to create a more comprehensive, privacy-preserving solution for the Linux community. Special recognition goes to the maintainers of hardware detection tools (lshw, dmidecode, lspci, lsusb, inxi) whose work enables this project.
