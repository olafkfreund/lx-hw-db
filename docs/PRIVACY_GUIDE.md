# Privacy and Anonymization Guide

This guide explains how the Linux Hardware Compatibility Database protects user privacy while collecting valuable hardware compatibility information.

## Privacy Philosophy

The Linux Hardware Compatibility Database is designed with **privacy by design**:

- **Collect Only What's Needed**: Only hardware compatibility information is collected
- **Anonymize Everything Identifying**: All personally identifiable information is cryptographically anonymized
- **Transparent Process**: Open source code allows community verification of privacy practices
- **User Control**: Users choose their privacy level and what information to share

## How Anonymization Works

### System ID Generation

Each system gets a unique, anonymous identifier generated using cryptographic hashing:

```rust
// Simplified example of the anonymization process
let system_info = format!("{}{}{}", hostname, mac_addresses, system_uuid);
let salt = generate_time_based_salt(privacy_level);
let anonymized_id = hmac_sha256(system_info, salt);
```

### Key Components

1. **Input Data**: Hostname, MAC addresses, system UUID, hardware serials
2. **Salt Generation**: Time-based salt that rotates based on privacy level  
3. **HMAC-SHA256**: Cryptographic hash function providing one-way anonymization
4. **Truncation**: Only first 12 characters of hash used as system ID

### Privacy Levels

#### Basic Privacy (24-hour salt rotation)
- **Salt Rotation**: Every 24 hours
- **Use Case**: Most home users and general systems
- **Anonymization Period**: Systems remain unlinked across 24-hour periods
- **Trade-off**: Allows correlation within 24 hours for debugging/validation

#### Enhanced Privacy (12-hour salt rotation) ⭐ **Recommended**
- **Salt Rotation**: Every 12 hours  
- **Use Case**: Work systems, privacy-conscious users
- **Anonymization Period**: Systems remain unlinked across 12-hour periods
- **Trade-off**: Balance between privacy and system identification

#### Strict Privacy (1-hour salt rotation)
- **Salt Rotation**: Every hour
- **Use Case**: Highly sensitive environments, maximum privacy
- **Anonymization Period**: Systems unlinked after 1 hour
- **Trade-off**: Maximum privacy, minimal correlation window

## What Gets Anonymized

### System Identifiers
```json
{
  "before_anonymization": {
    "hostname": "johns-laptop",
    "system_uuid": "12345678-1234-1234-1234-123456789abc"
  },
  "after_anonymization": {
    "anonymized_system_id": "abc123def456",
    "hostname": "[ANONYMIZED]"
  }
}
```

### Network Hardware
```json
{
  "before_anonymization": {
    "mac_address": "aa:bb:cc:dd:ee:ff",
    "interface_name": "wlp3s0"
  },
  "after_anonymization": {
    "mac_address": "xx:xx:xx:12:34:56",
    "interface_name": "wlan0"
  }
}
```

### Storage Devices
```json
{
  "before_anonymization": {
    "serial_number": "WD-WCC4N6123456",
    "model": "WD Blue SN550 NVMe SSD"
  },
  "after_anonymization": {
    "serial_number": "[ANON-789ABC]",
    "model": "WD Blue SN550 NVMe SSD"
  }
}
```

## What Remains Public

### Hardware Information
- **Vendor Names**: Intel, AMD, NVIDIA, etc.
- **Product Models**: Specific CPU, GPU, and hardware models
- **Driver Information**: Kernel modules and driver versions
- **Specifications**: Memory sizes, frequencies, capabilities

### System Information
- **Kernel Version**: Linux kernel version and architecture
- **Distribution**: Linux distribution (without personal customizations)
- **Hardware Topology**: How components are connected (PCIe lanes, USB ports)

### Example Public Data
```json
{
  "cpu": {
    "vendor": "AMD",
    "model": "Ryzen 7 5800X",
    "cores": 8,
    "threads": 16
  },
  "gpu": {
    "vendor": "NVIDIA",
    "model": "GeForce RTX 3080",
    "driver": "nvidia 525.60.11",
    "vram": "10GB"
  },
  "kernel": {
    "version": "6.16.0",
    "architecture": "x86_64"
  }
}
```

## Privacy Validation

### Automated PII Detection

The system scans for potential privacy leaks:

- **Email Addresses**: `[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}`
- **IP Addresses**: Public IPs (excludes private ranges like 192.168.x.x)
- **MAC Addresses**: Hardware network addresses
- **Hostnames/Usernames**: Obvious personal identifiers

### Manual Review Process

1. **Automated Scanning**: Every submission is automatically scanned
2. **Community Review**: Open source allows community verification
3. **Manual Audit**: Maintainers manually review flagged submissions
4. **Continuous Improvement**: Privacy protection is continuously enhanced

## Best Practices for Users

### Before Generating Reports

1. **Choose Appropriate Privacy Level**
   - Home systems: Basic or Enhanced
   - Work systems: Enhanced or Strict
   - Sensitive environments: Strict

2. **Review System State**
   - Ensure no sensitive files are mounted
   - Check for temporary files with personal information
   - Verify system hostname doesn't contain personal info

