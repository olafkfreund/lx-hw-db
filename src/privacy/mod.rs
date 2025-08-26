//! Privacy and anonymization system for hardware data

use crate::errors::{LxHwError, Result};
use crate::hardware::PrivacyLevel;
use chrono::{DateTime, Duration, Utc};
use ring::{hmac, rand};

/// Privacy manager for handling anonymization of hardware data
pub struct PrivacyManager {
    privacy_level: PrivacyLevel,
    salt_generator: SaltGenerator,
}

/// Salt generation for time-rotating anonymization keys
pub struct SaltGenerator {
    current_salt: Vec<u8>,
    salt_generated_at: DateTime<Utc>,
    rotation_period: Duration,
}

impl PrivacyManager {
    /// Create a new privacy manager with the specified privacy level
    pub fn new(privacy_level: PrivacyLevel) -> Result<Self> {
        let rotation_period = match privacy_level {
            PrivacyLevel::Basic => Duration::hours(24),
            PrivacyLevel::Enhanced => Duration::hours(12),
            PrivacyLevel::Strict => Duration::hours(1),
        };

        let salt_generator = SaltGenerator::new(rotation_period)?;

        Ok(Self { privacy_level, salt_generator })
    }

    /// Anonymize a hardware identifier using HMAC-SHA256
    pub fn anonymize_identifier(&mut self, identifier: &str) -> Result<String> {
        let salt = self.salt_generator.get_current_salt()?;
        let key = hmac::Key::new(hmac::HMAC_SHA256, salt);
        let signature = hmac::sign(&key, identifier.as_bytes());
        Ok(hex::encode(signature.as_ref()))
    }

    /// Anonymize a MAC address by preserving OUI but hashing device part
    pub fn anonymize_mac_address(&mut self, mac: &str) -> Result<String> {
        if mac.len() < 17 {
            return Err(LxHwError::PrivacyError("Invalid MAC address format".to_string()));
        }

        // Preserve OUI (first 3 octets) but anonymize device identifier
        let oui = &mac[..8]; // "XX:XX:XX"
        let device_part = &mac[9..]; // "XX:XX:XX"

        let anonymized_device = self.anonymize_identifier(device_part)?;
        let truncated = &anonymized_device[..6]; // Take first 6 chars for 3 octets

        // Format back to MAC address style
        Ok(format!(
            "{}:{}",
            oui,
            truncated
                .chars()
                .collect::<Vec<char>>()
                .chunks(2)
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join(":")
        ))
    }

    /// Get the current privacy level
    pub fn privacy_level(&self) -> PrivacyLevel {
        self.privacy_level
    }
}

impl SaltGenerator {
    /// Create a new salt generator with the specified rotation period
    pub fn new(rotation_period: Duration) -> Result<Self> {
        let mut salt = vec![0u8; 32]; // 256-bit salt
        let rng = rand::SystemRandom::new();
        rand::SecureRandom::fill(&rng, &mut salt).map_err(|_| {
            LxHwError::PrivacyError("Failed to generate cryptographic salt".to_string())
        })?;

        Ok(Self { current_salt: salt, salt_generated_at: Utc::now(), rotation_period })
    }

    /// Get the current salt, rotating if necessary
    pub fn get_current_salt(&mut self) -> Result<&[u8]> {
        let now = Utc::now();
        if now - self.salt_generated_at > self.rotation_period {
            self.rotate_salt()?;
        }
        Ok(&self.current_salt)
    }

    /// Force rotation of the salt
    fn rotate_salt(&mut self) -> Result<()> {
        let rng = rand::SystemRandom::new();
        rand::SecureRandom::fill(&rng, &mut self.current_salt).map_err(|_| {
            LxHwError::PrivacyError("Failed to rotate cryptographic salt".to_string())
        })?;

        self.salt_generated_at = Utc::now();
        Ok(())
    }
}
