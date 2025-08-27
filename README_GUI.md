# GTK4 GUI Development Guide

## Overview

The lx-hw-db project now includes a modern GTK4 graphical user interface built with libadwaita, providing an intuitive and privacy-focused experience for hardware detection and community contribution.

## Quick Start (NixOS)

### Development Shell

```bash
# Enter the development environment
nix develop

# Build the GUI application
cargo build --bin lx-hw-detect-gtk --release

# Run the GUI application
cargo run --bin lx-hw-detect-gtk
```

### Alternative: Using direnv

```bash
# Allow direnv to load the development shell automatically
direnv allow

# Build and run
cargo build --bin lx-hw-detect-gtk --release
./target/release/lx-hw-detect-gtk
```

## Architecture

### GUI Components

- **Application**: Main GTK4 application class with libadwaita integration
- **Window**: Main application window with sidebar navigation and view stack
- **Widgets**: Modular widget system for hardware display, progress tracking, and configuration
- **Models**: Comprehensive state management with reactive patterns
- **Utils**: Background task management and utility functions

### Key Features

- **Privacy-First Design**: Prominent privacy controls and indicators throughout the interface
- **Modern GNOME Design**: Full Adwaita compliance with dark/light mode support
- **Real-Time Progress**: Background hardware detection with live progress updates
- **Configuration Recommendations**: Visual display of driver and system configuration suggestions
- **Community Integration**: Export and submission workflows for GitHub-based collaboration

## Project Structure

```
src/
├── gui/
│   ├── mod.rs              # Main GUI module and initialization
│   ├── application.rs      # GTK4 application implementation
│   ├── window.rs           # Main window with navigation
│   ├── models.rs           # State management and data models
│   ├── utils.rs            # Background controllers and utilities
│   └── widgets/
│       ├── detection_progress.rs  # Hardware detection progress UI
│       ├── hardware_view.rs       # Hardware display interface
│       ├── configuration_view.rs  # Configuration recommendations
│       ├── device_card.rs         # Individual device display
│       └── export_dialog.rs       # Export and submission dialogs
├── resources/
│   └── style.css           # Custom GTK4 styling
└── bin/
    └── lx-hw-detect-gtk.rs # GUI application entry point
```

## Development Environment

### NixOS Flake

The project includes a comprehensive NixOS flake with:

- **Rust Toolchain**: Latest stable Rust with rust-analyzer and debugging tools
- **GTK4 Libraries**: Complete GTK4, libadwaita, and GNOME development stack
- **Build Tools**: pkg-config, meson, ninja for native library compilation
- **Hardware Tools**: lshw, dmidecode, lspci, lsusb, inxi for development testing
- **Development Tools**: cargo-watch, cargo-edit, cargo-audit for enhanced workflow

### System Requirements

- **NixOS**: Flake-enabled system with experimental features
- **GTK4**: Version 4.10+ with libadwaita 1.5+
- **Hardware**: Any system capable of running GTK4 applications
- **Memory**: 2GB+ RAM recommended for compilation

## Usage Workflow

### 1. Welcome Screen
- Project introduction and privacy overview
- Quick start guidance for new users
- Access to privacy settings

### 2. Hardware Detection
- Real-time progress tracking with individual tool status
- Background processing using Rust threading with glib integration
- Privacy level selection and data collection controls

### 3. Hardware Display  
- Expandable categories organized by device type
- Compatibility status indicators (supported/partial/unsupported/unknown)
- Device details with vendor, model, and driver information

### 4. Configuration Recommendations
- Driver installation suggestions with priority indicators
- Kernel parameter recommendations for optimal performance
- Package installation guidance for multiple distributions

### 5. Export & Submission
- Multiple export formats (YAML, JSON, Markdown)
- GitHub submission workflow integration
- Community contribution guidelines

## Privacy Features

### Anonymization Levels
- **Basic**: 24-hour salt rotation for hardware IDs
- **Enhanced**: 12-hour salt rotation with increased noise
- **Strict**: 1-hour salt rotation with maximum privacy protection

### Data Collection Controls
- Hardware information toggle (vendor/model data)
- Kernel compatibility toggle (driver and module data)
- Performance data toggle (benchmark and metrics data)

### Security Implementation
- HMAC-SHA256 hashing with time-rotating salts
- No raw hardware identifiers stored or transmitted
- Differential privacy with configurable noise levels
- Complete user control over data sharing preferences

## Internationalization

The GUI supports full internationalization using gettext-rs:

```rust
// Usage in code
use gettext_rs::gettext;
let localized = gettext("Hardware Detection");

// Macro for convenience
use crate::gui::t;
let text = t!("Privacy Settings");
```

## Theming and Styling

### Custom CSS Classes
- `.status-supported`, `.status-partial`, `.status-unsupported`, `.status-unknown`
- `.privacy-basic`, `.privacy-enhanced`, `.privacy-strict`
- `.detection-progress`, `.tool-progress-row`
- `.device-card`, `.config-section`

### Adwaita Integration
- Automatic dark/light mode adaptation
- System accent color integration
- Responsive design for different screen sizes
- Accessibility support with proper ARIA labels

## Testing

### Development Testing
```bash
# Run with debug logging
RUST_LOG=debug cargo run --bin lx-hw-detect-gtk

# Enable GTK debug features
GTK_DEBUG=interactive cargo run --bin lx-hw-detect-gtk

# Test with mock data (no root privileges required)
cargo run --bin lx-hw-detect-gtk -- --mock-data
```

### Hardware Testing
- Supports testing on live systems with actual hardware detection
- Mock data generation for development without hardware access
- Integration with existing CLI backend for consistency validation

## Performance Considerations

### Memory Management
- Rust's ownership model ensures memory safety throughout
- Proper GTK4 widget lifecycle management
- Background task cleanup on application exit

### UI Responsiveness  
- Background threading for hardware detection prevents UI blocking
- Efficient state management with minimal unnecessary updates
- Lazy loading of hardware device details

### Build Optimization
- Release builds use LTO (Link Time Optimization)
- Stripped binaries for reduced distribution size
- Optimized GTK4 library linking

## Future Enhancements

### Planned Features
- Real-time hardware monitoring with live updates
- Advanced filtering and search capabilities for hardware database
- Integration with system package managers for automated driver installation
- Export to additional formats (PDF reports, HTML summaries)

### Community Integration
- Direct GitHub issue creation from the GUI
- Community hardware database browsing interface
- Collaborative hardware testing coordination tools

## Contributing

### GUI Development Standards
- Follow existing code organization patterns
- Use proper error handling with comprehensive Result types
- Maintain Adwaita design language compliance
- Include comprehensive comments for UI logic

### Testing Requirements
- Test on both development and live hardware systems
- Validate privacy controls and data anonymization
- Ensure accessibility standards compliance
- Cross-platform testing (different GTK4 versions)

## Troubleshooting

### Common Issues

**Compilation Errors:**
```bash
# Ensure NixOS development shell is active
nix develop
echo $PKG_CONFIG_PATH  # Should show GTK4 library paths
```

**Runtime Errors:**
```bash
# Check GTK4 version compatibility
pkg-config --modversion gtk4

# Verify library loading
export LD_LIBRARY_PATH="${LD_LIBRARY_PATH}:$(pkg-config --libs gtk4 | tr ' ' ':')"
```

**Display Issues:**
```bash
# Test with different GTK backends
GDK_BACKEND=wayland cargo run --bin lx-hw-detect-gtk
GDK_BACKEND=x11 cargo run --bin lx-hw-detect-gtk
```

For additional support, see the main project documentation or open an issue on GitHub with detailed system information and error logs.