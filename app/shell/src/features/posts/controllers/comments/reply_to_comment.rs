use crate::features::posts::models::*;
use crate::features::posts::*;
use ratel_auth::User;

#[post("/api/posts/:post_id/comments/:comment_id/reply", user: User)]
pub async fn reply_to_comment_handler(
    post_id: FeedPartition,
    comment_id: PostCommentEntityType,
    content: String,
) -> Result<PostComment> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    let post_pk: Partition = post_id.into();
    let comment_sk: EntityType = comment_id.into();

    tracing::debug!("Handling reply to comment: {:?}", comment_sk);

    let comment = PostComment::reply(cli, post_pk, comment_sk, content, user).await?;

    Ok(comment)
}
