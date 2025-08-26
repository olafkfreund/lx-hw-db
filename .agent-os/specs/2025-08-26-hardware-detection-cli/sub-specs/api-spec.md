# CLI API Specification

> Spec: Hardware Detection CLI Tool
> Created: 2025-08-26
> Version: 1.0.0

## Command Line Interface

### Primary Command
```bash
lx-hw-detect [OPTIONS] [SUBCOMMANDS]
```

### Global Options

#### Privacy and Security
```bash
--privacy-level <LEVEL>     Set privacy level [default: enhanced]
                           Values: basic, enhanced, strict
                           
--salt-period <PERIOD>     Salt rotation period [default: auto]
                          Values: 1h, 12h, 24h, auto
                          
--no-anonymize            Skip hardware ID anonymization (development only)
```

#### Output Control
```bash
--output, -o <FILE>       Write output to file instead of stdout
--format <FORMAT>         Output format [default: markdown]
                         Values: markdown, json, yaml
                         
--template <FILE>         Use custom output template
--compact                 Generate compact output (minimal whitespace)
--verbose, -v             Increase verbosity (can be used multiple times)
--quiet, -q              Suppress all output except errors
```

#### Detection Control
```bash
--tools <LIST>           Comma-separated list of tools to use
                        Values: lshw,dmidecode,lspci,lsusb,inxi
                        Default: all available tools
                        
--timeout <SECONDS>      Global timeout for all detection tools [default: 30]
--tool-timeout <SECONDS> Individual tool timeout [default: 10]
--parallel               Run detection tools in parallel [default: true]
--no-parallel           Run detection tools sequentially
```

#### System Access
```bash
--no-root-tools          Skip tools that require root privileges
--sudo                   Automatically use sudo for privileged tools
--dry-run                Show what would be detected without running tools
```

### Subcommands

#### Detection Commands
```bash
lx-hw-detect detect      # Default detection command (same as no subcommand)
lx-hw-detect scan        # Alias for detect
```

#### Validation Commands  
```bash
lx-hw-detect validate <FILE>     # Validate existing hardware report
lx-hw-detect schema             # Print current schema specification  
lx-hw-detect check-tools        # Check availability of detection tools
```

#### Utility Commands
```bash
lx-hw-detect version            # Show version information
lx-hw-detect help [COMMAND]     # Show help for specific command
lx-hw-detect completion <SHELL> # Generate shell completion scripts
```

### Configuration File

#### Default Locations
1. `~/.config/lx-hw-detect/config.toml`
2. `~/.lx-hw-detect.toml`  
3. `./lx-hw-detect.toml`

#### Configuration Format
```toml
# ~/.config/lx-hw-detect/config.toml

[privacy]
level = "enhanced"           # basic|enhanced|strict
salt_period = "12h"         # 1h|12h|24h|auto
anonymize_serials = true
anonymize_mac_addresses = true
anonymize_bios_info = false

[detection]
tools = ["lshw", "dmidecode", "lspci", "lsusb", "inxi"]
timeout = 30                # Global timeout in seconds
tool_timeout = 10           # Per-tool timeout in seconds
parallel = true             # Run tools in parallel
require_root = false        # Skip root-only tools by default

[output]
format = "markdown"         # markdown|json|yaml
include_technical_details = true
include_raw_data = false
compact = false
template_file = ""

[system]
auto_detect_distribution = true
include_kernel_modules = true
include_loaded_drivers = true
include_hardware_details = true

# Tool-specific configuration
[tools.lshw]
enabled = true
format = "json"
sanitize = true
quiet = true

[tools.dmidecode]
enabled = true
types = ["0", "1", "2", "3", "4", "16", "17"]  # BIOS, System, Baseboard, etc.
decode_oem = false

[tools.lspci]  
enabled = true
verbose = true
show_numeric = true
show_kernel = true

[tools.lsusb]
enabled = true
verbose = false
show_tree = true

[tools.inxi]
enabled = true
full_output = true
machine_readable = false
extra_verbose = 3
```

## Exit Codes

### Standard Exit Codes
```
0    Success - Hardware detection completed successfully
1    General error - Unspecified error occurred
2    Tool not found - Required detection tool not available
3    Permission denied - Insufficient privileges for detection
4    Timeout - Detection process timed out
5    Parse error - Failed to parse tool output
6    Validation error - Generated report failed validation
7    IO error - File read/write error
8    Configuration error - Invalid configuration file or options
```

### Detailed Exit Code Handling
```rust
#[derive(Debug, Clone, Copy)]
pub enum ExitCode {
    Success = 0,
    GeneralError = 1,
    ToolNotFound = 2,
    PermissionDenied = 3,
    Timeout = 4,
    ParseError = 5,
    ValidationError = 6,
    IoError = 7,
    ConfigurationError = 8,
}

impl From<DetectionError> for ExitCode {
    fn from(error: DetectionError) -> Self {
        match error {
            DetectionError::ToolNotFound { .. } => ExitCode::ToolNotFound,
            DetectionError::InsufficientPrivileges { .. } => ExitCode::PermissionDenied,
            DetectionError::ToolTimeout { .. } => ExitCode::Timeout,
            DetectionError::ParseError { .. } => ExitCode::ParseError,
            _ => ExitCode::GeneralError,
        }
    }
}
```

## Environment Variables

