use crate::common::models::space::{SpaceCommon, SpaceUser};
use crate::features::spaces::pages::actions::actions::discussion::*;

#[mcp_tool(
    name = "unsubscribe_discussion",
    description = "Unsubscribe the current member from a discussion so they stop receiving comment notifications and emails."
)]
#[delete("/api/spaces/{space_id}/discussions/{discussion_sk}/subscribe", role: SpaceUserRole, member: SpaceUser, _space: SpaceCommon)]
pub async fn unsubscribe_discussion(
    #[mcp(description = "Space partition key")] space_id: SpacePartition,
    #[mcp(description = "Discussion sort key (e.g. 'SpacePost#<uuid>')")]
    discussion_sk: SpacePostEntityType,
) -> Result<()> {
    SpacePost::can_view(&role)?;
    // The discussion author (space Creator) always stays subscribed to their own
    // discussion — reject unsubscribe defensively (the UI also disables it).
    if matches!(role, SpaceUserRole::Creator) {
        return Err(SpaceActionDiscussionError::CannotUnsubscribeOwnDiscussion.into());
    }
    let _ = space_id;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let discussion_sk_entity: EntityType = discussion_sk.into();
    let space_post_pk = match &discussion_sk_entity {
        EntityType::SpacePost(id) => SpacePostPartition(id.clone()),
        _ => return Err(SpaceActionDiscussionError::InvalidDiscussionId.into()),
    };

    let (pk, sk) = SpacePostSubscription::keys(&space_post_pk, &member.pk);
    // Deleting a non-existent row is a no-op, so unsubscribe is idempotent.
    let item = SpacePostSubscription::delete_transact_write_item(&pk, &sk);
    crate::transact_write_items!(cli, vec![item]).map_err(|e| {
        crate::error!("unsubscribe_discussion failed: {e}");
        SpaceActionDiscussionError::CreateFailed
    })?;

    Ok(())
}
