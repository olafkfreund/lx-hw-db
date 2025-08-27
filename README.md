# Linux Hardware Compatibility Database

A privacy-focused, community-driven hardware compatibility database for Linux systems. Features comprehensive hardware detection, automated configuration recommendations, and transparent GitHub-based collaboration.

## Live Demo

Test the web interface at: https://olafkfreund.github.io/lx-hw-db/

## Project Status

**Phase 3 Active Development** - Hardware detection suite complete with privacy-preserving data collection, comprehensive validation system, and web interface.

### Implemented Components

**Hardware Detection CLI** (Rust)
- Multi-tool detection using lshw, dmidecode, lspci, lsusb, and inxi
- HMAC-SHA256 anonymization with time-rotating salts
- Real-time kernel module compatibility verification via sysfs
- Privacy levels: Basic, Enhanced, and Strict
- Output formats: YAML, JSON, Markdown with frontmatter

**Web Interface** (JavaScript/HTML5)
- FlexSearch-powered hardware search with real-time filtering
- Advanced compatibility matrix visualization
- Community configuration tips system
- Multi-format export (Shell, Ansible, Docker, NixOS, Markdown, JSON)
- Contributor achievement and recognition system

**Validation System**
- JSON schema validation for report structure
- Business logic validation for hardware configurations
- Privacy compliance verification
- Data consistency checks across components
- Kernel compatibility validation

## Technical Architecture

### Privacy Implementation

```
Hardware ID -> HMAC-SHA256(ID, rotating_salt) -> Anonymized ID
```

- Differential privacy with Laplace noise (epsilon=1.0)
- K-anonymity enforcement (k >= 5)
- No raw identifiers stored
- Time-based salt rotation every 24 hours

### Data Flow

```
Hardware Detection -> Anonymization -> Validation -> Storage -> Web Interface
       |                   |              |            |           |
    (Rust CLI)      (SHA256/HMAC)    (Schema)    (Git/JSON)   (FlexSearch)
```

## Installation and Usage

### Hardware Detection Tool

```bash
# Clone repository
git clone https://github.com/olafkfreund/lx-hw-db.git
cd lx-hw-db

# Build the detection tool
cargo build --release

# Run detection with specific tools
cargo run --bin lx-hw-detect -- detect --tools lshw,lspci,lsusb --output report.yaml

# Validate hardware reports
cargo run --bin lx-hw-detect -- validate report.yaml

# Check kernel compatibility
cargo run --bin lx-hw-detect -- check --device-id 10de:2206
```

### Web Interface

```bash
# Navigate to web directory
cd web

# Start development server
python3 serve.py

# Access at http://localhost:8000
```

Alternative ports are automatically selected if 8000 is occupied.

## Hardware Detection Capabilities

### Supported Detection Tools

| Tool | Coverage | Information Extracted |
|------|----------|----------------------|
| lshw | Comprehensive | CPU, memory, storage, network, USB, PCI devices |
| dmidecode | BIOS/UEFI | Motherboard, BIOS version, memory modules |
| lspci | PCI devices | Graphics cards, network controllers, storage controllers |
| lsusb | USB devices | USB peripherals, hubs, device hierarchy |
| inxi | System summary | Consolidated system information, hardware overview |

### Kernel Compatibility Analysis

The system performs real-time kernel module verification:

```bash
# Check specific device support
/sys/bus/pci/devices/0000:01:00.0/modalias -> pci:v000010DEd00002206...

# Match against modules.alias
alias pci:v000010DEd00002206* nvidia
```

## API Endpoints

### Hardware Data API

```
GET /api/hardware
GET /api/tips
GET /api/statistics
POST /api/hardware/submit
POST /api/tips/submit
```

### Data Formats

**Hardware Report Schema:**
```json
{
  "metadata": {
    "version": "0.1.0",
    "generated_at": "2025-08-27T10:00:00Z",
    "privacy_level": "Enhanced",
    "tools_used": ["lshw", "lspci"],
    "anonymized_system_id": "sha256_hash"
  },
  "system": {
    "kernel_version": "6.16.0",
    "architecture": "x86_64",
    "distribution": "NixOS 25.11"
  },
  "devices": []
}
```

