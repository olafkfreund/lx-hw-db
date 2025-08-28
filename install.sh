#!/bin/bash
# Linux Hardware Database Installation Script
# Supports multiple distributions and installation methods

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
VERSION="latest"
INSTALL_METHOD=""
FORCE_INSTALL=false
INSTALL_GUI=true
INSTALL_WEB=false
INSTALL_DIR="/usr/local"

# GitHub repository information
REPO="lx-hw-db/lx-hw-db"
GITHUB_API="https://api.github.com/repos/${REPO}"
GITHUB_RELEASES="${GITHUB_API}/releases"

print_info() {
    echo -e "${BLUE}INFO:${NC} $1"
}

print_success() {
    echo -e "${GREEN}SUCCESS:${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}WARNING:${NC} $1"
}

print_error() {
    echo -e "${RED}ERROR:${NC} $1"
    exit 1
}

show_help() {
    cat << EOF
Linux Hardware Database Installation Script

USAGE:
    $0 [OPTIONS]

OPTIONS:
    -h, --help              Show this help message
    -v, --version VERSION   Install specific version (default: latest)
    -m, --method METHOD     Installation method (auto, package, binary, cargo, docker, flatpak)
    -f, --force             Force installation even if already installed
    --no-gui                Skip GUI components installation
    --web                   Install web interface components
    --prefix DIR            Installation prefix (default: /usr/local)

INSTALLATION METHODS:
    auto                    Automatically detect best method for your system
    package                 Use distribution package manager (deb, rpm, etc.)
    binary                  Download and install pre-built binaries
    cargo                   Compile from source using Cargo
    docker                  Install Docker containers
    flatpak                 Install Flatpak package

EXAMPLES:
    $0                                  # Auto-install latest version
    $0 -v v0.1.0 -m binary             # Install specific version as binary
    $0 --no-gui --web --prefix /opt    # Install CLI and web interface to /opt
    $0 -m docker                       # Install Docker containers only

EOF
}

detect_architecture() {
    case "$(uname -m)" in
        x86_64) echo "x86_64" ;;
        aarch64|arm64) echo "aarch64" ;;
        armv7l) echo "armv7" ;;
        *) print_error "Unsupported architecture: $(uname -m)" ;;
    esac
}

detect_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if command -v lsb_release >/dev/null 2>&1; then
            lsb_release -si | tr '[:upper:]' '[:lower:]'
        elif [[ -f /etc/os-release ]]; then
            . /etc/os-release
            echo "$ID"
        elif [[ -f /etc/debian_version ]]; then
            echo "debian"
        elif [[ -f /etc/redhat-release ]]; then
            echo "rhel"
        else
            echo "unknown"
        fi
    else
        print_error "This script only supports Linux systems"
    fi
}

detect_package_manager() {
    local os=$1
    
    case $os in
        ubuntu|debian) echo "apt" ;;
        fedora|rhel|centos|rocky|almalinux) echo "dnf" ;;
        opensuse*|sles) echo "zypper" ;;
        arch|manjaro|endeavouros) echo "pacman" ;;
        alpine) echo "apk" ;;
        nixos) echo "nix" ;;
        *) echo "unknown" ;;
    esac
}

