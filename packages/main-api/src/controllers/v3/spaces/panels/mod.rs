pub mod create_panel;
pub mod delete_panel;
pub mod get_panel;
pub mod invite_participant;
pub mod list_panels;
pub mod list_participants;
pub mod update_panel;
pub use create_panel::*;
pub use delete_panel::*;
pub use get_panel::*;
pub use invite_participant::*;
pub use list_panels::*;
pub use list_participants::*;
pub use update_panel::*;

#[cfg(test)]
pub mod tests;

use crate::AppState;
use bdk::prelude::*;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub fn route() -> Router<AppState> {
    Router::new()
        .route("/", post(create_panel_handler).get(list_panels_handler))
        .route(
            "/:panel_pk",
            patch(update_panel_handler)
                .get(get_panel_handler)
                .delete(delete_panel_handler),
        )
        .route(
            "/:panel_pk/participants",
            patch(participate_panel_handler).get(list_participants_handler),
        )
}
