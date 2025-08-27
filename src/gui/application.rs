//! Main GTK4 application implementation

use gtk4::prelude::*;
use libadwaita as adw;
use adw::prelude::*;
use gio::prelude::*;
use crate::gui::window::MainWindow;

/// Application ID following reverse domain name convention
const APP_ID: &str = "org.lx_hw_db.HardwareDetector";

/// Main application struct
pub struct HardwareDetectorApp {
    app: adw::Application,
}

impl HardwareDetectorApp {
    /// Create a new application instance
    pub fn new() -> Self {
        let app = adw::Application::builder()
            .application_id(APP_ID)
            .build();

        let app_instance = Self { app };
        app_instance.setup_application();
        app_instance
    }

    /// Set up application callbacks and actions
    fn setup_application(&self) {
        self.app.connect_activate(|app| {
            Self::on_activate(app);
        });

        self.app.connect_startup(|app| {
            Self::on_startup(app);
        });

        // Set up application actions
        self.setup_actions();
    }

    /// Handle application activation
    fn on_activate(app: &adw::Application) {
        let window = MainWindow::new(app);
        window.present();
    }

    /// Handle application startup
    fn on_startup(app: &adw::Application) {
        // Load CSS styling
        Self::load_css();
        
        // Set up application menu
        Self::setup_app_menu(app);
    }

    /// Set up application-wide actions
    fn setup_actions(&self) {
        // About action
        let about_action = gio::ActionEntry::builder("about")
            .activate(|app: &adw::Application, _, _| {
                Self::show_about_dialog(app);
            })
            .build();

        // Preferences action
        let preferences_action = gio::ActionEntry::builder("preferences")
            .activate(|app: &adw::Application, _, _| {
                Self::show_preferences_dialog(app);
            })
            .build();

        // Quit action
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(|app: &adw::Application, _, _| {
                app.quit();
            })
            .build();

        self.app.add_action_entries([about_action, preferences_action, quit_action]);

        // Set up keyboard shortcuts
        self.app.set_accels_for_action("app.quit", &["<Ctrl>q"]);
        self.app.set_accels_for_action("app.preferences", &["<Ctrl>comma"]);
    }

