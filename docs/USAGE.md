# Usage Guide with Real-World Examples

This guide provides practical examples of using the Linux Hardware Database tools in real-world scenarios.

## Quick Start

### Basic Hardware Detection

```bash
# Detect all hardware and display summary
lx-hw-detect detect

# Detect with verbose output
lx-hw-detect detect --verbose

# Save hardware report to file
lx-hw-detect detect --output my-hardware.json

# Check compatibility for specific hardware
lx-hw-detect check --vendor nvidia --model "RTX 4080"
```

## Real-World Scenarios

### Scenario 1: Setting Up a New Linux System

**Problem**: You just installed Linux on a new laptop and want to check hardware compatibility.

```bash
# Step 1: Run complete hardware detection
lx-hw-detect detect --output laptop-report.json --verbose

# Step 2: Check for known issues with your hardware
lx-hw-detect check --report laptop-report.json

# Step 3: Submit your report to help the community
lx-hw-detect submit laptop-report.json --message "Dell XPS 13 on Ubuntu 22.04"
```

**Example Output**:

```json
{
  "system_info": {
    "manufacturer": "Dell Inc.",
    "model": "XPS 13 9320",
    "kernel_version": "6.2.0-39-generic",
    "distribution": "Ubuntu 22.04.3 LTS"
  },
  "compatibility_status": "excellent",
  "components": [
    {
      "type": "graphics",
      "vendor": "Intel",
      "model": "Iris Xe Graphics",
      "driver": "i915",
      "status": "working",
      "issues": []
    },
    {
      "type": "wireless",
      "vendor": "Intel",
      "model": "AX211",
      "driver": "iwlwifi",
      "status": "working",
      "performance_notes": "All WiFi 6E features working"
    }
  ]
}
```

### Scenario 2: Gaming Setup Compatibility Check

**Problem**: Building a Linux gaming rig and want to ensure all components work well.

```bash
# Check specific gaming components
lx-hw-detect check --vendor nvidia --model "RTX 4080" --category graphics
lx-hw-detect check --vendor amd --model "7800X3D" --category cpu
lx-hw-detect check --vendor corsair --category audio

# Generate gaming-focused report
lx-hw-detect detect --focus gaming --output gaming-build.json
```

**Gaming Report Analysis**:

```bash
# View gaming performance metrics
lx-hw-detect analyze gaming-build.json --metrics performance

# Check VR compatibility
lx-hw-detect check --report gaming-build.json --feature vr

# Verify HDR support
lx-hw-detect check --report gaming-build.json --feature hdr
```

### Scenario 3: Server Hardware Validation

**Problem**: Deploying Linux servers and need to verify hardware compatibility before production.

```bash
# Server-focused detection
lx-hw-detect detect --profile server --output server-report.json

# Check RAID controller compatibility
lx-hw-detect check --category storage --feature raid

# Validate network interfaces for bonding
lx-hw-detect check --category network --feature bonding

# Check power management features
lx-hw-detect check --category power --feature management
```

**Server Configuration Example**:

```toml
# ~/.config/lx-hw-detect/server-profile.toml
[detection]
focus_areas = ["storage", "network", "power", "management"]
deep_scan = true
include_benchmarks = true

[reporting]
include_performance_metrics = true
include_power_consumption = true
include_thermal_data = true
```

### Scenario 4: Troubleshooting Hardware Issues

**Problem**: WiFi is not working properly after kernel upgrade.

```bash
# Detect current WiFi hardware status
lx-hw-detect detect --category network --verbose

# Compare with known working configurations
lx-hw-detect compare --component "Intel AX200" --kernel-version "6.1.0"

# Check for similar reported issues
lx-hw-detect search --vendor intel --model ax200 --issue wifi

# Generate detailed diagnostic report
lx-hw-detect diagnose --category network --output wifi-diagnostic.json
```

### Scenario 5: Contributing to Hardware Database

**Problem**: You want to help improve Linux hardware support by contributing data.

```bash
# Generate comprehensive hardware report
lx-hw-detect detect --comprehensive --output comprehensive-report.json

# Add benchmark data
lx-hw-detect benchmark --output benchmark-data.json

# Merge reports
lx-hw-detect merge comprehensive-report.json benchmark-data.json --output full-report.json

# Submit with detailed information
lx-hw-detect submit full-report.json \
  --message "Lenovo ThinkPad P1 Gen 5 - Ubuntu 23.04" \
  --tags "laptop,workstation,hybrid-graphics" \
  --contact "user@example.com"
```

### Scenario 6: Enterprise Fleet Management

**Problem**: Managing hardware compatibility across a fleet of enterprise machines.

```bash
# Batch process multiple systems
for host in $(cat hostlist.txt); do
  ssh $host "lx-hw-detect detect --output /tmp/hw-report.json"
  scp $host:/tmp/hw-report.json reports/$host-report.json
done

# Analyze fleet compatibility
lx-hw-detect fleet-analyze reports/*.json --output fleet-summary.json

# Generate compatibility matrix
lx-hw-detect matrix --input fleet-summary.json --format csv
```

## Advanced Usage

### Custom Detection Profiles

Create specialized profiles for different use cases:

```toml
# ~/.config/lx-hw-detect/profiles/development.toml
[detection]
categories = ["graphics", "audio", "input", "display"]
deep_scan = true
include_benchmarks = false

[privacy]
hash_identifiers = true
anonymous_reporting = true

[output]
format = "json"
include_debug_info = true
```

```bash
# Use custom profile
lx-hw-detect detect --profile development --output dev-setup.json
```

### Automated Monitoring

Set up automated hardware monitoring:

