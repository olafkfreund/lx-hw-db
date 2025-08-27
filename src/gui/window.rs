//! Main application window implementation

use gtk4::prelude::*;
use libadwaita as adw;
use adw::prelude::*;
use gio::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::gui::{
    models::{SharedAppState, create_app_state},
    widgets::{DetectionProgress},
    utils::DetectionController,
};

/// Navigation pages in the sidebar
#[derive(Debug, Clone, Copy, PartialEq)]
enum NavigationPage {
    Welcome,
    Detection,
    Hardware,
    Configuration,
    Export,
}

/// Main application window
pub struct MainWindow {
    window: adw::ApplicationWindow,
    state: SharedAppState,
    view_stack: adw::ViewStack,
    sidebar: gtk4::ListBox,
    status_bar: gtk4::Box,
    privacy_indicator: gtk4::Label,
    detection_controller: Rc<RefCell<Option<DetectionController>>>,
}

impl MainWindow {
    /// Create a new main window
    pub fn new(app: &adw::Application) -> MainWindow {
        let state = create_app_state();
        
        // Create main window
        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title(&crate::gui::t("Linux Hardware Database"))
            .default_width(1000)
            .default_height(700)
            .build();

        // Create main layout
        let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        window.set_content(Some(&main_box));

        // Create header bar
        let header_bar = Self::create_header_bar(&state);
        main_box.append(&header_bar);

        // Create main content area with sidebar
        let paned = gtk4::Paned::new(gtk4::Orientation::Horizontal);
        main_box.append(&paned);

        // Create sidebar
        let sidebar_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        sidebar_box.set_width_request(250);
        sidebar_box.add_css_class("sidebar");
        
        let sidebar = Self::create_sidebar();
        sidebar_box.append(&sidebar);
        
        let sidebar_scrolled = gtk4::ScrolledWindow::builder()
            .child(&sidebar_box)
            .vexpand(true)
            .build();
        
        paned.set_start_child(Some(&sidebar_scrolled));

        // Create main content view stack
        let view_stack = adw::ViewStack::new();
        paned.set_end_child(Some(&view_stack));

        // Create status bar
        let status_bar = Self::create_status_bar(&state);
        main_box.append(&status_bar);

        // Set up pages
        Self::setup_pages(&view_stack, &state);

        // Set up sidebar navigation
        Self::setup_sidebar_navigation(&sidebar, &view_stack);

        // Create window instance
        let main_window = MainWindow {
            window: window.clone(),
            state: state.clone(),
            view_stack,
            sidebar,
            status_bar: status_bar.clone(),
            privacy_indicator: gtk4::Label::new(Some("Privacy: Basic")),
            detection_controller: Rc::new(RefCell::new(None)),
        };

        // Set up state change handlers
        main_window.setup_state_handlers();

        main_window
    }

    /// Create the header bar
    fn create_header_bar(state: &SharedAppState) -> adw::HeaderBar {
        let header_bar = adw::HeaderBar::new();
        
        // Title
        let title = gtk4::Label::new(Some(&crate::gui::t("Hardware Detector")));
        title.add_css_class("title");
        header_bar.set_title_widget(Some(&title));

        // Privacy level indicator in header
        let privacy_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
        
        let privacy_icon = gtk4::Image::from_icon_name("security-medium-symbolic");
        privacy_icon.set_pixel_size(16);
        privacy_box.append(&privacy_icon);

        let privacy_label = {
            let state = state.lock().unwrap();
            gtk4::Label::new(Some(&format!("Privacy: {:?}", state.privacy_level)))
        };
        privacy_label.add_css_class("privacy-indicator");
        privacy_box.append(&privacy_label);

        header_bar.pack_end(&privacy_box);

        // Menu button
        let menu_button = gtk4::MenuButton::new();
        menu_button.set_icon_name("open-menu-symbolic");
        
        let menu = gio::Menu::new();
        menu.append(Some(&crate::gui::t("_Preferences")), Some("app.preferences"));
        menu.append(Some(&crate::gui::t("_About")), Some("app.about"));
        menu_button.set_menu_model(Some(&menu));
        
        header_bar.pack_end(&menu_button);

        header_bar
    }

    /// Create the sidebar navigation
    fn create_sidebar() -> gtk4::ListBox {
        let sidebar = gtk4::ListBox::new();
        sidebar.add_css_class("navigation-sidebar");
        sidebar.set_selection_mode(gtk4::SelectionMode::Single);

        // Navigation items
        let nav_items = [
            ("welcome", "🏠", "Welcome", "Start here"),
            ("detection", "🔍", "Detection", "Detect hardware"),
            ("hardware", "🖥️", "Hardware", "View detected devices"),
            ("configuration", "⚙️", "Configuration", "System recommendations"),
            ("export", "📤", "Export", "Export and submit"),
        ];

        for (id, icon, title, subtitle) in nav_items.iter() {
            let row = adw::ActionRow::new();
            row.set_title(&crate::gui::t(title));
            row.set_subtitle(&crate::gui::t(subtitle));
            
            // Icon
            let icon_widget = gtk4::Label::new(Some(icon));
            icon_widget.set_width_request(32);
            row.add_prefix(&icon_widget);
            
            // Store page ID using widget name
            row.set_widget_name(id);
            
            sidebar.append(&row);
        }

        // Select welcome page by default
        sidebar.select_row(Some(&sidebar.row_at_index(0).unwrap()));

        sidebar
    }

