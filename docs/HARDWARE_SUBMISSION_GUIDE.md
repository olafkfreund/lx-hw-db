# Hardware Submission Guide

This comprehensive guide walks you through submitting hardware compatibility reports to the Linux Hardware Database (lx-hw-db).

## Overview

The hardware submission process is designed to be:
- **Privacy-preserving**: No personal information collected
- **Quality-focused**: Automated and community validation
- **Community-driven**: Peer review and collaborative improvement
- **Transparent**: Open process with full audit trail

## Prerequisites

### System Requirements

- **Linux System**: Any modern Linux distribution
- **Hardware Access**: Physical or virtual access to the hardware being reported
- **Internet Connection**: For downloading tools and submitting reports
- **GitHub Account**: For submitting pull requests (or use issue template)

### Required Tools

```bash
# The lx-hw-detect tool handles all detection automatically
# It will check for and use available tools:
# - lshw (hardware lister)
# - dmidecode (DMI/SMBIOS decoder) 
# - lspci (PCI device lister)
# - lsusb (USB device lister)
# - inxi (system information script)

# Download the latest release
curl -L -o lx-hw-detect "https://github.com/your-org/lx-hw-db/releases/latest/download/lx-hw-detect"
chmod +x lx-hw-detect

# Or build from source
git clone https://github.com/your-org/lx-hw-db.git
cd lx-hw-db
cargo build --release --bin lx-hw-detect
```

## Step-by-Step Submission Process

### Step 1: Generate Hardware Report

#### Choose Privacy Level

Select the appropriate privacy level for your use case:

```bash
# Basic Privacy (Recommended for most users)
# - Anonymizes serial numbers and MAC addresses
# - Preserves model numbers and specifications
# - 24-hour salt rotation for anonymization
./lx-hw-detect detect --privacy basic --output my-report.json

# Enhanced Privacy (Recommended for sharing)
# - Additional anonymization of system identifiers
# - 12-hour salt rotation
# - Slightly reduced statistical utility
./lx-hw-detect detect --privacy enhanced --output my-report.json

# Strict Privacy (Maximum protection)
# - Maximum anonymization with statistical noise
# - 1-hour salt rotation
# - Differential privacy techniques applied
./lx-hw-detect detect --privacy strict --output my-report.json
```

#### Tool Selection (Optional)

By default, all available tools are used. You can specify specific tools:

```bash
# Use only specific detection tools
./lx-hw-detect detect --tools lshw,lspci --output report.json

# Set custom timeout (default: 60 seconds)
./lx-hw-detect detect --timeout 120 --output report.json

# Combine options
./lx-hw-detect detect \
    --privacy enhanced \
    --tools lshw,dmidecode,lspci \
    --timeout 90 \
    --format yaml \
    --output detailed-report.yaml
```

#### Output Formats

Choose the format that works best for your workflow:

```bash
# JSON (default, best for automated processing)
./lx-hw-detect detect --format json --output report.json

# YAML (human-readable, good for manual review)
./lx-hw-detect detect --format yaml --output report.yaml

# Markdown (includes YAML frontmatter, great for GitHub)
./lx-hw-detect detect --format markdown --output report.md
```

### Step 2: Validate Your Report

Always validate your report before submission:

```bash
# Basic validation
./lx-hw-detect validate my-report.json

# Strict validation (recommended for submissions)
./lx-hw-detect validate my-report.json --strict

# Verbose validation (shows detailed information)
./lx-hw-detect validate my-report.json --strict --verbose
```

#### Common Validation Issues

**Missing Required Fields**
```bash
❌ ERROR: Missing required field: metadata.anonymized_system_id
```
*Solution*: Regenerate the report with the latest version of lx-hw-detect

**Privacy Level Mismatch**
```bash
⚠️  WARNING: Report contains potential personally identifiable information
```
*Solution*: Use a higher privacy level (`--privacy enhanced` or `--privacy strict`)

**Tool Detection Issues**
```bash
⚠️  WARNING: lshw not found, hardware detection may be incomplete
```
*Solution*: Install missing tools or use `--tools` to specify available ones

### Step 3: Prepare Report for Submission

#### File Naming Convention

Extract the required information from your report and rename the file:

```bash
# Extract system info from the report
SYSTEM_ID=$(jq -r '.metadata.anonymized_system_id' my-report.json)
KERNEL_VERSION=$(jq -r '.system.kernel_version' my-report.json)
ARCHITECTURE=$(jq -r '.system.architecture' my-report.json)
REPORT_DATE=$(date +%Y-%m-%d)

# Rename file to follow convention
mv my-report.json "${REPORT_DATE}_${KERNEL_VERSION}_${ARCHITECTURE}_${SYSTEM_ID}.json"
```

#### Directory Structure

Place the file in the correct directory:

```bash
# Create directory structure based on date
YEAR=$(date +%Y)
MONTH=$(date +%m)
mkdir -p hardware-reports/${YEAR}/${MONTH}

# Move file to correct location
mv "${REPORT_DATE}_${KERNEL_VERSION}_${ARCHITECTURE}_${SYSTEM_ID}.json" \
   hardware-reports/${YEAR}/${MONTH}/
```

