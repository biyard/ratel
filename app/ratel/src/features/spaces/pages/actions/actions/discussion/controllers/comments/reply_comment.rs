use crate::common::models::space::{SpaceUser, SpaceCommon};
use crate::features::spaces::pages::actions::actions::discussion::*;
use crate::features::spaces::pages::actions::models::SpaceAction;
#[cfg(feature = "server")]
use crate::features::spaces::space_common::models::space_reward::SpaceReward;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyCommentRequest {
    pub content: String,
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
        return Err(Error::BadRequest(
            "Discussion is not available in the current space status".into(),
        ));
    }

    let space_post_pk: SpacePostPartition = match &discussion_sk_entity {
        EntityType::SpacePost(id) => SpacePostPartition(id.clone()),
        _ => return Err(Error::BadRequest("Invalid discussion id".into())),
    };
    let (post_pk, post_sk) = SpacePost::keys(&space_id, &space_post_pk);
    let post = SpacePost::get(cli, &post_pk, Some(post_sk))
        .await?
        .ok_or(Error::NotFound("Discussion not found".into()))?;
    if post.status() != DiscussionStatus::InProgress {
        return Err(Error::DiscussionNotInProgress);
    }

    let comment_sk_entity: EntityType = comment_sk.into();

    let comment = SpacePostComment::reply(
        cli,
        space_id.clone(),
        space_post_pk,
        comment_sk_entity,
        req.content,
        &member,
    )
    .await?;

    let space_pk: Partition = space_id.clone().into();
    let agg_item =
        crate::features::spaces::space_common::models::aggregate::DashboardAggregate::inc_comments(
            &space_pk, 1,
        );
    crate::transact_write_items!(cli, vec![agg_item]).ok();

    match SpaceReward::get_by_action(
        cli,
        space_id.clone(),
        discussion_sk.to_string(),
        RewardUserBehavior::DiscussionComment,
    )
    .await
    {
        Ok(space_reward) => {
            if let Err(e) =
                SpaceReward::award(cli, &space_reward, user.pk.clone(), Some(space.user_pk.clone()))
                    .await
            {
                tracing::error!(
                    space_pk = %space_id,
                    action_id = %discussion_sk_entity,
                    error = %e,
                    "Failed to award discussion reply reward"
                );
            }
        }
        Err(e) => {
            tracing::warn!(
                space_pk = %space_id,
                action_id = %discussion_sk_entity,
                error = %e,
                "SpaceReward not found for discussion action"
            );
        }
    }

    {
        let author_partition = crate::features::activity::types::AuthorPartition::from(member.pk.clone());

        if let Err(e) = crate::features::activity::controllers::record_activity(
            cli,
            space_id.clone(),
            author_partition,
            discussion_sk.to_string(),
            crate::features::spaces::pages::actions::types::SpaceActionType::TopicDiscussion,
            space_action.activity_score,
            space_action.additional_score,
            crate::features::activity::types::SpaceActivityData::Discussion {
                discussion_id: discussion_sk.to_string(),
                is_first_contribution: false,
            },
            member.display_name.clone(),
            member.profile_url.clone(),
        ).await {
            tracing::error!(error = %e, "Failed to record reply activity");
        }
    }

    Ok(comment.into())
}
