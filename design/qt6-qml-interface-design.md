# Qt6 QML Interface Design for lx-hw-db
## Linux Hardware Detection Application

> **Design System**: Material Design 3 with Qt6 QML
> **Target Platform**: Linux Desktop (Primary), Responsive Design
> **Privacy Focus**: Transparency and user control throughout interface
> **Date**: August 27, 2025

---

## 1. Application Architecture & Navigation

### Main Window Layout
```qml
// MainWindow.qml - Overall application architecture
ApplicationWindow {
    id: window
    width: 1200
    height: 800
    minimumWidth: 900
    minimumHeight: 600
    
    // Material Design 3 theming
    Material.theme: Material.Light
    Material.primary: "#6750A4"      // Privacy-focused purple
    Material.accent: "#625B71"       // Secondary accent
    Material.background: "#FFFBFE"   // Surface container
    
    header: ToolBar {
        height: 64
        Material.background: Material.primary
        
        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 16
            anchors.rightMargin: 16
            
            // Privacy status indicator
            Row {
                spacing: 8
                Icon {
                    name: "shield-check"
                    color: privacyManager.isSecure ? "#4CAF50" : "#FF9800"
                }
                Label {
                    text: "Privacy: " + privacyManager.currentLevel
                    color: "white"
                    font.weight: Font.Medium
                }
            }
            
            Item { Layout.fillWidth: true }
            
            // Window controls
            ToolButton {
                icon.name: "settings"
                text: "Settings"
                onClicked: stackView.push("Settings.qml")
            }
            
            ToolButton {
                icon.name: "help"
                text: "Help"
                onClicked: helpDialog.open()
            }
        }
    }
    
    // Navigation rail (left sidebar)
    Row {
        anchors.fill: parent
        
        NavigationRail {
            width: 80
            height: parent.height
            
            NavigationRailItem {
                icon: "home"
                text: "Welcome"
                selected: stackView.currentItem.objectName === "welcome"
                onClicked: stackView.replace("WelcomeScreen.qml")
            }
            
            NavigationRailItem {
                icon: "search"
                text: "Detect"
                selected: stackView.currentItem.objectName === "detect"
                onClicked: stackView.replace("DetectionScreen.qml")
            }
            
            NavigationRailItem {
                icon: "devices"
                text: "Hardware"
                selected: stackView.currentItem.objectName === "hardware"
                onClicked: stackView.replace("HardwareScreen.qml")
                enabled: hardwareManager.detectionComplete
            }
            
            NavigationRailItem {
                icon: "tune"
                text: "Config"
                selected: stackView.currentItem.objectName === "config"
                onClicked: stackView.replace("ConfigScreen.qml")
                enabled: hardwareManager.detectionComplete
            }
            
            NavigationRailItem {
                icon: "share"
                text: "Submit"
                selected: stackView.currentItem.objectName === "submit"
                onClicked: stackView.replace("SubmissionScreen.qml")
                enabled: hardwareManager.detectionComplete
            }
            
            // Privacy settings always accessible
            Item { height: 20 }
            NavigationRailItem {
                icon: "shield"
                text: "Privacy"
                selected: stackView.currentItem.objectName === "privacy"
                onClicked: stackView.replace("PrivacyScreen.qml")
            }
        }
        
        // Main content area
        StackView {
            id: stackView
            Layout.fillWidth: true
            Layout.fillHeight: true
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
    footer: ToolBar {
        height: 32
        Material.background: "#F5F5F5"
        
        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 16
            anchors.rightMargin: 16
            
            Label {
                text: "Tools: " + toolManager.availableToolsCount + "/" + toolManager.totalTools
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
```

### Design Rationale
**Navigation Rail Choice**: Selected over bottom navigation because:
- Desktop-first application with ample horizontal space
- Persistent access to privacy settings
- Clear visual hierarchy with disabled states
- Icons + labels provide clear navigation context

---

## 2. Welcome/Introduction Screen

