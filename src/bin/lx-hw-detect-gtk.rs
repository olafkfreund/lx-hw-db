use lx_hw_detect::gui;
use std::process;

fn main() {
    if let Err(e) = gui::run() {
        eprintln!("Failed to run GUI application: {}", e);
        process::exit(1);
    }
}
