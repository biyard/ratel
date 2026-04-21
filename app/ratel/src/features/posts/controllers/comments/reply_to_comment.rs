use crate::features::posts::models::*;
use crate::features::posts::*;
use crate::features::auth::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ReplyToPostCommentRequest {
    pub content: String,
    #[serde(default)]
    pub images: Vec<String>,
}

#[post("/api/posts/:post_id/comments/:comment_id/reply", user: User)]
pub async fn reply_to_comment_handler(
    post_id: FeedPartition,
    comment_id: PostCommentEntityType,
    req: ReplyToPostCommentRequest,
) -> Result<PostComment> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    let post_pk: Partition = post_id.clone().into();
    let comment_sk: EntityType = comment_id.into();

    tracing::debug!("Handling reply to comment: {:?}", comment_sk);

    let parent_pk_str = post_pk.to_string();
    let parent_sk_str = comment_sk.to_string();

    let comment = PostComment::reply(
        cli,
        post_pk,
        comment_sk,
        req.content,
        req.images,
        user.clone(),
    )
    .await?;

    if let Err(e) = crate::features::essence::services::index_post_comment(cli, &comment).await {
        tracing::error!("failed to index post comment reply essence: {e}");
    }

    let cta_url = format!(
        "{}/posts/{}",
        crate::common::config::site_base_url(),
        post_id
    );

    // Send mention notifications
    crate::common::utils::mention::create_mention_notifications(
        cli,
        &comment.content,
        &user.pk,
        &user.display_name,
        &cta_url,
    )
    .await;

    // Fire reply-on-comment notification. Recipient resolution (parent author +
    // thread participants → emails) runs at send time, not here — the handler
    // only persists one notification row and returns.
    {
        let notification =
            crate::common::models::notification::Notification::new(
                crate::common::types::NotificationData::ReplyOnComment {
                    source: crate::common::utils::reply_notification::ReplyCommentSource::Post,
                    parent_comment_pk: parent_pk_str,
                    parent_comment_sk: parent_sk_str,
                    replier_pk: user.pk.to_string(),
                    replier_name: user.display_name.clone(),
                    reply_content: comment.content.clone(),
                    cta_url,
                },
            );
        if let Err(e) = notification.create(cli).await {
            tracing::error!("Failed to enqueue reply-on-comment notification: {}", e);
        }
    }

    Ok(comment)
}
