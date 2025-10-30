pub mod get_artwork;
pub mod list_artwork_history;
pub mod mint_artwork;
pub mod transfer_artwork;

use crate::AppState;
use bdk::prelude::*;
use by_axum::aide::axum::routing::{get, post};
use by_axum::axum::Router;
pub use get_artwork::*;
pub use list_artwork_history::*;
pub use mint_artwork::*;
pub use transfer_artwork::*;

pub fn route() -> Router<AppState> {
    Router::new()
        .route("/", get(get_space_artwork_handler))
        .route("/history", get(list_space_artwork_history_handler))
        .route("/mint", post(mint_space_artwork_handler))
        .route("/transfer", post(transfer_space_artwork_handler))
}