    /// Create the status bar
    fn create_status_bar(state: &SharedAppState) -> gtk4::Box {
        let status_bar = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
        status_bar.set_margin_top(6);
        status_bar.set_margin_bottom(6);
        status_bar.set_margin_start(12);
        status_bar.set_margin_end(12);
        status_bar.add_css_class("statusbar");

        // Status message
        let status_label = {
            let state = state.lock().unwrap();
            gtk4::Label::new(Some(&state.detection_status))
        };
        status_label.set_halign(gtk4::Align::Start);
        status_bar.append(&status_label);

        // Spacer
        let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        spacer.set_hexpand(true);
        status_bar.append(&spacer);

        // Tools available indicator
        let tools_label = gtk4::Label::new(Some(&crate::gui::t("5 detection tools available")));
        tools_label.add_css_class("dim-label");
        status_bar.append(&tools_label);

        // Separator
        let separator = gtk4::Separator::new(gtk4::Orientation::Vertical);
        status_bar.append(&separator);

        // Privacy indicator
        let privacy_label = {
            let state = state.lock().unwrap();
            gtk4::Label::new(Some(&format!("Privacy: {:?}", state.privacy_level)))
        };
        privacy_label.add_css_class("privacy-status");
        status_bar.append(&privacy_label);

        status_bar
    }

    /// Set up all pages in the view stack
    fn setup_pages(view_stack: &adw::ViewStack, state: &SharedAppState) {
        // Welcome page
        let welcome_page = Self::create_welcome_page(state);
        view_stack.add_titled_with_icon(
            &welcome_page,
            Some("welcome"),
            &crate::gui::t("Welcome"),
            "go-home-symbolic",
        );

        // Detection page
        let detection_page = Self::create_detection_page(state);
        view_stack.add_titled_with_icon(
            &detection_page,
            Some("detection"),
            &crate::gui::t("Detection"),
            "system-search-symbolic",
        );

        // Hardware page
        let hardware_page = Self::create_hardware_page(state);
        view_stack.add_titled_with_icon(
            &hardware_page,
            Some("hardware"),
            &crate::gui::t("Hardware"),
            "computer-symbolic",
        );

        // Configuration page
        let config_page = Self::create_configuration_page(state);
        view_stack.add_titled_with_icon(
            &config_page,
            Some("configuration"),
            &crate::gui::t("Configuration"),
            "preferences-system-symbolic",
        );

        // Export page
        let export_page = Self::create_export_page(state);
        view_stack.add_titled_with_icon(
            &export_page,
            Some("export"),
            &crate::gui::t("Export"),
            "document-save-symbolic",
        );

        // Set welcome as visible page
        view_stack.set_visible_child_name("welcome");
    }

    /// Create the welcome page
    fn create_welcome_page(_state: &SharedAppState) -> gtk4::Box {
        let page = gtk4::Box::new(gtk4::Orientation::Vertical, 24);
        page.set_valign(gtk4::Align::Center);
        page.set_halign(gtk4::Align::Center);
        page.set_margin_top(48);
        page.set_margin_bottom(48);
        page.set_margin_start(48);
        page.set_margin_end(48);

        // Welcome header
        let header = gtk4::Label::new(Some(&crate::gui::t("Welcome to Linux Hardware Detector")));
        header.add_css_class("title-1");
        page.append(&header);

        let description = gtk4::Label::new(Some(&crate::gui::t(
            "Privacy-preserving hardware detection and compatibility analysis for Linux systems"
        )));
        description.add_css_class("subtitle");
        description.set_wrap(true);
        description.set_justify(gtk4::Justification::Center);
        page.append(&description);

        // Action buttons
        let button_box = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
        button_box.set_width_request(300);

        let detect_button = gtk4::Button::with_label(&crate::gui::t("🔍 Start Hardware Detection"));
        detect_button.add_css_class("suggested-action");
        detect_button.add_css_class("pill");
        detect_button.set_size_request(-1, 48);
        button_box.append(&detect_button);

        let load_button = gtk4::Button::with_label(&crate::gui::t("📄 Load Existing Report"));
        load_button.add_css_class("pill");
        load_button.set_size_request(-1, 48);
        button_box.append(&load_button);

        let privacy_button = gtk4::Button::with_label(&crate::gui::t("⚙️ Configure Privacy Settings"));
        privacy_button.add_css_class("pill");
        privacy_button.set_size_request(-1, 48);
        button_box.append(&privacy_button);

        page.append(&button_box);

        // Privacy notice
        let privacy_notice = gtk4::Label::new(Some(&crate::gui::t(
            "🔒 All hardware identifiers are cryptographically anonymized with rotating salts. \
             No personal information is collected or stored."
        )));
        privacy_notice.add_css_class("dim-label");
        privacy_notice.set_wrap(true);
        privacy_notice.set_justify(gtk4::Justification::Center);
        privacy_notice.set_margin_top(24);
        page.append(&privacy_notice);

        page
    }

