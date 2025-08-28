//! Qt6 GUI entry point for lx-hw-detect
//!
//! Provides a modern Material Design 3 interface built with Qt6 and QML

use lx_hw_detect::qt6::Application;
use lx_hw_detect::Result;
use std::env;

fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    // Initialize and run Qt6 application
    let app = Application::new(&args)?;
    app.run()
}
