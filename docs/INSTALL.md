# Installation Guide

## System Requirements

### Hardware Detection Tool (CLI)

- **Operating System**: Linux (any modern distribution)
- **Architecture**: x86_64, ARM64
- **Rust**: 1.70+ (if building from source)
- **Memory**: 512MB RAM minimum
- **Storage**: 50MB available space

### GUI Applications

#### GTK4 Application
- **Operating System**: Linux with GTK4 support
- **Dependencies**: GTK4 4.10+, libadwaita 1.6+
- **Memory**: 1GB RAM minimum
- **Storage**: 100MB available space

#### Qt6 Application
- **Operating System**: Linux with Qt6 support  
- **Dependencies**: Qt6 6.5+, Qt6 QML modules
- **Memory**: 1GB RAM minimum
- **Storage**: 150MB available space
- **Status**: Demo version (see Known Issues)

### Database Indexer Tool

- **Operating System**: Linux, macOS, Windows
- **Architecture**: x86_64, ARM64
- **Rust**: 1.70+ (required for building)
- **Memory**: 2GB RAM minimum (for large datasets)
- **Storage**: 1GB available space

## Installation Methods

### Method 1: Pre-built Binaries (Recommended)

Download the latest release for your platform:

```bash
# Download for Linux x86_64
curl -L https://github.com/olafkfreund/lx-hw-db/releases/latest/download/lx-hw-detect-linux-x86_64.tar.gz | tar xz
sudo mv lx-hw-detect /usr/local/bin/

# Download for Linux ARM64
curl -L https://github.com/olafkfreund/lx-hw-db/releases/latest/download/lx-hw-detect-linux-arm64.tar.gz | tar xz
sudo mv lx-hw-detect /usr/local/bin/

# Verify installation
lx-hw-detect --version
```

### Method 2: Cargo Install (Rust Package Manager)

```bash
# Install from crates.io (when published)
cargo install lx-hw-detect

# Or install from GitHub directly
cargo install --git https://github.com/olafkfreund/lx-hw-db.git --bin lx-hw-detect
```

### Method 3: Build from Source

```bash
# Clone the repository
git clone https://github.com/olafkfreund/lx-hw-db.git
cd lx-hw-db

# Build CLI tools only
cargo build --release --bin lx-hw-detect
cargo build --release --bin lx-hw-indexer

# Build GTK4 GUI (requires GTK4 development libraries)
sudo apt install libgtk-4-dev libadwaita-1-dev  # Ubuntu/Debian
sudo dnf install gtk4-devel libadwaita-devel    # Fedora
cargo build --release --bin lx-hw-detect-gtk --features gtk-gui

# Build Qt6 GUI (requires Qt6 development libraries)
sudo apt install qt6-base-dev qt6-declarative-dev  # Ubuntu/Debian  
sudo dnf install qt6-qtbase-devel qt6-qtdeclarative-devel  # Fedora
cargo build --release --bin lx-hw-detect-qt6 --features qt6-gui

# Build all applications
cargo build --release --features all-gui

# Install to system
sudo cp target/release/lx-hw-detect /usr/local/bin/
sudo cp target/release/lx-hw-indexer /usr/local/bin/
sudo cp target/release/lx-hw-detect-gtk /usr/local/bin/ 2>/dev/null || true
sudo cp target/release/lx-hw-detect-qt6 /usr/local/bin/ 2>/dev/null || true
```

#### NixOS Users

```bash
# Enter development shell with all dependencies
nix develop

# Build applications in Nix environment
cargo build --release --features all-gui
```

### Method 4: Package Managers

#### Arch Linux (AUR)

```bash
# Using yay
yay -S lx-hw-detect

# Using paru
paru -S lx-hw-detect
```

#### Ubuntu/Debian (PPA)

```bash
# Add repository
sudo add-apt-repository ppa:lx-hw-db/stable
sudo apt update

# Install
sudo apt install lx-hw-detect
```

#### Fedora/RHEL (COPR)

```bash
# Enable repository
sudo dnf copr enable lx-hw-db/lx-hw-detect

# Install
sudo dnf install lx-hw-detect
```

