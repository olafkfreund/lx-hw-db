# Linux Hardware Compatibility Database

[![Build Indices](https://github.com/lx-hw-db/lx-hw-db/actions/workflows/build-indices.yml/badge.svg)](https://github.com/lx-hw-db/lx-hw-db/actions/workflows/build-indices.yml)
[![Deploy Pages](https://github.com/lx-hw-db/lx-hw-db/actions/workflows/deploy-pages.yml/badge.svg)](https://github.com/lx-hw-db/lx-hw-db/actions/workflows/deploy-pages.yml)
[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-blue.svg)](LICENSE)
[![Data: CC0](https://img.shields.io/badge/Data-CC0-green.svg)](https://creativecommons.org/public-domain/cc0/)

> 🐧 A community-driven, privacy-preserving Linux hardware compatibility database built entirely on GitHub infrastructure

## 🌟 Features

- **🔍 Fast Client-Side Search** - Search hardware compatibility across vendors, models, kernels, and distributions
- **📊 Real-Time Statistics** - Live dashboard showing compatibility trends and database health
- **🤖 Automated Processing** - GitHub Actions automatically build indices from community reports
- **🔒 Privacy-First** - All data is anonymized and contains no personally identifiable information
- **📱 Modern Interface** - Responsive web design with dark mode support
- **🌐 GitHub-Native** - Fully hosted on GitHub Pages with no external dependencies
- **⚡ Zero-Server Architecture** - Static site with pre-built JSON indices for instant search

## 🚀 Quick Start

### 🔗 Access the Database

Visit the live database: **[https://lx-hw-db.github.io/lx-hw-db/](https://lx-hw-db.github.io/lx-hw-db/)**

### 📡 API Access

The database provides REST-like JSON APIs:

```bash
# Get all vendors
curl https://lx-hw-db.github.io/lx-hw-db/api/vendors.json

# Get specific vendor details
curl https://lx-hw-db.github.io/lx-hw-db/api/vendors/nvidia.json

# Get compatibility statistics
curl https://lx-hw-db.github.io/lx-hw-db/api/statistics.json

# Search indices
curl https://lx-hw-db.github.io/lx-hw-db/indices/search-terms.json
```

### 🤝 Contributing Hardware Reports

Help improve Linux hardware compatibility by contributing your system's hardware report:

#### 1. Install the Detection Tool

```bash
cargo install lx-hw-detect
```

#### 2. Generate Your Hardware Report

```bash
# Generate with medium privacy level (recommended)
lx-hw-detect --privacy-level medium --output my-hardware-report.json

# Or with high privacy for maximum anonymization
lx-hw-detect --privacy-level high --output my-hardware-report.json
```

#### 3. Submit Your Report

1. Fork this repository
2. Add your report to the `hardware-reports/` directory:
   ```bash
   cp my-hardware-report.json hardware-reports/$(date +%Y-%m-%d)-$(hostname)-report.json
   ```
3. Create a pull request with your report

The automated system will process your report and update the database indices within minutes of merging.

## 🏗️ Architecture

### GitHub-Native Design

This project uses a novel **GitHub-native database architecture**:

```
📁 Repository Structure
├── hardware-reports/           # Raw JSON reports (community contributed)
├── indices/                   # Generated search indices  
├── api/                       # REST-like JSON API endpoints
├── statistics/                # Database analytics and trends
├── web/                       # Static website assets
└── .github/workflows/         # Automated processing pipeline
```

### Data Flow

1. **Community Contribution** → Hardware reports submitted via PRs
2. **GitHub Actions** → Automatically processes reports into search indices
3. **Static Hosting** → GitHub Pages serves the complete database
4. **Client Search** → Fast, offline-capable search using pre-built indices

### Key Components

- **🦀 Rust Indexer** (`lx-hw-indexer`) - Processes reports into structured indices
- **⚙️ GitHub Actions** - Automated CI/CD pipeline for index generation
- **🌐 Static Web App** - Modern JavaScript search interface
- **📊 Analytics Dashboard** - Real-time statistics and compatibility trends

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
