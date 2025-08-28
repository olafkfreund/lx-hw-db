# GPU Compatibility Database

This directory contains graphics hardware compatibility data for Linux systems.

## Structure

- `nvidia/` - NVIDIA graphics cards (GeForce, Quadro, Tesla)
- `amd/` - AMD graphics cards (Radeon, FirePro, Instinct)
- `intel/` - Intel graphics (integrated and discrete Arc)

## Driver Categories

- **Open Source**: Mesa, nouveau, amdgpu drivers
- **Proprietary**: NVIDIA proprietary, AMD AMDGPU-PRO
- **Hybrid**: Mixed driver configurations

## Testing Focus

- Driver installation and compatibility
- 3D acceleration and OpenGL/Vulkan support
- Video encode/decode capabilities
- Multi-monitor setup compatibility
- Power management and thermal control
- Gaming performance and stability