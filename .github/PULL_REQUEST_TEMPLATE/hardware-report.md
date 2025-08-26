# Hardware Compatibility Report Submission

## Summary
<!-- Provide a brief summary of the hardware configuration being submitted -->

**System Type**: <!-- e.g., Desktop, Laptop, Server, Single Board Computer -->
**Primary Use Case**: <!-- e.g., Gaming, Development, Server, Daily Use -->

## Hardware Report Details

### System Information
- **Anonymized System ID**: <!-- From metadata.anonymized_system_id -->
- **Kernel Version**: <!-- e.g., 6.16.0 -->
- **Distribution**: <!-- e.g., NixOS 25.11, Ubuntu 24.04 -->
- **Architecture**: <!-- e.g., x86_64, aarch64 -->
- **Privacy Level**: <!-- Basic, Enhanced, or Strict -->

### Hardware Summary
<!-- Brief overview of key components -->
- **CPU**: <!-- e.g., AMD Ryzen 7 5800X (8 cores, 16 threads) -->
- **Memory**: <!-- e.g., 32GB DDR4-3200 -->
- **Graphics**: <!-- e.g., NVIDIA GeForce RTX 3080 -->
- **Storage**: <!-- e.g., Samsung 980 PRO NVMe SSD -->
- **Network**: <!-- e.g., Intel I225-V Gigabit Ethernet -->
- **Other Notable Hardware**: <!-- e.g., WiFi 6E, Bluetooth 5.2, USB-C -->

### Compatibility Status
<!-- Check all that apply -->
- [ ] All hardware detected and working correctly
- [ ] Most hardware working, minor issues present  
- [ ] Some hardware not detected or working
- [ ] Major hardware compatibility issues
- [ ] Specialized hardware requiring additional drivers

### Known Issues and Workarounds
<!-- Detail any compatibility issues and solutions -->
```
Example:
- WiFi: Requires firmware-linux-nonfree package
- GPU: Needs nvidia-driver-525 or later
- Audio: Works with snd-hda-intel module loaded
- Bluetooth: Requires manual pairing on first boot
```

## File Information

### File Naming Convention
**File Name**: `YYYY-MM-DD_KERNEL-VERSION_ARCH_SYSTEM-ID.json`
**Example**: `2025-08-26_6.16.0_x86_64_abc123def456.json`

### File Location
**Directory**: `hardware-reports/YYYY/MM/`
**Full Path**: `hardware-reports/2025/08/2025-08-26_6.16.0_x86_64_abc123def456.json`

## Validation Checklist

### Pre-submission Validation
- [ ] Report generated with latest version of `lx-hw-detect`
- [ ] Ran `lx-hw-detect validate <file>` successfully (no errors)
- [ ] Confirmed privacy level is appropriate for public sharing
- [ ] Verified no personally identifiable information (PII) included
- [ ] Checked file follows naming convention
- [ ] File placed in correct directory structure

### Quality Assurance
- [ ] Hardware summary accurately reflects the detected hardware
- [ ] Known issues documented with available workarounds
- [ ] Report includes sufficient detail for community value
- [ ] File size is reasonable (typically < 100KB)

### Legal and Ethical
- [ ] Hardware report generated from my own system
- [ ] I have the right to share this hardware information
- [ ] I understand this will be publicly available under open license
- [ ] No proprietary or confidential information included

## Additional Information

### Detection Tools Used
<!-- List which tools were available during detection -->
- [ ] lshw (hardware lister)
- [ ] dmidecode (DMI/SMBIOS decoder)
- [ ] lspci (PCI device lister)
- [ ] lsusb (USB device lister)
- [ ] inxi (system information script)

### Community Value
<!-- Explain why this hardware report would be valuable to the community -->
```
Example:
- First report for this specific laptop model
- Demonstrates compatibility with latest kernel version
- Shows workaround for common hardware issue
- Provides baseline for similar configurations
```

### Testing and Verification
<!-- Optional: Additional testing performed -->
```
Example:
- Stress tested GPU under load for 2+ hours
- Verified WiFi performance across different networks
- Tested suspend/resume functionality
- Confirmed all USB ports working correctly
```

## Related Issues
<!-- Link any related GitHub issues -->
- Closes #<!-- issue number -->
- Related to #<!-- issue number -->

---

## Reviewer Guidelines

### For Maintainers
- [ ] Verify file follows naming convention
- [ ] Confirm file is in correct directory structure
- [ ] Run automated validation workflow
- [ ] Check for any potential PII leaks
- [ ] Ensure hardware summary matches report content
- [ ] Verify no duplicate submissions from same system

### Automated Checks
The following will be automatically verified:
- JSON schema validation
- Privacy compliance check
- File naming convention
- Directory structure
- Duplicate detection
- File size limits

---

**Thank you for contributing to the Linux Hardware Compatibility Database!** 🐧

Your submission helps the community understand hardware support and compatibility across different Linux configurations.