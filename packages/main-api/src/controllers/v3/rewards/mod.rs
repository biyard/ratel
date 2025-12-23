pub mod list_rewards;

use by_axum::axum::{Router, routing::get};

use crate::{AppState, Result};

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new().route("/", get(list_rewards::list_rewards_handler)))
}