check_dependencies() {
    local missing_deps=()
    
    # Check for required system utilities
    for cmd in curl tar; do
        if ! command -v "$cmd" >/dev/null 2>&1; then
            missing_deps+=("$cmd")
        fi
    done
    
    # Check for hardware detection tools
    for cmd in lshw dmidecode lspci lsusb; do
        if ! command -v "$cmd" >/dev/null 2>&1; then
            print_warning "Hardware detection tool '$cmd' not found - some features may be limited"
        fi
    done
    
    if [[ ${#missing_deps[@]} -gt 0 ]]; then
        print_error "Missing required dependencies: ${missing_deps[*]}"
    fi
}

get_latest_version() {
    print_info "Fetching latest release information..."
    local latest_url="${GITHUB_RELEASES}/latest"
    
    if ! curl -sf "$latest_url" | grep '"tag_name"' | cut -d'"' -f4; then
        print_error "Failed to fetch latest version from GitHub"
    fi
}

download_release_asset() {
    local version=$1
    local asset_name=$2
    local output_path=$3
    
    local download_url
    if [[ "$version" == "latest" ]]; then
        download_url="${GITHUB_RELEASES}/latest/download/${asset_name}"
    else
        download_url="${GITHUB_RELEASES}/download/${version}/${asset_name}"
    fi
    
    print_info "Downloading $asset_name..."
    if ! curl -fL -o "$output_path" "$download_url"; then
        print_error "Failed to download $asset_name from $download_url"
    fi
}

install_with_package_manager() {
    local os=$1
    local pkg_manager=$2
    local version=$3
    local arch
    arch=$(detect_architecture)
    
    case $pkg_manager in
        apt)
            local deb_arch
            case $arch in
                x86_64) deb_arch="amd64" ;;
                aarch64) deb_arch="arm64" ;;
                *) print_error "Unsupported architecture for Debian packages: $arch" ;;
            esac
            
            local deb_name="lx-hw-db_${version#v}_${deb_arch}.deb"
            local deb_path="/tmp/$deb_name"
            
            download_release_asset "$version" "$deb_name" "$deb_path"
            
            print_info "Installing Debian package..."
            sudo apt update
            sudo apt install -y "$deb_path"
            rm "$deb_path"
            ;;
            
        dnf)
            local rpm_name="lx-hw-db-${version#v}-1.${arch}.rpm"
            local rpm_path="/tmp/$rpm_name"
            
            download_release_asset "$version" "$rpm_name" "$rpm_path"
            
            print_info "Installing RPM package..."
            sudo dnf install -y "$rpm_path"
            rm "$rpm_path"
            ;;
            
        pacman)
            local pkg_name="lx-hw-db-${version#v}-1-${arch}.pkg.tar.gz"
            local pkg_path="/tmp/$pkg_name"
            
            download_release_asset "$version" "$pkg_name" "$pkg_path"
            
            print_info "Installing Arch package..."
            sudo pacman -U --noconfirm "$pkg_path"
            rm "$pkg_path"
            ;;
            
        nix)
            print_info "Installing with Nix..."
            if [[ -f flake.nix ]]; then
                nix profile install .#lx-hw-db
            else
                print_error "Nix installation requires running from the source repository"
            fi
            ;;
            
        *)
            print_error "Package manager '$pkg_manager' not supported for automated installation"
            ;;
    esac
}

install_binary() {
    local version=$1
    local arch
    arch=$(detect_architecture)
    
    # Determine target triple
    local target
    if command -v ldd >/dev/null 2>&1 && ldd --version 2>&1 | grep -q musl; then
        target="${arch}-unknown-linux-musl"
    else
        target="${arch}-unknown-linux-gnu"
    fi
    
    local archive_name="lx-hw-db-${version}-${target}.tar.gz"
    local archive_path="/tmp/$archive_name"
    
    download_release_asset "$version" "$archive_name" "$archive_path"
    
    print_info "Extracting binaries..."
    local extract_dir="/tmp/lx-hw-db-extract"
    mkdir -p "$extract_dir"
    tar -xzf "$archive_path" -C "$extract_dir"
    
    print_info "Installing binaries to $INSTALL_DIR..."
    sudo mkdir -p "$INSTALL_DIR/bin"
    sudo cp "$extract_dir/lx-hw-detect" "$INSTALL_DIR/bin/"
    sudo cp "$extract_dir/lx-hw-indexer" "$INSTALL_DIR/bin/"
    
    if $INSTALL_GUI && [[ -f "$extract_dir/lx-hw-detect-gtk" ]]; then
        sudo cp "$extract_dir/lx-hw-detect-gtk" "$INSTALL_DIR/bin/"
    fi
    
    sudo chmod +x "$INSTALL_DIR/bin/lx-hw-detect" "$INSTALL_DIR/bin/lx-hw-indexer"
    if [[ -f "$INSTALL_DIR/bin/lx-hw-detect-gtk" ]]; then
        sudo chmod +x "$INSTALL_DIR/bin/lx-hw-detect-gtk"
    fi
    
    # Install configuration, man pages, and completions if available
    if [[ -d "$extract_dir/config" ]]; then
        sudo mkdir -p /etc/lx-hw-db
        sudo cp -r "$extract_dir/config/"* /etc/lx-hw-db/
    fi
    
    if [[ -d "$extract_dir/docs/man" ]]; then
        sudo mkdir -p "$INSTALL_DIR/share/man/man1"
        sudo cp "$extract_dir/docs/man/"*.1 "$INSTALL_DIR/share/man/man1/"
    fi
    
    if [[ -d "$extract_dir/completions" ]]; then
        # Install bash completions
        if [[ -d "$extract_dir/completions/bash" ]]; then
            sudo mkdir -p "$INSTALL_DIR/share/bash-completion/completions"
            sudo cp "$extract_dir/completions/bash/"* "$INSTALL_DIR/share/bash-completion/completions/"
        fi
        
        # Install zsh completions  
        if [[ -d "$extract_dir/completions/zsh" ]]; then
            sudo mkdir -p "$INSTALL_DIR/share/zsh/site-functions"
            sudo cp "$extract_dir/completions/zsh/"* "$INSTALL_DIR/share/zsh/site-functions/"
        fi
        
        # Install fish completions
        if [[ -d "$extract_dir/completions/fish" ]]; then
            sudo mkdir -p "$INSTALL_DIR/share/fish/vendor_completions.d"
            sudo cp "$extract_dir/completions/fish/"* "$INSTALL_DIR/share/fish/vendor_completions.d/"
        fi
    fi
    
    # Cleanup
    rm -rf "$archive_path" "$extract_dir"
    
    # Update PATH if needed
    if [[ ":$PATH:" != *":$INSTALL_DIR/bin:"* ]]; then
        print_warning "Add $INSTALL_DIR/bin to your PATH:"
        echo "  export PATH=\"$INSTALL_DIR/bin:\$PATH\""
    fi
}

