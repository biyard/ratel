use crate::features::auth::OptionalUser;

use crate::features::spaces::pages::actions::actions::poll::*;

#[mcp_tool(name = "get_poll", description = "Get poll details including questions and the current user's response.")]
#[get("/api/spaces/{space_pk}/polls/{poll_sk}", role: SpaceUserRole, user: OptionalUser)]
pub async fn get_poll(
    #[mcp(description = "Space partition key")]
    space_pk: SpacePartition,
    #[mcp(description = "Poll sort key (e.g. 'SpacePoll#<uuid>')")]
    poll_sk: SpacePollEntityType,
) -> Result<PollResponse> {
    SpacePoll::can_view(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_id = space_pk;
    let poll_id = poll_sk;
    let space_pk: Partition = space_id.clone().into();
    let poll_sk_entity: EntityType = poll_id.clone().into();

    let poll = SpacePoll::get(cli, &space_pk, Some(poll_sk_entity.clone()))
        .await?
        .ok_or(Error::NotFound("Poll not found".into()))?;

    let mut response: PollResponse = poll.into();

    let space_action = crate::features::spaces::pages::actions::models::SpaceAction::get(
        cli,
        &CompositePartition(space_id.clone(), poll_id.clone()),
        Some(EntityType::SpaceAction),
    )
    .await?
    .ok_or(Error::SpaceActionNotFound)?;

    response.space_action = space_action;

    // Prerequisite polls are available as soon as the space is Open, regardless of
    // their individual started_at timer. Override NotStarted → InProgress so the
    // frontend allows interaction for Candidates completing prerequisites.
    if response.space_action.prerequisite && response.status == PollStatus::NotStarted {
        response.status = PollStatus::InProgress;
    }

    if let Some(user) = user.0 {
        let my_answer =
            SpacePollUserAnswer::find_one(cli, &space_pk, &poll_sk_entity, &user.pk).await?;
        response.my_response = my_answer.map(|a| a.answers);
    }

    Ok(response)
}
