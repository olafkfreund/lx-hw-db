# Automated GitHub Submission

The lx-hw-detect tool now supports automated GitHub submission with a single command! This feature automatically generates a hardware report, creates a GitHub fork, and submits a pull request.

## Quick Start

The simplest way to submit your hardware to the database:

```bash
# One command to rule them all!
./lx-hw-detect submit --github-username YOUR_USERNAME --auto-fork --yes

# The tool will:
# 1. Prompt for your GitHub token (securely)
# 2. Generate your hardware report automatically
# 3. Fork the repository if needed
# 4. Create a pull request with all proper formatting
# 5. Show you the PR URL when complete
```

## Prerequisites

Before using the automated submission:

1. **GitHub CLI**: Install the `gh` command-line tool
   ```bash
   # Ubuntu/Debian
   sudo apt install gh
   
   # macOS
   brew install gh
   
   # Other systems: https://cli.github.com/
   ```

2. **GitHub Personal Access Token**: Create a token with `repo` permissions at https://github.com/settings/tokens

3. **Hardware Detection Tools**: Install for best results
   ```bash
   # Ubuntu/Debian
   sudo apt install lshw dmidecode pciutils usbutils inxi
   
   # Fedora
   sudo dnf install lshw dmidecode pciutils usbutils inxi
   
   # Arch
   sudo pacman -S lshw dmidecode pciutils usbutils inxi
   ```

## Command Options

### Basic Submission

```bash
# Minimal command (will prompt for missing info)
./lx-hw-detect submit

# With GitHub credentials
./lx-hw-detect submit \
    --github-username YOUR_USERNAME \
    --github-token YOUR_TOKEN
```

### Advanced Options

```bash
# Complete automated submission
./lx-hw-detect submit \
    --github-username YOUR_USERNAME \
    --github-token YOUR_TOKEN \
    --description "Gaming desktop with RTX 4080" \
    --privacy enhanced \
    --tools lshw,lspci,dmidecode \
    --auto-fork \
    --yes

# Submit existing report file
./lx-hw-detect submit \
    --report my-hardware-report.json \
    --description "Updated report after kernel upgrade" \
    --github-username YOUR_USERNAME
```

### Privacy Levels

Choose the appropriate privacy level:

```bash
# Basic privacy (default)
./lx-hw-detect submit --privacy basic

# Enhanced privacy (recommended for public sharing)
./lx-hw-detect submit --privacy enhanced

# Maximum privacy 
./lx-hw-detect submit --privacy strict
```

## What Happens During Submission

The automated submission process:

### 1. **Credential Setup**
- Validates GitHub credentials
- Tests GitHub CLI access
- Sets up repository configuration

### 2. **Hardware Report Generation** (if no file provided)
- Runs hardware detection tools
- Applies privacy anonymization
- Generates properly formatted report
- Validates report schema

### 3. **Repository Management**
- Forks the main repository (if needed)
- Clones your fork locally
- Creates a feature branch with timestamp

### 4. **File Organization**
- Generates proper filename: `YYYY-MM-DD_KERNEL_ARCH_SYSTEM-ID.json`
- Places file in correct directory: `hardware-reports/YYYY/MM/`
- Follows all naming conventions

### 5. **Git Operations**
- Commits changes with detailed message
- Pushes branch to your GitHub fork
- Creates pull request with comprehensive description

### 6. **Automated Validation**
- Your PR triggers our CI/CD validation
- Checks file format, naming, privacy compliance
- Scans for potential issues
- Community review begins automatically

## Example Output

```bash
$ ./lx-hw-detect submit --github-username john --auto-fork --yes

🚀 Starting automated GitHub submission...

🔐 Validating GitHub credentials...
✅ GitHub credentials validated

📊 No report file provided, generating hardware report...
✅ Hardware report generated successfully

📋 Submission Summary:
═══════════════════════════════════════
📁 File: 2025-08-26_6.16.0_x86_64_abc123def456.json
💻 System: NixOS 25.11 6.16.0
🏗️  Architecture: x86_64
🔒 Privacy Level: Enhanced  
🛠️  Tools Used: lshw, dmidecode, lspci, lsusb
🖥️  CPU: AMD Ryzen 7 5800X
💾 Memory: 32GB
🎮 GPU: NVIDIA GeForce RTX 4080
📝 Description: AMD Ryzen 7 5800X with NVIDIA GeForce RTX 4080 on NixOS 25.11 6.16.0
═══════════════════════════════════════

🍴 Checking repository fork...
✅ Fork already exists

📥 Cloning repository...
✅ Repository cloned successfully

🌱 Creating feature branch...
✅ Feature branch 'hardware-report-20250826-abc123de' created

📁 Adding report file...
✅ Report file added to repository

💾 Committing changes...
✅ Changes committed successfully

📤 Pushing branch to GitHub...
✅ Branch pushed successfully

🔀 Creating pull request...
✅ Pull request created successfully

🎉 Submission completed successfully!
📋 Pull Request: https://github.com/john/lx-hw-db/pull/123

Next steps:
1. Your submission will be automatically validated
2. Community members will review your hardware report  
3. Once approved, it will be merged into the database

Thank you for contributing to the Linux Hardware Database! 🐧
```

## Troubleshooting

### Common Issues

**GitHub Authentication Failed**
```bash
❌ GitHub authentication failed. Please check your token.
```
*Solution*: Verify your token has `repo` permissions and hasn't expired

**Repository Fork Not Found**
```bash
❌ Repository fork not found. Use --auto-fork to create one automatically.
```
*Solution*: Add `--auto-fork` flag or manually fork the repository first

**Hardware Detection Incomplete**
```bash
⚠️  WARNING: lshw not found, hardware detection may be incomplete
```
*Solution*: Install missing detection tools or use `--tools` to specify available ones

**File Naming Issues**
```bash
❌ ERROR: File naming convention violation
```
*Solution*: The tool handles this automatically - this usually indicates a bug

### Getting Help

If you encounter issues:

1. **Check Prerequisites**: Ensure GitHub CLI is installed and working
2. **Verify Permissions**: Confirm your token has repository access
3. **Test Manually**: Try `gh auth status` to verify GitHub access
4. **Check Logs**: Use `--verbose` for detailed error information
5. **Report Issues**: Open an issue at https://github.com/your-org/lx-hw-db/issues

### Manual Fallback

If automated submission fails, you can always fall back to manual submission:

```bash
# Generate report manually
./lx-hw-detect detect --privacy enhanced --output report.json

# Follow manual submission instructions in CONTRIBUTING.md
```

## Benefits of Automated Submission

- **Zero Configuration**: No need to understand Git workflows
- **Error Prevention**: Automatic validation and proper formatting
- **Consistency**: Standardized commit messages and PR descriptions
- **Privacy Protection**: Built-in anonymization and validation
- **Speed**: Complete submission in under 2 minutes
- **User-Friendly**: Clear progress indicators and helpful error messages

## Security Considerations

- **Token Security**: Tokens are only used locally and never stored
- **Privacy Protection**: All hardware identifiers are anonymized before submission
- **Repository Safety**: Only affects your fork, not the main repository
- **Validation**: Multiple validation layers prevent malicious submissions

---

The automated submission feature makes contributing to the Linux Hardware Database as simple as running a single command. No Git knowledge required! 🚀