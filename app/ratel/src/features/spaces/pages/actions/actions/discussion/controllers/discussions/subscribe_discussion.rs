use crate::common::models::space::{SpaceCommon, SpaceUser};
use crate::features::spaces::pages::actions::actions::discussion::*;

#[mcp_tool(
    name = "subscribe_discussion",
    description = "Subscribe the current member to a discussion. While subscribed, they receive a notification and email for every new comment or reply. Idempotent."
)]
#[post("/api/spaces/{space_id}/discussions/{discussion_sk}/subscribe", role: SpaceUserRole, member: SpaceUser, _space: SpaceCommon)]
pub async fn subscribe_discussion(
    #[mcp(description = "Space partition key")] space_id: SpacePartition,
    #[mcp(description = "Discussion sort key (e.g. 'SpacePost#<uuid>')")]
    discussion_sk: SpacePostEntityType,
) -> Result<()> {
    SpacePost::can_view(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let discussion_sk_entity: EntityType = discussion_sk.into();
    let space_post_pk = match &discussion_sk_entity {
        EntityType::SpacePost(id) => SpacePostPartition(id.clone()),
        _ => return Err(SpaceActionDiscussionError::InvalidDiscussionId.into()),
    };

    // `create` is a conditional put (attribute_not_exists), so guard with a
    // point read to keep subscribe idempotent — already subscribed is success.
    let (pk, sk) = SpacePostSubscription::keys(&space_post_pk, &member.pk);
    let exists = SpacePostSubscription::get(cli, &pk, Some(sk))
        .await
        .map_err(|e| {
            crate::error!("subscribe_discussion existence check failed: {e}");
            SpaceActionDiscussionError::CreateFailed
        })?
        .is_some();
    if exists {
        return Ok(());
    }

    let sub = SpacePostSubscription::new(space_post_pk, space_id, &member.pk);
    sub.create(cli).await.map_err(|e| {
        crate::error!("subscribe_discussion failed: {e}");
        SpaceActionDiscussionError::CreateFailed
    })?;

    Ok(())
}
