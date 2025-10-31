pub mod list_invitations;
pub mod upsert_invitation;
pub mod verify_space_code;

pub use list_invitations::*;
pub use upsert_invitation::*;
pub use verify_space_code::*;

use crate::AppState;
use bdk::prelude::*;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub fn route() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(upsert_invitation_handler).get(list_invitations_handler),
        )
        .route("/verifications", get(verify_space_code_handler))
}