### Step 4: Submit Your Report

#### Option A: Pull Request (Recommended)

**Fork the Repository**
```bash
# Fork via GitHub web interface, then:
git clone https://github.com/YOUR_USERNAME/lx-hw-db.git
cd lx-hw-db
git remote add upstream https://github.com/original-org/lx-hw-db.git
```

**Create Feature Branch**
```bash
# Create descriptive branch name
BRANCH_NAME="hardware-report-$(date +%Y%m%d)-$(echo $SYSTEM_ID | cut -c1-8)"
git checkout -b $BRANCH_NAME
```

**Add Your Report**
```bash
# Add the properly named and placed file
git add hardware-reports/
git status  # Verify only your report is staged
```

**Create Detailed Commit**
```bash
# Extract key hardware info for commit message
CPU_INFO=$(jq -r '.hardware.cpu.vendor + " " + .hardware.cpu.model' $REPORT_FILE)
GPU_INFO=$(jq -r '.hardware.graphics[0].vendor + " " + .hardware.graphics[0].model' $REPORT_FILE)
DISTRO=$(jq -r '.system.distribution' $REPORT_FILE)

git commit -m "Add hardware report: $CPU_INFO, $GPU_INFO

System Information:
- CPU: $CPU_INFO
- GPU: $GPU_INFO
- Kernel: $KERNEL_VERSION
- Distribution: $DISTRO
- Architecture: $ARCHITECTURE
- Privacy Level: enhanced

Hardware Highlights:
- All major components detected successfully
- Full Linux compatibility verified
- No known issues or workarounds required

Contribution Details:
- Generated with lx-hw-detect v1.0.0
- Automated validation passed
- Ready for community review"
```

**Push and Create Pull Request**
```bash
git push origin $BRANCH_NAME

# Create pull request using GitHub web interface or CLI
gh pr create --title "Hardware Report: $CPU_INFO on $DISTRO" \
             --body-file .github/PULL_REQUEST_TEMPLATE/hardware-report.md
```

#### Option B: Issue Submission

For users not comfortable with Git:

