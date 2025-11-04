pub mod comments;
pub mod create_post;
pub mod delete_post;
mod dto;
pub mod get_post;
pub mod like_post;
pub mod list_posts;
pub mod post_response;
pub mod update_post;

pub use comments::*;
pub use create_post::*;
pub use delete_post::*;
pub use dto::*;
pub use get_post::*;
pub use like_post::*;
pub use list_posts::*;
pub use post_response::*;
pub use update_post::*;

#[cfg(test)]
pub mod tests;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
        .route("/", post(create_post_handler).get(list_posts_handler))
        .route("/:post_pk/likes", post(like_post_handler))
        .route("/:post_pk/comments", post(add_comment_handler))
        .route(
            "/:post_pk/comments/:comment_sk",
            post(reply_to_comment_handler).get(list_comments_handler),
        )
        .route(
            "/:post_pk/comments/:comment_sk/likes",
            post(like_comment_handler),
        )
        .route(
            "/:post_pk",
            get(get_post_handler)
                .patch(update_post_handler)
                .delete(delete_post_handler),
        ))
}
