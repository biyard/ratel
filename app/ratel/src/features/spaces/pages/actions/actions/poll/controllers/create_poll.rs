use crate::features::spaces::pages::actions::actions::poll::*;
use crate::features::spaces::pages::actions::models::SpaceAction;
use crate::features::spaces::space_common::models::aggregate::DashboardAggregate;

#[mcp_tool(name = "create_poll", description = "Create a new poll action in a space. Requires creator role.")]
#[post("/api/spaces/{space_pk}/polls", role: SpaceUserRole)]
pub async fn create_poll(
    #[mcp(description = "Space partition key")]
    space_pk: SpacePartition,
) -> Result<PollResponse> {
    SpacePoll::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let poll = SpacePoll::new(space_pk.clone())?;

    let space_action = SpaceAction::new(
        space_pk.clone(),
        SpacePollEntityType::from(poll.sk.clone()).to_string(),
        crate::features::spaces::pages::actions::types::SpaceActionType::Poll,
    );

    let space_pk_partition: Partition = space_pk.into();
    let _ = DashboardAggregate::get_or_create(cli, &space_pk_partition).await?;

    let mut items = vec![
        poll.create_transact_write_item(),
        space_action.create_transact_write_item(),
    ];
    items.push(DashboardAggregate::inc_polls(&space_pk_partition, 1));
    crate::transact_write_items!(cli, items)
        .map_err(|e| Error::Unknown(format!("Failed to create poll: {e}")))?;

    let mut ret: PollResponse = poll.into();
    ret.space_action = space_action;

    Ok(ret)
}
