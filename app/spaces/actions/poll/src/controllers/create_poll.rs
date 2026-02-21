use crate::*;
use ratel_auth::User;

#[post("/api/polls/{space_pk}/create", user: User)]
pub async fn create_poll(space_pk: SpacePartition) -> Result<PollResponse> {
    let cli = crate::config::get().common.dynamodb();

    let poll = SpacePoll::new(space_pk)?;
    poll.create(cli).await?;

    Ok(poll.into())
}