### Detection Control
```bash
LX_HW_DETECT_TOOLS="lshw,dmidecode,lspci"    # Override default tools
LX_HW_DETECT_TIMEOUT=60                       # Global timeout in seconds  
LX_HW_DETECT_PARALLEL=false                   # Disable parallel execution
LX_HW_DETECT_NO_ROOT=true                     # Skip privileged tools
```

### Privacy Settings
```bash
LX_HW_DETECT_PRIVACY_LEVEL=strict             # Override privacy level
LX_HW_DETECT_SALT_PERIOD=1h                   # Override salt rotation
LX_HW_DETECT_NO_ANONYMIZE=true                # Skip anonymization (dev only)
```

### Output Control
```bash
LX_HW_DETECT_FORMAT=json                      # Override output format
LX_HW_DETECT_OUTPUT=/tmp/hw-report.md         # Override output file
LX_HW_DETECT_VERBOSE=2                        # Set verbosity level
LX_HW_DETECT_QUIET=true                       # Enable quiet mode
```

## Usage Examples

### Basic Usage
```bash
# Default detection with enhanced privacy
lx-hw-detect

# Output to file
lx-hw-detect -o hardware-report.md

# JSON output for programmatic use
lx-hw-detect --format json -o hardware.json
```

### Privacy Levels
```bash
# Basic privacy (daily salt rotation)
lx-hw-detect --privacy-level basic

# Strict privacy (hourly salt rotation, maximum anonymization)
lx-hw-detect --privacy-level strict --salt-period 1h
```

### Selective Tool Usage
```bash
# Use only non-privileged tools
lx-hw-detect --no-root-tools

# Use specific tools only
lx-hw-detect --tools lspci,lsusb,inxi

# Skip problematic tools
lx-hw-detect --tools lshw,lspci,lsusb
```

### Troubleshooting
```bash
# Verbose output for debugging
lx-hw-detect -vv

# Check tool availability
lx-hw-detect check-tools

# Dry run to see what would be detected
lx-hw-detect --dry-run --verbose

# Validate existing report
lx-hw-detect validate hardware-report.md
```

### Advanced Usage
```bash
# Custom timeout settings
lx-hw-detect --timeout 60 --tool-timeout 15

# Use custom template
lx-hw-detect --template custom-report.md.j2

# Sequential execution for debugging
lx-hw-detect --no-parallel --verbose

# Development mode (no anonymization)
lx-hw-detect --no-anonymize --format json
```

## Error Handling and Messages

### User-Friendly Error Messages
```bash
# Tool not found
Error: lshw command not found
Suggestion: Install lshw using your package manager:
  - Ubuntu/Debian: sudo apt install lshw
  - Fedora/RHEL: sudo dnf install lshw  
  - Arch: sudo pacman -S lshw
  - openSUSE: sudo zypper install lshw

# Permission denied
Error: dmidecode requires root privileges
Suggestions:
  - Run with sudo: sudo lx-hw-detect
  - Skip privileged tools: lx-hw-detect --no-root-tools
  - Enable sudo option: lx-hw-detect --sudo

# Timeout error
Error: Hardware detection timed out after 30 seconds
Suggestions:
  - Increase timeout: lx-hw-detect --timeout 60
  - Run tools individually: lx-hw-detect --no-parallel
  - Skip slow tools: lx-hw-detect --tools lspci,lsusb
```

### Progress Indicators
```bash
# Default progress display
Detecting hardware...
[■■■□□] 60% - Running lspci... (2/3 tools completed)

# Verbose progress
[INFO] Starting hardware detection with 5 tools
[INFO] Running lshw (requires root)... ✓ completed (2.3s)
[INFO] Running dmidecode (requires root)... ✓ completed (1.1s)  
[INFO] Running lspci... ✓ completed (0.3s)
[INFO] Running lsusb... ✓ completed (0.2s)
[INFO] Running inxi... ✓ completed (1.8s)
[INFO] Generating report with enhanced privacy...
[INFO] Report saved to hardware-report.md
```

## Shell Integration

### Bash Completion
```bash
# Install completion
lx-hw-detect completion bash > /etc/bash_completion.d/lx-hw-detect

# Or for user-specific completion
lx-hw-detect completion bash > ~/.local/share/bash-completion/completions/lx-hw-detect
```

### Zsh Completion  
```bash
# Add to .zshrc
lx-hw-detect completion zsh > "${fpath[1]}/_lx-hw-detect"
```

### Fish Completion
```bash
lx-hw-detect completion fish > ~/.config/fish/completions/lx-hw-detect.fish
```

## Integration APIs

### Return Data Structure (JSON Format)
```json
{
  "success": true,
  "exit_code": 0,
  "execution_time_ms": 5240,
  "tools_used": ["lshw", "dmidecode", "lspci", "lsusb", "inxi"],
  "tools_skipped": [],
  "warnings": [],
  "report": {
    "metadata": {
      "report_id": "hw_a1b2c3d4e5f6",
      "generated_at": "2025-08-26T10:30:00Z",
      "tool_version": "1.0.0",
      "privacy_level": "enhanced"
    },
    "content": "# Hardware Compatibility Report\n..."
  }
}
```

### Error Response Structure
```json
{
  "success": false,
  "exit_code": 2,
  "error": {
    "type": "ToolNotFound",
    "message": "lshw command not found",
    "suggestions": [
      "Install lshw using your package manager",
      "Ubuntu/Debian: sudo apt install lshw"
    ]
  },
  "tools_attempted": ["lshw"],
  "tools_successful": [],
  "partial_results": null
}
```

This comprehensive CLI specification ensures consistent user experience while providing flexibility for different use cases and environments.