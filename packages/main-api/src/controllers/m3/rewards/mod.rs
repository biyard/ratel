pub mod update_reward;

pub use update_reward::*;

use bdk::prelude::{axum::routing::*, *};

pub fn route() -> crate::Result<by_axum::axum::Router<crate::AppState>> {
    Ok(axum::Router::new().route("/", patch(update_reward_handler)))
}