### Welcome Screen Design
```qml
// WelcomeScreen.qml
Page {
    objectName: "welcome"
    
    ScrollView {
        anchors.fill: parent
        contentWidth: availableWidth
        
        Column {
            width: parent.width
            spacing: 32
            
            // Hero section
            Card {
                width: parent.width - 64
                anchors.horizontalCenter: parent.horizontalCenter
                Material.elevation: 2
                
                Column {
                    anchors.fill: parent
                    anchors.margins: 32
                    spacing: 24
                    
                    Row {
                        anchors.horizontalCenter: parent.horizontalCenter
                        spacing: 16
                        
                        Image {
                            source: "icons/hardware-shield.svg"
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
                    model: ListModel {
                        ListElement {
                            level: "Basic"
                            color: "#4CAF50"
                            icon: "shield-outline"
                            description: "Device models, no serial numbers"
                        }
                        ListElement {
                            level: "Enhanced"
                            color: "#FF9800"
                            icon: "shield-half"
                            description: "Hashed identifiers, kernel info"
                        }
                        ListElement {
                            level: "Strict"
                            color: "#F44336"
                            icon: "shield"
                            description: "Maximum anonymization, noise injection"
                        }
                    }
                    
                    Card {
                        width: 200
                        height: 120
                        Material.elevation: 1
                        
                        Column {
                            anchors.centerIn: parent
                            spacing: 8
                            
                            Icon {
                                name: model.icon
                                color: model.color
                                size: 32
                                anchors.horizontalCenter: parent.horizontalCenter
                            }
                            
                            Label {
                                text: model.level
                                font.weight: Font.Bold
                                color: model.color
                                horizontalAlignment: Text.AlignHCenter
                            }
                            
                            Label {
                                text: model.description
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
                                privacyManager.setLevel(model.level)
                                privacyLevelIndicator.highlight(model.level)
                            }
                        }
                    }
                }
            }
            
            // Quick start section
            Card {
                width: parent.width - 64
                anchors.horizontalCenter: parent.horizontalCenter
                Material.elevation: 1
                
                Column {
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
                            icon: "shield-check"
                            clickable: false
                        }
                        
                        Chip {
                            text: "2. Detect hardware"
                            icon: "search"
                            clickable: false
                        }
                        
                        Chip {
                            text: "3. Review compatibility"
                            icon: "devices"
                            clickable: false
                        }
                        
                        Chip {
                            text: "4. Get recommendations"
                            icon: "tune"
                            clickable: false
                        }
                        
                        Chip {
                            text: "5. Share with community"
                            icon: "share"
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
                    icon.name: "play-arrow"
                    Material.background: Material.primary
                    onClicked: stackView.replace("DetectionScreen.qml")
                }
                
                Button {
                    text: "Privacy Settings"
                    icon.name: "shield"
                    flat: true
                    onClicked: stackView.replace("PrivacyScreen.qml")
                }
                
                Button {
                    text: "Learn More"
                    icon.name: "info"
                    flat: true
                    onClicked: Qt.openUrlExternally("https://github.com/lx-hw-db/lx-hw-db")
                }
            }
        }
    }
}
```

### Design Elements
- **Hero section**: Clear branding with privacy-focused messaging
- **Privacy preview**: Three-card layout showing privacy levels at a glance
- **Quick start**: Visual workflow showing user journey
- **Clear CTAs**: Primary action (Start Detection) with secondary options

---

## 3. Hardware Detection Screen

### Detection Screen Design
```qml
// DetectionScreen.qml
Page {
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
            Card {
                width: 200
                height: 60
                Material.elevation: 1
                Material.background: privacyManager.getStatusColor()
                
                Row {
                    anchors.centerIn: parent
                    spacing: 8
                    
                    Icon {
                        name: "shield-check"
                        color: "white"
                        size: 24
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
        Card {
            width: parent.width
            height: 120
            Material.elevation: 2
            
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
                
                // Tool progress cards
                Repeater {
                    model: detectionManager.tools
                    
                    Card {
                        width: parent.width
                        height: 80
                        Material.elevation: 1
                        
                        Row {
                            anchors.fill: parent
                            anchors.margins: 16
                            spacing: 16
                            
                            // Tool icon and status
                            Rectangle {
                                width: 48
                                height: 48
                                radius: 24
                                color: model.getStatusColor()
                                
                                Icon {
                                    anchors.centerIn: parent
                                    name: model.getStatusIcon()
                                    color: "white"
                                    size: 24
                                }
                            }
                            
                            // Tool details
                            Column {
                                width: parent.width * 0.6
                                anchors.verticalCenter: parent.verticalCenter
                                
                                Label {
                                    text: model.name
                                    font.weight: Font.Medium
                                }
                                
                                Label {
                                    text: model.description
                                    color: "#666"
                                    font.pixelSize: 12
                                    elide: Text.ElideRight
                                    width: parent.width
                                }
                                
                                ProgressBar {
                                    width: parent.width
                                    value: model.progress
                                    visible: model.status === "running"
                                    Material.accent: model.getStatusColor()
                                }
                            }
                            
                            // Results summary
                            Column {
                                anchors.verticalCenter: parent.verticalCenter
                                
                                Label {
                                    text: model.deviceCount + " devices"
                                    font.weight: Font.Bold
                                    color: model.status === "complete" ? Material.primary : "#666"
                                    visible: model.status === "complete"
                                }
                                
                                Label {
                                    text: model.processingTime
                                    color: "#666"
                                    font.pixelSize: 12
                                    visible: model.status === "complete"
                                }
                                
                                BusyIndicator {
                                    running: model.status === "running"
                                    visible: model.status === "running"
                                    Material.accent: Material.primary
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
                icon.name: detectionManager.isRunning ? "stop" : "play-arrow"
                Material.background: detectionManager.isRunning ? "#F44336" : Material.primary
                onClicked: detectionManager.toggleDetection()
            }
            
            Button {
                text: "View Hardware"
                icon.name: "arrow-forward"
                enabled: detectionManager.isComplete
                onClicked: stackView.replace("HardwareScreen.qml")
            }
            
            Button {
                text: "Privacy Settings"
                icon.name: "shield"
                flat: true
                onClicked: stackView.replace("PrivacyScreen.qml")
            }
        }
    }
}
```

### Key Features
- **Real-time progress**: Both overall and per-tool progress tracking
- **Privacy awareness**: Constant privacy level reminder
- **Clear status**: Visual indicators for each detection tool
- **Time estimation**: Remaining time calculations
- **Responsive actions**: Context-aware button states

