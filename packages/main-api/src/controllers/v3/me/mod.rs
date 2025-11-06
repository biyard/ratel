pub mod get_info;
pub mod update_user;

mod did;
pub mod list_my_drafts;
pub mod list_my_posts;
#[cfg(test)]
pub mod tests;

use get_info::get_info_handler;
use list_my_drafts::list_my_drafts_handler;
use list_my_posts::list_my_posts_handler;
use update_user::update_user_handler;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
        .nest("/did", did::route()?)
        .route("/", get(get_info_handler).patch(update_user_handler))
        .route("/posts", get(list_my_posts_handler))
        .route("/drafts", get(list_my_drafts_handler)))
}