### During Report Generation

```bash
# Use enhanced privacy for most situations
./target/release/lx-hw-detect detect \
    --privacy enhanced \
    --format json \
    --output my-report.json

# Validate privacy compliance before submission
./target/release/lx-hw-detect validate my-report.json --strict
```

### After Report Generation

1. **Manual Review**
   ```bash
   # Search for potential personal information
   grep -i "username\|email\|password" my-report.json
   grep -E '[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}' my-report.json
   ```

2. **Test Validation**
   ```bash
   # Ensure report passes all privacy checks
   ./target/release/lx-hw-detect validate my-report.json --verbose
   ```

## Privacy by Design Features

### Technical Safeguards

1. **One-Way Hashing**: HMAC-SHA256 provides cryptographically secure one-way transformation
2. **Salt Rotation**: Time-based salts prevent correlation across time periods
3. **Minimal Data**: Only hardware compatibility information is collected
4. **Local Processing**: Anonymization happens locally before any data leaves your system

### Procedural Safeguards

1. **Open Source**: All code is publicly auditable
2. **Community Review**: Multiple reviewers check submissions
3. **Automated Validation**: Consistent privacy checking for every submission
4. **Clear Guidelines**: Explicit documentation of what is and isn't collected

### Legal Safeguards

1. **CC0 License**: Data is dedicated to public domain
2. **No Tracking**: No personal information or usage tracking
3. **Right to Removal**: Users can request removal of their contributions
4. **Transparent Governance**: Open decision-making process

## Common Privacy Questions

### Q: Can my hardware reports be linked back to me?
**A**: No. The cryptographic anonymization with rotating salts makes it computationally infeasible to reverse-engineer the original system identifiers.

### Q: What if I submit multiple reports from the same system?
**A**: Reports from the same system within a salt rotation period will have the same anonymized ID, allowing correlation for duplicate detection. After the rotation period, new reports get different anonymous IDs.

### Q: Can I verify that my personal information was removed?
**A**: Yes. You can inspect the generated JSON file before submission to verify no personal information is present.

### Q: What happens if personal information is accidentally included?
**A**: Our automated systems scan for PII and will reject submissions containing it. If something is missed, you can request removal and we'll address it immediately.

### Q: Is it safe to submit reports from work systems?
**A**: Use Enhanced or Strict privacy levels for work systems. The anonymization protects system identity, but ensure you have permission to share hardware compatibility information from your organization.

### Q: How do I know the privacy protection is working correctly?
**A**: The entire codebase is open source and can be audited. You can examine the anonymization code, run tests, and verify the process yourself.

## Privacy Compliance

### GDPR Compliance (EU)
- **Lawful Basis**: Legitimate interest in hardware compatibility research
- **Data Minimization**: Only hardware compatibility data collected
- **Anonymization**: Personal identifiers cryptographically anonymized
- **Right to Erasure**: Users can request removal of contributions

### CCPA Compliance (California)
- **Personal Information**: Only anonymized hardware data collected
- **Opt-Out**: Users choose what to submit and can withdraw submissions
- **Transparency**: Clear documentation of data practices

## Reporting Privacy Issues

If you discover a potential privacy issue:

1. **Security Issues**: Email security@lx-hw-db.example.com (encrypted communication preferred)
2. **General Privacy**: Create a GitHub issue with the `privacy` label
3. **Submission Concerns**: Comment on your submission PR or issue

All privacy concerns are treated with high priority and addressed promptly.

---

## Technical Details

### Anonymization Algorithm

```rust
use hmac::{Hmac, Mac};
use sha2::Sha256;
type HmacSha256 = Hmac<Sha256>;

fn anonymize_system_id(
    hostname: &str,
    mac_addresses: &[String],
    system_uuid: &str,
    privacy_level: PrivacyLevel,
) -> Result<String, Error> {
    // Create deterministic system fingerprint
    let system_info = format!(
        "{}:{}:{}",
        hostname,
        mac_addresses.join(":"),
        system_uuid
    );
    
    // Generate time-based salt
    let salt = generate_salt_for_time(
        Utc::now(),
        privacy_level.rotation_period()
    );
    
    // Create HMAC
    let mut mac = HmacSha256::new_from_slice(&salt)?;
    mac.update(system_info.as_bytes());
    let result = mac.finalize();
    
    // Return first 12 characters as hex
    Ok(hex::encode(&result.into_bytes()[..6]))
}
```

### Salt Generation

```rust
fn generate_salt_for_time(
    current_time: DateTime<Utc>,
    rotation_period: Duration,
) -> Vec<u8> {
    // Round down to rotation period boundary
    let period_start = current_time
        .duration_trunc(rotation_period)
        .unwrap();
    
    // Create deterministic salt from period
    let salt_input = format!("lx-hw-db-salt-{}", period_start.timestamp());
    
    // Hash to create actual salt
    let mut hasher = Sha256::new();
    hasher.update(salt_input.as_bytes());
    hasher.finalize().to_vec()
}
```

This privacy system ensures that hardware compatibility data can be collected for community benefit while maintaining strong user privacy protection.