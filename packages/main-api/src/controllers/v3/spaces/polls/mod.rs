pub mod create_poll;
pub mod get_poll;
pub mod get_poll_result;
pub mod list_polls;
pub mod respond_poll;
pub mod update_poll;

pub use create_poll::*;
pub use get_poll::*;
pub use get_poll_result::*;
pub use list_polls::*;
pub use respond_poll::*;
pub use update_poll::*;

#[cfg(test)]
pub mod tests;

use crate::AppState;
use bdk::prelude::*;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub fn route() -> Router<AppState> {
    Router::new()
        .route("/", post(create_poll_handler).get(list_polls_handler))
        .route("/:poll_sk", get(get_poll_handler).put(update_poll_handler))
        .route("/:poll_sk/results", get(get_poll_result))
        .route("/:poll_sk/responses", post(respond_poll_handler))
}
