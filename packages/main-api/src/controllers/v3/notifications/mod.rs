pub mod update_notification_status;
pub use update_notification_status::*;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new().route("/status", patch(update_my_notifications_status_handler)))
}
