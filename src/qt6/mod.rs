//! Qt6 GUI implementation for lx-hw-db
//!
//! Provides a modern Material Design 3 interface demonstration.
//! Currently in demo mode - shows complete interface design without Qt6 dependencies.

pub mod application;
pub mod backend;
// pub mod models;  // Disabled for demo mode due to cxx-qt dependency

pub use application::Application;