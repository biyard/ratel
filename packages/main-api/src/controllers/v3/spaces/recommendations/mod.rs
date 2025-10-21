pub mod get_recommendation;
pub mod update_recommendation;

pub use get_recommendation::*;
pub use update_recommendation::*;

#[cfg(test)]
pub mod tests;

use crate::AppState;
use bdk::prelude::*;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub fn route() -> Router<AppState> {
    Router::new().route(
        "/",
        patch(update_recommendation_handler).get(get_recommendation_handler),
    )
}
