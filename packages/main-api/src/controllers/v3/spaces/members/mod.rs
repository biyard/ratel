pub mod list_invitations;
pub mod resent_invitation_code;
pub mod upsert_invitation;

pub use list_invitations::*;
pub use resent_invitation_code::*;
pub use upsert_invitation::*;

#[cfg(test)]
pub mod tests;

use crate::AppState;
use bdk::prelude::*;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub fn route() -> Router<AppState> {
    Router::new().route(
        "/",
        post(upsert_invitation_handler)
            .get(list_invitations_handler)
            .patch(resent_invitation_code_handler),
    )
}
