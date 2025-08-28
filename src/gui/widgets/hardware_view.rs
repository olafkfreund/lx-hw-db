//! Hardware view widget for displaying detected devices

use crate::gui::models::{HardwareCategory, HardwareDeviceDisplay};
use adw::prelude::*;
use gtk4::prelude::*;
use libadwaita as adw;

/// Widget for displaying hardware detection results
pub struct HardwareView {
    widget: gtk4::ScrolledWindow,
    content_box: gtk4::Box,
    device_groups: std::collections::HashMap<HardwareCategory, adw::ExpanderRow>,
}

impl HardwareView {
    /// Create a new hardware view
    pub fn new() -> Self {
        let widget = gtk4::ScrolledWindow::new();
        widget.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);

        let content_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        widget.set_child(Some(&content_box));

        Self { widget, content_box, device_groups: std::collections::HashMap::new() }
    }

    /// Get the main widget
    pub fn widget(&self) -> &gtk4::ScrolledWindow {
        &self.widget
    }

    /// Update the view with hardware devices
    pub fn update_devices(&mut self, devices: Vec<HardwareDeviceDisplay>) {
        // Clear existing content
        while let Some(child) = self.content_box.first_child() {
            self.content_box.remove(&child);
        }
        self.device_groups.clear();

        // Group devices by category
        let mut categories: std::collections::HashMap<
            HardwareCategory,
            Vec<HardwareDeviceDisplay>,
        > = std::collections::HashMap::new();

        for device in devices {
            categories.entry(device.category).or_insert_with(Vec::new).push(device);
        }

        // Create expandable sections for each category
        for (category, devices) in categories {
            let expander = self.create_category_section(&category, devices);
            self.content_box.append(&expander);
            self.device_groups.insert(category, expander);
        }
    }

    /// Create an expandable section for a hardware category
    fn create_category_section(
        &self,
        category: &HardwareCategory,
        devices: Vec<HardwareDeviceDisplay>,
    ) -> adw::ExpanderRow {
        let expander = adw::ExpanderRow::new();
        expander.set_title(&crate::gui::t(category.display_name()));
        expander.set_subtitle(&format!("{} devices", devices.len()));

        // Category icon
        let icon = gtk4::Image::from_icon_name(category.icon_name());
        expander.add_prefix(&icon);

        // Add devices to the expander
        for device in devices {
            let device_row = self.create_device_row(device);
            expander.add_row(&device_row);
        }

        expander
    }

    /// Create a row for a single device
    fn create_device_row(&self, device: HardwareDeviceDisplay) -> adw::ActionRow {
        let row = adw::ActionRow::new();
        row.set_title(&device.name);
        row.set_subtitle(&format!("{} - {}", device.vendor, device.model));

        // Status indicator
        let status_icon = gtk4::Image::from_icon_name(device.status.icon_name());
        status_icon.add_css_class(device.status.css_class());
        row.add_suffix(&status_icon);

        // Status label
        let status_label = gtk4::Label::new(Some(device.status.display_text()));
        status_label.add_css_class("dim-label");
        row.add_suffix(&status_label);

        row
    }
}

impl Default for HardwareView {
    fn default() -> Self {
        Self::new()
    }
}
