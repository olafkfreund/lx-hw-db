import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import QtQuick.Layouts
import "components"

Item {
    id: root
    
    // Header bar
    ToolBar {
        id: headerBar
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.right: parent.right
        height: 64
        Material.background: Material.primary
        
        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 16
            anchors.rightMargin: 16
            
            // Privacy status indicator
            Row {
                spacing: 8
                
                Image {
                    width: 24
                    height: 24
                    source: privacyManager.isSecure ? "qrc:/icons/shield-check.svg" : "qrc:/icons/shield-alert.svg"
                }
                
                Label {
                    text: "Privacy: " + privacyManager.currentLevel
                    color: "white"
                    font.weight: Font.Medium
                    anchors.verticalCenter: parent.verticalCenter
                }
            }
            
            Item { Layout.fillWidth: true }
            
            // Window controls
            ToolButton {
                icon.name: "settings"
                text: "Settings"
                onClicked: stackView.push("PrivacyScreen.qml")
            }
            
            ToolButton {
                icon.name: "help"
                text: "Help"
                onClicked: Qt.openUrlExternally("https://github.com/lx-hw-db/lx-hw-db")
            }
        }
    }
    
    // Main content area
    Row {
        anchors.top: headerBar.bottom
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: statusBar.top
        
        // Navigation rail (left sidebar)
        NavigationRail {
            id: navigationRail
            width: 80
            height: parent.height
            
            onPageSelected: function(pageId) {
                stackView.replace(pageId + "Screen.qml")
            }
        }
        
        // Main content view stack
        StackView {
            id: stackView
            width: parent.width - navigationRail.width
            height: parent.height
            initialItem: WelcomeScreen {}
            
            // Smooth page transitions
            pushEnter: Transition {
                PropertyAnimation {
                    property: "opacity"
                    from: 0
                    to: 1
                    duration: 200
                }
                PropertyAnimation {
                    property: "x"
                    from: width
                    to: 0
                    duration: 200
                    easing.type: Easing.OutCubic
                }
            }
            
            pushExit: Transition {
                PropertyAnimation {
                    property: "opacity"
                    from: 1
                    to: 0.7
                    duration: 200
                }
            }
        }
    }
    
    // Status bar
    ToolBar {
        id: statusBar
        anchors.bottom: parent.bottom
        anchors.left: parent.left
        anchors.right: parent.right
        height: 32
        Material.background: "#F5F5F5"
        
        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 16
            anchors.rightMargin: 16
            
            Label {
                text: "Tools: " + detectionManager.completedTools + "/" + detectionManager.totalTools
                font.pixelSize: 12
                color: "#666"
            }
            
            Item { Layout.fillWidth: true }
            
            Label {
                text: "Devices: " + hardwareManager.deviceCount
                font.pixelSize: 12
                color: "#666"
                visible: hardwareManager.detectionComplete
            }
            
            Label {
                text: "Privacy: " + privacyManager.anonymizationStatus
                font.pixelSize: 12
                color: privacyManager.isSecure ? "#4CAF50" : "#FF9800"
            }
        }
    }
}