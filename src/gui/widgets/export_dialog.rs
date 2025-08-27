//! Export dialog widget

use gtk4::prelude::*;
use libadwaita as adw;

/// Dialog for exporting hardware reports
pub struct ExportDialog {
    dialog: adw::MessageDialog,
}

impl ExportDialog {
    /// Create a new export dialog
    pub fn new(parent: &impl IsA<gtk4::Window>) -> Self {
        let dialog = adw::MessageDialog::new(
            Some(parent),
            Some(&crate::gui::t("Export Hardware Report")),
            Some(&crate::gui::t("Choose export format and options")),
        );

        dialog.add_response("cancel", &crate::gui::t("Cancel"));
        dialog.add_response("export", &crate::gui::t("Export"));
        dialog.set_response_appearance("export", adw::ResponseAppearance::Suggested);

        // TODO: Add export format selection and options

        Self { dialog }
    }

    /// Present the dialog
    pub fn present(&self) {
        self.dialog.present();
    }

    /// Connect response handler
    pub fn connect_response<F>(&self, callback: F)
    where
        F: Fn(&str) + 'static,
    {
        self.dialog.connect_response(None, move |_, response| {
            callback(response);
        });
    }
}