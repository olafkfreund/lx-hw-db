//! Internationalization support for the GUI
//!
//! This module provides a placeholder i18n system. In a full implementation,
//! this would use Fluent for proper internationalization support.

use std::sync::OnceLock;

/// Global internationalization state
static I18N_INITIALIZED: OnceLock<bool> = OnceLock::new();

/// Initialize the internationalization system
pub fn init_i18n() {
    I18N_INITIALIZED.get_or_init(|| {
        // TODO: Initialize Fluent bundles here
        // For now, we'll just use English strings directly
        true
    });
}

/// Get a localized string (placeholder implementation)
/// In a full implementation, this would look up strings in Fluent bundles
pub fn get_text(key: &str) -> String {
    // Placeholder: just return the key as English text
    // In a real implementation, this would:
    // 1. Look up the key in the appropriate Fluent bundle
    // 2. Handle fallbacks to English
    // 3. Support pluralization and variables
    key.to_string()
}

/// Macro for easier text retrieval
#[macro_export]
macro_rules! fl {
    ($key:expr) => {
        crate::gui::i18n::get_text($key)
    };
}
