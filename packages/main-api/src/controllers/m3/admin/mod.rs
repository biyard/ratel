pub mod list_admins;
pub mod get_admin;
pub mod promote_to_admin;
pub mod demote_admin;

pub use list_admins::*;
pub use get_admin::*;
pub use promote_to_admin::*;
pub use demote_admin::*;

#[cfg(test)]
pub mod tests;

use bdk::prelude::{axum::routing::*, *};

pub fn route() -> crate::Result<by_axum::axum::Router<crate::AppState>> {
    Ok(axum::Router::new()
        .route(
            "/",
            post(promote_to_admin_handler).get(list_admins_handler),
        )
        .route(
            "/:user_id",
            get(get_admin_handler).delete(demote_admin_handler),
        ))
}
