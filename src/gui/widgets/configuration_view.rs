//! Configuration recommendations view widget

use gtk4::prelude::*;
use libadwaita as adw;
use crate::configuration::Configuration;

/// Widget for displaying configuration recommendations
pub struct ConfigurationView {
    widget: gtk4::ScrolledWindow,
    content_box: gtk4::Box,
}

impl ConfigurationView {
    /// Create a new configuration view
    pub fn new() -> Self {
        let widget = gtk4::ScrolledWindow::new();
        widget.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);

        let content_box = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
        content_box.set_margin_top(24);
        content_box.set_margin_bottom(24);
        content_box.set_margin_start(24);
        content_box.set_margin_end(24);
        
        widget.set_child(Some(&content_box));

        Self {
            widget,
            content_box,
        }
    }

    /// Get the main widget
    pub fn widget(&self) -> &gtk4::ScrolledWindow {
        &self.widget
    }

    /// Update the view with configuration data
    pub fn update_configuration(&self, config: &Configuration) {
        // Clear existing content
        while let Some(child) = self.content_box.first_child() {
            self.content_box.remove(&child);
        }

        // Header
        let header = gtk4::Label::new(Some(&crate::gui::t("System Configuration Recommendations")));
        header.add_css_class("title-2");
        header.set_halign(gtk4::Align::Start);
        self.content_box.append(&header);

        // Compatibility score
        let score_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
        
        let score_label = gtk4::Label::new(Some(&format!(
            "🎯 Target: {} • Compatibility Score: {:.1}%", 
            config.target_distribution,
            config.compatibility_score * 100.0
        )));
        score_label.add_css_class("subtitle");
        score_box.append(&score_label);
        
        self.content_box.append(&score_box);

        // Driver recommendations
        if !config.driver_recommendations.is_empty() {
            let drivers_group = self.create_drivers_section(&config.driver_recommendations);
            self.content_box.append(&drivers_group);
        }

        // Kernel parameters
        if !config.kernel_parameters.is_empty() {
            let params_group = self.create_kernel_params_section(&config.kernel_parameters);
            self.content_box.append(&params_group);
        }

        // Package installations
        if !config.package_installations.is_empty() {
            let packages_group = self.create_packages_section(&config.package_installations);
            self.content_box.append(&packages_group);
        }
    }

    /// Create driver recommendations section
    fn create_drivers_section(&self, drivers: &[crate::configuration::DriverRecommendation]) -> adw::PreferencesGroup {
        let group = adw::PreferencesGroup::new();
        group.set_title(&crate::gui::t("Driver Recommendations"));
        
        for driver in drivers {
            let row = adw::ActionRow::new();
            row.set_title(&driver.recommended_driver);
            row.set_subtitle(&format!("{} - Priority: {}", driver.component_type, driver.installation_priority));
            
            if let Some(notes) = &driver.compatibility_notes {
                row.set_subtitle(&format!("{}\n{}", row.subtitle().unwrap_or_default(), notes));
            }

            // Priority indicator
            let priority_icon = if driver.installation_priority >= 8 {
                "emblem-important-symbolic"
            } else if driver.installation_priority >= 5 {
                "dialog-warning-symbolic"
            } else {
                "emblem-ok-symbolic"
            };
            
            let icon = gtk4::Image::from_icon_name(priority_icon);
            row.add_prefix(&icon);
            
            group.add(&row);
        }
        
        group
    }

    /// Create kernel parameters section
    fn create_kernel_params_section(&self, params: &[crate::configuration::KernelParameter]) -> adw::PreferencesGroup {
        let group = adw::PreferencesGroup::new();
        group.set_title(&crate::gui::t("Kernel Parameters"));
        
        for param in params {
            let row = adw::ActionRow::new();
            
            let param_text = if let Some(value) = &param.value {
                format!("{}={}", param.parameter, value)
            } else {
                param.parameter.clone()
            };
            
            row.set_title(&param_text);
            row.set_subtitle(&param.purpose);
            
            group.add(&row);
        }
        
        group
    }

    /// Create package installations section  
    fn create_packages_section(&self, packages: &[crate::configuration::PackageInstallation]) -> adw::PreferencesGroup {
        let group = adw::PreferencesGroup::new();
        group.set_title(&crate::gui::t("Package Installations"));
        
        for package in packages {
            let row = adw::ActionRow::new();
            row.set_title(&package.package_name);
            row.set_subtitle(&package.package_description);
            
            // Installation command as tooltip or expandable content
            row.set_tooltip_text(Some(&package.installation_command));
            
            group.add(&row);
        }
        
        group
    }
}

impl Default for ConfigurationView {
    fn default() -> Self {
        Self::new()
    }
}