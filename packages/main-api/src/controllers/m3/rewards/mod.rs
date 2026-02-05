pub mod list_transactions;
pub mod upsert_reward;

pub use list_transactions::*;
pub use upsert_reward::*;

#[cfg(test)]
pub mod tests;

use crate::*;
use axum::routing::*;

pub fn route() -> crate::Result<by_axum::axum::Router<crate::AppState>> {
    Ok(axum::Router::new()
        .route("/", put(upsert_reward_handler))
        .route("/transactions", get(list_transactions_handler)))
}
