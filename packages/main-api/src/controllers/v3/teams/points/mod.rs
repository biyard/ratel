mod get_team_rewards;
use get_team_rewards::get_team_rewards_handler;

mod list_team_point_transactions;
use list_team_point_transactions::list_team_point_transactions_handler;

#[cfg(test)]
pub mod tests;

use crate::AppState;
use bdk::prelude::*;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub fn route() -> Router<AppState> {
    Router::new()
        .route("/", get(get_team_rewards_handler))
        .route("/transactions", get(list_team_point_transactions_handler))
}
