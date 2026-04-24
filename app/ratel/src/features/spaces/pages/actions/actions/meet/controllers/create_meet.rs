use crate::features::spaces::pages::actions::actions::meet::*;
use crate::features::spaces::pages::actions::models::SpaceAction;
use crate::features::spaces::space_common::models::aggregate::DashboardAggregate;

#[mcp_tool(
    name = "create_meet",
    description = "Create a new meet action in a space. Requires creator role."
)]
#[post("/api/spaces/{space_pk}/meets", role: SpaceUserRole, space: crate::common::models::space::SpaceCommon)]
pub async fn create_meet(
    #[mcp(description = "Space partition key")] space_pk: SpacePartition,
) -> Result<MeetResponse> {
    SpaceMeet::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let meet = SpaceMeet::new(space_pk.clone())?;

    let space_action = SpaceAction::new(
        space_pk.clone(),
        SpaceMeetEntityType::from(meet.sk.clone()).to_string(),
        crate::features::spaces::pages::actions::types::SpaceActionType::Meet,
    );

    let space_pk_partition: Partition = space_pk.into();
    let _ = DashboardAggregate::get_or_create(cli, &space_pk_partition).await?;

    let mut items = vec![
        meet.create_transact_write_item(),
        space_action.create_transact_write_item(),
    ];
    items.push(DashboardAggregate::inc_meets(&space_pk_partition, 1));
    crate::transact_write_items!(cli, items).map_err(|e| {
        crate::error!("Failed to create meet: {e}");
        MeetActionError::CreateFailed
    })?;

    let mut ret: MeetResponse = meet.into();
    ret.space_action = space_action;
    Ok(ret)
}
