import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import QtQuick.Layouts
import "components"

Page {
    id: page
    objectName: "detect"
    
    Column {
        anchors.fill: parent
        anchors.margins: 32
        spacing: 24
        
        // Detection header
        Row {
            width: parent.width
            
            Column {
                width: parent.width * 0.7
                
                Label {
                    text: "Hardware Detection"
                    font.pixelSize: 24
                    font.weight: Font.Bold
                    color: Material.primary
                }
                
                Label {
                    text: "Scanning system hardware with privacy level: " + privacyManager.currentLevel
                    color: "#666"
                    font.pixelSize: 14
                }
            }
            
            Item { Layout.fillWidth: true }
            
            // Privacy indicator
            Rectangle {
                width: 200
                height: 60
                color: privacyManager.getStatusColor()
                radius: 8
                
                Row {
                    anchors.centerIn: parent
                    spacing: 8
                    
                    Image {
                        source: "qrc:/icons/shield-check.svg"
                        width: 24
                        height: 24
                    }
                    
                    Column {
                        Label {
                            text: privacyManager.currentLevel + " Privacy"
                            color: "white"
                            font.weight: Font.Bold
                        }
                        Label {
                            text: privacyManager.getProtectionSummary()
                            color: "white"
                            font.pixelSize: 12
                        }
                    }
                }
            }
        }
        
        // Overall progress
        Rectangle {
            width: parent.width
            height: 120
            color: "white"
            radius: 8
            border.color: "#E0E0E0"
            border.width: 1
            
            Column {
                anchors.fill: parent
                anchors.margins: 20
                spacing: 12
                
                Row {
                    width: parent.width
                    
                    Label {
                        text: "Overall Progress"
                        font.pixelSize: 18
                        font.weight: Font.Medium
                    }
                    
                    Item { Layout.fillWidth: true }
                    
                    Label {
                        text: detectionManager.completedTools + "/" + detectionManager.totalTools + " tools"
                        color: "#666"
                    }
                }
                
                ProgressBar {
                    width: parent.width
                    value: detectionManager.overallProgress
                    Material.accent: Material.primary
                }
                
                Row {
                    width: parent.width
                    
                    Label {
                        text: detectionManager.currentStatus
                        color: "#666"
                    }
                    
                    Item { Layout.fillWidth: true }
                    
                    Label {
                        text: "~" + detectionManager.estimatedTimeRemaining + " remaining"
                        color: "#666"
                        font.pixelSize: 12
                    }
                }
            }
        }
        
        // Individual tool progress
        ScrollView {
            width: parent.width
            height: 400
            contentWidth: availableWidth
            
            Column {
                width: parent.width
                spacing: 8
                
                Label {
                    text: "Detection Tools"
                    font.pixelSize: 16
                    font.weight: Font.Medium
                    color: Material.primary
                }
                
                // Mock tool progress cards
                Repeater {
                    model: [
                        {
                            name: "lshw",
                            description: "Complete hardware information in JSON format",
                            status: "complete",
                            progress: 1.0,
                            deviceCount: 8,
                            processingTime: "2.1s"
                        },
                        {
                            name: "dmidecode", 
                            description: "BIOS and motherboard details",
                            status: "complete",
                            progress: 1.0,
                            deviceCount: 3,
                            processingTime: "0.8s"
                        },
                        {
                            name: "lspci",
                            description: "PCI devices with kernel driver mapping", 
                            status: "running",
                            progress: 0.6,
                            deviceCount: 0,
                            processingTime: ""
                        },
                        {
                            name: "lsusb",
                            description: "USB peripherals detection",
                            status: "pending", 
                            progress: 0.0,
                            deviceCount: 0,
                            processingTime: ""
                        },
                        {
                            name: "inxi",
                            description: "User-friendly system summaries",
                            status: "pending",
                            progress: 0.0,
                            deviceCount: 0, 
                            processingTime: ""
                        }
                    ]
                    
                    Rectangle {
                        width: parent.width
                        height: 80
                        color: "white"
                        radius: 8
                        border.color: "#E0E0E0"
                        border.width: 1
                        
                        Row {
                            anchors.fill: parent
                            anchors.margins: 16
                            spacing: 16
                            
                            // Tool icon and status
                            Rectangle {
                                width: 48
                                height: 48
                                radius: 24
                                color: {
                                    switch(modelData.status) {
                                        case "complete": return "#4CAF50"
                                        case "running": return "#2196F3"
                                        case "error": return "#F44336"
                                        default: return "#9E9E9E"
                                    }
                                }
                                
                                Image {
                                    anchors.centerIn: parent
                                    source: {
                                        switch(modelData.status) {
                                            case "complete": return "qrc:/icons/check-circle.svg"
                                            case "running": return "qrc:/icons/refresh.svg"
                                            case "error": return "qrc:/icons/error.svg"  
                                            default: return "qrc:/icons/schedule.svg"
                                        }
                                    }
                                    width: 24
                                    height: 24
                                }
                            }
                            
                            // Tool details
                            Column {
                                width: parent.width * 0.6
                                anchors.verticalCenter: parent.verticalCenter
                                
                                Label {
                                    text: modelData.name
                                    font.weight: Font.Medium
                                }
                                
                                Label {
                                    text: modelData.description
                                    color: "#666"
                                    font.pixelSize: 12
                                    elide: Text.ElideRight
                                    width: parent.width
                                }
                                
                                ProgressBar {
                                    width: parent.width
                                    value: modelData.progress
                                    visible: modelData.status === "running"
                                }
                            }
                            
                            // Results summary
                            Column {
                                anchors.verticalCenter: parent.verticalCenter
                                
                                Label {
                                    text: modelData.deviceCount + " devices"
                                    font.weight: Font.Bold
                                    color: modelData.status === "complete" ? Material.primary : "#666"
                                    visible: modelData.status === "complete"
                                }
                                
                                Label {
                                    text: modelData.processingTime
                                    color: "#666"
                                    font.pixelSize: 12
                                    visible: modelData.status === "complete"
                                }
                                
                                BusyIndicator {
                                    running: modelData.status === "running"
                                    visible: modelData.status === "running"
                                    Material.accent: Material.primary
                                    width: 32
                                    height: 32
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Action buttons
        Row {
            anchors.horizontalCenter: parent.horizontalCenter
            spacing: 16
            
            Button {
                text: detectionManager.isRunning ? "Cancel Detection" : "Start Detection"
                icon.source: detectionManager.isRunning ? "qrc:/icons/stop.svg" : "qrc:/icons/play-arrow.svg"
                Material.background: detectionManager.isRunning ? "#F44336" : Material.primary
                Material.foreground: "white"
                onClicked: detectionManager.toggleDetection()
            }
            
            Button {
                text: "View Hardware"
                icon.source: "qrc:/icons/arrow-forward.svg"
                enabled: detectionManager.isComplete
                onClicked: stackView.replace("HardwareScreen.qml")
            }
            
            Button {
                text: "Privacy Settings"
                icon.source: "qrc:/icons/shield.svg"
                flat: true
                onClicked: stackView.replace("PrivacyScreen.qml")
            }
        }
    }
}