mod create_panel_quota;
mod delete_all_panels;
mod list_panel_attributes;
pub mod list_participants;
pub mod update_panel_quota;
mod update_space_panels;
use create_panel_quota::create_panel_quota_handler;
use list_panel_attributes::list_panel_attributes_handler;
use list_participants::*;
use update_panel_quota::*;
use update_space_panels::update_space_panels_handler;

#[cfg(test)]
pub mod tests;

use crate::*;

pub fn route() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(create_panel_quota_handler)
                .get(list_panel_attributes_handler)
                .patch(update_space_panels_handler)
                .delete(delete_all_panels::delete_all_panels_handler),
        )
        .route("/:panel_sk", patch(update_panel_quota_handler))
        .route("/participants", get(list_participants_handler))
}
