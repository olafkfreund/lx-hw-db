# End User Guide

A comprehensive guide for Linux desktop and laptop users who want to ensure optimal hardware compatibility.

## 🎯 Who This Guide Is For

This guide is designed for:
- **Linux Desktop Users**: Daily drivers of Linux desktops and laptops
- **Hardware Enthusiasts**: Users building custom systems or upgrading components
- **Linux Newcomers**: People switching from other operating systems
- **Gaming Enthusiasts**: Users optimizing systems for Linux gaming
- **Students & Developers**: Users needing reliable hardware for development work

## 🚀 Getting Started

### Understanding Hardware Compatibility

Linux hardware compatibility has improved dramatically, but challenges still exist:

**🟢 Excellent Compatibility**
- Most modern CPUs (Intel, AMD)
- Standard storage devices (SATA, NVMe)
- Common network adapters
- USB peripherals

**🟡 Usually Works with Configuration**
- Graphics cards (may need proprietary drivers)
- WiFi adapters (some require firmware)
- Audio devices (may need ALSA/PulseAudio tweaks)
- Printer and scanner devices

**🟠 Often Problematic**
- Brand new hardware (wait for kernel support)
- Specialized gaming peripherals
- Some laptop-specific features (fingerprint readers, special keys)
- Proprietary hardware with closed-source drivers

### How This Database Helps You

The Linux Hardware Compatibility Database provides:

1. **Real-World Compatibility Data**: Tested on actual systems by real users
2. **Privacy-Protected Submissions**: Your data is anonymized and secure
3. **Configuration Recommendations**: Step-by-step setup instructions
4. **Community-Driven Tips**: Solutions from experienced users
5. **Multiple Export Formats**: Get configurations in your preferred format

---

## 🌐 Using the Web Interface

The web interface is your primary tool for checking compatibility and finding configurations.

### Accessing the Interface

