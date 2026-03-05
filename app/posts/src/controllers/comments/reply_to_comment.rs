use crate::models::*;
use crate::*;
use ratel_auth::User;

#[post("/api/posts/:post_pk/comments/:comment_sk/reply", user: User)]
pub async fn reply_to_comment_handler(
    post_pk: FeedPartition,
    comment_sk: String,
    content: String,
) -> Result<PostComment> {
    let conf = crate::config::get();
    let cli = conf.dynamodb();

    let post_pk: Partition = post_pk.into();
    let comment_sk: EntityType = comment_sk
        .parse()
        .map_err(|_| Error::BadRequest("Invalid comment_sk".to_string()))?;

    tracing::debug!("Handling reply to comment: {:?}", comment_sk);

    let comment = PostComment::reply(cli, post_pk, comment_sk, content, user).await?;

    Ok(comment)
}