    /// Create the detection page
    fn create_detection_page(_state: &SharedAppState) -> gtk4::Box {
        let page = gtk4::Box::new(gtk4::Orientation::Vertical, 24);
        page.set_margin_top(24);
        page.set_margin_bottom(24);
        page.set_margin_start(24);
        page.set_margin_end(24);

        // Page header
        let header = gtk4::Label::new(Some(&crate::gui::t("Hardware Detection")));
        header.add_css_class("title-2");
        header.set_halign(gtk4::Align::Start);
        page.append(&header);

        // Detection progress widget (initially hidden)
        let progress_widget = DetectionProgress::new();
        progress_widget.widget().set_visible(false);
        page.append(progress_widget.widget());

        // Detection controls
        let controls_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
        
        let start_button = gtk4::Button::with_label(&crate::gui::t("Start Detection"));
        start_button.add_css_class("suggested-action");
        controls_box.append(&start_button);

        let cancel_button = gtk4::Button::with_label(&crate::gui::t("Cancel"));
        cancel_button.set_sensitive(false);
        controls_box.append(&cancel_button);

        page.append(&controls_box);

        // Tool status
        let tools_group = adw::PreferencesGroup::new();
        tools_group.set_title(&crate::gui::t("Available Detection Tools"));
        tools_group.set_margin_top(24);

        // TODO: Add actual tool checking
        let tools = ["lshw", "dmidecode", "lspci", "lsusb", "inxi"];
        for tool in tools.iter() {
            let row = adw::ActionRow::new();
            row.set_title(tool);
            row.set_subtitle(&format!("{} - Available", tool));
            
            let status_icon = gtk4::Image::from_icon_name("emblem-ok-symbolic");
            row.add_suffix(&status_icon);
            
            tools_group.add(&row);
        }

        page.append(&tools_group);

        page
    }

    /// Create the hardware page
    fn create_hardware_page(_state: &SharedAppState) -> gtk4::ScrolledWindow {
        let scrolled = gtk4::ScrolledWindow::new();
        scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);

        let page = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        scrolled.set_child(Some(&page));

        // Initially show placeholder
        let placeholder = gtk4::Label::new(Some(&crate::gui::t(
            "No hardware data available. \nRun hardware detection first."
        )));
        placeholder.add_css_class("dim-label");
        placeholder.set_valign(gtk4::Align::Center);
        placeholder.set_margin_top(48);
        placeholder.set_margin_bottom(48);
        page.append(&placeholder);

        scrolled
    }

    /// Create the configuration page
    fn create_configuration_page(_state: &SharedAppState) -> gtk4::ScrolledWindow {
        let scrolled = gtk4::ScrolledWindow::new();
        
        let page = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        scrolled.set_child(Some(&page));

        // Placeholder
        let placeholder = gtk4::Label::new(Some(&crate::gui::t(
            "No configuration recommendations available. \nRun hardware detection first."
        )));
        placeholder.add_css_class("dim-label");
        placeholder.set_valign(gtk4::Align::Center);
        placeholder.set_margin_top(48);
        placeholder.set_margin_bottom(48);
        page.append(&placeholder);

        scrolled
    }

    /// Create the export page
    fn create_export_page(_state: &SharedAppState) -> gtk4::Box {
        let page = gtk4::Box::new(gtk4::Orientation::Vertical, 24);
        page.set_margin_top(24);
        page.set_margin_bottom(24);
        page.set_margin_start(24);
        page.set_margin_end(24);

        let header = gtk4::Label::new(Some(&crate::gui::t("Export & Submit")));
        header.add_css_class("title-2");
        header.set_halign(gtk4::Align::Start);
        page.append(&header);

        let placeholder = gtk4::Label::new(Some(&crate::gui::t(
            "Export functionality will be available after hardware detection."
        )));
        placeholder.add_css_class("dim-label");
        page.append(&placeholder);

        page
    }

    /// Set up sidebar navigation handlers
    fn setup_sidebar_navigation(sidebar: &gtk4::ListBox, view_stack: &adw::ViewStack) {
        let view_stack_clone = view_stack.clone();
        
        sidebar.connect_row_selected(move |_, row| {
            if let Some(row) = row {
                let page_id = row.widget_name();
                view_stack_clone.set_visible_child_name(&page_id);
            }
        });
    }

    /// Set up state change handlers
    fn setup_state_handlers(&self) {
        // TODO: Set up actual state change handlers using channels
        // This would involve setting up a glib::MainContext receiver
        // to handle state updates from background threads
    }

    /// Present the window
    pub fn present(&self) {
        self.window.present();
    }
}

/// Make MainWindow implement common traits for easier usage
impl AsRef<adw::ApplicationWindow> for MainWindow {
    fn as_ref(&self) -> &adw::ApplicationWindow {
        &self.window
    }
}