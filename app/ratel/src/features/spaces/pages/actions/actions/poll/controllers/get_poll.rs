use crate::features::auth::OptionalUser;

use crate::features::spaces::pages::actions::actions::poll::*;

#[get("/api/spaces/{space_pk}/polls/{poll_sk}", role: SpaceUserRole, user: OptionalUser)]
pub async fn get_poll(
    space_pk: SpacePartition,
    poll_sk: SpacePollEntityType,
) -> Result<PollResponse> {
    SpacePoll::can_view(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_pk.into();
    let poll_sk_entity: EntityType = poll_sk.clone().into();

    let poll = SpacePoll::get(cli, &space_pk, Some(poll_sk_entity.clone()))
        .await?
        .ok_or(Error::NotFound("Poll not found".into()))?;

    let mut response: PollResponse = poll.into();

    let space_action = crate::features::spaces::pages::actions::models::SpaceAction::get(
        cli,
        &CompositePartition(space_pk.clone(), poll_sk.clone()),
        Some(EntityType::SpaceAction),
    )
    .await?
    .ok_or(Error::SpaceActionNotFound)?;

    response.space_action = space_action;

    if let Some(user) = user.0 {
        let my_answer =
            SpacePollUserAnswer::find_one(cli, &space_pk, &poll_sk_entity, &user.pk).await?;
        response.my_response = my_answer.map(|a| a.answers);
    }

    Ok(response)
}
