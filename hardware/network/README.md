# Network Hardware Compatibility Database

This directory contains network hardware compatibility data for Linux systems.

## Categories

- **Ethernet controllers** - Wired network interfaces
- **WiFi adapters** - Wireless network cards (PCIe, USB)
- **Bluetooth controllers** - Bluetooth adapters and chipsets
- **Mobile broadband** - 4G/5G modems and cards

## Driver Categories

- **Built-in kernel**: Native Linux kernel support
- **Additional firmware**: Requires proprietary firmware blobs
- **Out-of-tree**: Requires external driver installation

## Testing Focus

- Driver availability and stability
- Connection speed and reliability
- Power management (especially for WiFi)
- Advanced features (monitor mode, injection for WiFi)
- Wake-on-LAN functionality
- VLAN support
- Network offloading features