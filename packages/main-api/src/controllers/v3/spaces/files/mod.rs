pub mod get_files;
pub mod update_files;
pub mod get_file_links;

pub use get_files::*;
pub use update_files::*;
pub use get_file_links::*;

#[cfg(test)]
pub mod tests;

use crate::AppState;
use bdk::prelude::*;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub fn route() -> Router<AppState> {
    Router::new()
        .route("/", patch(update_files_handler).get(get_files_handler))
        .route("/links", get(list_file_links_handler))
        .route("/by-target", get(get_files_by_target_handler))
}