```bash
#!/bin/bash
# weekly-hw-check.sh

# Generate hardware report
lx-hw-detect detect --quiet --output /tmp/weekly-report.json

# Compare with baseline
if lx-hw-detect diff baseline-report.json /tmp/weekly-report.json; then
  echo "Hardware configuration unchanged"
else
  echo "Hardware changes detected!"
  lx-hw-detect diff baseline-report.json /tmp/weekly-report.json --detailed
fi

# Update baseline if approved
if [ "$1" = "--update-baseline" ]; then
  cp /tmp/weekly-report.json baseline-report.json
fi
```

### Integration with CI/CD

Example GitHub Actions workflow:

```yaml
# .github/workflows/hardware-validation.yml
name: Hardware Validation
on: [push, pull_request]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - name: Install lx-hw-detect
      run: cargo install --path .

    - name: Run hardware detection
      run: lx-hw-detect detect --output hardware-report.json

    - name: Validate compatibility
      run: lx-hw-detect check --report hardware-report.json --strict

    - name: Upload report
      uses: actions/upload-artifact@v3
      with:
        name: hardware-report
        path: hardware-report.json
```

### Database Indexing and Search

**For maintainers and contributors working with the database**:

```bash
# Build search indices from hardware reports
lx-hw-indexer generate --input data/reports/ --output _site/

# Update specific indices
lx-hw-indexer update --index vendor --input data/reports/

# Validate database integrity
lx-hw-indexer validate --data data/reports/ --indices _site/api/

# Generate statistics
lx-hw-indexer stats --input data/reports/ --output stats.json

# Build and serve locally for testing
lx-hw-indexer serve --port 8080
```

## Output Formats

### JSON (Default)

```json
{
  "metadata": {
    "tool_version": "0.1.0",
    "generated_at": "2025-01-15T10:30:00Z",
    "system_id": "sha256:abc123...",
    "kernel_version": "6.6.8-arch1-1",
    "distribution": "Arch Linux"
  },
  "hardware": {
    "cpu": {
      "vendor": "AMD",
      "model": "Ryzen 9 7900X",
      "cores": 12,
      "threads": 24,
      "compatibility": "excellent"
    }
  }
}
```

### YAML

```bash
lx-hw-detect detect --format yaml
```

```yaml
metadata:
  tool_version: "0.1.0"
  generated_at: "2025-01-15T10:30:00Z"
  system_id: "sha256:abc123..."
hardware:
  cpu:
    vendor: "AMD"
    model: "Ryzen 9 7900X"
    compatibility: "excellent"
```

### CSV (for analysis)

```bash
lx-hw-detect detect --format csv --output hardware.csv
```

### Human-Readable Summary

```bash
lx-hw-detect detect --format summary
```

```
Linux Hardware Compatibility Report
===================================

System: Dell XPS 13 9320
Kernel: 6.2.0-39-generic
Distribution: Ubuntu 22.04.3 LTS
Overall Status: EXCELLENT

Graphics: Intel Iris Xe Graphics [WORKING]
  Driver: i915 (built-in)
  Features: Hardware acceleration, external displays

Wireless: Intel AX211 [WORKING]  
  Driver: iwlwifi
  Features: WiFi 6E, Bluetooth 5.2

Audio: Realtek ALC3254 [WORKING]
  Driver: snd_hda_intel
  Features: Stereo output, microphone input
```

## Environment Variables

Configure behavior with environment variables:

```bash
# Data directory
export LX_HW_DETECT_DATA_DIR="$HOME/.local/share/lx-hw-detect"

# Config file location
export LX_HW_DETECT_CONFIG="$HOME/.config/lx-hw-detect/config.toml"

# Privacy mode
export LX_HW_DETECT_ANONYMOUS=true

# Verbose logging
export LX_HW_DETECT_LOG=debug

# Custom repository for submissions
export LX_HW_DETECT_REPO="myorg/hardware-reports"
```

## Tips and Best Practices

### 1. Regular Hardware Auditing

```bash
# Monthly hardware check
0 0 1 * * /usr/local/bin/lx-hw-detect detect --output /var/log/hw-$(date +\%Y\%m).json
```

### 2. Privacy Considerations

```bash
# Maximum privacy mode
lx-hw-detect detect --anonymous --no-serial-numbers --hash-all --output anonymous-report.json
```

### 3. Performance Optimization

```bash
# Quick scan for automation
lx-hw-detect detect --quick --essential-only

# Full scan for detailed analysis  
lx-hw-detect detect --comprehensive --benchmarks
```

### 4. Troubleshooting

```bash
# Enable debug logging
LX_HW_DETECT_LOG=debug lx-hw-detect detect

# Test specific component detection
lx-hw-detect detect --category graphics --dry-run

# Verify system permissions
lx-hw-detect check-permissions
```

## Support and Community

- **GitHub Repository**: <https://github.com/olafkfreund/lx-hw-db>
- **Issue Tracker**: <https://github.com/olafkfreund/lx-hw-db/issues>
- **Discussions**: <https://github.com/olafkfreund/lx-hw-db/discussions>
- **Database**: <https://olafkfreund.github.io/lx-hw-db>

### Contributing Hardware Reports

Your hardware reports help make Linux better for everyone:

1. Run `lx-hw-detect detect --comprehensive`
2. Review the generated report for sensitive information
3. Submit with `lx-hw-detect submit` or create a GitHub issue
4. Include any compatibility notes or workarounds

### Reporting Issues

When reporting problems:

1. Include full hardware report: `lx-hw-detect detect --verbose`
2. Specify your Linux distribution and kernel version
3. Describe the specific hardware behavior or issue
4. Include any error messages or logs
