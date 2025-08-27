//! Individual device card widget

use gtk4::prelude::*;
use libadwaita as adw;
use adw::prelude::*;
use crate::gui::models::HardwareDeviceDisplay;

/// Card widget for displaying individual hardware device information
pub struct DeviceCard {
    widget: adw::ActionRow,
}

impl DeviceCard {
    /// Create a new device card
    pub fn new(device: &HardwareDeviceDisplay) -> Self {
        let widget = adw::ActionRow::new();
        widget.set_title(&device.name);
        widget.set_subtitle(&format!("{} - {}", device.vendor, device.model));

        // Device icon based on category
        let icon = gtk4::Image::from_icon_name(device.category.icon_name());
        widget.add_prefix(&icon);

        // Status indicator
        let status_icon = gtk4::Image::from_icon_name(device.status.icon_name());
        status_icon.add_css_class(device.status.css_class());
        widget.add_suffix(&status_icon);

        // Status text
        let status_label = gtk4::Label::new(Some(device.status.display_text()));
        status_label.add_css_class("dim-label");
        widget.add_suffix(&status_label);

        // Make clickable for details
        widget.set_activatable(true);

        Self { widget }
    }

    /// Get the main widget
    pub fn widget(&self) -> &adw::ActionRow {
        &self.widget
    }

    /// Connect activation handler for showing device details
    pub fn connect_activated<F>(&self, callback: F)
    where
        F: Fn() + 'static,
    {
        self.widget.connect_activated(move |_| callback());
    }
}