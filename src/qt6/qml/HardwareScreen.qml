import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import "components"

Page {
    objectName: "hardware"
    
    Column {
        anchors.fill: parent
        anchors.margins: 24
        spacing: 20
        
        // Hardware summary header
        Row {
            width: parent.width
            
            Column {
                width: parent.width * 0.6
                
                Label {
                    text: "Detected Hardware"
                    font.pixelSize: 24
                    font.weight: Font.Bold
                    color: Material.primary
                }
                
                Label {
                    text: hardwareManager.deviceCount + " devices detected • " + 
                          hardwareManager.getSupportedCount() + " fully supported"
                    color: "#666"
                    font.pixelSize: 14
                }
            }
            
            Item { Layout.fillWidth: true }
            
            // Compatibility summary
            Row {
                spacing: 12
                
                Chip {
                    text: hardwareManager.getCountByStatus("supported") + " supported"
                    icon: "qrc:/icons/check-circle.svg"
                    Material.background: "#4CAF50"
                    textColor: "white"
                }
                
                Chip {
                    text: hardwareManager.getCountByStatus("partial") + " partial"
                    icon: "qrc:/icons/warning.svg"
                    Material.background: "#FF9800"
                    textColor: "white"
                }
                
                Chip {
                    text: hardwareManager.getCountByStatus("unsupported") + " unsupported"
                    icon: "qrc:/icons/error.svg"
                    Material.background: "#F44336"
                    textColor: "white"
                }
            }
        }
        
        // Hardware content placeholder
        Rectangle {
            width: parent.width
            height: 400
            color: "white"
            radius: 8
            border.color: "#E0E0E0"
            border.width: 1
            
            Column {
                anchors.centerIn: parent
                spacing: 16
                
                Image {
                    source: "qrc:/icons/devices.svg"
                    width: 64
                    height: 64
                    anchors.horizontalCenter: parent.horizontalCenter
                    opacity: 0.5
                }
                
                Label {
                    text: "Hardware View"
                    font.pixelSize: 18
                    font.weight: Font.Medium
                    color: "#666"
                    anchors.horizontalCenter: parent.horizontalCenter
                }
                
                Label {
                    text: "Detailed hardware information and compatibility status will be displayed here."
                    color: "#999"
                    anchors.horizontalCenter: parent.horizontalCenter
                    horizontalAlignment: Text.AlignHCenter
                }
            }
        }
        
        // Quick actions
        Row {
            anchors.horizontalCenter: parent.horizontalCenter
            spacing: 16
            
            Button {
                text: "Get Recommendations"
                icon.source: "qrc:/icons/tune.svg"
                Material.background: Material.primary
                Material.foreground: "white"
                onClicked: stackView.replace("ConfigScreen.qml")
            }
            
            Button {
                text: "Export Report" 
                icon.source: "qrc:/icons/download.svg"
            }
            
            Button {
                text: "Refresh Detection"
                icon.source: "qrc:/icons/refresh.svg"
                flat: true
                onClicked: stackView.replace("DetectionScreen.qml")
            }
        }
    }
}