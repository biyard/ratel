use super::*;
#[cfg(feature = "server")]
use crate::common::models::auth::UserFollow;
#[cfg(feature = "server")]
use crate::common::models::space::SpaceCommon;
#[cfg(feature = "server")]
use crate::features::auth::models::user::OptionalUser;
#[cfg(feature = "server")]
use crate::features::spaces::pages::actions::actions::discussion::SpacePostComment;
#[cfg(feature = "server")]
use crate::features::spaces::pages::actions::actions::follow::SpaceFollowUser;
#[cfg(feature = "server")]
use crate::features::spaces::pages::actions::actions::poll::SpacePollUserAnswer;
#[cfg(feature = "server")]
use crate::features::spaces::pages::actions::actions::quiz::{SpaceQuiz, SpaceQuizAttempt};
#[cfg(feature = "server")]
use std::collections::HashSet;

#[get("/api/spaces/{space_pk}/actions", role: SpaceUserRole, user: OptionalUser)]
pub async fn list_actions(space_pk: SpacePartition) -> Result<Vec<SpaceActionSummary>> {
    let cli = crate::features::spaces::pages::actions::config::get()
        .common
        .dynamodb();
    let space_pk: Partition = space_pk.into();

    let (space_actions, _) = SpaceAction::find_by_space(cli, &space_pk, SpaceAction::opt())
        .await
        .map_err(|e| Error::InternalServerError(format!("failed to load actions: {e:?}")))?;

    let mut actions: Vec<SpaceActionSummary> = space_actions.into_iter().map(Into::into).collect();

    let current_user = user.0;
    for action in actions.iter_mut() {
        // Quiz: always enrich quiz scores
        if action.action_type == SpaceActionType::Quiz {
            let quiz_id: SpaceQuizEntityType = action.action_id.clone().into();
            let quiz_sk: EntityType = quiz_id.clone().into();

            if let Some(quiz) = SpaceQuiz::get(cli, &space_pk, Some(quiz_sk)).await? {
                action.quiz_total_score = Some(quiz.questions.len() as i64);

                if let Some(user) = current_user.as_ref() {
                    if let Some(attempt) =
                        SpaceQuizAttempt::find_latest_by_quiz_user(cli, &quiz_id, &user.pk).await?
                    {
                        action.quiz_score = Some(attempt.score);
                        action.quiz_passed = Some(attempt.score >= quiz.pass_score);
                        action.user_participated = true;
                    }
                }
            }
            continue;
        }

        // For non-quiz types, only check user_participated when prerequisite is true
        if !action.prerequisite {
            continue;
        }

        if let Some(user) = current_user.as_ref() {
            match action.action_type {
                SpaceActionType::Poll => {
                    let poll_sk = EntityType::SpacePoll(action.action_id.clone());
                    if let Ok(Some(_)) =
                        SpacePollUserAnswer::find_one(cli, &space_pk, &poll_sk, &user.pk).await
                    {
                        action.user_participated = true;
                    }
                }
                SpaceActionType::TopicDiscussion => {
                    action.user_participated =
                        has_completed_discussion_action(cli, &action.action_id, &user.pk).await?;
                }
                SpaceActionType::Follow => {
                    action.user_participated =
                        has_completed_follow_action(cli, &space_pk, &user.pk).await?;
                }
                _ => {}
            }
        }
    }

    // Filter out actions that haven't started yet (for non-creators)
    let now = crate::common::utils::time::get_now_timestamp_millis();
    if !matches!(role, SpaceUserRole::Creator) {
        actions.retain(|action| {
            action
                .started_at
                .map(|started_at| now >= started_at)
                .unwrap_or(false)
        });
    }

    // Sort by started_at descending
    actions.sort_by(|a, b| b.started_at.cmp(&a.started_at));

    // Pre-action filtering for non-creators
    if !matches!(role, SpaceUserRole::Creator) {
        let has_pre_actions = actions.iter().any(|a| a.prerequisite);

        if has_pre_actions {
            let all_pre_actions_done = actions
                .iter()
                .filter(|a| a.prerequisite)
                .all(|a| a.user_participated);

            if !all_pre_actions_done {
                // Check if we should still show all actions based on space settings
                let space = SpaceCommon::get(cli, &space_pk, Some(EntityType::SpaceCommon)).await?;
                let show_all = if let Some(space) = space {
                    // Join Anytime OFF + space InProgress → show all
                    !space.join_anytime
                        && (matches!(space.status, Some(SpaceStatus::Started))
                            || matches!(space.status, Some(SpaceStatus::Finished)))
                } else {
                    false
                };

                if !show_all {
                    // Only show pre-actions
                    actions.retain(|a| a.prerequisite);
                }
            }
        }
    }

    debug!("actions: {:?}", actions);
    Ok(actions)
}