---

## 4. Hardware Display Screen

### Hardware Display Design
```qml
// HardwareScreen.qml
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
                
                // Support status chips
                Repeater {
                    model: ListModel {
                        ListElement { status: "supported"; count: 0; color: "#4CAF50"; icon: "check-circle" }
                        ListElement { status: "partial"; count: 0; color: "#FF9800"; icon: "warning" }
                        ListElement { status: "unsupported"; count: 0; color: "#F44336"; icon: "error" }
                        ListElement { status: "unknown"; count: 0; color: "#9E9E9E"; icon: "help" }
                    }
                    
                    Chip {
                        text: model.count + " " + model.status
                        icon: model.icon
                        Material.background: model.color
                        textColor: "white"
                        clickable: true
                        onClicked: hardwareView.filterByStatus(model.status)
                        
                        Component.onCompleted: {
                            model.count = hardwareManager.getCountByStatus(model.status)
                        }
                    }
                }
            }
        }
        
        // Filter and search bar
        Card {
            width: parent.width
            height: 60
            Material.elevation: 1
            
            Row {
                anchors.fill: parent
                anchors.margins: 12
                spacing: 12
                
                TextField {
                    id: searchField
                    width: 300
                    placeholderText: "Search devices, vendors, models..."
                    leftIcon: "search"
                    onTextChanged: hardwareView.filter(text)
                }
                
                ComboBox {
                    id: categoryFilter
                    width: 150
                    model: ["All Categories", "CPU", "GPU", "Memory", "Storage", "Network", "USB", "Audio"]
                    onCurrentTextChanged: hardwareView.filterByCategory(currentText)
                }
                
                ComboBox {
                    id: vendorFilter
                    width: 150
                    model: hardwareManager.getVendorList()
                    onCurrentTextChanged: hardwareView.filterByVendor(currentText)
                }
                
                Item { Layout.fillWidth: true }
                
                Button {
                    text: "Clear Filters"
                    icon.name: "clear"
                    flat: true
                    onClicked: {
                        searchField.clear()
                        categoryFilter.currentIndex = 0
                        vendorFilter.currentIndex = 0
                        hardwareView.clearFilters()
                    }
                }
            }
        }
        
        // Hardware categories view
        ScrollView {
            id: hardwareView
            width: parent.width
            height: parent.height - 200
            contentWidth: availableWidth
            
            Column {
                width: parent.width
                spacing: 16
                
                // Hardware categories
                Repeater {
                    model: hardwareManager.categories
                    
                    // Category section
                    Card {
                        width: parent.width
                        Material.elevation: 1
                        
                        Column {
                            anchors.fill: parent
                            anchors.margins: 0
                            
                            // Category header (clickable to expand/collapse)
                            ItemDelegate {
                                width: parent.width
                                height: 60
                                onClicked: model.expanded = !model.expanded
                                
                                Row {
                                    anchors.fill: parent
                                    anchors.margins: 16
                                    spacing: 12
                                    
                                    Rectangle {
                                        width: 40
                                        height: 40
                                        radius: 8
                                        color: model.color
                                        
                                        Icon {
                                            anchors.centerIn: parent
                                            name: model.icon
                                            color: "white"
                                            size: 24
                                        }
                                    }
                                    
                                    Column {
                                        anchors.verticalCenter: parent.verticalCenter
                                        
                                        Label {
                                            text: model.name
                                            font.pixelSize: 18
                                            font.weight: Font.Medium
                                        }
                                        
                                        Label {
                                            text: model.devices.length + " devices"
                                            color: "#666"
                                            font.pixelSize: 14
                                        }
                                    }
                                    
                                    Item { Layout.fillWidth: true }
                                    
                                    // Support status summary for category
                                    Row {
                                        spacing: 4
                                        
                                        Repeater {
                                            model: model.getSupportSummary()
                                            
                                            Rectangle {
                                                width: 8
                                                height: 8
                                                radius: 4
                                                color: model.color
                                            }
                                        }
                                    }
                                    
                                    Icon {
                                        name: model.expanded ? "expand-less" : "expand-more"
                                        color: "#666"
                                    }
                                }
                            }
                            
                            // Device list (expanded)
                            Column {
                                width: parent.width
                                visible: model.expanded
                                
                                Repeater {
                                    model: model.devices
                                    
                                    // Individual device card
                                    Card {
                                        width: parent.width - 32
                                        anchors.horizontalCenter: parent.horizontalCenter
                                        height: deviceInfo.height + 24
                                        Material.elevation: 0
                                        Material.background: "#FAFAFA"
                                        
                                        Column {
                                            id: deviceInfo
                                            anchors.fill: parent
                                            anchors.margins: 12
                                            
                                            Row {
                                                width: parent.width
                                                spacing: 12
                                                
                                                // Device status indicator
                                                Rectangle {
                                                    width: 8
                                                    height: parent.height
                                                    color: model.getStatusColor()
                                                    radius: 4
                                                }
                                                
                                                Column {
                                                    width: parent.width * 0.7
                                                    
                                                    Label {
                                                        text: model.displayName
                                                        font.weight: Font.Medium
                                                        font.pixelSize: 16
                                                        elide: Text.ElideRight
                                                        width: parent.width
                                                    }
                                                    
                                                    Label {
                                                        text: model.vendor + " • " + model.model
                                                        color: "#666"
                                                        font.pixelSize: 14
                                                        elide: Text.ElideRight
                                                        width: parent.width
                                                    }
                                                    
                                                    // Driver info
                                                    Row {
                                                        spacing: 8
                                                        visible: model.driverInfo.length > 0
                                                        
                                                        Icon {
                                                            name: "code"
                                                            size: 16
                                                            color: "#666"
                                                        }
                                                        
                                                        Label {
                                                            text: model.driverInfo
                                                            color: "#666"
                                                            font.pixelSize: 12
                                                        }
                                                    }
                                                }
                                                
                                                Item { Layout.fillWidth: true }
                                                
                                                // Compatibility status chip
                                                Chip {
                                                    text: model.compatibilityStatus
                                                    icon: model.getStatusIcon()
                                                    Material.background: model.getStatusColor()
                                                    textColor: "white"
                                                    small: true
                                                }
                                                
                                                // Action menu
                                                ToolButton {
                                                    icon.name: "more-vert"
                                                    onClicked: deviceContextMenu.open()
                                                    
                                                    Menu {
                                                        id: deviceContextMenu
                                                        
                                                        MenuItem {
                                                            text: "View Details"
                                                            icon.name: "info"
                                                            onTriggered: deviceDetailsDialog.open(model)
                                                        }
                                                        
                                                        MenuItem {
                                                            text: "Test Functionality"
                                                            icon.name: "play-arrow"
                                                            enabled: model.testable
                                                            onTriggered: hardwareManager.testDevice(model.id)
                                                        }
                                                        
                                                        MenuItem {
                                                            text: "Search Solutions"
                                                            icon.name: "search"
                                                            onTriggered: Qt.openUrlExternally(model.getSupportUrl())
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Quick actions
        Row {
            anchors.horizontalCenter: parent.horizontalCenter
            spacing: 16
            
            Button {
                text: "Get Recommendations"
                icon.name: "tune"
                Material.background: Material.primary
                onClicked: stackView.replace("ConfigScreen.qml")
            }
            
            Button {
                text: "Export Report"
                icon.name: "download"
                onClicked: exportDialog.open()
            }
            
            Button {
                text: "Refresh Detection"
                icon.name: "refresh"
                flat: true
                onClicked: stackView.replace("DetectionScreen.qml")
            }
        }
    }
    
    // Device details dialog
    DeviceDetailsDialog {
        id: deviceDetailsDialog
    }
    
    // Export dialog
    ExportDialog {
        id: exportDialog
    }
}
```

