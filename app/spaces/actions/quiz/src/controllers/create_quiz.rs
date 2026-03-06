use crate::*;

#[post("/api/spaces/{space_pk}/quizzes", role: SpaceUserRole)]
pub async fn create_quiz(space_pk: SpacePartition) -> Result<QuizResponse> {
    SpaceQuiz::can_edit(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let quiz = SpaceQuiz::new(space_pk.clone())?;

    quiz.create(cli).await?;

    let quiz_id: SpaceQuizEntityType = match &quiz.sk {
        EntityType::SpaceQuiz(id) => id.clone().into(),
        _ => SpaceQuizEntityType::default(),
    };
    let answers = quiz
        .questions
        .iter()
        .map(QuizCorrectAnswer::for_question)
        .collect::<Vec<_>>();
    let answer = SpaceQuizAnswer::new(space_pk, quiz_id, answers);
    answer.create(cli).await?;

    Ok(quiz.into())
}
