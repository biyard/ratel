pub mod create_space_post;
pub mod delete_space_post;
pub mod get_space_post;
pub mod list_categories;
pub mod list_space_comments;
pub mod list_space_posts;
pub mod update_space_post;

pub mod comments;

pub use comments::*;
pub use create_space_post::*;
pub use delete_space_post::*;
pub use get_space_post::*;
pub use list_categories::*;
pub use list_space_comments::*;
pub use list_space_posts::*;
pub use update_space_post::*;

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
            post(create_space_post_handler).get(list_space_posts_handler),
        )
        .route("/categories", get(list_categories_handler))
        .route(
            "/:space_post_pk",
            patch(update_space_post_handler)
                .delete(delete_space_post_handler)
                .get(get_space_post_handler),
        )
        .route(
            "/:space_post_pk/comments",
            post(add_space_comment_handler).get(list_space_comments_handler),
        )
        .route(
            "/:space_post_pk/comments/:space_post_comment_sk/likes",
            post(like_space_comment_handler),
        )
        .route(
            "/:space_post_pk/comments/:space_post_comment_sk",
            post(reply_space_comment_handler)
                .get(list_space_reply_comments_handler)
                .delete(delete_space_comment_handler)
                .patch(update_space_comment_handler),
        )
}