install_cargo() {
    print_info "Installing from source using Cargo..."
    
    # Check if Cargo is installed
    if ! command -v cargo >/dev/null 2>&1; then
        print_error "Cargo not found. Please install Rust: https://rustup.rs/"
    fi
    
    # Install system dependencies
    local os
    os=$(detect_os)
    local pkg_manager
    pkg_manager=$(detect_package_manager "$os")
    
    print_info "Installing build dependencies..."
    case $pkg_manager in
        apt)
            sudo apt update
            sudo apt install -y build-essential pkg-config libssl-dev
            if $INSTALL_GUI; then
                sudo apt install -y libgtk-4-dev libadwaita-1-dev
            fi
            ;;
        dnf)
            sudo dnf install -y gcc pkg-config openssl-devel
            if $INSTALL_GUI; then
                sudo dnf install -y gtk4-devel libadwaita-devel
            fi
            ;;
        pacman)
            sudo pacman -S --noconfirm base-devel pkg-config openssl
            if $INSTALL_GUI; then
                sudo pacman -S --noconfirm gtk4 libadwaita
            fi
            ;;
    esac
    
    # Install from crates.io or git
    if [[ "$VERSION" == "latest" ]] || [[ "$VERSION" =~ ^v.*$ ]]; then
        local cargo_args="--git https://github.com/${REPO}"
        if [[ "$VERSION" != "latest" ]]; then
            cargo_args+=" --tag $VERSION"
        fi
        
        cargo install $cargo_args --bin lx-hw-detect --bin lx-hw-indexer
        
        if $INSTALL_GUI; then
            cargo install $cargo_args --bin lx-hw-detect-gtk --features gtk-gui
        fi
    else
        print_error "Invalid version format for Cargo installation: $VERSION"
    fi
}

install_docker() {
    print_info "Setting up Docker containers..."
    
    # Check if Docker is installed
    if ! command -v docker >/dev/null 2>&1; then
        print_error "Docker not found. Please install Docker: https://docs.docker.com/get-docker/"
    fi
    
    # Pull images
    local image_tag="latest"
    if [[ "$VERSION" != "latest" ]]; then
        image_tag="$VERSION"
    fi
    
    print_info "Pulling Docker images..."
    docker pull "ghcr.io/${REPO,,}:${image_tag}-cli"
    
    if $INSTALL_WEB; then
        docker pull "ghcr.io/${REPO,,}:${image_tag}-web"
    fi
    
    # Create wrapper scripts
    sudo mkdir -p "$INSTALL_DIR/bin"
    
    cat > "/tmp/lx-hw-detect" << EOF
#!/bin/bash
docker run --rm -i --privileged \\
    -v /sys:/sys:ro \\
    -v /proc:/proc:ro \\
    -v /dev:/dev:ro \\
    -v "\$PWD":/data \\
    ghcr.io/${REPO,,}:${image_tag}-cli "\$@"
EOF
    
    sudo mv "/tmp/lx-hw-detect" "$INSTALL_DIR/bin/lx-hw-detect"
    sudo chmod +x "$INSTALL_DIR/bin/lx-hw-detect"
    
    if $INSTALL_WEB; then
        # Create docker-compose file
        cat > "/tmp/docker-compose.yml" << EOF
version: '3.8'
services:
  lx-hw-db-web:
    image: ghcr.io/${REPO,,}:${image_tag}-web
    ports:
      - "8000:8000"
    volumes:
      - ./data:/var/www/lx-hw-db/data
    restart: unless-stopped
EOF
        sudo mkdir -p /opt/lx-hw-db
        sudo mv "/tmp/docker-compose.yml" /opt/lx-hw-db/
        
        print_info "Web interface Docker Compose file installed to /opt/lx-hw-db/"
        print_info "Start with: cd /opt/lx-hw-db && docker compose up -d"
    fi
}

install_flatpak() {
    print_info "Installing Flatpak package..."
    
    # Check if Flatpak is installed
    if ! command -v flatpak >/dev/null 2>&1; then
        print_error "Flatpak not found. Please install Flatpak first."
    fi
    
    # Add Flathub if not already added
    if ! flatpak remotes | grep -q flathub; then
        print_info "Adding Flathub repository..."
        flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
    fi
    
    # Install the application
    print_info "Installing from Flathub..."
    flatpak install -y flathub org.lxhwdb.LxHwDb
}