### Device Details Dialog
```qml
// DeviceDetailsDialog.qml
Dialog {
    id: dialog
    width: 600
    height: 500
    modal: true
    anchors.centerIn: parent
    
    property var device
    
    function open(deviceModel) {
        device = deviceModel
        visible = true
    }
    
    header: Label {
        text: device ? device.displayName : "Device Details"
        font.pixelSize: 20
        font.weight: Font.Bold
        padding: 20
    }
    
    ScrollView {
        anchors.fill: parent
        contentWidth: availableWidth
        
        Column {
            width: parent.width
            spacing: 16
            
            // Basic info section
            Card {
                width: parent.width
                Material.elevation: 1
                
                Column {
                    anchors.fill: parent
                    anchors.margins: 16
                    spacing: 8
                    
                    Label {
                        text: "Basic Information"
                        font.weight: Font.Bold
                        color: Material.primary
                    }
                    
                    GridLayout {
                        columns: 2
                        width: parent.width
                        
                        Label { text: "Vendor:"; color: "#666" }
                        Label { text: device ? device.vendor : "" }
                        
                        Label { text: "Model:"; color: "#666" }
                        Label { text: device ? device.model : "" }
                        
                        Label { text: "Category:"; color: "#666" }
                        Label { text: device ? device.category : "" }
                        
                        Label { text: "PCI ID:"; color: "#666" }
                        Label { 
                            text: device ? device.pciId : ""
                            font.family: "monospace"
                        }
                    }
                }
            }
            
            // Compatibility section
            Card {
                width: parent.width
                Material.elevation: 1
                
                Column {
                    anchors.fill: parent
                    anchors.margins: 16
                    spacing: 12
                    
                    Row {
                        spacing: 8
                        
                        Label {
                            text: "Linux Compatibility"
                            font.weight: Font.Bold
                            color: Material.primary
                        }
                        
                        Chip {
                            text: device ? device.compatibilityStatus : ""
                            Material.background: device ? device.getStatusColor() : "#9E9E9E"
                            textColor: "white"
                            small: true
                        }
                    }
                    
                    Label {
                        text: device ? device.compatibilityNotes : ""
                        width: parent.width
                        wrapMode: Text.WordWrap
                        color: "#666"
                    }
                    
                    // Driver information
                    Column {
                        width: parent.width
                        spacing: 4
                        visible: device && device.drivers.length > 0
                        
                        Label {
                            text: "Available Drivers:"
                            font.weight: Font.Medium
                        }
                        
                        Repeater {
                            model: device ? device.drivers : []
                            
                            Row {
                                spacing: 8
                                
                                Rectangle {
                                    width: 6
                                    height: 6
                                    radius: 3
                                    color: model.recommended ? "#4CAF50" : "#9E9E9E"
                                    anchors.verticalCenter: parent.verticalCenter
                                }
                                
                                Label {
                                    text: model.name + (model.recommended ? " (Recommended)" : "")
                                    font.family: "monospace"
                                    color: model.recommended ? "#4CAF50" : "#666"
                                }
                            }
                        }
                    }
                }
            }
            
            // Technical specifications
            Card {
                width: parent.width
                Material.elevation: 1
                visible: device && device.specifications
                
                Column {
                    anchors.fill: parent
                    anchors.margins: 16
                    spacing: 8
                    
                    Label {
                        text: "Technical Specifications"
                        font.weight: Font.Bold
                        color: Material.primary
                    }
                    
                    // Dynamic specs based on device type
                    GridLayout {
                        columns: 2
                        width: parent.width
                        
                        Repeater {
                            model: device ? device.specifications : []
                            
                            Label { text: model.name + ":"; color: "#666" }
                            Label { text: model.value }
                        }
                    }
                }
            }
        }
    }
    
    footer: Row {
        spacing: 8
        
        Button {
            text: "Test Device"
            icon.name: "play-arrow"
            enabled: device && device.testable
            onClicked: {
                hardwareManager.testDevice(device.id)
                dialog.close()
            }
        }
        
        Button {
            text: "Search Solutions"
            icon.name: "search"
            onClicked: Qt.openUrlExternally(device.getSupportUrl())
        }
        
        Button {
            text: "Close"
            flat: true
            onClicked: dialog.close()
        }
    }
}
```

