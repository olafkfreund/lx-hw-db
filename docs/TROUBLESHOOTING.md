# Troubleshooting Guide

This guide helps you diagnose and fix common issues when using the Linux Hardware Compatibility Database tools.

## Table of Contents

- [Installation Issues](#installation-issues)
- [Hardware Detection Problems](#hardware-detection-problems)
- [Report Generation Failures](#report-generation-failures)
- [Validation Errors](#validation-errors)
- [Submission Problems](#submission-problems)
- [Performance Issues](#performance-issues)
- [Platform-Specific Issues](#platform-specific-issues)

## Installation Issues

### Rust Compilation Errors

#### Error: "error: package `lx-hw-detect` cannot be built because it requires rustc 1.70"

**Cause**: Your Rust toolchain is outdated.

**Solution**:
```bash
# Update Rust to the latest stable version
rustup update stable

# Verify version
rustc --version  # Should show 1.70+

# Try building again
cargo build --release --bin lx-hw-detect
```

#### Error: "linking with `cc` failed: exit status: 1"

**Cause**: Missing system development libraries.

**Solution**:

**Ubuntu/Debian**:
```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
cargo build --release --bin lx-hw-detect
```

**Fedora/RHEL**:
```bash
sudo dnf install gcc openssl-devel pkgconf-pkg-config
cargo build --release --bin lx-hw-detect
```

**Arch Linux**:
```bash
sudo pacman -S base-devel openssl pkgconf
cargo build --release --bin lx-hw-detect
```

#### Error: "failed to load source for dependency"

**Cause**: Network issues or dependency problems.

**Solution**:
```bash
# Clear cargo cache
cargo clean

# Update cargo index
cargo update

# Try offline build if you have dependencies cached
cargo build --release --offline
```

## Hardware Detection Problems

### No Hardware Detected

#### Error: "No hardware detected by any detector"

**Diagnostic Steps**:
```bash
# Check if detection tools are available
which lspci    # Should show path to lspci
which lsusb    # Should show path to lsusb
which inxi     # May not be installed

# Test tools manually
lspci -v       # Should show PCI devices
lsusb -t       # Should show USB devices

# Run detection with verbose output
./target/release/lx-hw-detect detect --verbose
```

**Solutions**:

1. **Install missing tools**:
   ```bash
   # Ubuntu/Debian
   sudo apt install pciutils usbutils inxi
   
   # Fedora/RHEL
   sudo dnf install pciutils usbutils inxi
   
   # Arch Linux
   sudo pacman -S pciutils usbutils inxi
   ```

2. **Check permissions**:
   ```bash
   # Some hardware info requires root access
   sudo ./target/release/lx-hw-detect detect --verbose
   ```

3. **Virtual machine limitations**:
   ```bash
   # VMs may not expose all hardware
   # Try with --allow-virtual flag (if implemented)
   ./target/release/lx-hw-detect detect --verbose
   ```

### Partial Hardware Detection

#### Issue: Some hardware components not detected

**Diagnostic**:
```bash
# Check specific detector outputs
lspci -v | grep -i "your-missing-hardware"
lsusb -v | grep -i "your-missing-hardware"
inxi -F   # Full system information
```

**Common Causes and Solutions**:

1. **Driver not loaded**:
   ```bash
   # Check loaded modules
   lsmod | grep -i "hardware-name"
   
   # Load missing driver
   sudo modprobe driver-name
   ```

2. **Hardware not powered**:
   - Check BIOS/UEFI settings
   - Ensure hardware is properly seated
   - Check power connections

3. **Unsupported hardware**:
   - Check if hardware requires proprietary drivers
   - Look for alternative driver options
   - Hardware may need firmware updates

## Report Generation Failures

### JSON Serialization Errors

#### Error: "Failed to serialize hardware report"

**Cause**: Usually internal data structure issues.

**Diagnostic**:
```bash
# Run with debug output
RUST_LOG=debug ./target/release/lx-hw-detect detect --format json

# Try alternative format for comparison
./target/release/lx-hw-detect detect --format yaml
```

**Solution**:
```bash
# Try with minimal detection
./target/release/lx-hw-detect detect --format json --minimal

# Report the issue with debug output
# Include the debug output in your bug report
```

### Privacy Anonymization Failures

#### Error: "Failed to anonymize system identifiers"

**Cause**: Issues with system ID generation or HMAC calculation.

**Diagnostic**:
```bash
# Check system identifiers manually
hostname
cat /etc/machine-id   # System UUID
ip link show          # Network interfaces
```

**Solution**:
```bash
# Try with different privacy level
./target/release/lx-hw-detect detect --privacy basic

# Generate machine-id if missing
sudo systemd-machine-id-setup
```

## Validation Errors

### Schema Validation Failures

#### Error: "Schema validation failed"

**Diagnostic**:
```bash
# Run validation with verbose output
./target/release/lx-hw-detect validate report.json --verbose

# Check JSON syntax
jq . report.json > /dev/null && echo "Valid JSON" || echo "Invalid JSON"
```

**Common Issues**:

1. **Missing required fields**:
   ```bash
   # Check report structure
   jq '.metadata' report.json  # Should show metadata
   jq '.system' report.json    # Should show system info
   jq '.hardware' report.json  # Should show hardware
   ```

2. **Invalid field values**:
   ```bash
   # Check specific fields mentioned in error
   jq '.system.kernel_version' report.json
   jq '.metadata.privacy_level' report.json
   ```

**Solution**: Regenerate the report with latest tool version:
```bash
./target/release/lx-hw-detect detect --format json --output new-report.json
```

### Privacy Validation Failures

#### Error: "Potential personally identifiable information detected"

**Diagnostic**:
```bash
# Check for common PII patterns
grep -E '[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}' report.json  # Email
grep -E '[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}' report.json  # IP addresses
grep -i "username\|hostname" report.json                               # Usernames/hostnames
```

**Solutions**:

1. **Use stricter privacy level**:
   ```bash
   ./target/release/lx-hw-detect detect --privacy strict --output report.json
   ```

2. **Clean up system before detection**:
   ```bash
   # Remove temporary files with personal info
   # Set generic hostname temporarily
   sudo hostname generic-system
   ./target/release/lx-hw-detect detect --privacy enhanced
   ```

3. **Manual cleanup** (not recommended):
   ```bash
   # Only if you understand the implications
   # Replace specific PII found in report
   sed -i 's/your-email@example.com/[EMAIL-REMOVED]/g' report.json
   ```

## Submission Problems

### File Naming Convention Errors

#### Error: "File naming convention violation"

**Diagnostic**:
```bash
# Check your filename format
ls -la your-report.json

# Expected format: YYYY-MM-DD_KERNEL-VERSION_ARCH_SYSTEM-ID.json
# Example: 2025-08-26_6.16.0_x86_64_abc123def456.json
```

**Solution**:
```bash
# Extract information from report
SYSTEM_ID=$(jq -r '.metadata.anonymized_system_id' report.json)
KERNEL_VERSION=$(jq -r '.system.kernel_version' report.json)
ARCH=$(jq -r '.system.architecture' report.json)
DATE=$(date +%Y-%m-%d)

# Rename file correctly
mv report.json "${DATE}_${KERNEL_VERSION}_${ARCH}_${SYSTEM_ID}.json"
```

### Directory Structure Errors

#### Error: "File in wrong directory"

**Diagnostic**:
```bash
# Check current file location
find . -name "*.json" -type f

# Expected: hardware-reports/YYYY/MM/filename.json
```

**Solution**:
```bash
# Create correct directory structure
DATE=$(echo your-filename.json | cut -d'_' -f1)
YEAR=$(echo $DATE | cut -d'-' -f1)
MONTH=$(echo $DATE | cut -d'-' -f2)

mkdir -p hardware-reports/$YEAR/$MONTH
mv your-filename.json hardware-reports/$YEAR/$MONTH/
```

### GitHub Actions Failures

#### Automated validation fails

**Diagnostic Steps**:

1. **Check the Actions tab** in your fork's GitHub repository
2. **Click on the failed workflow** to see detailed logs
3. **Look for specific error messages** in the validation steps

**Common Issues**:

1. **Validation command fails**:
   - Your report doesn't pass local validation
   - Run `./target/release/lx-hw-detect validate` locally first

2. **File permissions**:
   ```bash
   # Fix file permissions before committing
   chmod 644 your-report.json
   git add your-report.json
   git commit --amend --no-edit
   git push --force
   ```

3. **Branch protection**:
   - Ensure you're creating a PR from a feature branch
   - Don't push directly to main branch

## Performance Issues

### Slow Hardware Detection

#### Issue: Detection takes a very long time

**Diagnostic**:
```bash
# Time the detection process
time ./target/release/lx-hw-detect detect --format json

# Check system load
top
htop  # If available
```

**Solutions**:

1. **Disable slow detectors**:
   ```bash
   # Skip inxi if it's slow
   ./target/release/lx-hw-detect detect --skip-inxi
   ```

2. **System performance**:
   ```bash
   # Check disk I/O
   iotop  # If available
   
   # Check for background processes
   ps aux | grep -v "^\[" | sort -k3 -nr | head -10
   ```

3. **Virtual machine optimization**:
   - Increase VM memory allocation
   - Enable hardware acceleration if available
   - Use SSD storage for VM

### Large Report Files

#### Issue: Generated reports are unexpectedly large

**Diagnostic**:
```bash
# Check report size
ls -lh report.json

# Find largest sections
jq 'to_entries | map({key: .key, size: (.value | tostring | length)}) | sort_by(.size) | reverse' report.json
```

**Solutions**:

1. **Use minimal reporting**:
   ```bash
   ./target/release/lx-hw-detect detect --minimal
   ```

2. **Check for repetitive data**:
   ```bash
   # Look for repeated sections
   jq '.' report.json | grep -o '"[^"]*"' | sort | uniq -c | sort -nr | head -20
   ```

## Platform-Specific Issues

### Ubuntu/Debian

#### Issue: Permission denied accessing hardware

**Solution**:
```bash
# Add user to necessary groups
sudo usermod -a -G disk,dialout $USER

# Re-login or use newgrp
newgrp disk

# Install additional packages
sudo apt install hwinfo lshw-gtk
```

### Fedora/RHEL

#### Issue: SELinux blocks hardware access

**Diagnostic**:
```bash
# Check SELinux status
sestatus

# Check for denials
sudo ausearch -m avc -ts recent
```

**Solution**:
```bash
# Temporarily set permissive mode for testing
sudo setenforce 0

# Run detection
./target/release/lx-hw-detect detect --format json

# Re-enable SELinux
sudo setenforce 1

# Create custom policy if needed (advanced)
```

### Arch Linux

#### Issue: Rolling release compatibility

**Solution**:
```bash
# Update all packages first
sudo pacman -Syu

# Install from AUR if needed
yay -S hardware-detection-tools

# Use latest Rust version
rustup update nightly
rustup default nightly
```

### NixOS

#### Issue: Tools not in PATH

**Solution**:
```nix
# Add to shell.nix or configuration.nix
environment.systemPackages = with pkgs; [
  pciutils
  usbutils
  inxi
  lshw
  dmidecode
];

# Or use nix-shell
nix-shell -p pciutils usbutils inxi lshw dmidecode
```

## Getting Additional Help

### Collecting Debug Information

When reporting issues, include:

```bash
# System information
uname -a
lsb_release -a  # Or equivalent
rustc --version
cargo --version

# Detection tool versions
lspci --version
lsusb --version
inxi --version

# Error output
RUST_LOG=debug ./target/release/lx-hw-detect detect --verbose 2>&1 | tee debug.log

# Hardware summary (without personal info)
lspci -nn
lsusb
```

### Reporting Bugs

1. **Search existing issues** on GitHub first
2. **Use the bug report template** when creating new issues
3. **Include debug output** and system information
4. **Specify exact commands** that reproduce the problem
5. **Attach relevant files** (reports, logs) if safe to share

### Community Support

- **GitHub Issues**: Technical problems and bugs
- **GitHub Discussions**: General questions and community help
- **IRC/Discord**: Real-time community support (if available)

### Security Issues

For security-related issues (privacy leaks, vulnerabilities):

1. **Do not** create public GitHub issues
2. **Email** security contacts directly
3. **Use encrypted communication** if possible
4. **Provide detailed information** about the vulnerability

Remember: The community is here to help! Don't hesitate to ask questions or report issues.