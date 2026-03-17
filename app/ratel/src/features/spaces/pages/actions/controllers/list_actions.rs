use super::*;
#[cfg(feature = "server")]
use crate::features::auth::models::user::OptionalUser;
#[cfg(feature = "server")]
use crate::features::spaces::pages::actions::actions::quiz::{SpaceQuiz, SpaceQuizAttempt};

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
        if action.action_type != SpaceActionType::Quiz {
            continue;
        }

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
                }
            }
        }
    }

    // Sort by started_at descending
    actions.sort_by(|a, b| b.started_at.cmp(&a.started_at));

    debug!("actions: {:?}", actions);
    Ok(actions)
}