### Key Features
- **Categorized view**: Hardware grouped by type with expand/collapse
- **Rich filtering**: Search, category, and vendor filtering options
- **Status indicators**: Clear visual compatibility status for each device
- **Detailed info**: Comprehensive device details in modal dialog
- **Action-oriented**: Test functionality, search solutions, get help
- **Responsive design**: Adapts to different screen sizes and content

---

## 5. Configuration Recommendations Screen

### Configuration Screen Design
```qml
// ConfigScreen.qml
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
            Card {
                width: 200
                height: 120
                Material.elevation: 2
                
                Column {
                    anchors.centerIn: parent
                    spacing: 8
                    
                    CircularProgressIndicator {
                        anchors.horizontalCenter: parent.horizontalCenter
                        size: 60
                        value: configManager.compatibilityScore
                        color: configManager.getScoreColor()
                        
                        Label {
                            anchors.centerIn: parent
                            text: Math.round(configManager.compatibilityScore * 100) + "%"
                            font.weight: Font.Bold
                            color: configManager.getScoreColor()
                        }
                    }
                    
                    Label {
                        text: "System Compatibility"
                        font.weight: Font.Medium
                        anchors.horizontalCenter: parent.horizontalCenter
                    }
                    
                    Label {
                        text: configManager.getScoreDescription()
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
                icon.name: "download"
                Material.background: "#4CAF50"
                enabled: configManager.hasDriverRecommendations
                onClicked: driverInstallationDialog.open()
            }
            
            Button {
                text: "Apply Kernel Parameters"
                icon.name: "settings"
                enabled: configManager.hasKernelRecommendations
                onClicked: kernelConfigDialog.open()
            }
            
            Button {
                text: "Export Configuration"
                icon.name: "file-download"
                onClicked: exportConfigDialog.open()
            }
            
            Button {
                text: "Performance Tuning"
                icon.name: "tune"
                flat: true
                onClicked: performanceDialog.open()
            }
        }
        
        // Recommendations tabs
        TabView {
            width: parent.width
            height: parent.height - 200
            
            Tab {
                title: "Driver Recommendations"
                
                ScrollView {
                    anchors.fill: parent
                    contentWidth: availableWidth
                    
                    Column {
                        width: parent.width
                        spacing: 12
                        
                        // Priority drivers section
                        Card {
                            width: parent.width
                            Material.elevation: 1
                            visible: configManager.priorityDrivers.length > 0
                            
                            Column {
                                anchors.fill: parent
                                anchors.margins: 16
                                spacing: 12
                                
                                Row {
                                    spacing: 8
                                    
                                    Icon {
                                        name: "priority-high"
                                        color: "#F44336"
                                    }
                                    
                                    Label {
                                        text: "Priority Drivers (Required for full functionality)"
                                        font.weight: Font.Bold
                                        color: "#F44336"
                                    }
                                }
                                
                                Repeater {
                                    model: configManager.priorityDrivers
                                    
                                    DriverRecommendationCard {
                                        width: parent.width - 32
                                        driver: model
                                        priority: true
                                    }
                                }
                            }
                        }
                        
                        // Recommended drivers section
                        Card {
                            width: parent.width
                            Material.elevation: 1
                            
                            Column {
                                anchors.fill: parent
                                anchors.margins: 16
                                spacing: 12
                                
                                Label {
                                    text: "Recommended Drivers"
                                    font.weight: Font.Bold
                                    color: Material.primary
                                }
                                
                                Repeater {
                                    model: configManager.recommendedDrivers
                                    
                                    DriverRecommendationCard {
                                        width: parent.width - 32
                                        driver: model
                                        priority: false
                                    }
                                }
                            }
                        }
                        
                        // Optional drivers section
                        Card {
                            width: parent.width
                            Material.elevation: 1
                            visible: configManager.optionalDrivers.length > 0
                            
                            Column {
                                anchors.fill: parent
                                anchors.margins: 16
                                spacing: 12
                                
                                Label {
                                    text: "Optional Drivers (Enhanced Features)"
                                    font.weight: Font.Bold
                                    color: "#666"
                                }
                                
                                Repeater {
                                    model: configManager.optionalDrivers
                                    
                                    DriverRecommendationCard {
                                        width: parent.width - 32
                                        driver: model
                                        priority: false
                                        optional: true
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            Tab {
                title: "Kernel Parameters"
                
                ScrollView {
                    anchors.fill: parent
                    contentWidth: availableWidth
                    
                    Column {
                        width: parent.width
                        spacing: 12
                        
                        // Current kernel info
                        Card {
                            width: parent.width
                            Material.elevation: 1
                            
                            Row {
                                anchors.fill: parent
                                anchors.margins: 16
                                spacing: 16
                                
                                Icon {
                                    name: "info"
                                    color: Material.primary
                                    size: 24
                                }
                                
                                Column {
                                    anchors.verticalCenter: parent.verticalCenter
                                    
                                    Label {
                                        text: "Current Kernel: " + systemInfo.kernelVersion
                                        font.weight: Font.Medium
                                    }
                                    
                                    Label {
                                        text: systemInfo.distributionName + " • " + systemInfo.architectureName
                                        color: "#666"
                                        font.pixelSize: 14
                                    }
                                }
                            }
                        }
                        
                        // Kernel parameter recommendations
                        Repeater {
                            model: configManager.kernelRecommendations
                            
                            Card {
                                width: parent.width
                                Material.elevation: 1
                                
                                Column {
                                    anchors.fill: parent
                                    anchors.margins: 16
                                    spacing: 12
                                    
                                    Row {
                                        width: parent.width
                                        spacing: 12
                                        
                                        Rectangle {
                                            width: 4
                                            height: parent.height
                                            color: model.getImportanceColor()
                                            radius: 2
                                        }
                                        
                                        Column {
                                            width: parent.width * 0.7
                                            
                                            Label {
                                                text: model.name
                                                font.weight: Font.Medium
                                                font.pixelSize: 16
                                            }
                                            
                                            Label {
                                                text: model.description
                                                color: "#666"
                                                font.pixelSize: 14
                                                wrapMode: Text.WordWrap
                                                width: parent.width
                                            }
                                            
                                            // Parameter value
                                            Rectangle {
                                                width: parameterText.width + 16
                                                height: 32
                                                color: "#F5F5F5"
                                                radius: 4
                                                border.color: "#E0E0E0"
                                                
                                                Label {
                                                    id: parameterText
                                                    anchors.centerIn: parent
                                                    text: model.parameter
                                                    font.family: "monospace"
                                                    color: "#333"
                                                }
                                            }
                                        }
                                        
                                        Item { Layout.fillWidth: true }
                                        
                                        Column {
                                            anchors.verticalCenter: parent.verticalCenter
                                            spacing: 4
                                            
                                            Chip {
                                                text: model.importance
                                                Material.background: model.getImportanceColor()
                                                textColor: "white"
                                                small: true
                                            }
                                            
                                            Switch {
                                                checked: model.enabled
                                                onToggled: model.enabled = checked
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            Tab {
                title: "Package Installation"
                
                ScrollView {
                    anchors.fill: parent
                    contentWidth: availableWidth
                    
                    Column {
                        width: parent.width
                        spacing: 16
                        
                        // Distribution detection
                        Card {
                            width: parent.width
                            Material.elevation: 1
                            
                            Row {
                                anchors.fill: parent
                                anchors.margins: 16
                                spacing: 12
                                
                                Image {
                                    source: "icons/distro/" + systemInfo.distributionId + ".svg"
                                    width: 32
                                    height: 32
                                }
                                
                                Column {
                                    anchors.verticalCenter: parent.verticalCenter
                                    
                                    Label {
                                        text: systemInfo.distributionName
                                        font.weight: Font.Bold
                                    }
                                    
                                    Label {
                                        text: "Package manager: " + systemInfo.packageManager
                                        color: "#666"
                                        font.pixelSize: 14
                                    }
                                }
                            }
                        }
                        
                        // Installation commands by category
                        Repeater {
                            model: configManager.packageCategories
                            
                            Card {
                                width: parent.width
                                Material.elevation: 1
                                
                                Column {
                                    anchors.fill: parent
                                    anchors.margins: 16
                                    spacing: 12
                                    
                                    Row {
                                        width: parent.width
                                        
                                        Label {
                                            text: model.name
                                            font.weight: Font.Bold
                                            color: Material.primary
                                        }
                                        
                                        Item { Layout.fillWidth: true }
                                        
                                        Button {
                                            text: "Copy All"
                                            icon.name: "content-copy"
                                            flat: true
                                            onClicked: clipboard.setText(model.getAllCommands())
                                        }
                                        
                                        Button {
                                            text: "Run All"
                                            icon.name: "play-arrow"
                                            enabled: model.canAutoInstall
                                            onClicked: packageManager.installCategory(model.id)
                                        }
                                    }
                                    
                                    Label {
                                        text: model.description
                                        color: "#666"
                                        width: parent.width
                                        wrapMode: Text.WordWrap
                                    }
                                    
                                    // Individual package commands
                                    Repeater {
                                        model: model.packages
                                        
                                        Row {
                                            width: parent.width
                                            spacing: 8
                                            
                                            Switch {
                                                id: packageSwitch
                                                checked: model.selected
                                                onToggled: model.selected = checked
                                            }
                                            
                                            Column {
                                                width: parent.width - 100
                                                anchors.verticalCenter: parent.verticalCenter
                                                
                                                Label {
                                                    text: model.displayName
                                                    font.weight: Font.Medium
                                                    enabled: packageSwitch.checked
                                                }
                                                
                                                Rectangle {
                                                    width: parent.width
                                                    height: 32
                                                    color: packageSwitch.checked ? "#F5F5F5" : "#FAFAFA"
                                                    radius: 4
                                                    border.color: "#E0E0E0"
                                                    
                                                    Label {
                                                        anchors.left: parent.left
                                                        anchors.leftMargin: 8
                                                        anchors.verticalCenter: parent.verticalCenter
                                                        text: model.installCommand
                                                        font.family: "monospace"
                                                        color: packageSwitch.checked ? "#333" : "#999"
                                                        font.pixelSize: 12
                                                    }
                                                    
                                                    ToolButton {
                                                        anchors.right: parent.right
                                                        anchors.verticalCenter: parent.verticalCenter
                                                        icon.name: "content-copy"
                                                        enabled: packageSwitch.checked
                                                        onClicked: clipboard.setText(model.installCommand)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            Tab {
                title: "Performance Tuning"
                
                ScrollView {
                    anchors.fill: parent
                    contentWidth: availableWidth
                    
                    Column {
                        width: parent.width
                        spacing: 12
                        
                        // Performance presets
                        Card {
                            width: parent.width
                            Material.elevation: 1
                            
                            Column {
                                anchors.fill: parent
                                anchors.margins: 16
                                spacing: 12
                                
                                Label {
                                    text: "Performance Presets"
                                    font.weight: Font.Bold
                                    color: Material.primary
                                }
                                
                                ButtonGroup {
                                    id: presetGroup
                                }
                                
                                Row {
                                    spacing: 12
                                    
                                    Repeater {
                                        model: ["Balanced", "Performance", "Power Saving", "Gaming", "Workstation"]
                                        
                                        RadioButton {
                                            text: model
                                            ButtonGroup.group: presetGroup
                                            checked: index === 0
                                            onToggled: configManager.applyPreset(model)
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Individual tuning options
                        Repeater {
                            model: configManager.performanceOptions
                            
                            Card {
                                width: parent.width
                                Material.elevation: 1
                                
                                Column {
                                    anchors.fill: parent
                                    anchors.margins: 16
                                    spacing: 8
                                    
                                    Row {
                                        width: parent.width
                                        
                                        Column {
                                            width: parent.width * 0.7
                                            
                                            Label {
                                                text: model.name
                                                font.weight: Font.Medium
                                            }
                                            
                                            Label {
                                                text: model.description
                                                color: "#666"
                                                wrapMode: Text.WordWrap
                                                width: parent.width
                                            }
                                        }
                                        
                                        Item { Layout.fillWidth: true }
                                        
                                        // Control based on option type
                                        Loader {
                                            sourceComponent: {
                                                switch(model.type) {
                                                    case "boolean": return switchComponent
                                                    case "range": return sliderComponent
                                                    case "choice": return comboComponent
                                                    default: return switchComponent
                                                }
                                            }
                                            
                                            property var optionModel: model
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Component definitions for different control types
    Component {
        id: switchComponent
        Switch {
            checked: optionModel.value
            onToggled: optionModel.value = checked
        }
    }
    
    Component {
        id: sliderComponent
        Column {
            spacing: 4
            
            Slider {
                width: 150
                from: optionModel.min
                to: optionModel.max
                value: optionModel.value
                stepSize: optionModel.step
                onValueChanged: optionModel.value = value
            }
            
            Label {
                text: optionModel.value + " " + optionModel.unit
                font.pixelSize: 12
                color: "#666"
                anchors.horizontalCenter: parent.horizontalCenter
            }
        }
    }
    
    Component {
        id: comboComponent
        ComboBox {
            width: 150
            model: optionModel.choices
            currentIndex: optionModel.selectedIndex
            onCurrentIndexChanged: optionModel.selectedIndex = currentIndex
        }
    }
}
```

