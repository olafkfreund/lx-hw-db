//! GTK4 GUI implementation for lx-hw-detect
//! 
//! Provides a modern Adwaita-compliant interface for hardware detection,
//! configuration recommendations, and community submission.

use crate::errors::LxHwError;

// Full GUI modules (temporarily disabled to get simple GUI running)
// pub mod application;
// pub mod window;  
// pub mod widgets;
pub mod models;
// pub mod utils;
pub mod i18n;

// Working minimal GUI
pub mod simple;

/// Result type for GUI operations
pub type GuiResult<T> = Result<T, LxHwError>;

/// Initialize the GUI application
pub fn run() -> GuiResult<()> {
    // For now, use the working simple GUI while we finish fixing the comprehensive one
    simple::run_simple()
}

/// Simple internationalization function for development
/// In a full implementation, this would use Fluent bundles
pub fn t(text: &str) -> String {
    // For now, just return the English text
    // TODO: Replace with proper Fluent i18n system
    text.to_string()
}