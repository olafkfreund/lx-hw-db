#!/bin/bash
# Arch Linux specific installation script for Linux Hardware Database

set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}Installing Linux Hardware Database on Arch Linux...${NC}"

# Update package database
echo -e "${BLUE}Updating package database...${NC}"
sudo pacman -Sy

# Install dependencies
echo -e "${BLUE}Installing dependencies...${NC}"
sudo pacman -S --needed --noconfirm \
    curl \
    wget \
    lshw \
    dmidecode \
    pciutils \
    usbutils \
    util-linux \
    ca-certificates

# Check architecture
ARCH=$(uname -m)
echo -e "${BLUE}Detected architecture: $ARCH${NC}"

# Get latest release
echo -e "${BLUE}Fetching latest release...${NC}"
LATEST_URL="https://api.github.com/repos/lx-hw-db/lx-hw-db/releases/latest"
VERSION=$(curl -s "$LATEST_URL" | grep '"tag_name"' | cut -d'"' -f4)

if [ -z "$VERSION" ]; then
    echo "Failed to fetch latest version"
    exit 1
fi

echo -e "${BLUE}Latest version: $VERSION${NC}"

# Download and install package
PKG_FILE="lx-hw-db-${VERSION#v}-1-${ARCH}.pkg.tar.gz"
DOWNLOAD_URL="https://github.com/lx-hw-db/lx-hw-db/releases/download/$VERSION/$PKG_FILE"

echo -e "${BLUE}Downloading $PKG_FILE...${NC}"
cd /tmp
wget "$DOWNLOAD_URL"

echo -e "${BLUE}Installing package...${NC}"
sudo pacman -U --noconfirm "./$PKG_FILE"

# Cleanup
rm "$PKG_FILE"

echo -e "${GREEN}Installation complete!${NC}"
echo
echo "Usage:"
echo "  lx-hw-detect --help          # Show help"
echo "  lx-hw-detect detect          # Detect hardware"
echo "  lx-hw-detect-gtk             # Launch GUI (if available)"

# Check if GUI was installed
if command -v lx-hw-detect-gtk >/dev/null 2>&1; then
    echo
    echo "GUI application installed. You can also find it in your applications menu."
fi