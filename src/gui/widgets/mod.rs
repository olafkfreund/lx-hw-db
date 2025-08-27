//! Custom widgets for the GTK4 interface

pub mod detection_progress;
pub mod hardware_view;
pub mod configuration_view;
pub mod export_dialog;
pub mod device_card;

pub use detection_progress::DetectionProgress;
pub use hardware_view::HardwareView;
pub use configuration_view::ConfigurationView;
pub use export_dialog::ExportDialog;
pub use device_card::DeviceCard;