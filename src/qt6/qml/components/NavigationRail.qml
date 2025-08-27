import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import QtQuick.Layouts

Rectangle {
    id: root
    
    color: "#F5F5F5"
    border.color: "#E0E0E0"
    border.width: 1
    
    signal pageSelected(string pageId)
    
    property string currentPage: "Welcome"
    
    Column {
        anchors.fill: parent
        anchors.margins: 8
        spacing: 4
        
        NavigationRailItem {
            icon: "qrc:/icons/home.svg"
            text: "Welcome"
            selected: root.currentPage === "Welcome"
            onClicked: {
                root.currentPage = "Welcome"
                root.pageSelected("Welcome")
            }
        }
        
        NavigationRailItem {
            icon: "qrc:/icons/search.svg"
            text: "Detect"
            selected: root.currentPage === "Detection"
            onClicked: {
                root.currentPage = "Detection"
                root.pageSelected("Detection")
            }
        }
        
        NavigationRailItem {
            icon: "qrc:/icons/devices.svg"
            text: "Hardware"
            selected: root.currentPage === "Hardware"
            enabled: hardwareManager.detectionComplete
            onClicked: {
                root.currentPage = "Hardware"
                root.pageSelected("Hardware")
            }
        }
        
        NavigationRailItem {
            icon: "qrc:/icons/tune.svg"
            text: "Config"
            selected: root.currentPage === "Config"
            enabled: hardwareManager.detectionComplete
            onClicked: {
                root.currentPage = "Config"
                root.pageSelected("Config")
            }
        }
        
        NavigationRailItem {
            icon: "qrc:/icons/share.svg"
            text: "Submit"
            selected: root.currentPage === "Submission"
            enabled: hardwareManager.detectionComplete
            onClicked: {
                root.currentPage = "Submission"
                root.pageSelected("Submission")
            }
        }
        
        // Spacer
        Item { height: 20 }
        
        // Privacy settings always accessible
        NavigationRailItem {
            icon: "qrc:/icons/shield.svg"
            text: "Privacy"
            selected: root.currentPage === "Privacy"
            onClicked: {
                root.currentPage = "Privacy"
                root.pageSelected("Privacy")
            }
        }
        
        Item { Layout.fillHeight: true }
    }
}

// Navigation rail item component
component NavigationRailItem: Button {
    id: item
    
    property string icon: ""
    property bool selected: false
    
    width: parent.width - 16
    height: 64
    flat: true
    checkable: false
    
    background: Rectangle {
        color: item.selected ? Material.primary : (item.hovered ? "#E0E0E0" : "transparent")
        radius: 8
        opacity: item.enabled ? 1.0 : 0.5
    }
    
    Column {
        anchors.centerIn: parent
        spacing: 2
        
        Image {
            source: item.icon
            width: 24
            height: 24
            anchors.horizontalCenter: parent.horizontalCenter
            opacity: item.enabled ? 1.0 : 0.5
        }
        
        Label {
            text: item.text
            font.pixelSize: 10
            color: item.selected ? "white" : (item.enabled ? "#333" : "#999")
            anchors.horizontalCenter: parent.horizontalCenter
        }
    }
}