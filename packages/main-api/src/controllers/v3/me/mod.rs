pub mod get_info;
pub mod update_notification_status;
pub mod update_user;

mod did;
pub mod list_my_drafts;
pub mod list_my_notifications;
pub mod list_my_posts;
mod list_my_spaces;
mod memberships;

mod points;
#[cfg(test)]
pub mod tests;

use get_info::get_info_handler;
use list_my_drafts::list_my_drafts_handler;
use list_my_notifications::list_my_notifications_handler;
use list_my_posts::list_my_posts_handler;
use update_notification_status::update_my_notifications_status_handler;
use update_user::update_user_handler;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
        .nest("/did", did::route()?)
        .nest("/memberships", memberships::route()?)
        .nest("/points", points::route())
        .route("/", get(get_info_handler).patch(update_user_handler))
        .route("/posts", get(list_my_posts_handler))
        .route(
            "/notifications",
            get(list_my_notifications_handler).patch(update_my_notifications_status_handler),
        )
        .route("/spaces", get(list_my_spaces::list_my_spaces_handler))
        .route("/drafts", get(list_my_drafts_handler)))
}
