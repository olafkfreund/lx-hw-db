## Hardware Compatibility Report Submission

Thank you for contributing to the Linux Hardware Compatibility Database! This template helps ensure your hardware report follows our community standards.

### Hardware Information

**Device Category:** 
<!-- Select one: cpu, gpu, motherboard, network, storage, audio, usb, other -->

**Manufacturer:** 
<!-- e.g., Intel, AMD, NVIDIA, Realtek -->

**Device Model:** 
<!-- Full device name and model number -->

**Hardware ID:** 
<!-- PCI ID (vendor:device) or USB ID - will be anonymized -->

### Compatibility Details

**Linux Distribution:** 
<!-- e.g., Ubuntu 22.04, Fedora 38, Arch Linux, NixOS 23.11 -->

**Kernel Version:** 
<!-- e.g., 6.2.0, 6.5.3 -->

**Compatibility Status:**
- [ ] Full compatibility - works out of the box
- [ ] Partial compatibility - requires configuration
- [ ] Limited compatibility - basic functionality only  
- [ ] No compatibility - does not work

**Driver Used:** 
<!-- e.g., nvidia-driver-535, amdgpu, iwlwifi -->

### Testing Information

**Detection Tool Used:**
<!-- e.g., lx-hw-detect, manual submission -->

**Privacy Level:**
- [ ] Basic - Standard anonymization
- [ ] Enhanced - Additional privacy protections
- [ ] Strict - Maximum anonymization

**Testing Duration:** 
<!-- How long have you used this configuration? -->

### Additional Context

**Configuration Notes:**
<!-- Any special configuration required, kernel parameters, etc. -->

**Known Issues:**
<!-- Any problems or limitations discovered -->

**Performance Notes:**
<!-- Performance characteristics, benchmarks if available -->

### Submission Checklist

- [ ] Hardware report file follows naming convention: `hardware/{category}/{manufacturer}/{device}.yaml`
- [ ] All personal information has been removed/anonymized
- [ ] Report generated with privacy-compliant tools (lx-hw-detect recommended)
- [ ] Tested configuration for at least 48 hours without issues
- [ ] No serial numbers, MAC addresses, or other identifying information included
- [ ] Report includes required fields: device ID, compatibility status, kernel version
- [ ] Configuration is reproducible by others

### Community Guidelines

By submitting this hardware report, I confirm that:

- [ ] The information is accurate and based on real testing
- [ ] I have the right to share this hardware compatibility information
- [ ] The report contains no proprietary or confidential information
- [ ] I understand this will be published under CC0 (public domain)
- [ ] I have read and agree to the project's Code of Conduct

### Automated Validation

Once submitted, this PR will be automatically validated for:
- Schema compliance
- Privacy protection
- File format standards
- Hardware ID validity

Please wait for the validation checks to complete before requesting review.

---

**Note for Reviewers:** This hardware report will be automatically validated by GitHub Actions. Manual review should focus on accuracy, usefulness to the community, and compliance with quality standards.