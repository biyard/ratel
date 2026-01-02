mod get_my_rewards;
use get_my_rewards::get_my_rewards_handler;

mod list_my_point_transactions;
use list_my_point_transactions::list_my_point_transactions_handler;

#[cfg(test)]
pub mod tests;

use crate::AppState;
use bdk::prelude::*;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub fn route() -> Router<AppState> {
    Router::new()
        .route("/", get(get_my_rewards_handler))
        .route("/transactions", get(list_my_point_transactions_handler))
}
