use crate::features::auth::OptionalUser;

use crate::features::spaces::pages::actions::actions::quiz::*;

#[get("/api/spaces/{space_pk}/quizzes/{quiz_id}", role: SpaceUserRole, user: OptionalUser)]
pub async fn get_quiz(
    space_pk: SpacePartition,
    quiz_id: SpaceQuizEntityType,
) -> Result<QuizResponse> {
    SpaceQuiz::can_view(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_id = space_pk;
    let space_pk: Partition = space_id.clone().into();
    let quiz_sk: EntityType = quiz_id.clone().into();

    let quiz = SpaceQuiz::get(cli, &space_pk, Some(quiz_sk))
        .await?
        .ok_or(Error::NotFound("Quiz not found".into()))?;

    let mut response: QuizResponse = quiz.into();

    let space_action = crate::features::spaces::pages::actions::models::SpaceAction::get(
        cli,
        &CompositePartition(space_id, quiz_id.clone()),
        Some(EntityType::SpaceAction),
    )
    .await?
    .ok_or(Error::SpaceActionNotFound)?;

    response.title = space_action.title.clone();
    response.description = space_action.description.clone();
    response.started_at = space_action.started_at;
    response.ended_at = space_action.ended_at;
    response.space_action = space_action;

    if let Some(user) = user.0 {
        let limit: i32 = response
            .retry_count
            .saturating_add(1)
            .try_into()
            .unwrap_or(i32::MAX);
        let attempts = SpaceQuizAttempt::list_by_quiz_user(cli, &quiz_id, &user.pk, limit).await?;
        response.attempt_count = attempts.len() as i64;
        if let Some(latest) = attempts.first() {
            response.my_response = Some(latest.answers.clone());
            response.my_score = Some(latest.score);
            response.passed = Some(latest.score >= response.pass_score);
        }
    }

    Ok(response)
}
