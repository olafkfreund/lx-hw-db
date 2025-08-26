# Hardware Reports Directory

This directory contains community-submitted hardware compatibility reports for the Linux Hardware Database.

## Directory Structure

```
hardware-reports/
├── YYYY/           # Year (e.g., 2025)
│   ├── MM/         # Month (e.g., 08)
│   │   ├── hardware-report-file.json
│   │   └── ...
│   └── ...
├── index/          # Generated indexes and search data
│   ├── by-cpu.json
│   ├── by-gpu.json
│   ├── by-kernel.json
│   └── by-vendor.json
└── README.md       # This file
```

## File Naming Convention

Hardware report files must follow this naming convention:

```
YYYY-MM-DD_KERNEL-VERSION_ARCH_SYSTEM-ID.json
```

### Examples:
- `2025-08-26_6.16.0_x86_64_abc123def456.json`
- `2025-08-15_6.15.8_aarch64_def456ghi789.json`
- `2025-07-30_6.14.12_x86_64_ghi789jkl012.json`

### Components:
- **YYYY-MM-DD**: Date when the report was generated
- **KERNEL-VERSION**: Linux kernel version (e.g., 6.16.0)
- **ARCH**: System architecture (x86_64, aarch64, armv7l, i686, riscv64)
- **SYSTEM-ID**: First 12 characters of the anonymized system ID

## File Requirements

### Size Limits
- Maximum file size: 2MB
- Typical file size: 50-200KB
- Files exceeding 2MB will be rejected

### Content Requirements
- Must be valid JSON format
- Must pass `lx-hw-detect validate` with no errors
- Must use appropriate privacy level (Basic, Enhanced, or Strict)
- Must not contain personally identifiable information (PII)

### Schema Validation
All reports must conform to the hardware report JSON schema and pass validation:

```bash
lx-hw-detect validate your-report.json --strict
```

## Submission Process

### 1. Generate Report
```bash
# Generate hardware report
lx-hw-detect detect --format json --output my-system-report.json

# Validate the report
lx-hw-detect validate my-system-report.json --strict
```

### 2. Choose File Location
Based on your report's generation date:
```bash
# For a report generated on 2025-08-26
hardware-reports/2025/08/2025-08-26_6.16.0_x86_64_abc123def456.json
```

### 3. Submit Pull Request
1. Fork the repository
2. Add your report file to the appropriate directory
3. Create a pull request using the hardware report template
4. Include relevant information in the PR description

### 4. Automated Validation
Your submission will automatically be validated for:
- JSON schema compliance
- File naming convention
- Directory placement
- Privacy compliance
- Duplicate detection
- File size limits

## Privacy and Security

### Privacy Levels
- **Basic**: 24-hour salt rotation, suitable for most users
- **Enhanced**: 12-hour salt rotation, recommended for sensitive environments  
- **Strict**: 1-hour salt rotation, maximum privacy protection

### What Gets Anonymized
- System hostnames → Anonymous identifiers
- MAC addresses → Randomized addresses  
- Storage serial numbers → Anonymous identifiers
- System identifiers → Cryptographic hashes

### What Remains Public
- Hardware vendor names (Intel, AMD, NVIDIA, etc.)
- Hardware model names
- Driver information
- Kernel compatibility data
- System specifications

## Data Usage

### License
All submitted hardware reports are made available under the **Creative Commons CC0 1.0 Universal License**, meaning they are dedicated to the public domain.

### Usage Rights
By submitting a hardware report, you grant the community the right to:
- Use the data for Linux hardware compatibility research
- Include the data in compatibility databases and tools
- Analyze trends in hardware support and adoption
- Generate statistics and reports about Linux hardware compatibility

### Research Applications  
This data enables:
- Hardware compatibility analysis across kernel versions
- Vendor driver support tracking
- Distribution-specific hardware support comparison
- Automatic hardware recommendation systems
- Kernel regression detection

## Quality Standards

### Acceptance Criteria
Reports are accepted if they:
- Pass automated validation checks
- Provide valuable community data
- Follow privacy and security guidelines  
- Meet file naming and structure requirements

### Rejection Reasons
Reports may be rejected for:
- Validation failures (schema, privacy, etc.)
- Duplicate submissions from the same system
- Files containing PII or sensitive information
- Incorrect file naming or directory placement
- Files that are corrupted or incomplete

## Community Guidelines

### Contributor Expectations
- Submit only reports from your own hardware
- Ensure reports are generated with the latest `lx-hw-detect` version
- Provide accurate hardware summaries and issue descriptions
- Respect the privacy of others (no shared/borrowed systems without permission)

### Maintainer Review Process
1. Automated validation runs on all submissions
2. Manual review for edge cases or validation failures
3. Privacy audit for potential PII leaks
4. Community feedback period for complex submissions
5. Final approval and merge into database

## Statistics and Metrics

### Current Database Status
<!-- This would be automatically updated -->
- **Total Reports**: 0
- **Unique Systems**: 0
- **Kernel Versions**: 0
- **Architectures**: 0
- **Latest Update**: Not yet available

### Coverage Metrics
- **CPU Vendors**: Intel, AMD, ARM, etc.
- **GPU Vendors**: NVIDIA, AMD, Intel, etc.
- **Network Hardware**: Ethernet, WiFi, Bluetooth
- **Storage Types**: NVMe, SATA, USB, etc.

## Getting Help

### Common Issues
1. **Validation Failures**: Run `lx-hw-detect validate --verbose` for detailed error information
2. **File Naming**: Use the exact format specified above
3. **Directory Placement**: Ensure you're using the correct year/month structure
4. **Privacy Concerns**: Use Enhanced or Strict privacy levels for sensitive systems

### Support Channels
- **GitHub Issues**: For technical problems or questions
- **Discussions**: For general questions and community support  
- **Documentation**: Check the main project README for detailed guides

---

**Thank you for contributing to the Linux Hardware Compatibility Database!**

Your submissions help the entire Linux community understand hardware support and make informed decisions about hardware compatibility.