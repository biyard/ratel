use crate::*;
use space_common::models::aggregate::DashboardAggregate;

#[delete("/api/spaces/{space_id}/discussions/{discussion_sk}", role: SpaceUserRole)]
pub async fn delete_discussion(
    space_id: SpacePartition,
    discussion_sk: SpacePostEntityType,
) -> Result<()> {
    SpacePost::can_edit(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.into();
    let discussion_sk_entity: EntityType = discussion_sk.into();

    SpacePost::delete(cli, &space_pk, Some(discussion_sk_entity)).await?;

    // Decrement post count in aggregate
    let agg_item = DashboardAggregate::inc_posts(&space_pk, -1);
    transact_write_items!(cli, vec![agg_item]).ok();

    Ok(())
}
