import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import QtQuick.Layouts
import "components"

Page {
    objectName: "submission"
    
    Column {
        anchors.fill: parent
        anchors.margins: 24
        spacing: 20
        
        // Submission header
        Row {
            width: parent.width
            
            Column {
                width: parent.width * 0.6
                
                Label {
                    text: "Community Submission"
                    font.pixelSize: 24
                    font.weight: Font.Bold
                    color: Material.primary
                }
                
                Label {
                    text: "Help improve hardware compatibility for the Linux community"
                    color: "#666"
                    font.pixelSize: 14
                }
            }
            
            Item { Layout.fillWidth: true }
            
            // Privacy level indicator
            Rectangle {
                width: 180
                height: 60
                radius: 8
                color: privacyManager.getStatusColor()
                
                Row {
                    anchors.centerIn: parent
                    spacing: 8
                    
                    Text {
                        text: "🔒"
                        font.pixelSize: 16
                    }
                    
                    Label {
                        text: "Privacy: " + privacyManager.currentLevel
                        color: "white"
                        font.weight: Font.Medium
                    }
                }
            }
        }
        
        // Submission process steps
        Row {
            width: parent.width
            spacing: 20
            
            // Step indicators
            Repeater {
                model: ["Review", "GitHub", "Submit"]
                
                Column {
                    spacing: 8
                    
                    Rectangle {
                        width: 40
                        height: 40
                        radius: 20
                        color: index === 0 ? Material.primary : "#E0E0E0"
                        anchors.horizontalCenter: parent.horizontalCenter
                        
                        Label {
                            anchors.centerIn: parent
                            text: (index + 1).toString()
                            color: index === 0 ? "white" : "#666"
                            font.weight: Font.Bold
                        }
                    }
                    
                    Label {
                        text: modelData
                        anchors.horizontalCenter: parent.horizontalCenter
                        font.pixelSize: 14
                        color: index === 0 ? Material.primary : "#666"
                    }
                    
                    // Step connector line
                    Rectangle {
                        width: 40
                        height: 2
                        color: "#E0E0E0"
                        anchors.horizontalCenter: parent.horizontalCenter
                        visible: index < 2
                    }
                }
            }
        }
        
        // Content area
        ScrollView {
            width: parent.width
            height: parent.height - 200
            contentWidth: availableWidth
            
            Column {
                width: parent.width
                spacing: 16
                
                // Hardware report preview
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
                        
                        Row {
                            spacing: 8
                            
                            Text {
                                text: "📋"
                                font.pixelSize: 16
                            }
                            
                            Label {
                                text: "Hardware Report Preview"
                                font.weight: Font.Bold
                                color: Material.primary
                            }
                        }
                        
                        // Report summary
                        GridLayout {
                            columns: 2
                            width: parent.width
                            columnSpacing: 16
                            rowSpacing: 8
                            
                            Label { text: "Devices Detected:"; color: "#666" }
                            Label { text: "15 devices" }
                            
                            Label { text: "Privacy Level:"; color: "#666" }
                            Label { text: privacyManager.currentLevel }
                            
                            Label { text: "Anonymization:"; color: "#666" }
                            Label { text: privacyManager.anonymizationStatus }
                            
                            Label { text: "Report Size:"; color: "#666" }
                            Label { text: "2.3 KB (anonymized)" }
                        }
                        
                        // Data preview toggle
                        Row {
                            spacing: 8
                            
                            Button {
                                text: "Preview Data"
                                onClicked: dataPreviewDialog.open()
                            }
                            
                            Button {
                                text: "Download Report"
                                flat: true
                                onClicked: console.log("Download hardware report")
                            }
                        }
                    }
                }
                
                // GitHub authentication section
                Rectangle {
                    width: parent.width
                    height: githubContent.height + 32
                    radius: 8
                    color: "white"
                    border.color: "#E0E0E0"
                    border.width: 1
                    
                    Column {
                        id: githubContent
                        anchors.fill: parent
                        anchors.margins: 16
                        spacing: 12
                        
                        Row {
                            spacing: 8
                            
                            Text {
                                text: "🐙"
                                font.pixelSize: 16
                            }
                            
                            Label {
                                text: "GitHub Integration"
                                font.weight: Font.Bold
                                color: Material.primary
                            }
                        }
                        
                        Label {
                            text: "Connect your GitHub account to submit hardware reports to the community database"
                            color: "#666"
                            wrapMode: Text.WordWrap
                            width: parent.width
                        }
                        
                        Row {
                            spacing: 12
                            
                            TextField {
                                width: 250
                                placeholderText: "GitHub username"
                            }
                            
                            TextField {
                                width: 250
                                placeholderText: "Personal Access Token"
                                echoMode: TextInput.Password
                            }
                            
                            Button {
                                text: "Connect"
                                Material.background: "#24292e"
                                Material.foreground: "white"
                            }
                        }
                        
                        Label {
                            text: "⚠️ A Personal Access Token with 'public_repo' permissions is required for submission"
                            color: "#FF9800"
                            font.pixelSize: 12
                            wrapMode: Text.WordWrap
                            width: parent.width
                        }
                    }
                }
                
                // Submission options
                Rectangle {
                    width: parent.width
                    height: optionsContent.height + 32
                    radius: 8
                    color: "white"
                    border.color: "#E0E0E0"
                    border.width: 1
                    
                    Column {
                        id: optionsContent
                        anchors.fill: parent
                        anchors.margins: 16
                        spacing: 12
                        
                        Label {
                            text: "Submission Options"
                            font.weight: Font.Bold
                            color: Material.primary
                        }
                        
                        Column {
                            spacing: 8
                            
                            CheckBox {
                                text: "Include driver information and kernel module details"
                                checked: true
                            }
                            
                            CheckBox {
                                text: "Include performance benchmarks (if available)"
                                checked: false
                            }
                            
                            CheckBox {
                                text: "Include configuration recommendations"
                                checked: true
                            }
                            
                            CheckBox {
                                text: "Subscribe to updates for submitted hardware"
                                checked: false
                            }
                        }
                        
                        Label {
                            text: "Additional Information (Optional)"
                            font.weight: Font.Medium
                            color: "#666"
                        }
                        
                        ScrollView {
                            width: parent.width
                            height: 100
                            
                            TextArea {
                                placeholderText: "Additional context, issues, or notes about your hardware configuration..."
                                wrapMode: TextArea.Wrap
                                selectByMouse: true
                            }
                        }
                    }
                }
                
                // Submission history
                Rectangle {
                    width: parent.width
                    height: historyContent.height + 32
                    radius: 8
                    color: "white"
                    border.color: "#E0E0E0"
                    border.width: 1
                    
                    Column {
                        id: historyContent
                        anchors.fill: parent
                        anchors.margins: 16
                        spacing: 12
                        
                        Label {
                            text: "Submission History"
                            font.weight: Font.Bold
                            color: Material.primary
                        }
                        
                        // History entry example
                        Rectangle {
                            width: parent.width
                            height: 60
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
                                    anchors.verticalCenter: parent.verticalCenter
                                    
                                    Label {
                                        text: "Hardware Report #1"
                                        font.weight: Font.Medium
                                    }
                                    
                                    Label {
                                        text: "Submitted 2 days ago • 15 devices • Privacy: Enhanced"
                                        color: "#666"
                                        font.pixelSize: 12
                                    }
                                }
                                
                                Item { Layout.fillWidth: true }
                                
                                Chip {
                                    text: "Merged"
                                    Material.background: "#4CAF50"
                                    textColor: "white"
                                    small: true
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Submit button
        Row {
            anchors.horizontalCenter: parent.horizontalCenter
            spacing: 16
            
            Button {
                text: "Submit to Community"
                Material.background: Material.primary
                Material.foreground: "white"
                enabled: false // Enable when GitHub connected
                onClicked: console.log("Submit hardware report")
            }
            
            Button {
                text: "Save Draft"
                flat: true
                onClicked: console.log("Save draft")
            }
        }
    }
}