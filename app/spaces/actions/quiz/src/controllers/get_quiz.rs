use crate::*;

#[get("/api/spaces/{space_pk}/quizzes/{quiz_id}", role: SpaceUserRole)]
pub async fn get_quiz(space_pk: SpacePartition, quiz_id: SpaceQuizEntityType) -> Result<QuizResponse> {
    SpaceQuiz::can_view(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_pk.into();
    let quiz_sk: EntityType = quiz_id.into();

    let quiz = SpaceQuiz::get(cli, &space_pk, Some(quiz_sk))
        .await?
        .ok_or(Error::NotFound("Quiz not found".into()))?;

    Ok(quiz.into())
}