Visit: **[https://olafkfreund.github.io/lx-hw-db/](https://olafkfreund.github.io/lx-hw-db/)**

The interface works on:
- **Desktop Browsers**: Chrome, Firefox, Safari, Edge
- **Mobile Browsers**: Responsive design for phones and tablets
- **Offline Mode**: Core functionality works without internet connection

### Navigation Overview

**🏠 Home Page**
- Quick search bar
- Featured hardware recommendations
- Recent community contributions
- Statistics dashboard

**🔍 Search & Browse**
- Advanced search with filters
- Category browsing (CPU, GPU, Storage, etc.)
- Vendor-specific listings
- Kernel compatibility matrix

**💡 Configuration Tips**
- Community-submitted configuration guides
- Distribution-specific instructions
- Troubleshooting solutions
- Performance optimization tips

**🏆 Community**
- Contributor leaderboard
- Achievement system
- Community statistics
- Recognition badges

### Searching for Hardware

#### Quick Search

1. **Enter Hardware Details**:
   ```
   Examples:
   - "RTX 4090"
   - "AMD Ryzen 7950X"
   - "Intel AX200"
   - "Realtek ALC892"
   ```

2. **Use Device IDs**: Most precise search method
   ```
   Examples:
   - "10de:2684" (NVIDIA RTX 4090)
   - "1022:1480" (AMD Ryzen CPU)
   - "8086:2723" (Intel WiFi adapter)
   ```

3. **Browse by Category**: Explore all hardware in a specific category

#### Advanced Search Filters

**Compatibility Status**
- 🟢 Full compatibility
- 🟡 Partial compatibility (configuration needed)
- 🟠 Limited compatibility
- 🔴 No compatibility / major issues

**Kernel Version**
- Current stable kernels
- LTS (Long Term Support) kernels
- Specific kernel version ranges
- Future kernel support

**Linux Distribution**
- Ubuntu and derivatives
- Fedora and Red Hat family
- Arch Linux and derivatives
- Debian and derivatives
- SUSE family
- NixOS
- Gentoo
- Other distributions

**Use Case Filtering**
- Gaming systems
- Development workstations
- General desktop use
- Server/headless systems
- Media production
- Scientific computing

### Understanding Search Results

Each hardware entry shows:

**🏷️ Basic Information**
- Hardware name and model
- Vendor and device IDs
- Component category
- Last tested date

**📊 Compatibility Overview**
```
Status: 🟢 Full Compatibility
Kernel: 5.15+ (LTS), 6.1+ (Stable)
Driver: Built-in (i915) / Proprietary available
Tested: 147 systems, 92% success rate
```

**⚙️ Configuration Requirements**
- Required kernel modules
- Necessary firmware packages
- Configuration file changes
- Boot parameter modifications

**🤝 Community Contributions**
- User-submitted tips and solutions
- Known issues and workarounds
- Performance optimization suggestions
- Distribution-specific notes

---

## 💻 Using the CLI Tool

For advanced users who prefer command-line tools or need scripted hardware detection.

### Installation

#### Option 1: Build from Source
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/lx-hw-db/lx-hw-db.git
cd lx-hw-db
cargo build --release

# The binary will be at: ./target/release/lx-hw-detect
```

#### Option 2: Distribution Packages (When Available)
```bash
# Ubuntu/Debian (future)
sudo apt install lx-hw-detect

# Fedora (future)
sudo dnf install lx-hw-detect

# Arch Linux (AUR - future)
yay -S lx-hw-detect

# NixOS
nix-env -iA nixpkgs.lx-hw-detect
```

### Basic Usage

#### Generate Hardware Report

```bash
# Basic hardware detection
lx-hw-detect detect

# Specify output file
lx-hw-detect detect --output my-system.yaml

# Choose output format
lx-hw-detect detect --format json --output my-system.json
lx-hw-detect detect --format markdown --output my-system.md
```

#### Check Specific Hardware

```bash
# Check by device ID
lx-hw-detect check --device-id 10de:2684

# Check by vendor and model
lx-hw-detect check --vendor nvidia --model "RTX 4090"

# Check kernel compatibility
lx-hw-detect check --kernel 6.1 --device-id 10de:2684
```

#### Privacy Controls

```bash
# Set privacy level
lx-hw-detect config --privacy-level strict

# View current privacy settings
lx-hw-detect config --show-privacy

# Preview anonymization without submitting
lx-hw-detect detect --dry-run
```

### Advanced CLI Features

#### Hardware Analysis

```bash
# Analyze hardware report for issues
lx-hw-detect analyze my-system.yaml

# Check for kernel compatibility issues
lx-hw-detect analyze --check-kernel-compat my-system.yaml

# Performance analysis
lx-hw-detect analyze --performance-check my-system.yaml
```

#### Configuration Generation

```bash
# Generate kernel parameters
lx-hw-detect generate-config --type kernel-params my-system.yaml

# Generate module configuration
lx-hw-detect generate-config --type modules my-system.yaml

# Generate Xorg configuration
lx-hw-detect generate-config --type xorg my-system.yaml
```

#### Batch Operations

```bash
# Detect multiple systems
for host in server1 server2 server3; do
    ssh $host "lx-hw-detect detect" > $host-hardware.yaml
done

# Compare hardware reports
lx-hw-detect compare system1.yaml system2.yaml

# Merge multiple reports
lx-hw-detect merge --output combined.yaml system1.yaml system2.yaml
```

---

## 🖥️ GUI Applications

User-friendly graphical interfaces for interactive hardware detection and community participation.

### GTK4 Application

The recommended GUI for Linux desktop environments.

#### Installation and Launch

```bash
# Build with GTK4 support
cargo build --release --features gtk-gui

# Launch application
./target/release/lx-hw-detect-gtk
```

#### Application Screens

**🏠 Welcome Screen**
- Project overview and privacy explanation
- Quick start workflow guidance
- Privacy level selection
- Getting started tutorials

**🔍 Detection Screen**
- Real-time hardware detection progress
- Tool-specific status (lshw, dmidecode, lspci, etc.)
- Progress bars and status indicators
- Detailed logging and error reporting

**💾 Hardware Screen**
- Complete detected hardware listing
- Interactive compatibility matrix
- Device-specific details and specifications
- Compatibility status visualization

**⚙️ Configuration Screen**
- Hardware-specific driver recommendations
- Kernel parameter suggestions
- Package installation commands
- Configuration file previews

**📤 Submission Screen**
- GitHub-integrated community submission
- Privacy preview and confirmation
- Submission status and validation
- Contribution tracking

**🛡️ Privacy Screen**
- Interactive privacy level adjustment
- Data anonymization preview
- Privacy implications explanation
- Custom privacy settings

#### GTK4 Features

- **Material Design**: Modern, clean interface following GNOME design principles
- **Responsive Layout**: Adapts to different window sizes and orientations
- **Accessibility**: Full keyboard navigation and screen reader support
- **Theme Integration**: Follows system light/dark theme preferences
- **Multi-Language Support**: Internationalization ready (English default)

### Qt6 Application

Cross-platform GUI application with Material Design 3 theming.

#### Current Status

⚠️ **Note**: The Qt6 application currently runs in demonstration mode due to cxx-qt integration complexity with Qt6 6.9+. All interface designs are complete and functional, but backend integration is pending cxx-qt updates.

#### Launch Demo

```bash
# Build and run Qt6 demo
cargo build --release --features qt6-gui
./target/release/lx-hw-detect-qt6
```

The demo showcases:
- Complete Material Design 3 interface
- All planned screens and workflows
- Privacy-focused theming and interactions
- Cross-platform compatibility design

---

## 🔧 Hardware-Specific Guidance

### Graphics Cards (GPUs)

Graphics compatibility is often the most complex aspect of Linux hardware.

#### NVIDIA GPUs

**🟢 Excellent Support (Recent Cards)**
- RTX 40-series (Ada Lovelace)
- RTX 30-series (Ampere)
- GTX 16-series (Turing)

**Configuration Requirements:**
```bash
# Install NVIDIA proprietary drivers
sudo apt install nvidia-driver-535  # Ubuntu
sudo dnf install akmod-nvidia        # Fedora

# Enable DRM kernel mode setting
echo "nvidia_drm.modeset=1" | sudo tee -a /etc/default/grub
sudo update-grub && sudo reboot
```

**🟡 Good Support with Configuration (Older Cards)**
- GTX 10-series (Pascal)
- GTX 9-series (Maxwell)

**Common Issues and Solutions:**
- **Screen Tearing**: Enable ForceFullCompositionPipeline
- **Multiple Monitor Issues**: Configure Xorg with nvidia-settings
- **Wayland Compatibility**: Requires GBM support (driver 495+)

#### AMD GPUs

**🟢 Excellent Open-Source Support**
- RX 7000-series (RDNA 3)
- RX 6000-series (RDNA 2)
- RX 5000-series (RDNA 1)

**Configuration (Usually Automatic):**
```bash
# Drivers included in kernel
# No additional installation typically needed

# For latest features, install Mesa drivers
sudo apt install mesa-vulkan-drivers  # Ubuntu
```

**🟡 Older GPUs**
- R9/RX 400-500 series (GCN 4.0)
- May require amdgpu driver setup

#### Intel Integrated Graphics

**🟢 Excellent Built-in Support**
- Intel Xe (12th gen+)
- Intel UHD (8th-11th gen)
- Intel HD (4th-7th gen)

**Usually works out of the box with:**
- i915 kernel driver (automatic)
- Mesa drivers (included in most distributions)

### WiFi Adapters

WiFi compatibility depends heavily on chipset manufacturer.

#### Intel WiFi (Recommended)

**🟢 Excellent Support**
- AX series (WiFi 6/6E)
- AC series (WiFi 5)
- N series (WiFi 4)

**Firmware Installation:**
```bash
# Usually included, but if needed:
sudo apt install firmware-iwlwifi    # Debian/Ubuntu
sudo dnf install iwl*-firmware       # Fedora
```

#### Realtek WiFi

**🟡 Mixed Support**
- **Good**: RTL8188EU, RTL8192EU
- **Poor**: RTL8821CE (common in laptops)

**Solutions for Problematic Realtek:**
```bash
# Install additional drivers
sudo apt install rtl8821ce-dkms     # Ubuntu
# Or compile from GitHub repositories
```

#### Broadcom WiFi

**🟠 Often Problematic**
- Requires proprietary firmware
- Limited distribution support

**Solutions:**
```bash
# Ubuntu/Debian
sudo apt install broadcom-sta-dkms

# Enable additional repositories
sudo apt install software-properties-common
sudo add-apt-repository multiverse
sudo apt update && sudo apt install bcmwl-kernel-source
```

### Audio Hardware

Modern Linux audio is generally well-supported.

#### Built-in Audio

**🟢 Usually Works**
- Intel HDA
- Realtek ALC codecs
- VIA VT17xx series

**Configuration Tools:**
- **PulseAudio**: Most distributions (GUI: pavucontrol)
- **PipeWire**: Modern distributions (GUI: helvum, qpwgraph)
- **ALSA**: Low-level configuration (alsamixer)

#### USB Audio Interfaces

**🟢 Class-Compliant Devices**
- Most modern USB audio interfaces
- No additional drivers needed

**🟡 Requires Configuration**
- Professional audio interfaces
- May need Jack configuration
- Latency optimization needed

---

## 📱 Mobile and Hardware Detection

### Browser-Based Hardware Detection

The web interface can detect some hardware directly in your browser:

**✅ What Can Be Detected:**
- GPU model and vendor (via WebGL)
- CPU architecture and core count
- Screen resolution and color depth
- Available memory (approximate)
- Browser capabilities

**❌ What Cannot Be Detected:**
- Detailed hardware specifications
- Kernel modules and compatibility
- Storage devices and networking hardware
- BIOS/UEFI information

### Using Mobile Devices

The web interface works on mobile devices for:
- **Research**: Looking up hardware before purchasing
- **Comparison**: Comparing different hardware options
- **Community**: Participating in discussions and forums
- **Reading**: Accessing configuration tips and guides

---

## 🔐 Privacy and Security for End Users

### Understanding Data Collection

**🛡️ What We Protect:**
- Your personal identity
- System serial numbers
- Network configuration
- User files and data
- Location information

**📊 What We Collect (Anonymized):**
- Hardware vendor and model IDs (hashed)
- Kernel version and architecture
- Compatibility test results
- Performance characteristics

### Privacy Controls

#### Privacy Levels Explained

**Basic Privacy**
- Hardware IDs anonymized with HMAC-SHA256
- System information generalized
- Timestamps rounded to nearest hour
- Suitable for most home users

**Enhanced Privacy**
- Additional statistical noise injection
- Time-based salt rotation (24 hours)
- IP address never logged
- Suitable for privacy-conscious users

**Strict Privacy**
- Maximum anonymization and noise
- Differential privacy techniques
- K-anonymity enforcement (k ≥ 5)
- Suitable for enterprise or sensitive environments

#### Controlling Your Data

```bash
# View current privacy settings
lx-hw-detect config --show-privacy

# Change privacy level
lx-hw-detect config --privacy-level enhanced

# Preview data before submission
lx-hw-detect detect --dry-run --show-anonymization

# Local-only detection (no submission)
lx-hw-detect detect --local-only
```

### Security Best Practices

**For CLI Usage:**
```bash
# Always validate downloads
curl -L https://github.com/lx-hw-db/lx-hw-db/releases/latest/lx-hw-detect.sha256
sha256sum lx-hw-detect && compare with published hash

# Run with minimal privileges
# No root access required for detection
./lx-hw-detect detect

# Review reports before submission
less my-hardware-report.yaml
```

**For Web Usage:**
- Use the official website: https://olafkfreund.github.io/lx-hw-db/
- Verify HTTPS connection and certificate
- Review browser permissions before hardware detection
- Clear browser data after use on shared computers

---

## 🎯 Use Case Examples

### Scenario 1: Building a Gaming PC

**Goal**: Ensure all components work well for Linux gaming.

**Step 1**: Research components in the database
```
Search: "RTX 4090" → Check compatibility with your target distribution
Search: "AMD Ryzen 7950X" → Verify no known issues
Search: "MSI B650 motherboard" → Check for BIOS/audio issues
```

**Step 2**: Check gaming-specific compatibility
- Look for Steam Deck verified games compatibility
- Check for Proton/Wine performance notes
- Verify VRR/G-Sync/FreeSync support

**Step 3**: Plan your configuration
- Export configuration as shell script
- Review kernel parameters for gaming optimization
- Plan driver installation sequence

### Scenario 2: Upgrading Laptop WiFi

**Goal**: Replace problematic WiFi card with compatible one.

**Step 1**: Identify current hardware
```bash
lx-hw-detect detect --component wifi
# Or check in GUI: Hardware Screen → Network Adapters
```

**Step 2**: Research replacement options
- Search database for Intel AX200/AX210 (recommended)
- Check laptop compatibility (Mini PCIe vs M.2)
- Verify antenna connector compatibility

**Step 3**: Confirm compatibility
- Check if firmware is available in your distribution
- Verify kernel module support
- Look for user success stories

### Scenario 3: Switching from Windows

**Goal**: Ensure your current Windows PC will work with Linux.

**Step 1**: Inventory existing hardware
- Use Windows Device Manager to get device IDs
- Search database for each component
- Identify potential problem areas

**Step 2**: Create compatibility plan
- List components that need attention
- Plan driver installation sequence
- Identify firmware requirements

**Step 3**: Test with live distribution
- Create bootable USB with your planned distribution
- Test all hardware functions
- Generate hardware report and compare with database

### Scenario 4: Enterprise Workstation Deployment

**Goal**: Validate hardware for company-wide Linux deployment.

**Step 1**: Test representative systems
```bash
# Generate reports for each system type
lx-hw-detect detect --output workstation-type1.yaml
lx-hw-detect detect --output workstation-type2.yaml
```

**Step 2**: Analyze compatibility across fleet
```bash
# Compare different systems
lx-hw-detect compare workstation-*.yaml

# Check for common issues
lx-hw-detect analyze --report-problems workstation-*.yaml
```

**Step 3**: Generate deployment configurations
- Export Ansible playbooks for automated setup
- Create standard kernel parameter configurations
- Document known issues and workarounds

---

## 🆘 Troubleshooting Common Issues

### Hardware Not Detected

**Problem**: CLI tool doesn't detect some hardware

**Solutions**:
1. **Check tool permissions**:
   ```bash
   # Some tools need root access for full detection
   sudo lx-hw-detect detect --tools dmidecode,lspci
   ```

2. **Verify tools are installed**:
   ```bash
   # Install missing detection tools
   sudo apt install lshw dmidecode pciutils usbutils  # Ubuntu
   sudo dnf install lshw dmidecode pciutils usbutils  # Fedora
   ```

3. **Check kernel modules**:
   ```bash
   # Ensure hardware modules are loaded
   lsmod | grep -i [your-hardware]
   ```

### GUI Application Won't Start

**Problem**: GTK4 application fails to launch

**Solutions**:
1. **Install GTK4 dependencies**:
   ```bash
   sudo apt install libgtk-4-1 libadwaita-1-0  # Ubuntu
   sudo dnf install gtk4 libadwaita            # Fedora
   ```

2. **Check Wayland/X11 compatibility**:
   ```bash
   # Force X11 if Wayland causes issues
   GDK_BACKEND=x11 ./lx-hw-detect-gtk
   ```

3. **Verify Rust installation**:
   ```bash
   rustc --version  # Should be 1.70+
   cargo --version
   ```

### Privacy Concerns

**Problem**: Worried about data privacy

**Solutions**:
1. **Use local-only mode**:
   ```bash
   lx-hw-detect detect --local-only
   ```

2. **Review anonymization**:
   ```bash
   lx-hw-detect detect --dry-run --show-anonymization
   ```

3. **Set maximum privacy**:
   ```bash
   lx-hw-detect config --privacy-level strict
   ```

### Web Interface Issues

**Problem**: Search not working or slow performance

**Solutions**:
1. **Clear browser cache**: Refresh the page with Ctrl+F5
2. **Disable browser extensions**: Test in incognito/private mode
3. **Check JavaScript**: Ensure JavaScript is enabled
4. **Try different browser**: Test with Firefox/Chrome/Safari

---

## 📚 Additional Resources

### Learning More About Hardware Compatibility

**📖 Recommended Reading**:
- [Linux Hardware Database (LHD)](https://linux-hardware.org)
- [Arch Linux Hardware Compatibility Lists](https://wiki.archlinux.org/title/Hardware_compatibility_list)
- [Ubuntu Hardware Certification](https://ubuntu.com/certified)
- [Linux Kernel Documentation](https://www.kernel.org/doc/Documentation/)

**🎥 Video Tutorials** (Community Created):
- Hardware detection workflow demonstrations
- GUI application usage guides
- Privacy settings explanations
- Community contribution walkthroughs

**🤝 Community Resources**:
- [r/linuxhardware subreddit](https://reddit.com/r/linuxhardware)
- [Linux Hardware Support Discord](https://discord.gg/linuxhardware)
- Distribution-specific forums and wikis

### Staying Updated

**📧 Notifications**:
- Subscribe to hardware compatibility updates
- Get notified when your hardware gets new compatibility data
- Receive alerts about driver updates

**📱 Mobile Apps** (Future):
- Hardware compatibility checker mobile app
- QR code scanning for quick hardware lookup
- Offline database for checking compatibility on-the-go

---

## ✅ End User Checklist

After reading this guide, you should be able to:

- [ ] Search for hardware compatibility using the web interface
- [ ] Understand compatibility status indicators and their meanings
- [ ] Generate hardware reports using CLI tools or GUI applications
- [ ] Configure privacy settings appropriate for your needs
- [ ] Find and apply community configuration tips
- [ ] Export configurations in multiple formats (shell, Ansible, etc.)
- [ ] Contribute your own hardware compatibility data
- [ ] Troubleshoot common hardware detection issues
- [ ] Plan hardware purchases with confidence
- [ ] Help other community members with similar hardware

**🎉 Ready to Go!**

You now have comprehensive knowledge for using the Linux Hardware Compatibility Database as an end user. Whether you're building a new system, upgrading components, or troubleshooting compatibility issues, you have the tools and knowledge to make informed decisions.

---

**💬 Need Help?**
- **🐛 Bug Reports**: [GitHub Issues](https://github.com/lx-hw-db/lx-hw-db/issues)
- **❓ Questions**: [GitHub Discussions](https://github.com/lx-hw-db/lx-hw-db/discussions)
- **📖 More Docs**: [Documentation Home](../README.md)
- **🤝 Community**: [Community Channels](../support/channels.md)

*This guide is community-maintained and regularly updated. Last updated: 2025-08-27*