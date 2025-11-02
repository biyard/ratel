pub mod create_did;
pub mod deactivate_did;
pub mod reconcil_identity_verification;
pub mod resolve_did;
pub mod update_did;

#[cfg(test)]
mod tests;

use create_did::create_did_handler;
use deactivate_did::deactivate_did_handler;
use reconcil_identity_verification::reconcil_identity_verification_handler;
use resolve_did::resolve_did_handler;
use update_did::update_did_handler;

use crate::*;
use by_axum::aide::axum::routing::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
        .route("/", post(create_did_handler))
        .route(
            "/:did",
            get(resolve_did_handler)
                .put(update_did_handler)
                .delete(deactivate_did_handler),
        )
        .route("/kyc", post(reconcil_identity_verification_handler)))
}
