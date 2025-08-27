import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material

Button {
    id: chip
    
    property string icon: ""
    property bool clickable: true
    property bool small: false
    property color textColor: "white"
    
    height: small ? 24 : 32
    padding: small ? 4 : 8
    flat: true
    enabled: clickable
    
    background: Rectangle {
        color: chip.Material.background || Material.primary
        radius: height / 2
        opacity: chip.enabled ? 1.0 : 0.7
        
        border.color: chip.flat ? "transparent" : Qt.darker(color, 1.1)
        border.width: chip.flat ? 0 : 1
    }
    
    Row {
        anchors.centerIn: parent
        spacing: chip.small ? 4 : 6
        
        Image {
            source: chip.icon
            width: chip.small ? 12 : 16
            height: chip.small ? 12 : 16
            visible: chip.icon !== ""
            anchors.verticalCenter: parent.verticalCenter
        }
        
        Label {
            text: chip.text
            color: chip.textColor
            font.pixelSize: chip.small ? 10 : 12
            font.weight: Font.Medium
            anchors.verticalCenter: parent.verticalCenter
        }
    }
}