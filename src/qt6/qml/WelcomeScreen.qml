import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import QtQuick.Layouts
import "components"

Page {
    id: page
    objectName: "welcome"
    
    ScrollView {
        anchors.fill: parent
        contentWidth: availableWidth
        
        Column {
            width: parent.width
            spacing: 32
            
            // Hero section
            Rectangle {
                id: heroCard
                width: parent.width - 64
                anchors.horizontalCenter: parent.horizontalCenter
                height: heroContent.height + 64
                color: "white"
                radius: 12
                
                // Shadow effect
                layer.enabled: true
                layer.effect: DropShadow {
                    verticalOffset: 2
                    color: "#20000000"
                    radius: 8
                    spread: 0
                }
                
                Column {
                    id: heroContent
                    anchors.fill: parent
                    anchors.margins: 32
                    spacing: 24
                    
                    Row {
                        anchors.horizontalCenter: parent.horizontalCenter
                        spacing: 16
                        
                        Image {
                            source: "qrc:/icons/hardware-shield.svg"
                            width: 64
                            height: 64
                        }
                        
                        Column {
                            anchors.verticalCenter: parent.verticalCenter
                            
                            Label {
                                text: "Linux Hardware Database"
                                font.pixelSize: 28
                                font.weight: Font.Bold
                                color: Material.primary
                            }
                            
                            Label {
                                text: "Privacy-First Hardware Compatibility Detection"
                                font.pixelSize: 16
                                color: "#666"
                            }
                        }
                    }
                    
                    Label {
                        text: "Detect your hardware, check Linux compatibility, and contribute to the community database while maintaining complete privacy control."
                        width: parent.width
                        wrapMode: Text.WordWrap
                        font.pixelSize: 14
                        lineHeight: 1.4
                        horizontalAlignment: Text.AlignHCenter
                    }
                }
            }
            
            // Privacy overview cards
            Row {
                anchors.horizontalCenter: parent.horizontalCenter
                spacing: 16
                
                // Privacy levels preview
                Repeater {
                    model: [
                        {
                            level: "Basic",
                            color: "#4CAF50",
                            icon: "qrc:/icons/shield-outline.svg",
                            description: "Device models, no serial numbers"
                        },
                        {
                            level: "Enhanced", 
                            color: "#FF9800",
                            icon: "qrc:/icons/shield-half.svg",
                            description: "Hashed identifiers, kernel info"
                        },
                        {
                            level: "Strict",
                            color: "#F44336", 
                            icon: "qrc:/icons/shield.svg",
                            description: "Maximum anonymization, noise injection"
                        }
                    ]
                    
                    Rectangle {
                        width: 200
                        height: 120
                        color: "white"
                        radius: 8
                        
                        // Shadow effect
                        layer.enabled: true
                        layer.effect: DropShadow {
                            verticalOffset: 1
                            color: "#10000000"
                            radius: 4
                            spread: 0
                        }
                        
                        Column {
                            anchors.centerIn: parent
                            spacing: 8
                            
                            Image {
                                source: modelData.icon
                                width: 32
                                height: 32
                                anchors.horizontalCenter: parent.horizontalCenter
                            }
                            
                            Label {
                                text: modelData.level
                                font.weight: Font.Bold
                                color: modelData.color
                                horizontalAlignment: Text.AlignHCenter
                            }
                            
                            Label {
                                text: modelData.description
                                width: 180
                                wrapMode: Text.WordWrap
                                font.pixelSize: 12
                                color: "#666"
                                horizontalAlignment: Text.AlignHCenter
                            }
                        }
                        
                        MouseArea {
                            anchors.fill: parent
                            onClicked: {
                                privacyManager.setLevel(modelData.level)
                            }
                        }
                    }
                }
            }
            
            // Quick start section
            Rectangle {
                width: parent.width - 64
                anchors.horizontalCenter: parent.horizontalCenter
                height: quickStartContent.height + 48
                color: "white"
                radius: 8
                
                // Shadow effect
                layer.enabled: true
                layer.effect: DropShadow {
                    verticalOffset: 1
                    color: "#10000000"
                    radius: 4
                    spread: 0
                }
                
                Column {
                    id: quickStartContent
                    anchors.fill: parent
                    anchors.margins: 24
                    spacing: 16
                    
                    Label {
                        text: "Quick Start"
                        font.pixelSize: 20
                        font.weight: Font.Medium
                        color: Material.primary
                    }
                    
                    Flow {
                        width: parent.width
                        spacing: 12
                        
                        Chip {
                            text: "1. Choose privacy level"
                            icon: "qrc:/icons/shield-check.svg"
                            clickable: false
                        }
                        
                        Chip {
                            text: "2. Detect hardware"
                            icon: "qrc:/icons/search.svg"
                            clickable: false
                        }
                        
                        Chip {
                            text: "3. Review compatibility"
                            icon: "qrc:/icons/devices.svg"
                            clickable: false
                        }
                        
                        Chip {
                            text: "4. Get recommendations"
                            icon: "qrc:/icons/tune.svg"
                            clickable: false
                        }
                        
                        Chip {
                            text: "5. Share with community"
                            icon: "qrc:/icons/share.svg"
                            clickable: false
                        }
                    }
                }
            }
            
            // Action buttons
            Row {
                anchors.horizontalCenter: parent.horizontalCenter
                spacing: 16
                
                Button {
                    text: "Start Detection"
                    icon.source: "qrc:/icons/play-arrow.svg"
                    Material.background: Material.primary
                    Material.foreground: "white"
                    onClicked: stackView.replace("DetectionScreen.qml")
                }
                
                Button {
                    text: "Privacy Settings"
                    icon.source: "qrc:/icons/shield.svg"
                    flat: true
                    onClicked: stackView.replace("PrivacyScreen.qml")
                }
                
                Button {
                    text: "Learn More"
                    icon.source: "qrc:/icons/info.svg"
                    flat: true
                    onClicked: Qt.openUrlExternally("https://github.com/lx-hw-db/lx-hw-db")
                }
            }
        }
    }
}