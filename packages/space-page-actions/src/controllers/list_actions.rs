use std::collections::HashSet;

use crate::*;
use ratel_auth::models::user::OptionalUser;
// TODO: If bookmark-based pagination is needed, consider introducing a separate DynamoDB entity
#[get("/api/actions", user: OptionalUser)]
pub async fn list_actions(
    space_pk: SpacePartition,
    // bookmark: Option<String>,
) -> Result<Vec<SpaceAction>> {
    let cli = crate::config::get().common.dynamodb();

    let poll_future = SpacePoll::query(cli, space_pk.clone(), SpacePoll::opt_all());
    let ((polls, _),) = tokio::try_join!(poll_future)?;

    let actions = if let Some(user) = user.0 {
        let space_pk_partition: Partition = space_pk.into();

        let keys: Vec<_> = polls
            .iter()
            .map(|poll| SpacePollUserAnswer::keys(&user.pk, &poll.sk, &space_pk_partition))
            .collect();
        let (user_participated,) = tokio::try_join!(SpacePollUserAnswer::batch_get(cli, keys))?;

        let participated_poll_sks: HashSet<String> = user_participated
            .into_iter()
            .filter_map(|a| match a.sk {
                EntityType::SpacePollUserAnswer(_, poll_sk) => Some(poll_sk),
                _ => None,
            })
            .collect();

        polls
            .into_iter()
            .map(|poll| {
                let participated = participated_poll_sks.contains(&poll.sk.to_string());
                (poll, participated).into()
            })
            .collect()
    } else {
        polls.into_iter().map(|poll| (poll, false).into()).collect()
    };

    Ok(actions)
}
