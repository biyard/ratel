use crate::features::spaces::pages::actions::actions::discussion::*;
use crate::features::spaces::space_common::models::aggregate::DashboardAggregate;

#[mcp_tool(name = "delete_discussion", description = "Delete a discussion from a space. Requires creator role.")]
#[delete("/api/spaces/{space_id}/discussions/{discussion_sk}", role: SpaceUserRole)]
pub async fn delete_discussion(
    #[mcp(description = "Space partition key")]
    space_id: SpacePartition,
    #[mcp(description = "Discussion sort key (e.g. 'SpacePost#<uuid>')")]
    discussion_sk: SpacePostEntityType,
) -> Result<()> {
    SpacePost::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.into();
    let discussion_sk_entity: EntityType = discussion_sk.into();

    SpacePost::delete(cli, &space_pk, Some(discussion_sk_entity)).await?;

    // Decrement post count in aggregate
    let agg_item = DashboardAggregate::inc_posts(&space_pk, -1);
    crate::transact_write_items!(cli, vec![agg_item]).ok();

    Ok(())
}
