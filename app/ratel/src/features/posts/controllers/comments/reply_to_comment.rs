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
    let comment_sk: EntityType = comment_id.clone().into();

    tracing::debug!("Handling reply to comment: {:?}", comment_sk);

    // Fetch parent comment + prior replies before mutating so we can notify
    // the original author and everyone who has participated in the thread.
    let parent_comment = PostComment::get(cli, &post_pk, Some(comment_sk.clone())).await?;
    let prior_replies = PostComment::list_by_comment(
        cli,
        post_pk.clone(),
        comment_sk.clone(),
        None,
    )
    .await
    .map(|(items, _)| items)
    .unwrap_or_default();

    let comment = PostComment::reply(
        cli,
        post_pk,
        comment_sk,
        req.content,
        req.images,
        user.clone(),
    )
    .await?;

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

    // Send reply-on-comment notifications to parent author + thread participants
    if let Some(parent) = parent_comment {
        let mut recipient_pks: Vec<Partition> = Vec::new();
        recipient_pks.push(parent.author_pk.clone());
        for reply in prior_replies {
            recipient_pks.push(reply.author_pk);
        }

        let comment_preview =
            crate::common::utils::reply_notification::build_preview(&parent.content);
        let reply_preview =
            crate::common::utils::reply_notification::build_preview(&comment.content);

        crate::common::utils::reply_notification::create_reply_on_comment_notifications(
            cli,
            recipient_pks,
            &user.pk,
            &user.display_name,
            &comment_preview,
            &reply_preview,
            &cta_url,
        )
        .await;
    }

    Ok(comment)
}
