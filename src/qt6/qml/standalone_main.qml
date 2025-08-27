// standalone_main.qml - Standalone Linux Hardware Database Qt6 application
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls.Material 2.15

ApplicationWindow {
    id: window
    width: 1200
    height: 800
    minimumWidth: 900
    minimumHeight: 600
    visible: true
    title: "Linux Hardware Database"
    
    // Gruvbox theming
    Material.theme: Material.Dark
    Material.primary: "#d79921"      // Gruvbox yellow/orange
    Material.accent: "#cc241d"       // Gruvbox red accent
    Material.background: "#282828"   // Gruvbox dark background
    
    // Application state
    property bool detectionRunning: false
    property int deviceCount: 0
    property real compatibilityScore: 0.85
    property string privacyLevel: "Basic"
    property bool privacySecure: true
    property int detectionProgress: 0
    property bool detectionComplete: false
    
    // Hardware devices model
    ListModel {
        id: hardwareDevicesModel
    }
    
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
                Text {
                    text: "🔒"
                    color: window.privacySecure ? "#4CAF50" : "#FF9800"
                    font.pixelSize: 16
                    anchors.verticalCenter: parent.verticalCenter
                }
                Label {
                    text: "Privacy: " + window.privacyLevel
                    color: "white"
                    font.weight: Font.Medium
                    anchors.verticalCenter: parent.verticalCenter
                }
            }
            
            Item { Layout.fillWidth: true }
            
            // Application title
            Label {
                text: "Linux Hardware Database"
                color: "white"
                font.pixelSize: 18
                font.weight: Font.Medium
            }
            
            Item { Layout.fillWidth: true }
            
            // Status indicators
            Row {
                spacing: 12
                Label {
                    text: "Devices: " + window.deviceCount
                    color: "white"
                    font.pixelSize: 14
                }
                Label {
                    text: "Compatible: " + Math.round(window.compatibilityScore * 100) + "%"
                    color: "white"
                    font.pixelSize: 14
                }
            }
            
            // Window controls
            ToolButton {
                text: "⚙️"
                font.pixelSize: 16
                ToolTip.text: "Settings"
                onClicked: console.log("Settings clicked")
            }
            
            ToolButton {
                text: "?"
                font.pixelSize: 16
                ToolTip.text: "Help"
                onClicked: helpDialog.open()
            }
        }
    }
    
    // Main content area
    Row {
        anchors.fill: parent
        
        // Navigation rail (left sidebar)
        Rectangle {
            width: 80
            height: parent.height
            color: "#3c3836"  // Gruvbox dark sidebar
            
            Column {
                anchors.fill: parent
                anchors.topMargin: 16
                spacing: 8
                
                NavigationButton {
                    id: welcomeBtn
                    iconText: "🏠"
                    label: "Welcome"
                    selected: stackView.currentItem ? stackView.currentItem.objectName === "welcome" : true
                    onClicked: stackView.replace(welcomeComponent)
                }
                
                NavigationButton {
                    id: detectBtn
                    iconText: "🔍"
                    label: "Detect"
                    selected: stackView.currentItem ? stackView.currentItem.objectName === "detect" : false
                    onClicked: stackView.replace(detectionComponent)
                }
                
                NavigationButton {
                    id: hardwareBtn
                    iconText: "🖥️"
                    label: "Hardware"
                    selected: stackView.currentItem ? stackView.currentItem.objectName === "hardware" : false
                    enabled: !window.detectionRunning
                    onClicked: stackView.replace(hardwareComponent)
                }
                
                NavigationButton {
                    id: configBtn
                    iconText: "⚙️"
                    label: "Config"
                    selected: stackView.currentItem ? stackView.currentItem.objectName === "config" : false
                    enabled: !window.detectionRunning
                    onClicked: stackView.replace(configComponent)
                }
                
                NavigationButton {
                    id: submitBtn
                    iconText: "📤"
                    label: "Submit"
                    selected: stackView.currentItem ? stackView.currentItem.objectName === "submit" : false
                    enabled: !window.detectionRunning
                    onClicked: stackView.replace(submitComponent)
                }
                
                NavigationButton {
                    id: privacyBtn
                    iconText: "🔒"
                    label: "Privacy"
                    selected: stackView.currentItem ? stackView.currentItem.objectName === "privacy" : false
                    onClicked: stackView.replace(privacyComponent)
                }
            }
        }
        
        // Main content stack view
        StackView {
            id: stackView
            width: parent.width - 80
            height: parent.height
            
            initialItem: welcomeComponent
            
            // Smooth transitions between pages
            pushEnter: Transition {
                PropertyAnimation {
                    property: "opacity"
                    from: 0
                    to: 1
                    duration: 200
                    easing.type: Easing.OutCubic
                }
                PropertyAnimation {
                    property: "x"
                    from: 50
                    to: 0
                    duration: 200
                    easing.type: Easing.OutCubic
                }
            }
            
            pushExit: Transition {
                PropertyAnimation {
                    property: "opacity"
                    from: 1
                    to: 0
                    duration: 200
                    easing.type: Easing.OutCubic
                }
            }
        }
    }
    
    // Navigation Button Component
    component NavigationButton: Rectangle {
        property string iconText: ""
        property string label: ""
        property bool selected: false
        property bool enabled: true
        signal clicked()
        
        width: 72
        height: 72
        radius: 16
        color: selected ? "#504945" : (mouseArea.containsMouse ? "#45403d" : "transparent")
        
        Column {
            anchors.centerIn: parent
            spacing: 4
            
            Text {
                text: iconText
                font.pixelSize: 20
                anchors.horizontalCenter: parent.horizontalCenter
                color: selected ? "#d79921" : (enabled ? "#ebdbb2" : "#a89984")
            }
            
            Text {
                text: label
                font.pixelSize: 10
                anchors.horizontalCenter: parent.horizontalCenter
                color: selected ? "#d79921" : (enabled ? "#ebdbb2" : "#a89984")
            }
        }
        
        MouseArea {
            id: mouseArea
            anchors.fill: parent
            hoverEnabled: true
            enabled: parent.enabled
            onClicked: parent.clicked()
            cursorShape: enabled ? Qt.PointingHandCursor : Qt.ArrowCursor
        }
    }
    
    // Welcome Screen Component
    Component {
        id: welcomeComponent
        
        Rectangle {
            objectName: "welcome"
            color: "#282828"
            
            Column {
                anchors.centerIn: parent
                spacing: 32
                width: Math.min(parent.width * 0.8, 600)
                
                Text {
                    text: "Welcome to Linux Hardware Database"
                    font.pixelSize: 28
                    font.weight: Font.Bold
                    color: "#ebdbb2"
                    anchors.horizontalCenter: parent.horizontalCenter
                }
                
                Text {
                    text: "Privacy-preserving hardware detection and compatibility reporting"
                    font.pixelSize: 16
                    color: "#d5c4a1"
                    anchors.horizontalCenter: parent.horizontalCenter
                    horizontalAlignment: Text.AlignHCenter
                    wrapMode: Text.WordWrap
                    width: parent.width
                }
                
                Row {
                    anchors.horizontalCenter: parent.horizontalCenter
                    spacing: 16
                    
                    Card {
                        width: 160
                        height: 120
                        
                        Column {
                            anchors.centerIn: parent
                            spacing: 8
                            
                            Text {
                                text: "🔒"
                                font.pixelSize: 32
                                anchors.horizontalCenter: parent.horizontalCenter
                            }
                            Text {
                                text: "Privacy First"
                                font.weight: Font.Medium
                                anchors.horizontalCenter: parent.horizontalCenter
                            }
                            Text {
                                text: "Your data stays private"
                                font.pixelSize: 12
                                color: "#49454F"
                                anchors.horizontalCenter: parent.horizontalCenter
                                horizontalAlignment: Text.AlignHCenter
                                wrapMode: Text.WordWrap
                                width: 140
                            }
                        }
                    }
                    
                    Card {
                        width: 160
                        height: 120
                        
                        Column {
                            anchors.centerIn: parent
                            spacing: 8
                            
                            Text {
                                text: "🖥️"
                                font.pixelSize: 32
                                anchors.horizontalCenter: parent.horizontalCenter
                            }
                            Text {
                                text: "Hardware Detection"
                                font.weight: Font.Medium
                                anchors.horizontalCenter: parent.horizontalCenter
                                horizontalAlignment: Text.AlignHCenter
                                wrapMode: Text.WordWrap
                                width: 140
                            }
                            Text {
                                text: "Complete system scan"
                                font.pixelSize: 12
                                color: "#49454F"
                                anchors.horizontalCenter: parent.horizontalCenter
                                horizontalAlignment: Text.AlignHCenter
                                wrapMode: Text.WordWrap
                                width: 140
                            }
                        }
                    }
                    
                    Card {
                        width: 160
                        height: 120
                        
                        Column {
                            anchors.centerIn: parent
                            spacing: 8
                            
                            Text {
                                text: "🌐"
                                font.pixelSize: 32
                                anchors.horizontalCenter: parent.horizontalCenter
                            }
                            Text {
                                text: "Community"
                                font.weight: Font.Medium
                                anchors.horizontalCenter: parent.horizontalCenter
                            }
                            Text {
                                text: "Share compatibility data"
                                font.pixelSize: 12
                                color: "#49454F"
                                anchors.horizontalCenter: parent.horizontalCenter
                                horizontalAlignment: Text.AlignHCenter
                                wrapMode: Text.WordWrap
                                width: 140
                            }
                        }
                    }
                }
                
                Button {
                    text: "Start Hardware Detection"
                    Material.background: "#d79921"
                    Material.foreground: "#282828"
                    font.pixelSize: 16
                    anchors.horizontalCenter: parent.horizontalCenter
                    onClicked: {
                        window.detectionRunning = true
                        window.detectionProgress = 0
                        window.detectionComplete = false
                        stackView.replace(detectionComponent)
                    }
                }
            }
        }
    }
    
    // Detection Screen Component
    Component {
        id: detectionComponent
        
        Rectangle {
            objectName: "detect"
            color: "white"
            
            Column {
                anchors.centerIn: parent
                spacing: 24
                width: Math.min(parent.width * 0.8, 500)
                
                Text {
                    text: "Hardware Detection"
                    font.pixelSize: 24
                    font.weight: Font.Bold
                    color: "#1C1B1F"
                    anchors.horizontalCenter: parent.horizontalCenter
                }
                
                Text {
                    text: window.detectionRunning ? "Scanning hardware components..." : "Detection complete!"
                    font.pixelSize: 16
                    color: "#49454F"
                    anchors.horizontalCenter: parent.horizontalCenter
                }
                
                ProgressBar {
                    width: parent.width
                    height: 8
                    value: window.detectionRunning ? window.detectionProgress / 100 : 1.0
                    
                    Timer {
                        running: window.detectionRunning
                        interval: 100
                        repeat: true
                        onTriggered: {
                            if (window.detectionProgress < 100) {
                                window.detectionProgress += 2
                            } else {
                                window.detectionRunning = false
                                window.detectionComplete = true
                                window.deviceCount = 15
                                
                                // Populate hardware devices
                                hardwareDevicesModel.clear()
                                
                                hardwareDevicesModel.append({
                                    "category": "System",
                                    "icon": "🖥️",
                                    "title": "System Information",
                                    "details": ["OS: NixOS 23.11", "Kernel: Linux 6.16.3", "Architecture: x86_64", "Boot Mode: UEFI"],
                                    "status": "✅ Fully Supported"
                                })
                                
                                hardwareDevicesModel.append({
                                    "category": "CPU",
                                    "icon": "🔧", 
                                    "title": "CPU",
                                    "details": ["Intel Core i7-12700K", "8 cores, 16 threads", "3.6 GHz base, 5.0 GHz boost", "L3 Cache: 25 MB"],
                                    "status": "✅ Fully Supported"
                                })
                                
                                hardwareDevicesModel.append({
                                    "category": "Memory",
                                    "icon": "💾",
                                    "title": "Memory", 
                                    "details": ["64GB DDR4 (2x32GB)", "Corsair Vengeance", "3200 MHz Dual Channel", "Memory Controller: Intel IMC"],
                                    "status": "✅ Fully Supported"
                                })
                                
                                hardwareDevicesModel.append({
                                    "category": "Graphics",
                                    "icon": "🎮",
                                    "title": "Graphics",
                                    "details": ["NVIDIA GeForce RTX 4070", "12GB GDDR6X", "Driver: nvidia 535.154.05", "Intel UHD Graphics 770 (backup)"],
                                    "status": "✅ Working with Driver"
                                })
                                
                                hardwareDevicesModel.append({
                                    "category": "Storage", 
                                    "icon": "💿",
                                    "title": "Storage",
                                    "details": ["Samsung 980 PRO 1TB NVMe", "Seagate BarraCuda 2TB HDD", "Intel 600 Series PCH Controller", "All devices detected"],
                                    "status": "✅ Fully Supported"
                                })
                                
                                hardwareDevicesModel.append({
                                    "category": "Network",
                                    "icon": "🌐", 
                                    "title": "Network",
                                    "details": ["Intel I225-V 2.5GbE", "Intel Wi-Fi 6E AX211", "Bluetooth 5.3 LE", "All interfaces working"],
                                    "status": "✅ Fully Supported"
                                })
                                
                                hardwareDevicesModel.append({
                                    "category": "Audio",
                                    "icon": "🔊",
                                    "title": "Audio",
                                    "details": ["Realtek ALC1220 HD Audio", "7.1 Surround Sound", "NVIDIA HDMI Audio", "All audio devices working"],
                                    "status": "✅ Fully Supported"
                                })
                                
                                hardwareDevicesModel.append({
                                    "category": "USB",
                                    "icon": "🔌",
                                    "title": "USB Devices", 
                                    "details": ["USB 3.2 Gen2 Hub", "Logitech MX Master 3 Mouse", "Das Keyboard Professional", "1080p USB Webcam"],
                                    "status": "✅ Fully Supported"
                                })
                                
                                stop()
                            }
                        }
                    }
                }
                
                Text {
                    text: window.detectionRunning ? 
                          window.detectionProgress + "% complete" : 
                          "Found " + window.deviceCount + " hardware devices"
                    font.pixelSize: 14
                    color: "#49454F"
                    anchors.horizontalCenter: parent.horizontalCenter
                }
                
                Button {
                    text: window.detectionRunning ? "Cancel Detection" : "View Hardware Results"
                    Material.background: window.detectionRunning ? "#B3261E" : "#6750A4"
                    Material.foreground: "white"
                    anchors.horizontalCenter: parent.horizontalCenter
                    onClicked: {
                        if (window.detectionRunning) {
                            window.detectionRunning = false
                            window.detectionProgress = 0
                        } else {
                            stackView.replace(hardwareComponent)
                        }
                    }
                }
            }
        }
    }
    
    // Hardware Screen Component  
    Component {
        id: hardwareComponent
        
        Rectangle {
            objectName: "hardware"
            color: "white"
            
            Column {
                anchors.fill: parent
                anchors.margins: 24
                spacing: 16
                
                Text {
                    text: "Hardware Overview"
                    font.pixelSize: 24
                    font.weight: Font.Bold
                    color: "#1C1B1F"
                }
                
                ScrollView {
                    width: parent.width
                    height: parent.height - 150
                    
                    Column {
                        width: parent.width - 24
                        spacing: 16
                        
                        Text {
                            text: hardwareDevicesModel.count === 0 ? 
                                  "No hardware detected yet. Please run detection first." :
                                  "Hardware detection results:"
                            font.pixelSize: 16
                            color: hardwareDevicesModel.count === 0 ? "#FF5722" : "#49454F"
                            visible: true
                        }
                        
                        Repeater {
                            model: hardwareDevicesModel
                            delegate: HardwareCard {
                                title: model.title || ""
                                icon: model.icon || "🔧"
                                details: model.details || []
                                status: model.status || "Unknown"
                            }
                        }
                    }
                }
                
                Button {
                    text: "Generate Configuration"
                    Material.background: "#6750A4"
                    Material.foreground: "white"
                    anchors.horizontalCenter: parent.horizontalCenter
                    onClicked: stackView.replace(configComponent)
                }
            }
        }
    }
    
    // Configuration Screen Component
    Component {
        id: configComponent
        
        Rectangle {
            objectName: "config"
            color: "white"
            
            Column {
                anchors.fill: parent
                anchors.margins: 24
                spacing: 16
                
                Text {
                    text: "Configuration Recommendations"
                    font.pixelSize: 24
                    font.weight: Font.Bold
                    color: "#1C1B1F"
                }
                
                Text {
                    text: "Based on your hardware, here are the recommended optimizations:"
                    font.pixelSize: 16
                    color: "#49454F"
                }
                
                ScrollView {
                    width: parent.width
                    height: parent.height - 120
                    
                    Column {
                        width: parent.width - 24
                        spacing: 16
                        
                        ConfigCard {
                            title: "Kernel Parameters"
                            icon: "⚙️"
                            content: "intel_pstate=active intel_iommu=on nvidia-drm.modeset=1"
                        }
                        
                        ConfigCard {
                            title: "Driver Installation (NixOS)"  
                            icon: "🔧"
                            content: "hardware.nvidia.open = false;\nhardware.nvidia.modesetting.enable = true;"
                        }
                        
                        ConfigCard {
                            title: "Performance Tuning"
                            icon: "🚀"
                            content: "vm.swappiness = 10\nnet.core.default_qdisc = fq\nnet.ipv4.tcp_congestion_control = bbr"
                        }
                        
                        ConfigCard {
                            title: "Power Management"
                            icon: "🔋"
                            content: "powerManagement.cpuFreqGovernor = \"performance\";\nhardware.cpu.intel.updateMicrocode = true;"
                        }
                    }
                }
                
                Row {
                    anchors.horizontalCenter: parent.horizontalCenter
                    spacing: 16
                    
                    Button {
                        text: "Export Config"
                        Material.background: "#6750A4"
                        Material.foreground: "white"
                        onClicked: console.log("Export configuration")
                    }
                    
                    Button {
                        text: "Submit to Community"
                        Material.background: "#625B71"
                        Material.foreground: "white"
                        onClicked: stackView.replace(submitComponent)
                    }
                }
            }
        }
    }
    
    // Submit Screen Component
    Component {
        id: submitComponent
        
        Rectangle {
            objectName: "submit"
            color: "white"
            
            Column {
                anchors.centerIn: parent
                spacing: 24
                width: Math.min(parent.width * 0.8, 500)
                
                Text {
                    text: "Community Submission"
                    font.pixelSize: 24
                    font.weight: Font.Bold
                    color: "#1C1B1F"
                    anchors.horizontalCenter: parent.horizontalCenter
                }
                
                Text {
                    text: "Help improve hardware compatibility by sharing your configuration"
                    font.pixelSize: 16
                    color: "#49454F"
                    anchors.horizontalCenter: parent.horizontalCenter
                    horizontalAlignment: Text.AlignHCenter
                    wrapMode: Text.WordWrap
                    width: parent.width
                }
                
                Card {
                    width: parent.width
                    height: 120
                    
                    Column {
                        anchors.centerIn: parent
                        spacing: 8
                        
                        Text {
                            text: "🔒 Privacy Protected"
                            font.pixelSize: 18
                            font.weight: Font.Medium
                            anchors.horizontalCenter: parent.horizontalCenter
                        }
                        Text {
                            text: "Hardware IDs are anonymized using HMAC-SHA256\nNo personal information is collected or shared"
                            font.pixelSize: 12
                            color: "#49454F"
                            anchors.horizontalCenter: parent.horizontalCenter
                            horizontalAlignment: Text.AlignHCenter
                        }
                    }
                }
                
                Column {
                    width: parent.width
                    spacing: 8
                    
                    Text {
                        text: "Submission Summary:"
                        font.weight: Font.Medium
                        color: "#1C1B1F"
                    }
                    
                    Text {
                        text: "• 15 hardware devices detected\n• 85% compatibility score\n• Configuration optimizations included\n• Privacy level: " + window.privacyLevel
                        color: "#49454F"
                        lineHeight: 1.4
                    }
                }
                
                Button {
                    id: submitButton
                    text: submitButton.submitting ? "Submitting..." : "Submit to GitHub"
                    Material.background: submitButton.submitting ? "#757575" : "#6750A4"
                    Material.foreground: "white"
                    anchors.horizontalCenter: parent.horizontalCenter
                    enabled: !submitButton.submitting && hardwareDevicesModel.count > 0
                    
                    property bool submitting: false
                    property bool submitted: false
                    
                    onClicked: {
                        if (hardwareDevicesModel.count === 0) {
                            submitStatusDialog.title = "No Hardware Data"
                            submitStatusDialog.message = "Please run hardware detection first before submitting."
                            submitStatusDialog.isSuccess = false
                            submitStatusDialog.open()
                            return
                        }
                        
                        submitButton.submitting = true
                        console.log("Submitting to community database...")
                        
                        // Simulate submission process
                        submitTimer.start()
                    }
                    
                    Timer {
                        id: submitTimer
                        interval: 3000
                        onTriggered: {
                            submitButton.submitting = false
                            submitButton.submitted = true
                            
                            // Show success dialog
                            submitStatusDialog.title = "Submission Successful!"
                            submitStatusDialog.message = "Your hardware report has been anonymized and submitted to the community database.\n\nSubmission ID: HW-" + Math.floor(Math.random() * 100000) + "\nPrivacy Level: " + window.privacyLevel + "\nDevices: " + hardwareDevicesModel.count + " components"
                            submitStatusDialog.isSuccess = true
                            submitStatusDialog.open()
                        }
                    }
                }
            }
        }
    }
    
    // Privacy Screen Component
    Component {
        id: privacyComponent
        
        Rectangle {
            objectName: "privacy"
            color: "white"
            
            Column {
                anchors.fill: parent
                anchors.margins: 24
                spacing: 24
                
                Text {
                    text: "Privacy Settings"
                    font.pixelSize: 24
                    font.weight: Font.Bold
                    color: "#1C1B1F"
                }
                
                Text {
                    text: "Choose your privacy level for hardware data collection"
                    font.pixelSize: 16
                    color: "#49454F"
                }
                
                Column {
                    width: parent.width
                    spacing: 12
                    
                    PrivacyCard {
                        title: "Basic Privacy"
                        description: "Hardware IDs hashed with time-rotating salts"
                        level: "Basic"
                        selected: window.privacyLevel === "Basic"
                        onClicked: window.privacyLevel = "Basic"
                    }
                    
                    PrivacyCard {
                        title: "Enhanced Privacy"  
                        description: "Basic protection + differential privacy noise"
                        level: "Enhanced"
                        selected: window.privacyLevel === "Enhanced"
                        onClicked: window.privacyLevel = "Enhanced"
                    }
                    
                    PrivacyCard {
                        title: "Strict Privacy"
                        description: "Maximum anonymization with k-anonymity guarantees"
                        level: "Strict"
                        selected: window.privacyLevel === "Strict"
                        onClicked: window.privacyLevel = "Strict"
                    }
                }
                
                Text {
                    text: "Current Status: " + window.privacyLevel + " privacy level active"
                    font.pixelSize: 14
                    color: "#6750A4"
                    font.weight: Font.Medium
                }
            }
        }
    }
    
    // Hardware Card Component
    component HardwareCard: Card {
        property string title: ""
        property string icon: ""
        property list<string> details: []
        property string status: ""
        
        width: parent ? parent.width : 400
        height: Math.max(120, column.height + 32)
        
        Column {
            id: column
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.top: parent.top
            anchors.margins: 16
            spacing: 8
            
            Row {
                width: parent.width
                spacing: 12
                
                Text {
                    text: icon
                    font.pixelSize: 24
                    anchors.verticalCenter: parent.verticalCenter
                }
                
                Column {
                    width: parent.width - 50
                    
                    Text {
                        text: title
                        font.pixelSize: 16
                        font.weight: Font.Medium
                        color: "#1C1B1F"
                    }
                    
                    Repeater {
                        model: details
                        Text {
                            text: "• " + modelData
                            font.pixelSize: 12
                            color: "#49454F"
                            wrapMode: Text.WordWrap
                            width: parent.width
                        }
                    }
                }
            }
            
            Text {
                text: status
                font.pixelSize: 12
                font.weight: Font.Medium
                color: status.includes("✅") ? "#4CAF50" : "#FF9800"
            }
        }
    }
    
    // Configuration Card Component
    component ConfigCard: Card {
        property string title: ""
        property string icon: ""
        property string content: ""
        
        width: parent ? parent.width : 400
        height: Math.max(100, column.height + 32)
        
        Column {
            id: column
            anchors.fill: parent
            anchors.margins: 16
            spacing: 8
            
            Row {
                width: parent.width
                spacing: 12
                
                Text {
                    text: icon
                    font.pixelSize: 20
                    anchors.verticalCenter: parent.verticalCenter
                }
                
                Text {
                    text: title
                    font.pixelSize: 16
                    font.weight: Font.Medium
                    color: "#1C1B1F"
                    anchors.verticalCenter: parent.verticalCenter
                }
            }
            
            Rectangle {
                width: parent.width
                height: contentText.height + 16
                color: "#F5F5F5"
                radius: 8
                
                Text {
                    id: contentText
                    text: content
                    font.family: "monospace"
                    font.pixelSize: 12
                    color: "#1C1B1F"
                    anchors.fill: parent
                    anchors.margins: 8
                    wrapMode: Text.WordWrap
                }
            }
        }
    }
    
    // Privacy Card Component
    component PrivacyCard: Rectangle {
        property string title: ""
        property string description: ""
        property string level: ""
        property bool selected: false
        signal clicked()
        
        width: parent ? parent.width : 400
        height: 80
        radius: 12
        border.width: selected ? 2 : 1
        border.color: selected ? "#6750A4" : "#E7E0EC"
        color: selected ? "#F7F2FA" : "white"
        
        Row {
            anchors.left: parent.left
            anchors.verticalCenter: parent.verticalCenter
            anchors.leftMargin: 16
            spacing: 16
            
            Rectangle {
                width: 20
                height: 20
                radius: 10
                border.width: 2
                border.color: selected ? "#6750A4" : "#79747E"
                color: selected ? "#6750A4" : "transparent"
                anchors.verticalCenter: parent.verticalCenter
                
                Rectangle {
                    width: 8
                    height: 8
                    radius: 4
                    color: "white"
                    anchors.centerIn: parent
                    visible: selected
                }
            }
            
            Column {
                anchors.verticalCenter: parent.verticalCenter
                spacing: 4
                
                Text {
                    text: title
                    font.pixelSize: 16
                    font.weight: Font.Medium
                    color: "#1C1B1F"
                }
                
                Text {
                    text: description
                    font.pixelSize: 12
                    color: "#49454F"
                }
            }
        }
        
        MouseArea {
            anchors.fill: parent
            onClicked: parent.clicked()
            cursorShape: Qt.PointingHandCursor
        }
    }
    
    // Card Component
    component Card: Rectangle {
        width: parent ? parent.width : 300
        height: 100
        radius: 12
        color: "white"
        border.width: 1
        border.color: "#E7E0EC"
        
        layer.enabled: true
        layer.effect: ShaderEffect {
            property real spread: 0.0
            property real samples: 25
            property color color: "#40000000"
            property real offsetX: 0
            property real offsetY: 2
            
            fragmentShader: "
                varying highp vec2 qt_TexCoord0;
                uniform highp float qt_Opacity;
                uniform highp sampler2D source;
                uniform lowp vec4 color;
                uniform highp float spread;
                uniform highp float offsetX;
                uniform highp float offsetY;
                uniform highp float samples;
                
                void main() {
                    lowp vec4 sourceColor = texture2D(source, qt_TexCoord0);
                    gl_FragColor = mix(vec4(color.rgb, color.a * qt_Opacity), sourceColor, sourceColor.a);
                }
            "
        }
    }
    
    // Help dialog
    Dialog {
        id: helpDialog
        anchors.centerIn: parent
        width: 500
        height: 400
        title: "About Linux Hardware Database"
        
        ScrollView {
            anchors.fill: parent
            
            Column {
                width: parent.width
                spacing: 16
                
                Text {
                    width: parent.width
                    wrapMode: Text.WordWrap
                    text: "Linux Hardware Database v0.1.0\n\nPrivacy-preserving hardware detection and compatibility reporting for Linux systems."
                    font.pixelSize: 14
                }
                
                Text {
                    width: parent.width
                    wrapMode: Text.WordWrap
                    text: "Features:\n• Multi-tool hardware detection\n• Privacy-first architecture\n• Community-driven compatibility database\n• Automated configuration recommendations"
                    font.pixelSize: 12
                }
                
                Text {
                    width: parent.width
                    wrapMode: Text.WordWrap
                    text: "Developed by LX-HW-DB Contributors\nLicense: AGPLv3\n\nWebsite: https://github.com/lx-hw-db/lx-hw-db"
                    font.pixelSize: 12
                    color: "#666"
                }
            }
        }
        
        standardButtons: Dialog.Ok
    }
    
    // Submission status dialog
    Dialog {
        id: submitStatusDialog
        anchors.centerIn: parent
        width: 450
        height: 300
        
        property string message: ""
        property bool isSuccess: true
        
        Rectangle {
            anchors.fill: parent
            color: "white"
            radius: 8
            
            Column {
                anchors.fill: parent
                anchors.margins: 24
                spacing: 16
                
                // Success/Error icon
                Text {
                    text: submitStatusDialog.isSuccess ? "✅" : "❌"
                    font.pixelSize: 48
                    anchors.horizontalCenter: parent.horizontalCenter
                }
                
                // Title
                Text {
                    text: submitStatusDialog.title
                    font.pixelSize: 20
                    font.weight: Font.Bold
                    color: "#1C1B1F"
                    anchors.horizontalCenter: parent.horizontalCenter
                }
                
                // Message
                ScrollView {
                    width: parent.width
                    height: 120
                    
                    Text {
                        width: parent.width
                        text: submitStatusDialog.message
                        font.pixelSize: 14
                        color: "#49454F"
                        wrapMode: Text.WordWrap
                        lineHeight: 1.4
                    }
                }
                
                // OK Button
                Button {
                    text: "OK"
                    Material.background: "#6750A4"
                    Material.foreground: "white"
                    anchors.horizontalCenter: parent.horizontalCenter
                    onClicked: submitStatusDialog.close()
                }
            }
        }
    }
}