install_web_interface() {
    local version=$1
    local web_bundle="lx-hw-db-web-${version}.tar.gz"
    local bundle_path="/tmp/$web_bundle"
    
    download_release_asset "$version" "$web_bundle" "$bundle_path"
    
    print_info "Installing web interface..."
    sudo mkdir -p /var/www/lx-hw-db
    sudo tar -xzf "$bundle_path" -C /var/www/lx-hw-db --strip-components=1
    
    # Run installation script
    cd /var/www/lx-hw-db
    sudo bash install.sh
    
    rm "$bundle_path"
    
    print_success "Web interface installed and running on http://localhost:8000"
}

determine_install_method() {
    local os=$1
    local pkg_manager=$2
    
    # Priority order: package > binary > cargo
    case $pkg_manager in
        apt|dnf|pacman) echo "package" ;;
        *) echo "binary" ;;
    esac
}

check_existing_installation() {
    if command -v lx-hw-detect >/dev/null 2>&1; then
        local current_version
        current_version=$(lx-hw-detect --version 2>/dev/null | head -n1 | awk '{print $NF}' || echo "unknown")
        
        if ! $FORCE_INSTALL; then
            print_warning "Linux Hardware Database is already installed (version: $current_version)"
            read -p "Continue with installation? (y/N): " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                print_info "Installation cancelled"
                exit 0
            fi
        else
            print_info "Found existing installation (version: $current_version), forcing reinstall"
        fi
    fi
}

main() {
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_help
                exit 0
                ;;
            -v|--version)
                VERSION="$2"
                shift 2
                ;;
            -m|--method)
                INSTALL_METHOD="$2"
                shift 2
                ;;
            -f|--force)
                FORCE_INSTALL=true
                shift
                ;;
            --no-gui)
                INSTALL_GUI=false
                shift
                ;;
            --web)
                INSTALL_WEB=true
                shift
                ;;
            --prefix)
                INSTALL_DIR="$2"
                shift 2
                ;;
            *)
                print_error "Unknown option: $1"
                ;;
        esac
    done
    
    print_info "Linux Hardware Database Installation Script"
    print_info "=========================================="
    
    # Detect system information
    local os arch
    os=$(detect_os)
    arch=$(detect_architecture)
    local pkg_manager
    pkg_manager=$(detect_package_manager "$os")
    
    print_info "Detected system: $os ($arch)"
    print_info "Package manager: $pkg_manager"
    
    # Check dependencies
    check_dependencies
    
    # Check for existing installation
    check_existing_installation
    
    # Determine version to install
    if [[ "$VERSION" == "latest" ]]; then
        VERSION=$(get_latest_version)
        print_info "Latest version: $VERSION"
    fi
    
    # Determine installation method
    if [[ -z "$INSTALL_METHOD" ]]; then
        INSTALL_METHOD=$(determine_install_method "$os" "$pkg_manager")
    fi
    
    print_info "Installation method: $INSTALL_METHOD"
    print_info "Version: $VERSION"
    print_info "Install GUI: $INSTALL_GUI"
    print_info "Install web: $INSTALL_WEB"
    
    # Perform installation
    case $INSTALL_METHOD in
        package)
            install_with_package_manager "$os" "$pkg_manager" "$VERSION"
            ;;
        binary)
            install_binary "$VERSION"
            ;;
        cargo)
            install_cargo
            ;;
        docker)
            install_docker
            ;;
        flatpak)
            install_flatpak
            ;;
        auto)
            local auto_method
            auto_method=$(determine_install_method "$os" "$pkg_manager")
            INSTALL_METHOD="$auto_method"
            main # Restart with determined method
            exit $?
            ;;
        *)
            print_error "Unknown installation method: $INSTALL_METHOD"
            ;;
    esac
    
    # Install web interface if requested
    if $INSTALL_WEB && [[ "$INSTALL_METHOD" != "docker" ]]; then
        install_web_interface "$VERSION"
    fi
    
    print_success "Linux Hardware Database installed successfully!"
    print_info ""
    print_info "Usage:"
    print_info "  lx-hw-detect --help          # Show CLI help"
    print_info "  lx-hw-detect detect          # Detect hardware"
    print_info "  lx-hw-indexer --help         # Show indexer help"
    
    if $INSTALL_GUI && command -v lx-hw-detect-gtk >/dev/null 2>&1; then
        print_info "  lx-hw-detect-gtk             # Launch GUI interface"
    fi
    
    if $INSTALL_WEB; then
        print_info "  Web interface: http://localhost:8000"
    fi
    
    print_info ""
    print_info "Documentation: https://github.com/${REPO}/tree/main/docs"
    print_info "Report issues: https://github.com/${REPO}/issues"
}

# Run main function with all arguments
main "$@"