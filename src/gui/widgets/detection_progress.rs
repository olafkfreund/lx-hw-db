//! Hardware detection progress widget

use gtk4::prelude::*;
use libadwaita as adw;
use adw::prelude::*;
use std::collections::HashMap;

/// Detection progress widget showing real-time progress for each tool
pub struct DetectionProgress {
    widget: gtk4::Box,
    progress_bars: HashMap<String, gtk4::ProgressBar>,
    status_label: gtk4::Label,
    overall_progress: gtk4::ProgressBar,
    cancel_button: gtk4::Button,
}

impl DetectionProgress {
    /// Create a new detection progress widget
    pub fn new() -> Self {
        let widget = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
        widget.add_css_class("detection-progress");

        // Overall status
        let status_label = gtk4::Label::new(Some(&crate::gui::t("Starting hardware detection...")));
        status_label.add_css_class("title-3");
        status_label.set_halign(gtk4::Align::Start);
        widget.append(&status_label);

        // Overall progress bar
        let overall_progress = gtk4::ProgressBar::new();
        overall_progress.set_show_text(true);
        overall_progress.set_text(Some("0%"));
        widget.append(&overall_progress);

        // Individual tool progress group
        let tools_group = adw::PreferencesGroup::new();
        tools_group.set_title(&crate::gui::t("Detection Progress"));
        tools_group.set_margin_top(12);
        
        let mut progress_bars = HashMap::new();

        // Create progress rows for each detection tool
        let tools = [
            ("lshw", "System Information", "Comprehensive hardware detection"),
            ("dmidecode", "BIOS & Memory", "Motherboard and memory information"),
            ("lspci", "PCI Devices", "PCI bus and expansion cards"),
            ("lsusb", "USB Devices", "USB peripherals and controllers"),
            ("inxi", "System Summary", "Additional system information"),
        ];

        for (tool_id, tool_name, description) in tools.iter() {
            let row = adw::ActionRow::new();
            row.set_title(&crate::gui::t(tool_name));
            row.set_subtitle(&crate::gui::t(description));

            // Status icon (initially pending)
            let status_icon = gtk4::Image::from_icon_name("emblem-synchronizing-symbolic");
            status_icon.add_css_class("dim-label");
            row.add_prefix(&status_icon);

            // Progress bar
            let progress_bar = gtk4::ProgressBar::new();
            progress_bar.set_width_request(100);
            progress_bar.set_valign(gtk4::Align::Center);
            row.add_suffix(&progress_bar);

            // Status label
            let status_label = gtk4::Label::new(Some(&crate::gui::t("Pending")));
            status_label.add_css_class("dim-label");
            status_label.set_width_request(80);
            row.add_suffix(&status_label);

            tools_group.add(&row);
            progress_bars.insert(tool_id.to_string(), progress_bar);
        }

        widget.append(&tools_group);

        // Statistics
        let stats_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 24);
        stats_box.set_margin_top(12);

        let devices_label = gtk4::Label::new(Some(&crate::gui::t("📊 Found so far: 0 devices")));
        devices_label.set_halign(gtk4::Align::Start);
        stats_box.append(&devices_label);

        let privacy_label = gtk4::Label::new(Some(&crate::gui::t("🔒 Privacy level: Basic (24h salt rotation)")));
        privacy_label.set_halign(gtk4::Align::Start);
        stats_box.append(&privacy_label);

        let time_label = gtk4::Label::new(Some(&crate::gui::t("⏱️ Estimated time remaining: --")));
        time_label.set_halign(gtk4::Align::Start);
        stats_box.append(&time_label);

        widget.append(&stats_box);

        // Cancel button
        let button_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        button_box.set_halign(gtk4::Align::End);
        button_box.set_margin_top(12);

        let cancel_button = gtk4::Button::with_label(&crate::gui::t("Cancel Detection"));
        cancel_button.add_css_class("destructive-action");
        button_box.append(&cancel_button);

        widget.append(&button_box);

