# Quick Start Guide

Get up and running with the Linux Hardware Compatibility Database in just 5 minutes!

## 🎯 What You'll Accomplish

- ✅ Check hardware compatibility for your system
- ✅ Understand privacy controls and data collection
- ✅ Generate your first hardware compatibility report
- ✅ Explore the web interface and community features

---

## 🔧 Option 1: Try the Web Interface (Fastest)

Perfect for checking compatibility without installing anything.

### Step 1: Open the Web Interface
Visit: **[https://olafkfreund.github.io/lx-hw-db/](https://olafkfreund.github.io/lx-hw-db/)**

### Step 2: Search for Your Hardware
1. **Search by Component**: Enter your CPU, GPU, or motherboard model
2. **Browse by Category**: Click on component categories to explore
3. **Check Compatibility**: Look for compatibility status indicators:
   - 🟢 **Full** - Excellent compatibility, no known issues
   - 🟡 **Partial** - Works with minor configuration needed
   - 🟠 **Limited** - Basic functionality only
   - 🔴 **None** - Known compatibility issues

### Step 3: Get Configuration Recommendations
1. Click on any hardware component for detailed compatibility info
2. View **Configuration Tips** from the community
3. Export configurations in your preferred format:
   - Shell scripts for immediate use
   - Ansible playbooks for automation
   - Docker configurations for containerized setups
   - NixOS modules for declarative systems

### Step 4: Contribute Browser-Detected Hardware (Optional)
1. Click **"Profile Builder"** in the top navigation
2. Allow browser hardware detection (WebGL for GPU info)
3. Review detected components and privacy settings
4. Click **"Generate Profile"** to contribute anonymized data

---

## 💻 Option 2: Use the Command Line Tool

Perfect for detailed system analysis and automated workflows.

### Step 1: Clone and Build (5 minutes)

```bash
# Clone the repository
git clone https://github.com/olafkfreund/lx-hw-db.git
cd lx-hw-db

# Build the detection tool (requires Rust 1.70+)
cargo build --release

# Alternative: Use NixOS development environment
nix develop  # If you're on NixOS
```

### Step 2: Run Your First Detection

```bash
# Quick hardware scan with basic privacy
./target/release/lx-hw-detect detect --privacy-level basic

# Comprehensive scan with maximum privacy protection
./target/release/lx-hw-detect detect --privacy-level strict --output report.yaml

# Check specific hardware compatibility
./target/release/lx-hw-detect check --device-id 10de:2206
```

**Privacy Levels Explained:**
- **Basic**: Hardware IDs are anonymized, system info generalized
- **Enhanced**: Additional noise injection, time-based salt rotation  
- **Strict**: Maximum anonymization, differential privacy applied

### Step 3: View Your Report

```bash
# View generated report
cat report.yaml

# Validate report structure
./target/release/lx-hw-detect validate report.yaml

# Analyze kernel compatibility
./target/release/lx-hw-detect analyze report.yaml
```

---

## 🖥️ Option 3: Use the GUI Applications

Perfect for interactive exploration and visual workflows.

### GTK4 Application (Recommended for Linux)

```bash
# Build and run GTK4 application
cargo build --release --features gtk-gui
./target/release/lx-hw-detect-gtk

# Or install dependencies and run directly
sudo apt install libgtk-4-dev libadwaita-1-dev  # Ubuntu/Debian
cargo run --bin lx-hw-detect-gtk --features gtk-gui
```

**GUI Features:**
- 🏠 **Welcome Screen**: Privacy overview and workflow guidance
- 🔍 **Detection Screen**: Real-time progress with tool-specific status
- 💾 **Hardware Screen**: Complete device listing with compatibility matrix
- ⚙️ **Configuration Screen**: Driver recommendations and kernel parameters
- 📤 **Submission Screen**: GitHub-integrated community contribution
- 🛡️ **Privacy Screen**: Interactive privacy controls with data preview

### Qt6 Application (Cross-Platform)

```bash
# Build Qt6 application (requires Qt6 development libraries)
cargo build --release --features qt6-gui
./target/release/lx-hw-detect-qt6
```

*Note: Qt6 application currently runs in demonstration mode due to cxx-qt integration complexity. Full functionality available in GTK4 version.*

---

## 🌐 Checking Hardware Compatibility

### Before Buying New Hardware

1. **Search Existing Database**:
   - Use the web interface to search for specific models
   - Check compatibility matrices for your target use case
   - Review community configuration tips

2. **Check Similar Hardware**:
   - Look for hardware with the same chipset/vendor
   - Review compatibility patterns across similar models
   - Check kernel version requirements

3. **Understand Compatibility Levels**:
   ```
   🟢 Full      → Buy with confidence, excellent Linux support
   🟡 Partial   → Usable with minor configuration, check tips
   🟠 Limited   → Basic functionality only, consider alternatives
   🔴 None      → Avoid or expect significant compatibility issues
   ```

### After Installing New Hardware

1. **Generate Compatibility Report**:
   ```bash
   ./target/release/lx-hw-detect detect --output new-hardware.yaml
   ```

2. **Check for Configuration Tips**:
   - Search the web interface for your specific hardware
   - Look for kernel parameter recommendations
   - Check for required driver packages

3. **Contribute Your Experience**:
   - Submit your hardware report to help others
   - Share configuration tips if you solved issues
   - Report compatibility improvements in newer kernels

---

## 🔐 Privacy-First Quick Start

Understanding data collection and privacy controls:

### What Data Is Collected?

✅ **Safe to Collect:**
- Hardware vendor IDs (anonymized with HMAC-SHA256)
- Device model numbers (anonymized)
- Kernel version and architecture
- Compatibility status and driver information

❌ **Never Collected:**
- Serial numbers or unique device identifiers
- Personal files or user data
- Network configuration or IP addresses
- Personally identifiable system information

### Privacy Controls

```bash
# View privacy settings
./target/release/lx-hw-detect config --show-privacy

# Set privacy level
./target/release/lx-hw-detect config --privacy-level strict

# Preview what data would be shared
./target/release/lx-hw-detect detect --dry-run --show-anonymization
```

### Anonymization Process

```
Your Hardware ID: "NVIDIA RTX 4090 [Device 10de:2206]"
                         ↓
HMAC-SHA256 with rotating salt
                         ↓
Anonymized ID: "sha256:a1b2c3d4e5f6..."
```

**Key Privacy Features:**
- 🔐 **Cryptographic Hashing**: All hardware IDs use HMAC-SHA256
- ⏰ **Time-Based Salts**: Salts rotate every 24 hours
- 📊 **Differential Privacy**: Statistical noise prevents individual identification
- 🎭 **K-Anonymity**: Ensure configurations appear at least 5 times in dataset

---

## 🤝 Contributing Your First Report

Help the community by sharing your hardware compatibility data:

### Automated Submission (GUI)

1. Open the GTK4 application: `cargo run --bin lx-hw-detect-gtk --features gtk-gui`
2. Complete hardware detection on the Detection Screen
3. Review privacy settings on the Privacy Screen
4. Navigate to Submission Screen
5. Click "Submit to GitHub" (requires GitHub authentication)

### Manual Submission (Advanced)

1. **Generate Report**:
   ```bash
   ./target/release/lx-hw-detect detect --output my-system-report.yaml
   ```

2. **Validate Report**:
   ```bash
   ./target/release/lx-hw-detect validate my-system-report.yaml
   ```

3. **Submit via Pull Request**:
   - Fork the repository on GitHub
   - Add your report to `hardware-reports/YYYY/MM/`
   - Create pull request with report details

---

## 🎉 What's Next?

### Explore Advanced Features

- **🔍 Advanced Search**: Use complex queries and filters in the web interface
- **📊 Statistics Dashboard**: Explore hardware trends and compatibility patterns
- **⚙️ Configuration Profiles**: Build custom configuration profiles for specific use cases
- **🏆 Community Contributions**: Participate in the contributor recognition system

### Learn More

- **[Installation Guide](installation.md)**: Comprehensive setup instructions for all platforms
- **[End User Guide](../user-guides/end-user.md)**: Detailed usage guide for desktop Linux users
- **[API Documentation](../api/README.md)**: Integrate hardware data into your applications
- **[Contributing Guide](../community/onboarding.md)**: Get involved in the community

### Get Help

- **🐛 Issues**: [GitHub Issues](https://github.com/lx-hw-db/lx-hw-db/issues)
- **💬 Discussions**: [GitHub Discussions](https://github.com/lx-hw-db/lx-hw-db/discussions)
- **📖 Documentation**: Full documentation at [docs.lx-hw-db.org](https://docs.lx-hw-db.org)
- **🤝 Community**: [Community Channels](../support/channels.md)

---

## 📋 Quick Start Checklist

- [ ] Explored web interface at https://olafkfreund.github.io/lx-hw-db/
- [ ] Searched for your hardware components
- [ ] Understood compatibility status indicators
- [ ] Generated your first hardware report (CLI or GUI)
- [ ] Reviewed privacy settings and anonymization
- [ ] Explored configuration recommendations
- [ ] (Optional) Submitted your first compatibility report
- [ ] (Optional) Joined community discussions

**🎊 Congratulations! You're now ready to make informed hardware decisions with privacy-first compatibility data.**

---

*Need help? Check our [FAQ](../support/faq.md) or ask in [GitHub Discussions](https://github.com/lx-hw-db/lx-hw-db/discussions).*