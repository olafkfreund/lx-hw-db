//! Linux Hardware Detection Library
//!
//! A privacy-preserving hardware detection library for Linux systems that
//! collects hardware information using multiple detection tools while
//! implementing comprehensive anonymization and privacy protection.

pub mod cli;
pub mod detectors;
pub mod errors;
pub mod github_submit;
#[cfg(feature = "gtk-gui")]
pub mod gui;
pub mod hardware;
pub mod indexer;
pub mod output;
pub mod privacy;
#[cfg(feature = "qt6-gui")]
pub mod qt6;
pub mod validation;

pub use errors::{LxHwError, Result};
pub use hardware::{HardwareReport, PrivacyLevel, SystemInfo};
