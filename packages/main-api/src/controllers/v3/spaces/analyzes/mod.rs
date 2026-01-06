pub mod get_analyze;
pub mod update_lda;
pub mod upsert_analyze;

pub use get_analyze::*;
pub use update_lda::*;
pub use upsert_analyze::*;

#[cfg(test)]
pub mod tests;

use crate::AppState;
use bdk::prelude::*;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub fn route() -> Router<AppState> {
    Router::new()
        .route("/", post(upsert_analyze_handler).get(get_analyze_handler))
        .route("/lda", post(update_lda_handler))
}
