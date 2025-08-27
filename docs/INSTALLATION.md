# Installation Guide

This guide provides installation instructions for lx-hw-db across major Linux distributions.

## Quick Install

### Debian/Ubuntu

```bash
# Download and install .deb package
curl -LO https://github.com/lx-hw-db/lx-hw-db/releases/latest/download/lx-hw-db_0.1.0-1_amd64.deb
sudo dpkg -i lx-hw-db_0.1.0-1_amd64.deb

# Install dependencies if needed
sudo apt install -f
```

### Arch Linux

```bash
# Install from AUR using yay
yay -S lx-hw-db

# Or using paru
paru -S lx-hw-db

# Manual installation from AUR
git clone https://aur.archlinux.org/lx-hw-db.git
cd lx-hw-db
makepkg -si
```

### Fedora/RHEL/CentOS

```bash
# Download and install .rpm package
curl -LO https://github.com/lx-hw-db/lx-hw-db/releases/latest/download/lx-hw-db-0.1.0-1.x86_64.rpm
sudo rpm -ivh lx-hw-db-0.1.0-1.x86_64.rpm

# Or using dnf
sudo dnf install https://github.com/lx-hw-db/lx-hw-db/releases/latest/download/lx-hw-db-0.1.0-1.x86_64.rpm
```

### NixOS

Add to your `configuration.nix`:

```nix
{ config, pkgs, ... }:

{
  # System-wide installation
  environment.systemPackages = with pkgs; [
    lx-hw-db
  ];
  
  # Enable the service (optional)
  services.lx-hw-db = {
    enable = true;
    web.enable = true;  # Enable web interface
    privacyLevel = "basic";
  };
}
```

Or for user installation with Home Manager:

```nix
{ config, pkgs, ... }:

{
  programs.lx-hw-db = {
    enable = true;
    settings = {
      privacyLevel = "enhanced";
      enableKernelAnalysis = true;
    };
    enablePeriodicDetection = true;
  };
}
```

## Detailed Installation Instructions

### Prerequisites

All distributions require these hardware detection tools:

- `lshw` - Hardware information tool
- `dmidecode` - BIOS/motherboard information (requires root)
- `pciutils` - PCI device information
- `usbutils` - USB device information
- `util-linux` - System utilities

Optional tools for enhanced detection:
- `inxi` - System information script

### Debian/Ubuntu

#### Method 1: Package Installation (Recommended)

```bash
# Add dependencies
sudo apt update
sudo apt install lshw dmidecode pciutils usbutils util-linux

# Download and install package
wget https://github.com/lx-hw-db/lx-hw-db/releases/latest/download/lx-hw-db_0.1.0-1_amd64.deb
sudo dpkg -i lx-hw-db_0.1.0-1_amd64.deb

# Verify installation
lx-hw-detect --version
```

#### Method 2: Build from Source

```bash
# Install build dependencies
sudo apt update
sudo apt install build-essential cargo rustc libssl-dev pkg-config git

# Clone and build
git clone https://github.com/lx-hw-db/lx-hw-db.git
cd lx-hw-db
cargo build --release

# Install binaries
sudo cp target/release/lx-hw-detect /usr/local/bin/
sudo cp target/release/lx-hw-indexer /usr/local/bin/
```

### Arch Linux

#### Method 1: AUR Installation (Recommended)

```bash
# Using yay AUR helper
yay -S lx-hw-db

# Using paru AUR helper  
paru -S lx-hw-db

# Dependencies are automatically installed
```

#### Method 2: Manual AUR Build

```bash
# Clone AUR package
git clone https://aur.archlinux.org/lx-hw-db.git
cd lx-hw-db

# Review PKGBUILD (important for security)
less PKGBUILD

# Build and install
makepkg -si
```

#### Method 3: Build from Source

```bash
# Install dependencies
sudo pacman -S rust cargo openssl pkgconf lshw dmidecode pciutils usbutils

# Build from source
git clone https://github.com/lx-hw-db/lx-hw-db.git
cd lx-hw-db
cargo build --release

# Install
sudo cp target/release/lx-hw-detect /usr/bin/
sudo cp target/release/lx-hw-indexer /usr/bin/
```

### Fedora/RHEL/CentOS

#### Method 1: RPM Installation (Recommended)

```bash
# Install dependencies
sudo dnf install lshw dmidecode pciutils usbutils util-linux

# Download and install RPM
curl -LO https://github.com/lx-hw-db/lx-hw-db/releases/latest/download/lx-hw-db-0.1.0-1.x86_64.rpm
sudo rpm -ivh lx-hw-db-0.1.0-1.x86_64.rpm

# Or install directly with dnf
sudo dnf install https://github.com/lx-hw-db/lx-hw-db/releases/latest/download/lx-hw-db-0.1.0-1.x86_64.rpm
```

#### Method 2: Build from Source

```bash
# Install build dependencies
sudo dnf install rust cargo openssl-devel pkgconfig gcc git

# Clone and build
git clone https://github.com/lx-hw-db/lx-hw-db.git
cd lx-hw-db
cargo build --release

# Install binaries
sudo cp target/release/lx-hw-detect /usr/bin/
sudo cp target/release/lx-hw-indexer /usr/bin/
```

### NixOS

#### Method 1: System Configuration

Add to `/etc/nixos/configuration.nix`:

