# Contributing to Linux Hardware Compatibility Database

Welcome to the Linux Hardware Compatibility Database! This project aims to build a comprehensive, community-driven database of hardware compatibility information for Linux systems.

## Quick Start

### Prerequisites

1. **Linux System**: You'll need access to a Linux system to generate hardware reports
2. **Rust Toolchain**: Install Rust 1.70+ from [rustup.rs](https://rustup.rs/)
3. **Git**: For cloning and submitting changes
4. **GitHub Account**: For submitting hardware reports and contributing

### Installation

```bash
# Clone the repository
git clone https://github.com/username/lx-hw-db.git
cd lx-hw-db

# Build the hardware detection tool
cargo build --release --bin lx-hw-detect

# The tool will be available at ./target/release/lx-hw-detect
```

## Hardware Report Submission

### Step 1: Generate Your Hardware Report

```bash
# Generate a hardware report with appropriate privacy level
./target/release/lx-hw-detect detect \
    --format json \
    --privacy enhanced \
    --output my-system-report.json

# Validate your report before submission
./target/release/lx-hw-detect validate my-system-report.json --strict
```

#### Privacy Levels

Choose the appropriate privacy level for your situation:

- **Basic** (24-hour salt rotation): Suitable for most users
- **Enhanced** (12-hour salt rotation): Recommended for sensitive environments
- **Strict** (1-hour salt rotation): Maximum privacy protection

### Step 2: Prepare for Submission

#### File Naming Convention

Your hardware report must follow this exact naming pattern:

```
YYYY-MM-DD_KERNEL-VERSION_ARCH_SYSTEM-ID.json
```

**Example**: `2025-08-26_6.16.0_x86_64_abc123def456.json`

#### Directory Structure

Place your file in the correct directory based on the report generation date:

```
hardware-reports/YYYY/MM/your-report-file.json
```

**Example**: `hardware-reports/2025/08/2025-08-26_6.16.0_x86_64_abc123def456.json`

### Step 3: Submit Your Report

#### Option A: Pull Request (Recommended)

1. **Fork the Repository**

   ```bash
   # Fork via GitHub UI, then clone your fork
   git clone https://github.com/YOUR-USERNAME/lx-hw-db.git
   cd lx-hw-db
   ```

2. **Create a Feature Branch**

   ```bash
   git checkout -b hardware-report-$(date +%Y%m%d)
   ```

3. **Add Your Report**

   ```bash
   # Create the directory if it doesn't exist
   mkdir -p hardware-reports/$(date +%Y)/$(date +%m)

   # Copy your report file to the correct location
   cp my-system-report.json hardware-reports/$(date +%Y)/$(date +%m)/$(date +%Y-%m-%d)_6.16.0_x86_64_abc123def456.json
   ```

4. **Commit and Push**

   ```bash
   git add hardware-reports/
   git commit -m "Add hardware report for [System Description]

   - CPU: [Your CPU Model]
   - GPU: [Your GPU Model]  
   - Kernel: [Your Kernel Version]
   - Distribution: [Your Distribution]"

   git push origin hardware-report-$(date +%Y%m%d)
   ```

5. **Create Pull Request**
   - Use the hardware report PR template
   - Fill in all required information
   - Wait for automated validation to complete

#### Option B: GitHub Issue

If you're not comfortable with Git workflows, you can submit via GitHub Issues:

1. Go to the [Hardware Report Issue Template](https://github.com/username/lx-hw-db/issues/new?template=hardware-submission.yml)
2. Fill in all required fields
3. Attach your hardware report file
4. Submit the issue

## What Happens Next?

### Automated Validation

Every submission goes through automated validation:

1. **File Naming**: Verifies correct naming convention
2. **Directory Structure**: Ensures proper placement
3. **Schema Validation**: Validates JSON structure and required fields  
4. **Privacy Compliance**: Scans for potential PII leaks
5. **Duplicate Detection**: Checks for existing reports from the same system
6. **Size and Format**: Verifies file size limits and UTF-8 encoding

### Manual Review

If automated validation passes:

1. **Privacy Audit**: Manual review for any missed PII
2. **Quality Check**: Ensures report provides community value
3. **Technical Review**: Validates hardware information accuracy

### Approval and Integration

Approved reports are:

1. Merged into the main database
2. Indexed for search and discovery
3. Made available under CC0 1.0 Universal License
4. Used for compatibility analysis and statistics

## Troubleshooting

### Common Validation Errors

#### File Naming Issues

```
❌ ERROR: File naming convention violation: my-report.json
Expected format: YYYY-MM-DD_KERNEL-VERSION_ARCH_SYSTEM-ID.json
```

**Solution**: Rename your file following the exact pattern. Extract the system ID from your report's `metadata.anonymized_system_id` field.

#### Directory Placement

```  
❌ ERROR: File in wrong directory: hardware-reports/2025/09/2025-08-26_...
Expected path: hardware-reports/2025/08/2025-08-26_...
```

**Solution**: Move the file to match the date in the filename (YYYY-MM-DD determines the directory).

#### Schema Validation Failures

```
❌ ERROR: my-report.json failed lx-hw-detect validation
```

**Solution**:

1. Run validation locally: `./target/release/lx-hw-detect validate my-report.json --verbose`
2. Check the detailed error output
3. Regenerate the report if needed

#### Duplicate Detection

```
❌ ERROR: Duplicate system ID found!
Existing file: hardware-reports/2025/07/2025-07-15_6.15.0_x86_64_abc123def456.json
```

**Solutions**:

- If this is an updated report: Remove or update the existing file and explain changes in PR description
- If this is a genuine duplicate: You may already have a report in the database

#### PII Detection Warnings

```
⚠️  Warning: Potential email address found in my-report.json
```

**Solution**: Regenerate the report with a higher privacy level (`--privacy strict`) or manually review and anonymize the detected information.

### Getting Help

#### Support Channels

1. **GitHub Issues**: [Create an issue](https://github.com/username/lx-hw-db/issues/new) for technical problems
2. **GitHub Discussions**: [Join discussions](https://github.com/username/lx-hw-db/discussions) for general questions
3. **Documentation**: Check the main [README](README.md) and [hardware-reports README](hardware-reports/README.md)

#### Before Asking for Help

1. **Check the logs**: Run commands with `--verbose` for detailed output
2. **Verify file naming**: Double-check your file follows the naming convention exactly
3. **Test locally**: Always run `lx-hw-detect validate` locally first
4. **Search existing issues**: Your problem might already be documented

#### Common Questions

**Q: Can I submit multiple reports from the same system?**
A: Yes, but only if there are significant changes (kernel upgrade, hardware changes, etc.). Update reports should replace older ones.

**Q: My hardware isn't detected properly. Can I still submit?**
A: Absolutely! Reports with detection issues are valuable for improving compatibility.

**Q: Is my data really anonymous?**
A: Yes. The tool uses cryptographic hashing with rotating salts to anonymize identifying information while preserving hardware compatibility data.

**Q: Can I submit reports from work/shared systems?**
A: Only submit reports from systems you own or have explicit permission to profile.

## Development Contributions

### Code Contributions

We welcome code contributions! Key areas:

1. **Hardware Detection**: Improve detection algorithms for specific hardware
2. **Privacy Protection**: Enhance anonymization techniques
3. **Validation System**: Add new validation rules
4. **Documentation**: Improve guides and documentation

### Setting Up Development Environment

```bash
# Clone the repository
git clone https://github.com/username/lx-hw-db.git
cd lx-hw-db

# Install development dependencies
cargo install cargo-watch cargo-tarpaulin

# Run tests
cargo test

# Run with auto-reload during development
cargo watch -x "test" -x "run -- detect --help"
```

### Code Style and Standards

- **Rust Edition**: 2021
- **MSRV**: Rust 1.70+
- **Formatting**: Use `cargo fmt`
- **Linting**: Use `cargo clippy`
- **Testing**: Maintain >90% code coverage

### Pull Request Process

1. Fork the repository
2. Create a feature branch from `main`
3. Make your changes with tests
4. Ensure all tests pass: `cargo test`
5. Run formatting and linting: `cargo fmt && cargo clippy`
6. Create detailed pull request with:
   - Clear description of changes
   - Test coverage information
   - Breaking changes (if any)

## Community Guidelines

### Code of Conduct

This project follows the [Contributor Covenant](CODE_OF_CONDUCT.md). We're committed to providing a welcoming and inclusive environment for all contributors.

### Contribution Expectations

- **Hardware Reports**: Submit only from systems you own or have permission to profile
- **Code Quality**: Follow established patterns and include appropriate tests  
- **Documentation**: Update documentation for any user-facing changes
- **Respect Privacy**: Never submit or request personally identifiable information

### Recognition

Contributors are recognized in several ways:

- **Hardware Reports**: Listed in contributor statistics
- **Code Contributions**: Added to repository contributors
- **Significant Contributions**: Mentioned in release notes

## License and Legal

### Data License

All hardware reports are released under **CC0 1.0 Universal License** (Public Domain). By submitting a report, you:

- Grant the community unrestricted use of the hardware compatibility data
- Waive all copyright claims to the submitted information
- Confirm the data contains no personally identifiable information

### Code License  

Source code contributions are licensed under the project's main license. See [LICENSE](LICENSE) for details.

### Privacy Statement

- **Data Collection**: Only hardware compatibility information is collected
- **Anonymization**: All identifying information is cryptographically anonymized
- **No Tracking**: No personal information or usage tracking is performed
- **Data Use**: Data is used solely for Linux hardware compatibility research

---

## Quick Reference

### Commands

```bash
# Generate report
./target/release/lx-hw-detect detect --format json --privacy enhanced --output report.json

# Validate report  
./target/release/lx-hw-detect validate report.json --strict

# Build project
cargo build --release

# Run tests
cargo test
```

### File Naming

```
YYYY-MM-DD_KERNEL-VERSION_ARCH_SYSTEM-ID.json
2025-08-26_6.16.0_x86_64_abc123def456.json
```

### Directory Structure

```
hardware-reports/YYYY/MM/filename.json
hardware-reports/2025/08/2025-08-26_6.16.0_x86_64_abc123def456.json
```

**Thank you for contributing to the Linux Hardware Compatibility Database!** 🐧

Your participation helps the entire Linux community make informed hardware decisions and improves compatibility for everyone.
