import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import QtQuick.Layouts
import "components"

Page {
    objectName: "config"
    
    Column {
        anchors.fill: parent
        anchors.margins: 24
        spacing: 20
        
        // Configuration header with system score
        Row {
            width: parent.width
            
            Column {
                width: parent.width * 0.6
                
                Label {
                    text: "System Configuration"
                    font.pixelSize: 24
                    font.weight: Font.Bold
                    color: Material.primary
                }
                
                Label {
                    text: "Optimized recommendations for your hardware"
                    color: "#666"
                    font.pixelSize: 14
                }
            }
            
            Item { Layout.fillWidth: true }
            
            // System compatibility score
            Rectangle {
                width: 200
                height: 120
                radius: 8
                color: "white"
                border.color: "#E0E0E0"
                border.width: 1
                
                Column {
                    anchors.centerIn: parent
                    spacing: 8
                    
                    // Circular progress indicator (simplified)
                    Rectangle {
                        width: 60
                        height: 60
                        radius: 30
                        color: "transparent"
                        border.color: "#4CAF50"
                        border.width: 4
                        anchors.horizontalCenter: parent.horizontalCenter
                        
                        Label {
                            anchors.centerIn: parent
                            text: "85%"
                            font.weight: Font.Bold
                            color: "#4CAF50"
                        }
                    }
                    
                    Label {
                        text: "System Compatibility"
                        font.weight: Font.Medium
                        anchors.horizontalCenter: parent.horizontalCenter
                    }
                    
                    Label {
                        text: "Very Good"
                        font.pixelSize: 12
                        color: "#666"
                        anchors.horizontalCenter: parent.horizontalCenter
                    }
                }
            }
        }
        
        // Quick actions row
        Row {
            width: parent.width
            spacing: 12
            
            Button {
                text: "Install Drivers"
                Material.background: "#4CAF50"
                Material.foreground: "white"
                onClicked: console.log("Install drivers")
            }
            
            Button {
                text: "Apply Kernel Parameters"
                onClicked: console.log("Apply kernel parameters")
            }
            
            Button {
                text: "Export Configuration"
                onClicked: console.log("Export configuration")
            }
            
            Button {
                text: "Performance Tuning"
                flat: true
                onClicked: console.log("Performance tuning")
            }
        }
        
        // Recommendations content
        ScrollView {
            width: parent.width
            height: parent.height - 200
            contentWidth: availableWidth
            
            Column {
                width: parent.width
                spacing: 16
                
                // Priority drivers section
                Rectangle {
                    width: parent.width
                    height: priorityContent.height + 32
                    radius: 8
                    color: "white"
                    border.color: "#E0E0E0"
                    border.width: 1
                    
                    Column {
                        id: priorityContent
                        anchors.fill: parent
                        anchors.margins: 16
                        spacing: 12
                        
                        Row {
                            spacing: 8
                            
                            Text {
                                text: "⚠️"
                                font.pixelSize: 16
                            }
                            
                            Label {
                                text: "Priority Drivers (Required for full functionality)"
                                font.weight: Font.Bold
                                color: "#F44336"
                            }
                        }
                        
                        // Driver recommendation card
                        Rectangle {
                            width: parent.width - 32
                            height: 80
                            radius: 4
                            color: "#FAFAFA"
                            border.color: "#E0E0E0"
                            border.width: 1
                            
                            Row {
                                anchors.fill: parent
                                anchors.margins: 12
                                spacing: 12
                                
                                Rectangle {
                                    width: 4
                                    height: parent.height
                                    color: "#F44336"
                                    radius: 2
                                }
                                
                                Column {
                                    width: parent.width * 0.6
                                    anchors.verticalCenter: parent.verticalCenter
                                    
                                    Label {
                                        text: "NVIDIA Driver 525.147.05"
                                        font.weight: Font.Medium
                                        font.pixelSize: 16
                                    }
                                    
                                    Label {
                                        text: "Required for RTX 3080 functionality"
                                        color: "#666"
                                        font.pixelSize: 14
                                    }
                                    
                                    Label {
                                        text: "Package: nvidia-driver-525"
                                        color: "#666"
                                        font.pixelSize: 12
                                        font.family: "monospace"
                                    }
                                }
                                
                                Item { Layout.fillWidth: true }
                                
                                Button {
                                    text: "Install"
                                    Material.background: "#F44336"
                                    Material.foreground: "white"
                                    anchors.verticalCenter: parent.verticalCenter
                                }
                            }
                        }
                    }
                }
                
                // Recommended drivers section
                Rectangle {
                    width: parent.width
                    height: recommendedContent.height + 32
                    radius: 8
                    color: "white"
                    border.color: "#E0E0E0"
                    border.width: 1
                    
                    Column {
                        id: recommendedContent
                        anchors.fill: parent
                        anchors.margins: 16
                        spacing: 12
                        
                        Label {
                            text: "Recommended Drivers"
                            font.weight: Font.Bold
                            color: Material.primary
                        }
                        
                        // Driver recommendation
                        Rectangle {
                            width: parent.width - 32
                            height: 80
                            radius: 4
                            color: "#FAFAFA"
                            border.color: "#E0E0E0"
                            border.width: 1
                            
                            Row {
                                anchors.fill: parent
                                anchors.margins: 12
                                spacing: 12
                                
                                Rectangle {
                                    width: 4
                                    height: parent.height
                                    color: "#4CAF50"
                                    radius: 2
                                }
                                
                                Column {
                                    width: parent.width * 0.6
                                    anchors.verticalCenter: parent.verticalCenter
                                    
                                    Label {
                                        text: "AMD Microcode Updates"
                                        font.weight: Font.Medium
                                        font.pixelSize: 16
                                    }
                                    
                                    Label {
                                        text: "Performance and security updates for AMD CPU"
                                        color: "#666"
                                        font.pixelSize: 14
                                    }
                                    
                                    Label {
                                        text: "Package: amd64-microcode"
                                        color: "#666"
                                        font.pixelSize: 12
                                        font.family: "monospace"
                                    }
                                }
                                
                                Item { Layout.fillWidth: true }
                                
                                Button {
                                    text: "Install"
                                    Material.background: "#4CAF50"
                                    Material.foreground: "white"
                                    anchors.verticalCenter: parent.verticalCenter
                                }
                            }
                        }
                    }
                }
                
                // Kernel parameters section
                Rectangle {
                    width: parent.width
                    height: kernelContent.height + 32
                    radius: 8
                    color: "white"
                    border.color: "#E0E0E0"
                    border.width: 1
                    
                    Column {
                        id: kernelContent
                        anchors.fill: parent
                        anchors.margins: 16
                        spacing: 12
                        
                        Row {
                            spacing: 8
                            
                            Text {
                                text: "ℹ️"
                                font.pixelSize: 16
                            }
                            
                            Label {
                                text: "Kernel Parameters"
                                font.weight: Font.Bold
                                color: Material.primary
                            }
                        }
                        
                        Label {
                            text: "Current Kernel: 6.5.0-generic (Ubuntu 22.04 • x86_64)"
                            color: "#666"
                            font.pixelSize: 14
                        }
                        
                        // Parameter recommendation
                        Rectangle {
                            width: parent.width - 32
                            height: paramContent.height + 24
                            radius: 4
                            color: "#FAFAFA"
                            border.color: "#E0E0E0"
                            border.width: 1
                            
                            Column {
                                id: paramContent
                                anchors.fill: parent
                                anchors.margins: 12
                                spacing: 8
                                
                                Label {
                                    text: "NVIDIA Power Management"
                                    font.weight: Font.Medium
                                    font.pixelSize: 16
                                }
                                
                                Label {
                                    text: "Enables proper power management for NVIDIA GPUs"
                                    color: "#666"
                                    font.pixelSize: 14
                                    wrapMode: Text.WordWrap
                                    width: parent.width
                                }
                                
                                Rectangle {
                                    width: parameterText.width + 16
                                    height: 32
                                    color: "#F5F5F5"
                                    radius: 4
                                    border.color: "#E0E0E0"
                                    
                                    Label {
                                        id: parameterText
                                        anchors.centerIn: parent
                                        text: "nvidia.NVreg_DynamicPowerManagement=0x02"
                                        font.family: "monospace"
                                        color: "#333"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Apply configuration button
        Row {
            anchors.horizontalCenter: parent.horizontalCenter
            spacing: 16
            
            Button {
                text: "Apply All Recommendations"
                Material.background: Material.primary
                Material.foreground: "white"
                onClicked: console.log("Apply all recommendations")
            }
            
            Button {
                text: "Create Custom Preset"
                flat: true
                onClicked: console.log("Create custom preset")
            }
        }
    }
}