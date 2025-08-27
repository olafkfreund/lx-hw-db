//! Build script for lx-hw-db

fn main() {
    #[cfg(feature = "qt6-gui")]
    {
        println!("cargo:warning=Qt6 GUI feature enabled - running in demo mode");
        println!("cargo:warning=Full cxx-qt integration requires compatible Qt6 QML development libraries");
        
        // For demo mode, we don't need to link Qt6 libraries
        // The application will show the complete interface design
        // without actual Qt6 GUI functionality
    }
    
    // Print cargo rerun directives
    println!("cargo:rerun-if-changed=src/qt6/");
    println!("cargo:rerun-if-changed=build.rs");
}