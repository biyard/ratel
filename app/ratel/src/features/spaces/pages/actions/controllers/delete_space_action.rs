use super::*;
#[cfg(feature = "server")]
use crate::common::{EntityType, Partition};
#[cfg(feature = "server")]
use crate::features::spaces::space_common::models::aggregate::DashboardAggregate;
#[cfg(feature = "server")]
use crate::features::spaces::space_common::models::dashboard::aggregate as _;

#[delete("/api/spaces/{space_id}/actions/{action_id}", role: SpaceUserRole, space: crate::common::models::space::SpaceCommon)]
pub async fn delete_space_action(space_id: SpacePartition, action_id: String) -> Result<()> {
    if role != SpaceUserRole::Creator {
        return Err(Error::NoPermission);
    }

    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let action_pk = CompositePartition(space_id.clone(), action_id.clone());
    let space_action = SpaceAction::get(cli, &action_pk, Some(EntityType::SpaceAction))
        .await?
        .ok_or(Error::NotFound("Action not found".into()))?;

    // Once the action has been published (Ongoing / Finish) it is locked —
    // creators can no longer delete it. Designing / legacy (None) actions
    // remain deletable.
    if crate::features::spaces::pages::actions::is_action_locked(
        space.status.clone(),
        space_action.status.as_ref(),
    ) {
        return Err(Error::ActionLocked);
    }

    let space_pk: Partition = space_id.into();

    match space_action.space_action_type {
        SpaceActionType::Poll => {
            let poll_sk = EntityType::SpacePoll(action_id);
            let poll = crate::features::spaces::pages::actions::actions::poll::SpacePoll::get(
                cli,
                &space_pk,
                Some(poll_sk.clone()),
            )
            .await?
            .ok_or(Error::NotFound("Poll not found".into()))?;

            if poll.user_response_count > 0 {
                return Err(SpaceActionError::ActionDeleteFailed.into());
            }

            let txs = vec![
                crate::features::spaces::pages::actions::actions::poll::SpacePoll::delete_transact_write_item(
                    &space_pk,
                    &poll_sk,
                ),
                SpaceAction::delete_transact_write_item(&space_action.pk, &EntityType::SpaceAction),
                DashboardAggregate::inc_polls(&space_pk, -1),
            ];
            crate::transact_write_items!(cli, txs).map_err(|e| {
                crate::error!("Failed to delete poll action: {e}");
                SpaceActionError::ActionDeleteFailed
            })?;
        }
        SpaceActionType::TopicDiscussion => {
            let discussion_sk = EntityType::SpacePost(action_id);
            let discussion =
                crate::features::spaces::pages::actions::actions::discussion::SpacePost::get(
                    cli,
                    &space_pk,
                    Some(discussion_sk.clone()),
                )
                .await?
                .ok_or(
                    crate::features::spaces::pages::actions::actions::discussion::SpaceActionDiscussionError::NotFound,
                )?;

            if discussion.comments > 0 {
                return Err(SpaceActionError::ActionDeleteFailed.into());
            }

            let txs = vec![
                crate::features::spaces::pages::actions::actions::discussion::SpacePost::delete_transact_write_item(
                    &space_pk,
                    &discussion_sk,
                ),
                SpaceAction::delete_transact_write_item(&space_action.pk, &EntityType::SpaceAction),
                DashboardAggregate::inc_posts(&space_pk, -1),
            ];
            crate::transact_write_items!(cli, txs).map_err(|e| {
                crate::error!("Failed to delete discussion action: {e}");
                SpaceActionError::ActionDeleteFailed
            })?;
        }
        SpaceActionType::Quiz => {
            let quiz_sk = EntityType::SpaceQuiz(action_id.clone());
            let quiz = crate::features::spaces::pages::actions::actions::quiz::SpaceQuiz::get(
                cli,
                &space_pk,
                Some(quiz_sk.clone()),
            )
            .await?
            .ok_or(Error::NotFound("Quiz not found".into()))?;

            if quiz.user_response_count > 0 {
                return Err(SpaceActionError::ActionDeleteFailed.into());
            }

            let quiz_answer_sk = EntityType::SpaceQuizAnswer(action_id);
            let mut txs = vec![
                crate::features::spaces::pages::actions::actions::quiz::SpaceQuiz::delete_transact_write_item(
                    &space_pk,
                    &quiz_sk,
                ),
                SpaceAction::delete_transact_write_item(&space_action.pk, &EntityType::SpaceAction),
            ];

            if crate::features::spaces::pages::actions::actions::quiz::SpaceQuizAnswer::get(
                cli,
                &space_pk,
                Some(quiz_answer_sk.clone()),
            )
            .await?
            .is_some()
            {
                txs.push(
                    crate::features::spaces::pages::actions::actions::quiz::SpaceQuizAnswer::delete_transact_write_item(
                        &space_pk,
                        &quiz_answer_sk,
                    ),
                );
            }

            crate::transact_write_items!(cli, txs).map_err(|e| {
                crate::error!("Failed to delete quiz action: {e}");
                SpaceActionError::ActionDeleteFailed
            })?;
        }
        SpaceActionType::Follow => {
            let follow_sk = EntityType::SpaceActionFollow(action_id);
            let mut txs = vec![
                crate::features::spaces::pages::actions::actions::follow::SpaceFollowAction::delete_transact_write_item(
                    &space_pk,
                    &follow_sk,
                ),
                SpaceAction::delete_transact_write_item(&space_action.pk, &EntityType::SpaceAction),
            ];

            let mut bookmark: Option<String> = None;
            loop {
                let mut opt =
                    crate::features::spaces::pages::actions::actions::follow::SpaceFollowUser::opt(
                    )
                    .sk(EntityType::SpaceSubscriptionUser(String::default()).to_string())
                    .limit(100);
                if let Some(bk) = bookmark.clone() {
                    opt = opt.bookmark(bk);
                }

                let (users, next_bookmark) = crate::features::spaces::pages::actions::actions::follow::SpaceFollowUser::query(
                    cli,
                    space_pk.clone(),
                    opt,
                )
                .await?;

                for user in users {
                    txs.push(
                        crate::features::spaces::pages::actions::actions::follow::SpaceFollowUser::delete_transact_write_item(
                            &user.pk,
                            &user.sk,
                        ),
                    );
                }

                if next_bookmark.is_none() {
                    break;
                }
                bookmark = next_bookmark;
            }

            crate::transact_write_all_items!(cli, txs);
        }
        SpaceActionType::Meet => {
            let meet_sk = EntityType::SpaceMeet(action_id);
            let meet = crate::features::spaces::pages::actions::actions::meet::SpaceMeet::get(
                cli,
                &space_pk,
                Some(meet_sk.clone()),
            )
            .await?
            .ok_or(Error::NotFound("Meet not found".into()))?;
            let _ = meet;

            let txs = vec![
                crate::features::spaces::pages::actions::actions::meet::SpaceMeet::delete_transact_write_item(
                    &space_pk,
                    &meet_sk,
                ),
                SpaceAction::delete_transact_write_item(&space_action.pk, &EntityType::SpaceAction),
                DashboardAggregate::inc_meets(&space_pk, -1),
            ];
            crate::transact_write_items!(cli, txs).map_err(|e| {
                crate::error!("Failed to delete meet action: {e}");
                SpaceActionError::ActionDeleteFailed
            })?;
        }
    }

    Ok(())
}
