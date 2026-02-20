use crate::*;
use ratel_auth::models::user::OptionalUser;
// TODO: If bookmark-based pagination is needed, consider introducing a separate DynamoDB entity
#[get("/api/actions", user: OptionalUser)]
pub async fn list_actions(
    space_pk: SpacePartition,
    // bookmark: Option<String>,
) -> Result<Vec<SpaceAction>> {
    let cli = crate::config::get().common.dynamodb();
    let poll_future = SpacePoll::query(cli, space_pk, SpacePoll::opt_all());
    let ((polls, _),) = tokio::try_join!(poll_future)?;

    // Check User Participated
    // batch_get
    // let keys = polls
    //     .iter()
    //     .map(|poll| SpacePollUserAnswer::key(space_pk, poll.pk))
    //     .collect();
    // let user_participated_future = SpacePollUserAnswer::batch_get(cli, keys);
    // let ((user_participated, _),) = tokio::try_join!(user_participated_future)?;
    let actions = polls.into_iter().map(|poll| (poll, false).into()).collect();
    Ok(actions)
}