## Post-Installation Setup

### 1. Verify Installation

```bash
# Check version
lx-hw-detect --version

# Run quick system check
lx-hw-detect --check-system
```

### 2. Set Permissions (Linux)

Some hardware detection requires elevated privileges:

```bash
# Add current user to required groups
sudo usermod -a -G audio,video,input,plugdev $USER

# Create udev rules for hardware access (optional)
sudo tee /etc/udev/rules.d/99-lx-hw-detect.rules << 'EOF'
# Allow lx-hw-detect to access hardware information
SUBSYSTEM=="pci", GROUP="plugdev", MODE="0644"
SUBSYSTEM=="usb", GROUP="plugdev", MODE="0644"
KERNEL=="event*", GROUP="input", MODE="0640"
EOF

# Reload udev rules
sudo udevadm control --reload-rules
sudo udevadm trigger
```

### 3. Configure Data Directory

```bash
# Create local data directory
mkdir -p ~/.local/share/lx-hw-detect

# Set environment variable (optional)
echo 'export LX_HW_DETECT_DATA_DIR="$HOME/.local/share/lx-hw-detect"' >> ~/.bashrc
source ~/.bashrc
```

## Configuration

### Config File Location

The configuration file is automatically created at:

- **Linux**: `~/.config/lx-hw-detect/config.toml`
- **macOS**: `~/Library/Application Support/lx-hw-detect/config.toml`
- **Windows**: `%APPDATA%\lx-hw-detect\config.toml`

### Default Configuration

```toml
# Privacy settings
[privacy]
# Enable anonymous hardware reporting
anonymous_reporting = true

# Hash system identifiers for privacy
hash_identifiers = true

# Include performance benchmarks
include_benchmarks = false

# Detection settings
[detection]
# Scan USB devices
scan_usb = true

# Scan PCI devices  
scan_pci = true

# Scan audio devices
scan_audio = true

# Scan network interfaces
scan_network = true

# Deep hardware analysis (slower)
deep_scan = false

# Output settings
[output]
# Default output format
format = "json"

# Include verbose hardware details
verbose = false

# Save reports locally
save_local = true

# Reporting settings
[reporting]
# Automatically submit compatibility reports
auto_submit = false

# GitHub repository for submissions
repository = "lx-hw-db/reports"

# Include system logs in reports
include_logs = false
```

## Troubleshooting

### Common Issues

#### Permission Denied Errors

```bash
# Run with sudo for system hardware access
sudo lx-hw-detect detect --output hardware-report.json

# Or fix permissions as described above
```

#### Missing Dependencies

```bash
# Ubuntu/Debian
sudo apt install pciutils usbutils alsa-utils

# Fedora/RHEL  
sudo dnf install pciutils usbutils alsa-utils

# Arch Linux
sudo pacman -S pciutils usbutils alsa-utils
```

#### Build Errors

```bash
# Update Rust toolchain
rustup update

# Install build dependencies
sudo apt install build-essential pkg-config libssl-dev

# Clean and rebuild
cargo clean
cargo build --release
```

### Getting Help

- **GitHub Issues**: <https://github.com/olafkfreund/lx-hw-db/issues>
- **Documentation**: <https://olafkfreund.github.io/lx-hw-db/docs>
- **Community**: <https://github.com/olafkfreund/lx-hw-db/discussions>

## Uninstallation

### Remove Binary

```bash
# If installed via package manager
sudo apt remove lx-hw-detect        # Ubuntu/Debian
sudo dnf remove lx-hw-detect        # Fedora/RHEL
sudo pacman -R lx-hw-detect         # Arch Linux

# If installed manually
sudo rm /usr/local/bin/lx-hw-detect
sudo rm /usr/local/bin/lx-hw-indexer
```

### Remove Configuration

```bash
# Remove config and data directories
rm -rf ~/.config/lx-hw-detect
rm -rf ~/.local/share/lx-hw-detect
```

### Remove Udev Rules

```bash
# Remove custom udev rules
sudo rm /etc/udev/rules.d/99-lx-hw-detect.rules
sudo udevadm control --reload-rules
```
