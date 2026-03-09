use crate::features::spaces::actions::main::*;
#[cfg(feature = "server")]
use ratel_auth::models::user::OptionalUser;

// TODO: If bookmark-based pagination is needed, consider introducing a separate DynamoDB entity
#[get("/api/spaces/{space_pk}/actions", role: SpaceUserRole, user: OptionalUser)]
pub async fn list_actions(
    space_pk: SpacePartition,
    // bookmark: Option<String>,
) -> Result<Vec<SpaceAction>> {
    use std::collections::HashSet;

    let cli = crate::features::spaces::actions::main::config::get().common.dynamodb();
    let space_pk: Partition = space_pk.into();
    let poll_future = SpacePoll::query(
        cli,
        space_pk.clone(),
        SpacePoll::opt_all().sk(EntityType::SpacePoll(String::default()).to_string()),
    );
    let quiz_future = crate::features::spaces::actions::quiz::SpaceQuiz::query(
        cli,
        space_pk.clone(),
        crate::features::spaces::actions::quiz::SpaceQuiz::opt_all()
            .sk(EntityType::SpaceQuiz(String::default()).to_string()),
    );
    let discussion_future = crate::features::spaces::actions::discussion::SpacePost::query(
        cli,
        space_pk.clone(),
        crate::features::spaces::actions::discussion::SpacePost::opt_all()
            .sk(EntityType::SpacePost(String::default()).to_string()),
    );
    let subscription_future = crate::features::spaces::actions::subscription::SpaceSubscription::get(
        cli,
        &space_pk,
        Some(EntityType::SpaceSubscription),
    );
    let ((polls, _), (quizzes, _), (discussions, _), subscription) = tokio::try_join!(
        poll_future,
        quiz_future,
        discussion_future,
        subscription_future
    )
    .map_err(|e| Error::InternalServerError(format!("failed to load actions: {e:?}")))?;

    let mut actions: Vec<SpaceAction> = if let Some(user) = user.0 {
        let keys: Vec<_> = polls
            .iter()
            .map(|poll| SpacePollUserAnswer::keys(&user.pk, &poll.sk, &space_pk))
            .collect();
        let (user_participated,) = tokio::try_join!(SpacePollUserAnswer::batch_get(cli, keys))
            .map_err(|e| {
                Error::InternalServerError(format!("failed to load user poll participation: {e:?}"))
            })?;
        let participated_poll_sks: HashSet<String> = user_participated
            .into_iter()
            .filter_map(|a| match a.sk {
                EntityType::SpacePollUserAnswer(_, poll_sk) => {
                    Some(SpacePollUserAnswer::parse_wrong_sk(poll_sk))
                }
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

    let quiz_actions: Vec<SpaceAction> = quizzes.into_iter().map(Into::into).collect();
    actions.extend(quiz_actions);

    // Add discussions to the actions list
    let discussion_actions: Vec<SpaceAction> = discussions
        .into_iter()
        .map(|post| (post, role).into())
        .collect();
    actions.extend(discussion_actions);
    if let Some(subscription) = subscription {
        actions.push(subscription.into());
    }

    // Sort by updated_at descending
    actions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

    debug!("actions: {:?}", actions);
    Ok(actions)
}
