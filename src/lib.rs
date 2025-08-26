//! Linux Hardware Detection Library
//! 
//! A privacy-preserving hardware detection library for Linux systems that
//! collects hardware information using multiple detection tools while
//! implementing comprehensive anonymization and privacy protection.

pub mod detectors;
pub mod privacy;
pub mod output;
pub mod hardware;
pub mod errors;
pub mod cli;
pub mod validation;

pub use errors::{Result, LxHwError};
pub use hardware::{HardwareReport, SystemInfo, PrivacyLevel};