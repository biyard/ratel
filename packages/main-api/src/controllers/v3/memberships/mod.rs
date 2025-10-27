// User-facing membership handlers
pub mod binance_webhook;
pub mod cancel_membership;
pub mod get_my_membership;
pub mod purchase_membership;
pub mod renew_membership;

pub use binance_webhook::*;
pub use cancel_membership::*;
pub use get_my_membership::*;
pub use purchase_membership::*;
pub use renew_membership::*;

use crate::AppState;
use bdk::prelude::*;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub fn route() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            patch(purchase_membership_handler).get(get_my_membership_handler),
        )
        .route("/webhooks", post(binance_webhook_handler))
        .route("/renew", patch(renew_membership_handler))
        .route("/cancel", patch(cancel_membership_handler))
}
