use crate::common::models::space::{SpaceCommon, SpaceUser};
use crate::features::spaces::pages::actions::actions::discussion::*;
use crate::features::spaces::pages::actions::models::SpaceAction;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct AddCommentRequest {
    pub content: String,
    #[serde(default)]
    pub images: Vec<String>,
}

#[mcp_tool(
    name = "add_comment",
    description = "Add a comment to a discussion. Requires participant role and discussion in progress."
)]
#[post("/api/spaces/{space_id}/discussions/{discussion_sk}/comments", role: SpaceUserRole, member: SpaceUser, space: SpaceCommon, user: crate::features::auth::User )]
pub async fn add_comment(
    #[mcp(description = "Space partition key")] space_id: SpacePartition,
    #[mcp(description = "Discussion sort key (e.g. 'SpacePost#<uuid>')")]
    discussion_sk: SpacePostEntityType,
    #[mcp(description = "Comment content as JSON: {\"content\": \"text\"}")] req: AddCommentRequest,
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

    let deps_met = crate::features::spaces::pages::actions::services::dependency::dependencies_met(
        cli,
        &space,
        &space_action,
        &member.pk,
    )
    .await?;

    if !crate::features::spaces::pages::actions::can_execute_space_action(
        role,
        space_action.prerequisite,
        space.status,
        space_action.status.as_ref(),
        deps_met,
        space.join_anytime,
    ) {
        return Err(SpaceActionDiscussionError::NotAvailableInCurrentStatus.into());
    }

    let space_post_id = match &discussion_sk_entity {
        EntityType::SpacePost(id) => SpacePostPartition(id.clone()),
        _ => return Err(SpaceActionDiscussionError::InvalidDiscussionId.into()),
    };
    let (post_pk, post_sk) = SpacePost::keys(&space_id, &space_post_id);
    let _post = SpacePost::get(cli, &post_pk, Some(post_sk))
        .await?
        .ok_or(SpaceActionDiscussionError::NotFound)?;

    let comment = SpacePost::comment(
        cli,
        space_id.clone(),
        space_post_id,
        req.content,
        req.images,
        &member,
    )
    .await?;

    let space_pk: Partition = space_id.clone().into();

    // Essence indexing happens via the DynamoDB Stream pipeline.
    let agg_item =
        crate::features::spaces::space_common::models::aggregate::DashboardAggregate::inc_comments(
            &space_pk, 1,
        );
    crate::transact_write_items!(cli, vec![agg_item]).ok();

    crate::features::spaces::space_common::services::bump_participant_activity(
        cli, &space_pk, &member.pk,
    )
    .await;

    // Reward payout + XP recording run on EventBridge via SPACE_POST_COMMENT#
    // INSERT → handle_discussion_xp. Only the user's first contribution in a
    // discussion is rewarded (enforced by RewardPeriod::Once in the award helper).

    // Send mention notifications — deep-link to the newly created comment.
    // `Route::SpaceDiscussionCommentPage` matches `/spaces/:sid/discussions/:did/comments/:cid`
    // and `DiscussionArenaPage` scrolls + highlights the target comment on
    // mount. Comment id is in the path (not query/fragment) because Dioxus
    // Router strips both query strings and fragments during URL
    // normalization on hydration.
    {
        let comment_id_str = match &comment.sk {
            EntityType::SpacePostComment(id) => id.clone(),
            _ => String::new(),
        };
        let cta_url = format!(
            "{}/spaces/{}/discussions/{}/comments/{}",
            crate::common::config::site_base_url(),
            space_id,
            discussion_sk,
            comment_id_str,
        );

        crate::common::utils::mention::create_mention_notifications(
            cli,
            &comment.content,
            &member.pk,
            &member.display_name,
            &cta_url,
        )
        .await;
    }

    Ok(comment.into())
}
