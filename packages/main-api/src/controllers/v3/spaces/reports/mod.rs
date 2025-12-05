pub mod create_report;
pub mod get_pricing_challenge;
pub mod get_report;
pub mod publish_report;
pub mod set_pricing;
pub mod update_report;

pub use create_report::*;
pub use get_pricing_challenge::*;
pub use get_report::*;
pub use publish_report::*;
pub use set_pricing::*;
pub use update_report::*;

#[cfg(test)]
pub mod tests;

use crate::AppState;
use bdk::prelude::*;
use by_axum::axum::routing::{get, patch, post};
use by_axum::axum::Router;

pub fn route() -> Router<AppState> {
    Router::new()
        // POST to create a new report
        .route("/", post(create_report_handler))
        // GET and PATCH for author to view/edit their draft report
        // (separate from x402-protected consumer endpoint at parent level)
        .route("/draft", get(get_report_handler).patch(update_report_handler))
        .route("/pricing/challenge", post(get_pricing_challenge_handler))
        .route("/pricing", post(set_pricing_handler))
        .route("/publish", post(publish_report_handler))
}
