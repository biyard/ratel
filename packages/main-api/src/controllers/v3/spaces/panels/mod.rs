pub mod create_panel_quota;
pub mod delete_panel_quota;
pub mod get_panel;
pub mod list_participants;
pub mod update_panel;
pub mod update_panel_quota;
pub use create_panel_quota::*;
pub use delete_panel_quota::*;
pub use get_panel::*;
pub use list_participants::*;
pub use update_panel::*;
pub use update_panel_quota::*;

#[cfg(test)]
pub mod tests;

use crate::AppState;
use bdk::prelude::*;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub fn route() -> Router<AppState> {
    Router::new()
        .route("/", get(get_panel_handler).patch(update_panel_handler))
        .route(
            "/quotas",
            post(create_panel_quota_handler)
                .delete(delete_panel_quota_handler)
                .patch(update_panel_quota_handler),
        )
        .route("/participants", get(list_participants_handler))
}
