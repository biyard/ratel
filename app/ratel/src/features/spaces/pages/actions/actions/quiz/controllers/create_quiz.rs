use crate::features::spaces::pages::actions::actions::quiz::*;

#[post("/api/spaces/{space_pk}/quizzes", role: SpaceUserRole)]
pub async fn create_quiz(space_pk: SpacePartition) -> Result<QuizResponse> {
    SpaceQuiz::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let quiz = SpaceQuiz::new(space_pk.clone())?;

    let space_action = crate::features::spaces::pages::actions::models::SpaceAction::new(
        space_pk.clone(),
        SpaceQuizEntityType::from(quiz.sk.clone()).to_string(),
        crate::features::spaces::pages::actions::types::SpaceActionType::Quiz,
    );
    let items = vec![
        quiz.create_transact_write_item(),
        space_action.create_transact_write_item(),
    ];
    crate::transact_write_items!(cli, items).map_err(|e| {
        crate::features::spaces::pages::actions::actions::quiz::Error::Unknown(format!(
            "Failed to create quiz: {e}"
        ))
    })?;

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

    let mut response: QuizResponse = quiz.into();
    response.title = space_action.title.clone();
    response.description = space_action.description.clone();
    response.started_at = space_action.started_at;
    response.ended_at = space_action.ended_at;
    response.space_action = space_action;

    Ok(response)
}