1. Visit the [Hardware Report Submission Issue Template](https://github.com/your-org/lx-hw-db/issues/new?template=hardware-submission.yml)
2. Fill out all required fields in the template
3. Attach your validated hardware report file
4. Submit the issue for processing by maintainers

### Step 5: Automated Validation

Your submission will automatically go through several validation checks:

#### File Structure Validation
- ✅ **Naming Convention**: Verifies `YYYY-MM-DD_KERNEL_ARCH_SYSTEM-ID.json` format
- ✅ **Directory Placement**: Ensures file is in `hardware-reports/YYYY/MM/` structure
- ✅ **File Size**: Confirms file is under 2MB limit
- ✅ **Encoding**: Validates UTF-8 encoding

#### Content Validation  
- ✅ **JSON Schema**: Validates against official hardware report schema
- ✅ **Required Fields**: Ensures all mandatory metadata and hardware fields present
- ✅ **Privacy Compliance**: Scans for potential personally identifiable information
- ✅ **Technical Validation**: Runs lx-hw-detect validation with strict mode

#### Quality Checks
- ✅ **Duplicate Detection**: Checks for existing reports from same system
- ✅ **Hardware Completeness**: Analyzes coverage of major hardware components
- ✅ **Compatibility Information**: Validates compatibility status and notes
- ✅ **Timestamp Validation**: Ensures report generation timestamp is reasonable

#### Security Scanning
- ✅ **PII Detection**: Scans for email addresses, IP addresses, usernames
- ✅ **File Permissions**: Verifies appropriate file permissions
- ✅ **Content Safety**: Checks for potentially malicious content

### Step 6: Community Review

After automated validation passes:

#### Initial Review (1-3 days)
- Community members review for quality and completeness
- Basic technical validation by experienced contributors
- Feedback provided for any improvements needed

#### Expert Review (3-7 days, if applicable)
- Hardware category experts validate technical claims
- In-depth analysis of compatibility information
- Verification of unusual or rare hardware configurations

#### Final Approval (7-14 days)
- Maintainer review for final approval
- Integration into main database
- Indexing for search and discovery

### Step 7: Post-Submission

#### Tracking Your Submission
- Monitor pull request for automated validation results
- Respond to community feedback and questions
- Make requested changes if needed
- Celebrate when your contribution is merged! 🎉

#### Recognition
- Your contribution will be listed in contributor statistics
- Hardware reports help build community knowledge
- Significant contributions may be featured in project updates

## Advanced Submission Scenarios

### Updating Existing Reports

When hardware or software changes significantly:

```bash
# Generate new report
./lx-hw-detect detect --privacy enhanced --output updated-report.json

# Validate
./lx-hw-detect validate updated-report.json --strict

# Create update pull request
git checkout -b update-hardware-report-$(date +%Y%m%d)

# Either replace existing file or add new one with updated timestamp
# Include explanation of changes in commit message and PR description
```

### Multiple System Reports

For users with multiple systems:

```bash
# Generate reports for each system
for system in desktop laptop server; do
    ./lx-hw-detect detect \
        --privacy enhanced \
        --output ${system}-report.json
done

# Validate all reports
for report in *-report.json; do
    ./lx-hw-detect validate "$report" --strict
done

# Submit as single pull request with multiple files
# Include summary of all systems in PR description
```

### Organization Bulk Submissions

For organizations submitting many reports:

1. **Contact Maintainers**: Reach out before bulk submission
2. **Coordinate Timeline**: Plan submission schedule to manage review load
3. **Provide Context**: Include organizational use case and hardware deployment info
4. **Quality Assurance**: Extra validation for bulk submissions

## Troubleshooting Common Issues

### Hardware Detection Problems

**Issue**: Some hardware not detected
```bash
⚠️  WARNING: GPU information incomplete
```

**Solutions**:
1. Ensure all detection tools are installed and accessible
2. Run with elevated privileges if needed: `sudo ./lx-hw-detect detect`
3. Check tool-specific requirements (lshw, dmidecode, etc.)
4. Use verbose mode to see detailed detection logs: `--verbose`

**Issue**: Permission errors
```bash
❌ ERROR: dmidecode requires root privileges
```

**Solutions**:
1. Run with sudo: `sudo ./lx-hw-detect detect`
2. Use tools-specific flags: `--tools lspci,lsusb` (avoid tools requiring root)
3. Set up sudo access for specific detection tools

### Validation Failures

**Issue**: Schema validation fails
```bash
❌ ERROR: Report does not conform to hardware schema
```

**Solutions**:
1. Regenerate report with latest lx-hw-detect version
2. Check for tool version compatibility
3. Validate individual sections: `--validate-section system`

**Issue**: PII detection warnings
```bash
⚠️  WARNING: Potential email address found
```

**Solutions**:
1. Use higher privacy level: `--privacy strict`
2. Review detection tool configurations
3. Manually inspect and confirm no actual PII present

### Submission Process Issues

**Issue**: Duplicate system ID detected
```bash
❌ ERROR: Duplicate system ID found in existing report
```

**Solutions**:
1. Check if you've already submitted a report
2. If updating existing report, remove old one and explain in PR
3. If legitimate duplicate, contact maintainers for guidance

**Issue**: File naming convention errors
```bash
❌ ERROR: File naming convention violation
```

**Solutions**:
1. Extract exact values from report metadata
2. Use provided script for automatic renaming
3. Double-check date format: YYYY-MM-DD

## Getting Help

### Support Resources

**Documentation**
- [README.md](../README.md): Project overview and quick start
- [CONTRIBUTING.md](../CONTRIBUTING.md): Detailed contribution guidelines
- [Hardware Reports README](../hardware-reports/README.md): Database structure

**Community Support**
- [GitHub Discussions](https://github.com/your-org/lx-hw-db/discussions): General questions
- [GitHub Issues](https://github.com/your-org/lx-hw-db/issues): Bug reports and technical issues
- Matrix/IRC: Real-time community support

**Before Asking for Help**
1. Read this guide completely
2. Check existing issues and discussions
3. Try validation with verbose output: `--verbose`
4. Include complete error messages and system information

### Frequently Asked Questions

**Q: Can I submit reports from virtual machines?**
A: Yes! VM reports are valuable for virtualization compatibility. Include VM platform info in notes.

**Q: What if my hardware has known issues?**
A: Please submit anyway! Reports with issues are extremely valuable for the community.

**Q: How often should I update my reports?**
A: Update when major changes occur: kernel upgrades, hardware changes, or resolved compatibility issues.

**Q: Is my personal data really protected?**
A: Yes. The tool uses cryptographic anonymization and no personal data is stored or transmitted.

**Q: Can I submit reports from work computers?**
A: Only with proper authorization. Ensure you have permission to profile and share hardware information.

---

## Quick Reference

### Essential Commands
```bash
# Generate report
./lx-hw-detect detect --privacy enhanced --output report.json

# Validate report
./lx-hw-detect validate report.json --strict

# Check naming convention
echo "Format: YYYY-MM-DD_KERNEL_ARCH_SYSTEM-ID.json"
```

### File Locations
```bash
# Correct directory structure
hardware-reports/YYYY/MM/YYYY-MM-DD_KERNEL_ARCH_SYSTEM-ID.json

# Example
hardware-reports/2025/08/2025-08-26_6.16.0_x86_64_abc123def456.json
```

### Privacy Levels
- **basic**: Standard anonymization, 24-hour salt rotation
- **enhanced**: Additional anonymization, 12-hour salt rotation (recommended)
- **strict**: Maximum privacy with statistical noise, 1-hour salt rotation

Thank you for contributing to the Linux Hardware Compatibility Database! Your submission helps the entire Linux community make better hardware decisions. 🐧✨