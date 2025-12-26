pub mod list_transactions;
pub mod update_reward;

pub use list_transactions::*;
pub use update_reward::*;

use bdk::prelude::{axum::routing::*, *};

pub fn route() -> crate::Result<by_axum::axum::Router<crate::AppState>> {
    Ok(axum::Router::new()
        .route("/", patch(update_reward_handler))
        .route("/transactions", get(list_transactions_handler)))
}