```nix
{ config, pkgs, ... }:

{
  environment.systemPackages = with pkgs; [
    lx-hw-db
  ];
  
  # Optional: Enable system service
  services.lx-hw-db = {
    enable = true;
    privacyLevel = "basic";
    web = {
      enable = true;
      host = "0.0.0.0";  # Bind to all interfaces
      port = 8080;
    };
  };
  
  # Open firewall port for web interface
  networking.firewall.allowedTCPPorts = [ 8080 ];
}
```

Then rebuild your system:
```bash
sudo nixos-rebuild switch
```

#### Method 2: Home Manager

Add to your home configuration:

```nix
{ config, pkgs, ... }:

{
  programs.lx-hw-db = {
    enable = true;
    settings = {
      privacyLevel = "enhanced";
      enableKernelAnalysis = true;
      defaultFormat = "yaml";
    };
    
    # Enable periodic detection
    enablePeriodicDetection = true;
    periodicDetectionCalendar = "daily";
    
    # Shell integration
    enableBashIntegration = true;
    enableZshIntegration = true;
  };
}
```

#### Method 3: Nix Package Manager

```bash
# Install with nix-env
nix-env -iA nixpkgs.lx-hw-db

# Or with nix profile (Nix 2.4+)
nix profile install nixpkgs#lx-hw-db
```

### openSUSE

#### Method 1: Build from Source

```bash
# Install dependencies
sudo zypper install rust cargo libopenssl-devel pkg-config gcc git lshw dmidecode pciutils usbutils

# Clone and build
git clone https://github.com/lx-hw-db/lx-hw-db.git
cd lx-hw-db
cargo build --release

# Install binaries
sudo cp target/release/lx-hw-detect /usr/bin/
sudo cp target/release/lx-hw-indexer /usr/bin/
```

### Gentoo

#### Method 1: Build from Source

```bash
# Install dependencies
sudo emerge --ask dev-lang/rust sys-apps/lshw sys-apps/dmidecode sys-apps/pciutils sys-apps/usbutils

# Clone and build
git clone https://github.com/lx-hw-db/lx-hw-db.git
cd lx-hw-db
cargo build --release

# Install binaries
sudo cp target/release/lx-hw-detect /usr/bin/
sudo cp target/release/lx-hw-indexer /usr/bin/
```

## Post-Installation

### Configuration

After installation, you can customize the configuration:

```bash
# System-wide configuration
sudo cp /usr/share/lx-hw-db/examples/config.toml /etc/lx-hw-db/config.toml
sudo $EDITOR /etc/lx-hw-db/config.toml

# User-specific configuration
mkdir -p ~/.config/lx-hw-db
cp /usr/share/lx-hw-db/examples/config.toml ~/.config/lx-hw-db/config.toml
$EDITOR ~/.config/lx-hw-db/config.toml
```

### Verification

Test your installation:

```bash
# Check version
lx-hw-detect --version

# Check available tools
lx-hw-detect check

# Perform basic hardware detection
lx-hw-detect detect --privacy basic

# Generate configuration recommendations
lx-hw-detect recommend --distribution $(lsb_release -si)
```

### Enable Web Interface (Optional)

To enable the web interface:

```bash
# Edit configuration to enable web interface
sudo $EDITOR /etc/lx-hw-db/config.toml

# Set in [web] section:
# enabled = true
# host = "127.0.0.1"
# port = 8080

# Start the service
sudo systemctl enable --now lx-hw-db-server

# Access web interface
xdg-open http://localhost:8080
```

## Troubleshooting

### Common Issues

1. **Permission denied errors**
   ```bash
   # Some tools require root privileges
   sudo lx-hw-detect detect
   ```

2. **Missing tools**
   ```bash
   # Check which tools are available
   lx-hw-detect check
   
   # Install missing dependencies
   # Ubuntu/Debian: sudo apt install lshw dmidecode pciutils usbutils
   # Fedora: sudo dnf install lshw dmidecode pciutils usbutils
   # Arch: sudo pacman -S lshw dmidecode pciutils usbutils
   ```

3. **Configuration file not found**
   ```bash
   # Create default configuration
   mkdir -p ~/.config/lx-hw-db
   lx-hw-detect config --generate > ~/.config/lx-hw-db/config.toml
   ```

### Getting Help

- Documentation: `man lx-hw-detect`
- Help: `lx-hw-detect --help`
- Issues: https://github.com/lx-hw-db/lx-hw-db/issues
- Community: https://github.com/lx-hw-db/lx-hw-db/discussions

## Uninstallation

### Debian/Ubuntu
```bash
sudo apt remove lx-hw-db
```

### Arch Linux
```bash
sudo pacman -R lx-hw-db
```

### Fedora/RHEL/CentOS
```bash
sudo dnf remove lx-hw-db
# or
sudo rpm -e lx-hw-db
```

### NixOS
Remove from your configuration and rebuild, or:
```bash
nix-env -e lx-hw-db
```

### Manual Cleanup
```bash
# Remove user data
rm -rf ~/.local/share/lx-hw-db
rm -rf ~/.config/lx-hw-db
rm -rf ~/.cache/lx-hw-db

# Remove system files (if manually installed)
sudo rm -f /usr/bin/lx-hw-detect /usr/bin/lx-hw-indexer
sudo rm -rf /etc/lx-hw-db
```