        Self {
            widget,
            progress_bars,
            status_label,
            overall_progress,
            cancel_button,
        }
    }

    /// Get the main widget
    pub fn widget(&self) -> &gtk4::Box {
        &self.widget
    }

    /// Update overall progress
    pub fn set_overall_progress(&self, fraction: f64, message: &str) {
        self.overall_progress.set_fraction(fraction);
        self.overall_progress.set_text(Some(&format!("{:.0}%", fraction * 100.0)));
        self.status_label.set_text(message);
    }

    /// Update progress for a specific tool
    pub fn set_tool_progress(&self, tool: &str, fraction: f64, status: ToolStatus) {
        if let Some(progress_bar) = self.progress_bars.get(tool) {
            progress_bar.set_fraction(fraction);

            // Update the parent row's icon and status
            if let Some(parent) = progress_bar.parent() {
                if let Some(row) = parent.parent() {
                    let row = row.downcast_ref::<adw::ActionRow>().unwrap();
                    
                    // Update status icon
                    let icon = match status {
                        ToolStatus::Pending => "emblem-synchronizing-symbolic",
                        ToolStatus::Running => "emblem-synchronizing-symbolic",
                        ToolStatus::Complete => "emblem-ok-symbolic",
                        ToolStatus::Error => "emblem-important-symbolic",
                    };
                    
                    // Find and update the prefix icon
                    let mut child = row.first_child();
                    while let Some(widget) = child {
                        if let Some(image) = widget.downcast_ref::<gtk4::Image>() {
                            image.set_icon_name(Some(icon));
                            
                            // Add CSS class for status
                            image.remove_css_class("dim-label");
                            match status {
                                ToolStatus::Complete => image.add_css_class("success"),
                                ToolStatus::Error => image.add_css_class("error"),
                                _ => image.add_css_class("dim-label"),
                            }
                            break;
                        }
                        child = widget.next_sibling();
                    }

                    // Update status label
                    let status_text = match status {
                        ToolStatus::Pending => crate::gui::t("Pending"),
                        ToolStatus::Running => crate::gui::t("Running"),
                        ToolStatus::Complete => crate::gui::t("Complete"),
                        ToolStatus::Error => crate::gui::t("Error"),
                    };

                    // Find and update the status label (last suffix)
                    let mut child = row.last_child();
                    while let Some(widget) = child {
                        if let Some(label) = widget.downcast_ref::<gtk4::Label>() {
                            label.set_text(&status_text);
                            break;
                        }
                        child = widget.prev_sibling();
                    }
                }
            }
        }
    }

    /// Update device count
    pub fn set_device_count(&self, count: u32) {
        // Find and update the devices label
        if let Some(stats_box) = self.widget.last_child() {
            if let Some(devices_label) = stats_box.first_child() {
                if let Some(label) = devices_label.downcast_ref::<gtk4::Label>() {
                    label.set_text(&format!("📊 Found so far: {} devices", count));
                }
            }
        }
    }

    /// Update time remaining estimate
    pub fn set_time_remaining(&self, seconds: u32) {
        // Find and update the time label
        if let Some(stats_box) = self.widget.last_child().and_then(|w| w.prev_sibling()) {
            if let Some(time_label) = stats_box.last_child() {
                if let Some(label) = time_label.downcast_ref::<gtk4::Label>() {
                    if seconds == 0 {
                        label.set_text(&crate::gui::t("⏱️ Completed"));
                    } else {
                        label.set_text(&format!("⏱️ Estimated time remaining: {} seconds", seconds));
                    }
                }
            }
        }
    }

    /// Connect cancel button handler
    pub fn connect_cancel<F>(&self, callback: F)
    where
        F: Fn() + 'static,
    {
        self.cancel_button.connect_clicked(move |_| callback());
    }

    /// Reset all progress to initial state
    pub fn reset(&self) {
        self.overall_progress.set_fraction(0.0);
        self.overall_progress.set_text(Some("0%"));
        self.status_label.set_text(&crate::gui::t("Ready to start detection..."));

        // Reset all tool progress bars
        for progress_bar in self.progress_bars.values() {
            progress_bar.set_fraction(0.0);
            self.set_tool_progress_by_bar(progress_bar, 0.0, ToolStatus::Pending);
        }

        self.set_device_count(0);
        self.set_time_remaining(0);
    }

    /// Set progress by progress bar reference
    fn set_tool_progress_by_bar(&self, progress_bar: &gtk4::ProgressBar, fraction: f64, status: ToolStatus) {
        progress_bar.set_fraction(fraction);

        // Update parent row status (same logic as set_tool_progress but with bar reference)
        if let Some(parent) = progress_bar.parent() {
            if let Some(row) = parent.parent() {
                let row = row.downcast_ref::<adw::ActionRow>().unwrap();
                
                let icon = match status {
                    ToolStatus::Pending => "emblem-synchronizing-symbolic",
                    ToolStatus::Running => "emblem-synchronizing-symbolic", 
                    ToolStatus::Complete => "emblem-ok-symbolic",
                    ToolStatus::Error => "emblem-important-symbolic",
                };

                // Update prefix icon
                let mut child = row.first_child();
                while let Some(widget) = child {
                    if let Some(image) = widget.downcast_ref::<gtk4::Image>() {
                        image.set_icon_name(Some(icon));
                        
                        image.remove_css_class("dim-label");
                        image.remove_css_class("success");
                        image.remove_css_class("error");
                        
                        match status {
                            ToolStatus::Complete => image.add_css_class("success"),
                            ToolStatus::Error => image.add_css_class("error"),
                            _ => image.add_css_class("dim-label"),
                        }
                        break;
                    }
                    child = widget.next_sibling();
                }
            }
        }
    }
}

/// Status of a detection tool
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToolStatus {
    Pending,
    Running,
    Complete,
    Error,
}