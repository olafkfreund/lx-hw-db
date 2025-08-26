//! Error types and handling for the hardware detection system

use thiserror::Error;

/// Main error type for hardware detection operations
#[derive(Error, Debug)]
pub enum LxHwError {
    #[error("Hardware detection failed: {0}")]
    DetectionError(String),

    #[error("Privacy anonymization failed: {0}")]
    PrivacyError(String),

    #[error("System command failed: {command}")]
    SystemCommandError { command: String },

    #[error("System error: {message}")]
    SystemError { message: String },

    #[error("Invalid input: {message}")]
    InvalidInput { message: String },

    #[error("IO operation failed: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization failed: {0}")]
    SerializationError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Database error: {message}")]
    DatabaseError { message: String },

    #[error("Validation error: {message}")]
    ValidationError { message: String },
}

/// Result type alias for hardware detection operations
pub type Result<T> = std::result::Result<T, LxHwError>;

impl From<serde_json::Error> for LxHwError {
    fn from(err: serde_json::Error) -> Self {
        Self::SerializationError(format!("JSON error: {}", err))
    }
}

impl From<serde_yaml::Error> for LxHwError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::SerializationError(format!("YAML error: {}", err))
    }
}
