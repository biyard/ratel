pub mod ai_chat;
pub mod get_files;
pub mod update_files;
pub mod get_file_links;
pub mod delete_file;

pub use ai_chat::*;
pub use get_files::*;
pub use update_files::*;
pub use get_file_links::*;
pub use delete_file::*;

#[cfg(test)]
pub mod tests;

use crate::AppState;
use bdk::prelude::*;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub fn route() -> Router<AppState> {
    Router::new()
        .route("/", patch(update_files_handler).get(get_files_handler).delete(delete_file_handler))
        .route("/links", get(list_file_links_handler))
        .route("/links/target", get(get_files_by_target_handler))
        .route("/:file_id/ai-chat", post(ai_chat_handler))
}