#[cfg(feature = "server")]
async fn has_completed_discussion_action(
    cli: &aws_sdk_dynamodb::Client,
    action_id: &str,
    user_pk: &Partition,
) -> Result<bool> {
    let discussion_pk = Partition::SpacePost(action_id.to_string());
    let prefixes = [
        EntityType::SpacePostComment(String::default()).to_string(),
        EntityType::SpacePostCommentReply(String::default(), String::default()).to_string(),
    ];

    for sk_prefix in prefixes {
        let mut bookmark: Option<String> = None;

        loop {
            let opt = if let Some(next_bookmark) = bookmark.clone() {
                SpacePostComment::opt()
                    .sk(sk_prefix.clone())
                    .bookmark(next_bookmark)
                    .limit(100)
            } else {
                SpacePostComment::opt().sk(sk_prefix.clone()).limit(100)
            };

            let (comments, next_bookmark) = SpacePostComment::find_by_user_pk(cli, user_pk, opt)
                .await
                .map_err(|err| {
                    Error::InternalServerError(format!(
                        "failed to verify discussion prerequisite: {err}"
                    ))
                })?;

            if comments.iter().any(|comment| comment.pk == discussion_pk) {
                return Ok(true);
            }

            if next_bookmark.is_none() {
                break;
            }

            bookmark = next_bookmark;
        }
    }

    Ok(false)
}

#[cfg(feature = "server")]
async fn has_completed_follow_action(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    user_pk: &Partition,
) -> Result<bool> {
    let space = SpaceCommon::get(cli, space_pk, Some(EntityType::SpaceCommon))
        .await?
        .ok_or_else(|| Error::NotFound("space not found".to_string()))?;

    let mut target_user_pks = Vec::new();
    let mut bookmark: Option<String> = None;

    loop {
        let opt = if let Some(next_bookmark) = bookmark.clone() {
            SpaceFollowUser::opt()
                .sk(EntityType::SpaceSubscriptionUser(String::default()).to_string())
                .bookmark(next_bookmark)
                .limit(100)
        } else {
            SpaceFollowUser::opt()
                .sk(EntityType::SpaceSubscriptionUser(String::default()).to_string())
                .limit(100)
        };

        let (users, next_bookmark) = SpaceFollowUser::query(cli, space_pk.clone(), opt)
            .await
            .map_err(|err| {
                Error::InternalServerError(format!(
                    "failed to load follow prerequisite targets: {err}"
                ))
            })?;

        target_user_pks.extend(
            users
                .into_iter()
                .filter(|user| user.user_pk != Partition::None)
                .map(|user| user.user_pk),
        );

        if next_bookmark.is_none() {
            break;
        }

        bookmark = next_bookmark;
    }

    if !target_user_pks
        .iter()
        .any(|target| target == &space.user_pk)
    {
        target_user_pks.push(space.user_pk);
    }

    let mut deduped_targets = Vec::new();
    let mut seen = HashSet::new();
    for target_user_pk in target_user_pks {
        let target_key = target_user_pk.to_string();
        if seen.insert(target_key) {
            deduped_targets.push(target_user_pk);
        }
    }

    let keys: Vec<_> = deduped_targets
        .iter()
        .map(|target_user_pk| UserFollow::follower_keys(target_user_pk, user_pk))
        .collect();

    if keys.is_empty() {
        return Ok(true);
    }

    let follows = UserFollow::batch_get(cli, keys).await.map_err(|err| {
        Error::InternalServerError(format!("failed to verify follow prerequisite: {err}"))
    })?;

    Ok(follows.len() == deduped_targets.len())
}
