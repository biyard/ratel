use crate::*;

// TODO: If bookmark-based pagination is needed, consider introducing a separate DynamoDB entity
#[get("/api/actions")]
pub async fn list_actions(
    space_pk: SpacePartition,
    // bookmark: Option<String>,
) -> Result<Vec<SpaceAction>> {
    let cli = crate::config::get().common.dynamodb();
    let poll_future = SpacePoll::query(cli, space_pk, SpacePoll::opt_all());
    let ((polls, _),) = tokio::try_join!(poll_future)?;

    let actions = polls.into_iter().map(|poll| poll.into()).collect();
    Ok(actions)
}
