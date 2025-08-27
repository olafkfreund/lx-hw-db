import QtQuick
import QtQuick.Controls.Material
import org.lxhwdb.backend

ApplicationWindow {
    id: window
    
    width: 1200
    height: 800
    minimumWidth: 900
    minimumHeight: 600
    visible: true
    
    title: "Linux Hardware Database"
    
    // Material Design 3 theming
    Material.theme: Material.Light
    Material.primary: "#6750A4"      // Privacy-focused purple
    Material.accent: "#625B71"       // Secondary accent
    Material.background: "#FFFBFE"   // Surface container
    
    // Backend singletons
    property HardwareManager hardwareManager: HardwareManager {}
    property PrivacyManager privacyManager: PrivacyManager {}
    property DetectionManager detectionManager: DetectionManager {}
    property ConfigManager configManager: ConfigManager {}
    
    // Create the main window with all components
    MainWindow {
        anchors.fill: parent
    }
}