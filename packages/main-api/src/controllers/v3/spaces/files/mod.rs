pub mod ai_chat;
pub mod get_files;
pub mod update_files;

pub use ai_chat::*;
pub use get_files::*;
pub use update_files::*;

#[cfg(test)]
pub mod tests;

use crate::AppState;
use bdk::prelude::*;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub fn route() -> Router<AppState> {
    Router::new()
        .route("/", patch(update_files_handler).get(get_files_handler))
        .route("/:file_id/ai-chat", post(ai_chat_handler))
}
