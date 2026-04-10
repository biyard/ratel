#[cfg(feature = "server")]
pub mod space_status_change_notification;

#[cfg(feature = "server")]
pub use space_status_change_notification::handle_space_status_change;
