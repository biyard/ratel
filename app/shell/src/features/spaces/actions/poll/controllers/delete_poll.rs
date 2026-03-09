use crate::features::spaces::actions::poll::*;
use crate::features::spaces::space_common::models::aggregate::DashboardAggregate;

#[delete("/api/spaces/{space_pk}/polls/{poll_sk}", role: SpaceUserRole)]
pub async fn delete_poll(space_pk: SpacePartition, poll_sk: SpacePollEntityType) -> Result<String> {
    SpacePoll::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_pk.into();
    let poll_sk_entity: EntityType = poll_sk.into();

    let _poll = SpacePoll::get(cli, &space_pk, Some(poll_sk_entity.clone()))
        .await?
        .ok_or(Error::NotFound("Poll not found".into()))?;

    SpacePoll::delete(cli, &space_pk, Some(poll_sk_entity)).await?;

    // Decrement poll count in aggregate
    let agg_item = DashboardAggregate::inc_polls(&space_pk, -1);
    crate::transact_write_items!(cli, vec![agg_item]).ok();

    Ok("success".to_string())
}