## Configuration Management

### Supported Export Formats

**Shell Scripts**
- POSIX-compliant shell commands
- Distribution-specific package managers
- Kernel parameter configurations

**Ansible Playbooks**
- Idempotent configuration management
- Multi-distribution support
- Role-based organization

**NixOS Configurations**
- Declarative system configuration
- Reproducible builds
- Hardware-specific modules

**Docker Containers**
- Containerized driver installations
- GPU passthrough configurations
- Network optimization settings

## Development

### Building from Source

**Prerequisites:**
- Rust 1.70+ (hardware detection tool)
- Python 3.8+ (web server)
- Node.js 16+ (optional, for development)

**Build Commands:**
```bash
# Build all components
cargo build --release --all

# Run tests
cargo test --all

# Format code
cargo fmt --all

# Run linter
cargo clippy --all
```

### Project Structure

```
lx-hw-db/
├── src/                    # Rust source code
│   ├── detectors/         # Hardware detection modules
│   ├── validation/        # Report validation system
│   └── privacy/           # Anonymization implementation
├── web/                   # Web interface
│   ├── js/               # JavaScript modules
│   ├── css/              # Stylesheets
│   └── data/             # JSON data files
├── tests/                 # Integration tests
└── docs/                  # Documentation
```

## Privacy and Security

### Data Collection Principles

1. **Minimal Collection** - Only hardware identifiers required for compatibility
2. **Immediate Anonymization** - Hashing applied at collection point
3. **No User Tracking** - No correlation between submissions
4. **Transparent Processing** - All anonymization code is open source

### Security Measures

- Input validation on all user submissions
- Command injection prevention in configuration tips
- XSS protection in web interface
- Rate limiting on API endpoints
- Automated security scanning for malicious patterns

## Contributing

### Hardware Reports

Submit hardware reports via pull request to the `hardware/` directory following the established schema.

### Configuration Tips

Community tips require:
- Multi-distribution testing
- Security review
- Performance validation
- Documentation

### Code Contributions

1. Fork the repository
2. Create a feature branch
3. Implement with tests
4. Submit pull request

## Performance Metrics

### Detection Performance

| Operation | Time | Memory |
|-----------|------|--------|
| Full hardware scan | ~2s | <50MB |
| Anonymization | <100ms | <10MB |
| Report generation | <500ms | <20MB |
| Validation | <200ms | <15MB |

### Web Interface Performance

- Initial load: <1s
- Search indexing: <500ms for 10,000 entries
- Search query: <50ms response time
- Export generation: <200ms

## Roadmap

### Phase 3 (Current)
- Complete hardware detector suite
- Report validation system
- Community submission workflow
- GitHub Actions automation

### Phase 4 (Planned)
- Advanced search capabilities
- RESTful API implementation
- CDN distribution
- Automated compatibility testing

### Phase 5 (Future)
- Machine learning recommendations
- Federated database synchronization
- Enterprise deployment features
- Hardware vendor integration

## Technical Specifications

### Supported Platforms

- Linux kernel 4.19+
- x86_64, ARM64 architectures
- systemd or OpenRC init systems

### Browser Requirements

- ES6+ JavaScript support
- WebGL for GPU detection
- Local Storage API
- Fetch API support

### Database Schema

- Git-based storage with YAML frontmatter
- Hierarchical organization by hardware category
- Vendor/device ID indexing
- Kernel version compatibility matrix

## License

GNU Affero General Public License v3.0 (AGPL-3.0)

Hardware compatibility data: Creative Commons Zero v1.0 Universal (CC0-1.0)

## Acknowledgments

Built with privacy-first principles and community collaboration. Special thanks to the Linux kernel developers, hardware vendors providing documentation, and the open source community.

## Support

Report issues: https://github.com/olafkfreund/lx-hw-db/issues

Documentation: https://github.com/olafkfreund/lx-hw-db/wiki

Community: Discussions available on GitHub Discussions