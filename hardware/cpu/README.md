# CPU Compatibility Database

This directory contains processor compatibility data for Linux systems.

## Structure

- `intel/` - Intel processors (Core, Xeon, Atom, etc.)
- `amd/` - AMD processors (Ryzen, EPYC, Threadripper, etc.)

## Compatibility Status

- **full**: Complete hardware support with optimal performance
- **partial**: Basic functionality with some limitations
- **limited**: Minimal support, reduced performance
- **none**: No Linux support or major issues

## Testing Focus

- CPU feature support (AVX, virtualization, power management)
- Thermal management and scaling
- Performance under different kernel versions
- Compatibility with various motherboard chipsets