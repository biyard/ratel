use crate::common::models::space::{SpaceCommon, SpaceUser};
use crate::features::spaces::pages::actions::actions::discussion::*;
use crate::features::spaces::pages::actions::models::SpaceAction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyCommentRequest {
    pub content: String,
    #[serde(default)]
    pub images: Vec<String>,
}

#[post("/api/spaces/{space_id}/discussions/{discussion_sk}/comments/{comment_sk}/reply", role: SpaceUserRole, member: SpaceUser, space: SpaceCommon, user: crate::features::auth::User)]
pub async fn reply_comment(
    space_id: SpacePartition,
    discussion_sk: SpacePostEntityType,
    comment_sk: SpacePostCommentEntityType,
    req: ReplyCommentRequest,
) -> Result<DiscussionCommentResponse> {
    SpacePost::can_view(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let discussion_sk_entity: EntityType = discussion_sk.clone().into();
    let space_action = SpaceAction::get(
        cli,
        &CompositePartition(space_id.clone(), discussion_sk.to_string()),
        Some(EntityType::SpaceAction),
    )
    .await?
    .ok_or(Error::SpaceActionNotFound)?;

    if !crate::features::spaces::pages::actions::can_execute_space_action(
        role,
        space_action.prerequisite,
        space.status,
        space.join_anytime,
    ) {
        return Err(SpaceActionDiscussionError::NotAvailableInCurrentStatus.into());
    }

    let space_post_pk: SpacePostPartition = match &discussion_sk_entity {
        EntityType::SpacePost(id) => SpacePostPartition(id.clone()),
        _ => return Err(SpaceActionDiscussionError::InvalidDiscussionId.into()),
    };
    let (post_pk, post_sk) = SpacePost::keys(&space_id, &space_post_pk);
    let post = SpacePost::get(cli, &post_pk, Some(post_sk))
        .await?
        .ok_or(SpaceActionDiscussionError::NotFound)?;
    if post.status() != DiscussionStatus::InProgress {
        return Err(Error::DiscussionNotInProgress);
    }

    let comment_sk_entity: EntityType = comment_sk.into();

    let parent_pk_str: String = {
        let p: Partition = space_post_pk.clone().into();
        p.to_string()
    };
    let parent_sk_str = comment_sk_entity.to_string();
    // Preserve the parent's UUID for the mention CTA — `comment_sk_entity`
    // is moved into `SpacePostComment::reply` below, so extract first.
    let parent_comment_id: String = match &comment_sk_entity {
        EntityType::SpacePostComment(id) => id.clone(),
        _ => String::new(),
    };

    let comment = SpacePostComment::reply(
        cli,
        space_id.clone(),
        space_post_pk,
        comment_sk_entity,
        req.content,
        req.images,
        &member,
    )
    .await?;

    let space_pk: Partition = space_id.clone().into();
    let agg_item =
        crate::features::spaces::space_common::models::aggregate::DashboardAggregate::inc_comments(
            &space_pk, 1,
        );
    crate::transact_write_items!(cli, vec![agg_item]).ok();

    // Reward payout + XP recording run on EventBridge via SPACE_POST_COMMENT#
    // INSERT → handle_discussion_xp. Only the first reply per user per
    // discussion is rewarded (RewardPeriod::Once gate).

    // XP recording is now handled via EventBridge on SPACE_POST_COMMENT_REPLY# INSERT

    // Deep-link to the parent comment — replies are lazy-loaded, so we
    // can't jump to the reply itself in Phase 1. Landing on the parent
    // still works: the recipient sees the highlighted parent with the
    // "N replies" toggle right beneath it, one click away. Comment id is
    // in the path (not query/fragment) because Dioxus Router strips both
    // during URL normalization.
    let cta_url = format!(
        "{}/spaces/{}/discussions/{}/c/{}",
        crate::common::config::site_base_url(),
        space_id,
        discussion_sk,
        parent_comment_id,
    );

    // Send mention notifications
    crate::common::utils::mention::create_mention_notifications(
        cli,
        &comment.content,
        &member.pk,
        &member.display_name,
        &cta_url,
    )
    .await;

    // Fire reply-on-comment notification. Recipient resolution (parent author +
    // thread participants → emails) runs at send time, not here — the handler
    // only persists one notification row and returns.
    {
        let notification = crate::common::models::notification::Notification::new(
            crate::common::types::NotificationData::ReplyOnComment {
                source:
                    crate::common::utils::reply_notification::ReplyCommentSource::SpaceDiscussion,
                parent_comment_pk: parent_pk_str,
                parent_comment_sk: parent_sk_str,
                replier_pk: member.pk.to_string(),
                replier_name: member.display_name.clone(),
                reply_content: comment.content.clone(),
                cta_url,
            },
        );
        if let Err(e) = notification.create(cli).await {
            tracing::error!("Failed to enqueue reply-on-comment notification: {}", e);
        }
    }

    Ok(comment.into())
}
