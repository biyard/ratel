mod delete_notification;
mod list_notifications;
mod mark_all_as_read;
mod mark_as_read;

use delete_notification::delete_notification_handler;
use list_notifications::list_notifications_handler;
use mark_all_as_read::mark_all_as_read_handler;
use mark_as_read::mark_as_read_handler;

use crate::*;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
        .route("/", get(list_notifications_handler))
        .route("/mark-as-read", post(mark_as_read_handler))
        .route("/mark-all-as-read", post(mark_all_as_read_handler))
        .route("/:notification_id", delete(delete_notification_handler)))
}

#[cfg(test)]
mod tests;
