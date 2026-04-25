use crate::features::spaces::pages::actions::actions::meet::*;
use crate::features::spaces::space_common::models::aggregate::DashboardAggregate;

#[mcp_tool(
    name = "delete_meet",
    description = "Delete a meet action from a space. Requires creator role."
)]
#[delete("/api/spaces/{space_pk}/meets/{meet_sk}", role: SpaceUserRole)]
pub async fn delete_meet(
    #[mcp(description = "Space partition key")] space_pk: SpacePartition,
    #[mcp(description = "Meet sort key (e.g. 'SpaceMeet#<uuid>')")] meet_sk: SpaceMeetEntityType,
) -> Result<String> {
    SpaceMeet::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_pk.into();
    let meet_sk_entity: EntityType = meet_sk.into();

    let _meet = SpaceMeet::get(cli, &space_pk, Some(meet_sk_entity.clone()))
        .await?
        .ok_or(MeetActionError::NotFound)?;

    SpaceMeet::delete(cli, &space_pk, Some(meet_sk_entity))
        .await
        .map_err(|e| {
            crate::error!("delete meet failed: {e}");
            MeetActionError::DeleteFailed
        })?;

    // Decrement meet count in aggregate
    let agg_item = DashboardAggregate::inc_meets(&space_pk, -1);
    crate::transact_write_items!(cli, vec![agg_item]).ok();

    Ok("success".to_string())
}