### Driver Recommendation Card Component
```qml
// DriverRecommendationCard.qml
Card {
    property var driver
    property bool priority: false
    property bool optional: false
    
    height: driverContent.height + 24
    Material.elevation: priority ? 2 : 1
    Material.background: priority ? "#FFF3E0" : "white"
    border.color: priority ? "#FF9800" : "transparent"
    border.width: priority ? 1 : 0
    
    Column {
        id: driverContent
        anchors.fill: parent
        anchors.margins: 12
        spacing: 8
        
        Row {
            width: parent.width
            spacing: 12
            
            Rectangle {
                width: 40
                height: 40
                radius: 8
                color: driver.getStatusColor()
                
                Icon {
                    anchors.centerIn: parent
                    name: driver.getIcon()
                    color: "white"
                }
            }
            
            Column {
                width: parent.width * 0.6
                anchors.verticalCenter: parent.verticalCenter
                
                Label {
                    text: driver.displayName
                    font.weight: Font.Medium
                }
                
                Label {
                    text: driver.description
                    color: "#666"
                    font.pixelSize: 12
                    wrapMode: Text.WordWrap
                    width: parent.width
                }
                
                Row {
                    spacing: 4
                    visible: driver.devices.length > 0
                    
                    Icon {
                        name: "devices"
                        size: 12
                        color: "#666"
                    }
                    
                    Label {
                        text: driver.devices.length + " devices"
                        font.pixelSize: 12
                        color: "#666"
                    }
                }
            }
            
            Item { Layout.fillWidth: true }
            
            Column {
                anchors.verticalCenter: parent.verticalCenter
                spacing: 4
                
                Row {
                    spacing: 4
                    
                    Chip {
                        text: driver.source
                        Material.background: driver.getSourceColor()
                        textColor: "white"
                        small: true
                    }
                    
                    Chip {
                        text: driver.license
                        Material.background: "#9E9E9E"
                        textColor: "white"
                        small: true
                        visible: driver.license !== ""
                    }
                }
                
                Row {
                    spacing: 8
                    
                    Button {
                        text: "Install"
                        icon.name: "download"
                        Material.background: "#4CAF50"
                        enabled: driver.installable
                        onClicked: driverManager.install(driver.id)
                        height: 32
                        font.pixelSize: 12
                    }
                    
                    Button {
                        text: "Info"
                        icon.name: "info"
                        flat: true
                        onClicked: driverInfoDialog.open(driver)
                        height: 32
                        font.pixelSize: 12
                    }
                }
            }
        }
        
        // Installation command preview
        Rectangle {
            width: parent.width
            height: 28
            color: "#F5F5F5"
            radius: 4
            border.color: "#E0E0E0"
            visible: driver.installCommand !== ""
            
            Row {
                anchors.fill: parent
                anchors.margins: 8
                spacing: 8
                
                Label {
                    text: driver.installCommand
                    font.family: "monospace"
                    font.pixelSize: 11
                    color: "#333"
                    anchors.verticalCenter: parent.verticalCenter
                }
                
                Item { Layout.fillWidth: true }
                
                ToolButton {
                    icon.name: "content-copy"
                    height: 20
                    width: 20
                    onClicked: clipboard.setText(driver.installCommand)
                }
            }
        }
    }
}
```

