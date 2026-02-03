pub mod dto;
pub mod list_payments;

pub use dto::*;
pub use list_payments::*;

use bdk::prelude::{axum::routing::*, *};

pub fn route() -> crate::Result<by_axum::axum::Router<crate::AppState>> {
    Ok(axum::Router::new().route("/", get(list_all_payments_handler)))
}
