use crate::features::spaces::pages::actions::actions::meet::*;
use crate::features::spaces::pages::actions::models::SpaceAction;

#[mcp_tool(
    name = "get_meet",
    description = "Fetch a meet action with its companion SpaceAction row."
)]
#[get("/api/spaces/{space_pk}/meets/{meet_sk}", role: SpaceUserRole)]
pub async fn get_meet(
    #[mcp(description = "Space partition key")] space_pk: SpacePartition,
    #[mcp(description = "Meet sort key (e.g. 'SpaceMeet#<uuid>')")] meet_sk: SpaceMeetEntityType,
) -> Result<MeetResponse> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_id = space_pk;
    let meet_id = meet_sk;
    let space_pk: Partition = space_id.clone().into();
    let meet_sk_entity: EntityType = meet_id.clone().into();

    let meet = SpaceMeet::get(cli, &space_pk, Some(meet_sk_entity.clone()))
        .await?
        .ok_or(MeetActionError::NotFound)?;

    let mut response: MeetResponse = meet.into();

    let space_action = SpaceAction::get(
        cli,
        &CompositePartition(space_id.clone(), meet_id.clone()),
        Some(EntityType::SpaceAction),
    )
    .await?
    .ok_or(Error::SpaceActionNotFound)?;

    response.space_action = space_action;

    Ok(response)
}