### Key Features for Driver Recommendations
- **Priority-based display**: Critical drivers highlighted with visual urgency
- **Installation preview**: Command preview with copy-to-clipboard functionality  
- **Source transparency**: Clear indication of driver source (official/community/proprietary)
- **Device mapping**: Shows which devices benefit from each driver
- **Action-oriented**: Direct installation buttons with progress feedback

---

## Design Implementation Summary

This comprehensive Qt6 QML interface design delivers a professional hardware detection application with the following key strengths:

### **Material Design 3 Implementation**
- **Consistent theming**: Privacy-focused color palette with proper elevation and spacing
- **Component library**: Custom Chip, Icon, and CircularProgressIndicator components
- **Responsive layout**: Navigation rail that adapts to screen sizes
- **Smooth animations**: Page transitions and micro-interactions following Material guidelines

### **Privacy-First User Experience**
- **Transparent data handling**: Clear privacy level indicators throughout the interface
- **User empowerment**: Granular controls over data collection and sharing
- **Trust building**: Anonymization preview and detailed privacy explanations
- **Compliance ready**: GDPR-compliant data export and deletion features

### **Professional Hardware Tool Features**
- **Comprehensive detection**: Real-time progress tracking for lshw, dmidecode, lspci, lsusb, inxi
- **Expert configuration**: Driver recommendations, kernel parameters, package installation
- **Community integration**: Seamless GitHub submission workflow with history tracking
- **Technical depth**: Detailed device information with compatibility assessment

### **Accessibility and Usability**
- **Keyboard navigation**: Full keyboard accessibility support throughout
- **Screen reader support**: Proper semantic roles and descriptions
- **Progressive disclosure**: Complex features revealed as needed
- **Error prevention**: Clear validation and helpful guidance

### **Implementation Architecture**
- **Modular design**: Reusable components with consistent API patterns
- **Responsive components**: Adaptive layouts for different screen sizes
- **Performance optimized**: Efficient rendering with proper state management
- **Extensible structure**: Easy to add new features and detection tools

This Qt6 QML design successfully translates the GTK4 functionality into a modern Material Design 3 interface while enhancing the privacy-first approach and community collaboration features that make lx-hw-db unique in the hardware compatibility space.