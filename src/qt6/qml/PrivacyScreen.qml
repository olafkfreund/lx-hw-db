import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import QtQuick.Layouts
import "components"

Page {
    objectName: "privacy"
    
    Column {
        anchors.fill: parent
        anchors.margins: 24
        spacing: 20
        
        // Privacy header
        Row {
            width: parent.width
            
            Column {
                width: parent.width * 0.6
                
                Label {
                    text: "Privacy Settings"
                    font.pixelSize: 24
                    font.weight: Font.Bold
                    color: Material.primary
                }
                
                Label {
                    text: "Configure data collection and anonymization levels"
                    color: "#666"
                    font.pixelSize: 14
                }
            }
            
            Item { Layout.fillWidth: true }
            
            // Current privacy status
            Rectangle {
                width: 180
                height: 80
                radius: 8
                color: privacyManager.getStatusColor()
                
                Column {
                    anchors.centerIn: parent
                    spacing: 4
                    
                    Text {
                        text: "🔒"
                        font.pixelSize: 20
                        anchors.horizontalCenter: parent.horizontalCenter
                    }
                    
                    Label {
                        text: privacyManager.currentLevel
                        color: "white"
                        font.weight: Font.Bold
                        anchors.horizontalCenter: parent.horizontalCenter
                    }
                    
                    Label {
                        text: privacyManager.getProtectionSummary()
                        color: "white"
                        font.pixelSize: 12
                        anchors.horizontalCenter: parent.horizontalCenter
                    }
                }
            }
        }
        
        // Privacy level selection
        ScrollView {
            width: parent.width
            height: parent.height - 120
            contentWidth: availableWidth
            
            Column {
                width: parent.width
                spacing: 16
                
                // Basic Privacy Level
                Rectangle {
                    width: parent.width
                    height: basicContent.height + 32
                    radius: 8
                    color: privacyManager.currentLevel === "Basic" ? "#E8F5E8" : "white"
                    border.color: privacyManager.currentLevel === "Basic" ? "#4CAF50" : "#E0E0E0"
                    border.width: privacyManager.currentLevel === "Basic" ? 2 : 1
                    
                    Column {
                        id: basicContent
                        anchors.fill: parent
                        anchors.margins: 16
                        spacing: 12
                        
                        Row {
                            spacing: 12
                            
                            RadioButton {
                                checked: privacyManager.currentLevel === "Basic"
                                onClicked: privacyManager.setLevel("Basic")
                            }
                            
                            Column {
                                spacing: 4
                                
                                Row {
                                    spacing: 8
                                    
                                    Label {
                                        text: "Basic Privacy"
                                        font.weight: Font.Bold
                                        font.pixelSize: 18
                                        color: Material.primary
                                    }
                                    
                                    Rectangle {
                                        width: 60
                                        height: 20
                                        radius: 10
                                        color: "#4CAF50"
                                        
                                        Label {
                                            anchors.centerIn: parent
                                            text: "Default"
                                            color: "white"
                                            font.pixelSize: 10
                                        }
                                    }
                                }
                                
                                Label {
                                    text: "Device models only, no personal identifiers"
                                    color: "#666"
                                    font.pixelSize: 14
                                }
                            }
                        }
                        
                        // What's collected
                        Column {
                            spacing: 8
                            width: parent.width
                            
                            Label {
                                text: "What's collected:"
                                font.weight: Font.Medium
                                color: "#333"
                            }
                            
                            Column {
                                spacing: 4
                                
                                Row {
                                    spacing: 8
                                    Text { text: "✓"; color: "#4CAF50" }
                                    Label { text: "Hardware model names and specifications"; color: "#666"; font.pixelSize: 13 }
                                }
                                
                                Row {
                                    spacing: 8
                                    Text { text: "✓"; color: "#4CAF50" }
                                    Label { text: "Driver versions and compatibility status"; color: "#666"; font.pixelSize: 13 }
                                }
                                
                                Row {
                                    spacing: 8
                                    Text { text: "✗"; color: "#F44336" }
                                    Label { text: "Serial numbers, MAC addresses, or personal IDs"; color: "#666"; font.pixelSize: 13 }
                                }
                            }
                        }
                    }
                }
                
                // Enhanced Privacy Level
                Rectangle {
                    width: parent.width
                    height: enhancedContent.height + 32
                    radius: 8
                    color: privacyManager.currentLevel === "Enhanced" ? "#FFF3E0" : "white"
                    border.color: privacyManager.currentLevel === "Enhanced" ? "#FF9800" : "#E0E0E0"
                    border.width: privacyManager.currentLevel === "Enhanced" ? 2 : 1
                    
                    Column {
                        id: enhancedContent
                        anchors.fill: parent
                        anchors.margins: 16
                        spacing: 12
                        
                        Row {
                            spacing: 12
                            
                            RadioButton {
                                checked: privacyManager.currentLevel === "Enhanced"
                                onClicked: privacyManager.setLevel("Enhanced")
                            }
                            
                            Column {
                                spacing: 4
                                
                                Row {
                                    spacing: 8
                                    
                                    Label {
                                        text: "Enhanced Privacy"
                                        font.weight: Font.Bold
                                        font.pixelSize: 18
                                        color: Material.primary
                                    }
                                    
                                    Rectangle {
                                        width: 80
                                        height: 20
                                        radius: 10
                                        color: "#FF9800"
                                        
                                        Label {
                                            anchors.centerIn: parent
                                            text: "Recommended"
                                            color: "white"
                                            font.pixelSize: 10
                                        }
                                    }
                                }
                                
                                Label {
                                    text: "Cryptographic hashing with time-rotation"
                                    color: "#666"
                                    font.pixelSize: 14
                                }
                            }
                        }
                        
                        // What's collected
                        Column {
                            spacing: 8
                            width: parent.width
                            
                            Label {
                                text: "Privacy features:"
                                font.weight: Font.Medium
                                color: "#333"
                            }
                            
                            Column {
                                spacing: 4
                                
                                Row {
                                    spacing: 8
                                    Text { text: "✓"; color: "#4CAF50" }
                                    Label { text: "HMAC-SHA256 hashing of all hardware identifiers"; color: "#666"; font.pixelSize: 13 }
                                }
                                
                                Row {
                                    spacing: 8
                                    Text { text: "✓"; color: "#4CAF50" }
                                    Label { text: "Time-rotating salts prevent cross-correlation"; color: "#666"; font.pixelSize: 13 }
                                }
                                
                                Row {
                                    spacing: 8
                                    Text { text: "✓"; color: "#4CAF50" }
                                    Label { text: "K-anonymity ensures at least 5 matching configs"; color: "#666"; font.pixelSize: 13 }
                                }
                            }
                        }
                    }
                }
                
                // Strict Privacy Level
                Rectangle {
                    width: parent.width
                    height: strictContent.height + 32
                    radius: 8
                    color: privacyManager.currentLevel === "Strict" ? "#FFEBEE" : "white"
                    border.color: privacyManager.currentLevel === "Strict" ? "#F44336" : "#E0E0E0"
                    border.width: privacyManager.currentLevel === "Strict" ? 2 : 1
                    
                    Column {
                        id: strictContent
                        anchors.fill: parent
                        anchors.margins: 16
                        spacing: 12
                        
                        Row {
                            spacing: 12
                            
                            RadioButton {
                                checked: privacyManager.currentLevel === "Strict"
                                onClicked: privacyManager.setLevel("Strict")
                            }
                            
                            Column {
                                spacing: 4
                                
                                Row {
                                    spacing: 8
                                    
                                    Label {
                                        text: "Strict Privacy"
                                        font.weight: Font.Bold
                                        font.pixelSize: 18
                                        color: Material.primary
                                    }
                                    
                                    Rectangle {
                                        width: 60
                                        height: 20
                                        radius: 10
                                        color: "#F44336"
                                        
                                        Label {
                                            anchors.centerIn: parent
                                            text: "Maximum"
                                            color: "white"
                                            font.pixelSize: 10
                                        }
                                    }
                                }
                                
                                Label {
                                    text: "Differential privacy with statistical noise"
                                    color: "#666"
                                    font.pixelSize: 14
                                }
                            }
                        }
                        
                        // What's collected
                        Column {
                            spacing: 8
                            width: parent.width
                            
                            Label {
                                text: "Maximum protection:"
                                font.weight: Font.Medium
                                color: "#333"
                            }
                            
                            Column {
                                spacing: 4
                                
                                Row {
                                    spacing: 8
                                    Text { text: "✓"; color: "#4CAF50" }
                                    Label { text: "Enhanced-level protection plus noise injection"; color: "#666"; font.pixelSize: 13 }
                                }
                                
                                Row {
                                    spacing: 8
                                    Text { text: "✓"; color: "#4CAF50" }
                                    Label { text: "Differential privacy (ε=1.0) with Laplace noise"; color: "#666"; font.pixelSize: 13 }
                                }
                                
                                Row {
                                    spacing: 8
                                    Text { text: "⚠️"; color: "#FF9800" }
                                    Label { text: "Some statistical precision lost for maximum privacy"; color: "#666"; font.pixelSize: 13 }
                                }
                            }
                        }
                    }
                }
                
                // Privacy preview section
                Rectangle {
                    width: parent.width
                    height: previewContent.height + 32
                    radius: 8
                    color: "white"
                    border.color: "#E0E0E0"
                    border.width: 1
                    
                    Column {
                        id: previewContent
                        anchors.fill: parent
                        anchors.margins: 16
                        spacing: 12
                        
                        Label {
                            text: "Data Preview"
                            font.weight: Font.Bold
                            color: Material.primary
                        }
                        
                        Label {
                            text: "Example of how your data would appear with current privacy settings:"
                            color: "#666"
                            wrapMode: Text.WordWrap
                            width: parent.width
                        }
                        
                        Rectangle {
                            width: parent.width
                            height: 100
                            color: "#F5F5F5"
                            radius: 4
                            border.color: "#E0E0E0"
                            border.width: 1
                            
                            ScrollView {
                                anchors.fill: parent
                                anchors.margins: 8
                                contentWidth: availableWidth
                                
                                Label {
                                    text: getPrivacyPreview()
                                    font.family: "monospace"
                                    font.pixelSize: 11
                                    wrapMode: Text.WordWrap
                                    width: parent.width
                                }
                            }
                        }
                    }
                }
                
                // GDPR compliance section
                Rectangle {
                    width: parent.width
                    height: gdprContent.height + 32
                    radius: 8
                    color: "white"
                    border.color: "#E0E0E0"
                    border.width: 1
                    
                    Column {
                        id: gdprContent
                        anchors.fill: parent
                        anchors.margins: 16
                        spacing: 12
                        
                        Label {
                            text: "Data Rights & Compliance"
                            font.weight: Font.Bold
                            color: Material.primary
                        }
                        
                        Label {
                            text: "Your privacy rights under GDPR and other data protection laws:"
                            color: "#666"
                            wrapMode: Text.WordWrap
                            width: parent.width
                        }
                        
                        Column {
                            spacing: 8
                            
                            Row {
                                spacing: 12
                                
                                Button {
                                    text: "Export My Data"
                                    onClicked: console.log("Export user data")
                                }
                                
                                Button {
                                    text: "Request Deletion"
                                    onClicked: console.log("Request data deletion")
                                }
                                
                                Button {
                                    text: "Privacy Policy"
                                    flat: true
                                    onClicked: console.log("Open privacy policy")
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    function getPrivacyPreview() {
        switch(privacyManager.currentLevel) {
            case "Basic":
                return 'cpu_model: "AMD Ryzen 7 5800X"\ngpu_model: "NVIDIA GeForce RTX 3080"\nmotherboard: "ASUS ROG STRIX B550-F"';
            case "Enhanced": 
                return 'cpu_id: "sha256:a7b3c2d1e5f8..."\ngpu_id: "sha256:9f4e3c2a1b6d..."\nmotherboard_id: "sha256:5c8a7b2e9f1d..."';
            case "Strict":
                return 'cpu_category: "high_performance_8_core"\ngpu_category: "discrete_gaming_gpu"\nmotherboard_category: "amd_b550_chipset"';
            default:
                return 'No preview available';
        }
    }
}