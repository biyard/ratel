use crate::features::auth::OptionalUser;

use crate::features::spaces::actions::quiz::*;

#[get("/api/spaces/{space_pk}/quizzes/{quiz_id}", role: SpaceUserRole, user: OptionalUser)]
pub async fn get_quiz(
    space_pk: SpacePartition,
    quiz_id: SpaceQuizEntityType,
) -> Result<QuizResponse> {
    SpaceQuiz::can_view(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_pk.into();
    let quiz_sk: EntityType = quiz_id.clone().into();

    let quiz = SpaceQuiz::get(cli, &space_pk, Some(quiz_sk))
        .await?
        .ok_or(Error::NotFound("Quiz not found".into()))?;

    let mut response: QuizResponse = quiz.into();

    if let Some(user) = user.0 {
        let limit = response.retry_count.max(1) as i32;
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