    /// Load application CSS styling
    fn load_css() {
        let css_provider = gtk4::CssProvider::new();
        // For now, just load some basic CSS inline
        css_provider.load_from_data("
            .sidebar { background-color: @sidebar_bg_color; }
            .statusbar { border-top: 1px solid @borders; }
            .privacy-indicator { font-size: smaller; }
            .detection-progress { margin: 12px; }
            .status-supported { color: @success_color; }
            .status-partial { color: @warning_color; }
            .status-warning { color: @warning_color; }
            .status-error { color: @error_color; }
            .status-unknown { color: @dim_label; }
            .success { color: @success_color; }
            .error { color: @error_color; }
        ");
        
        if let Some(display) = gtk4::gdk::Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &css_provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    /// Set up application menu
    fn setup_app_menu(app: &adw::Application) {
        let menu = gio::Menu::new();
        
        menu.append(Some(&crate::gui::t("_Preferences")), Some("app.preferences"));
        menu.append(Some(&crate::gui::t("_About Hardware Detector")), Some("app.about"));
        menu.append(Some(&crate::gui::t("_Quit")), Some("app.quit"));

        app.set_menubar(Some(&menu));
    }

    /// Show about dialog
    fn show_about_dialog(app: &adw::Application) {
        let window = app.active_window().expect("No active window");
        
        let about = adw::AboutDialog::builder()
            .application_name(&crate::gui::t("Linux Hardware Database"))
            .application_icon(APP_ID)
            .developer_name("Linux Hardware Database Contributors")
            .version(env!("CARGO_PKG_VERSION"))
            .website("https://github.com/lx-hw-db/lx-hw-db")
            .issue_url("https://github.com/lx-hw-db/lx-hw-db/issues")
            .copyright("Copyright © 2025 Linux Hardware Database Project")
            .license_type(gtk4::License::Agpl30)
            .comments(&crate::gui::t("Privacy-preserving Linux hardware detection and compatibility reporting"))
            .build();

        about.add_credit_section(
            Some(&crate::gui::t("Contributors")),
            &["Linux Hardware Database Contributors"],
        );

        about.present(Some(&window));
    }

    /// Show preferences dialog
    fn show_preferences_dialog(app: &adw::Application) {
        let window = app.active_window().expect("No active window");
        
        // Create preferences window
        let prefs = adw::PreferencesDialog::new();
        prefs.set_title(&crate::gui::t("Preferences"));
        
        // Privacy page
        let privacy_page = Self::create_privacy_preferences_page();
        prefs.add(&privacy_page);
        
        // Detection tools page
        let tools_page = Self::create_tools_preferences_page();
        prefs.add(&tools_page);

        prefs.present(Some(&window));
    }

    /// Create privacy preferences page
    fn create_privacy_preferences_page() -> adw::PreferencesPage {
        let page = adw::PreferencesPage::builder()
            .title(&crate::gui::t("Privacy"))
            .icon_name("security-medium-symbolic")
            .build();

        // Privacy level group
        let privacy_group = adw::PreferencesGroup::builder()
            .title(&crate::gui::t("Privacy Level"))
            .description(&crate::gui::t("Configure how your hardware data is anonymized"))
            .build();

        // Privacy level selection
        let privacy_row = adw::ComboRow::builder()
            .title(&crate::gui::t("Anonymization Level"))
            .subtitle(&crate::gui::t("Higher levels provide better privacy but may reduce compatibility analysis accuracy"))
            .build();

        let privacy_model = gtk4::StringList::new(&[
            &crate::gui::t("Basic - 24 hour salt rotation"),
            &crate::gui::t("Enhanced - 12 hour salt rotation"),
            &crate::gui::t("Strict - 1 hour salt rotation"),
        ]);

        privacy_row.set_model(Some(&privacy_model));
        privacy_row.set_selected(0); // Default to basic

        privacy_group.add(&privacy_row);

        // Data collection group
        let data_group = adw::PreferencesGroup::builder()
            .title(&crate::gui::t("Data Collection"))
            .description(&crate::gui::t("Choose what information to include in reports"))
            .build();

        // Create switches for data collection options
        let options = [
            ("hardware_info", "Hardware Information", "Include vendor and model information"),
            ("kernel_info", "Kernel Compatibility", "Include kernel module and driver information"),
            ("performance_data", "Performance Data", "Include benchmark and performance metrics"),
        ];

        for (key, title, subtitle) in options.iter() {
            let switch_row = adw::SwitchRow::builder()
                .title(&crate::gui::t(title))
                .subtitle(&crate::gui::t(subtitle))
                .active(key != &"performance_data") // Performance data off by default
                .build();

            data_group.add(&switch_row);
        }

        page.add(&privacy_group);
        page.add(&data_group);

        page
    }

    /// Create detection tools preferences page
    fn create_tools_preferences_page() -> adw::PreferencesPage {
        let page = adw::PreferencesPage::builder()
            .title(&crate::gui::t("Detection Tools"))
            .icon_name("applications-utilities-symbolic")
            .build();

        let tools_group = adw::PreferencesGroup::builder()
            .title(&crate::gui::t("Hardware Detection Tools"))
            .description(&crate::gui::t("Configure which tools to use for hardware detection"))
            .build();

        // Tool availability and configuration
        let tools = [
            ("lshw", "lshw", "Comprehensive hardware information", true),
            ("dmidecode", "dmidecode", "BIOS and motherboard information (requires root)", true),
            ("lspci", "lspci", "PCI device information", true),
            ("lsusb", "lsusb", "USB device information", true),
            ("inxi", "inxi", "System information summary", false),
        ];

        for (_id, name, description, default_enabled) in tools.iter() {
            let switch_row = adw::SwitchRow::builder()
                .title(*name)
                .subtitle(&crate::gui::t(description))
                .active(*default_enabled)
                .build();

            // TODO: Add logic to check if tool is actually available
            // and disable the switch if not found
            
            tools_group.add(&switch_row);
        }

        page.add(&tools_group);

        page
    }

    /// Run the application
    pub fn run(&self) -> i32 {
        self.app.run().into()
    }
}