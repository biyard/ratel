pub mod ai_chat;
pub mod download_analyze;
pub mod download_analyze_url;
pub mod get_analyze;
pub mod update_analyze;
pub mod upsert_analyze;

pub use ai_chat::*;
pub use download_analyze::*;
pub use download_analyze_url::*;
pub use get_analyze::*;
pub use update_analyze::*;
pub use upsert_analyze::*;

#[cfg(test)]
pub mod tests;

use crate::AppState;
use bdk::prelude::*;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub fn route() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(upsert_analyze_handler)
                .get(get_analyze_handler)
                .patch(update_analyze_handler),
        )
        .route("/download", post(download_analyze_handler))
        .route("/download-url", get(download_analyze_url_handler))
        .route("/:analyze_pk/ai-chat", post(ai_chat_handler))
}
