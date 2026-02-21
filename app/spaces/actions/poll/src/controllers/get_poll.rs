use crate::*;
use ratel_auth::models::user::OptionalUser;

#[get("/api/polls/{space_pk}/{poll_sk}", user: OptionalUser)]
pub async fn get_poll(
    space_pk: SpacePartition,
    poll_sk: SpacePollEntityType,
) -> Result<PollResponse> {
    let cli = crate::config::get().common.dynamodb();
    let space_pk: Partition = space_pk.into();
    let poll_sk_entity: EntityType = poll_sk.into();

    let poll = SpacePoll::get(cli, &space_pk, Some(poll_sk_entity.clone()))
        .await?
        .ok_or(Error::NotFound("Poll not found".into()))?;

    let mut response: PollResponse = poll.into();

    if let Some(user) = user.0 {
        let my_answer =
            SpacePollUserAnswer::find_one(cli, &space_pk, &poll_sk_entity, &user.pk).await?;
        response.my_response = my_answer.map(|a| a.answers);
    }

    Ok(response)
}
