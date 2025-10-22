pub mod create_membership;
pub mod delete_membership;
pub mod get_membership;
pub mod list_memberships;
pub mod update_membership;

pub use create_membership::*;
pub use delete_membership::*;
pub use get_membership::*;
pub use list_memberships::*;
pub use update_membership::*;

#[cfg(test)]
pub mod tests;

use bdk::prelude::{axum::routing::*, *};

pub fn route() -> crate::Result<by_axum::axum::Router<crate::AppState>> {
    Ok(axum::Router::new()
        .route(
            "/",
            post(create_membership_handler).get(list_memberships_handler),
        )
        .route(
            "/:membership_id",
            get(get_membership_handler)
                .patch(update_membership_handler)
                .delete(delete_membership_handler),
        ))
}
