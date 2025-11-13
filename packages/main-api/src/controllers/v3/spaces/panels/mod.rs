pub mod create_panel_quota;
mod delete_all_panels;
pub mod delete_panel_quota;
mod list_panel_attributes;
pub mod list_participants;
pub mod update_panel_quota;
pub use create_panel_quota::*;
pub use delete_panel_quota::*;
use list_panel_attributes::list_panel_attributes_handler;
pub use list_participants::*;
pub use update_panel_quota::*;

#[cfg(test)]
pub mod tests;

use crate::*;

pub fn route() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(create_panel_quota_handler)
                .get(list_panel_attributes_handler)
                .delete(delete_all_panels::delete_all_panels_handler),
        )
        .route(
            "/:panel_sk",
            delete(delete_panel_quota_handler).patch(update_panel_quota_handler),
        )
        .route("/participants", get(list_participants_handler))
}
