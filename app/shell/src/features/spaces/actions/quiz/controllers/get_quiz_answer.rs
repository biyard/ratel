use crate::features::spaces::actions::quiz::*;

#[get("/api/spaces/{space_pk}/quizzes/{quiz_id}/answers", role: SpaceUserRole)]
pub async fn get_quiz_answer(
    space_pk: SpacePartition,
    quiz_id: SpaceQuizEntityType,
) -> Result<QuizAnswerResponse> {
    SpaceQuiz::can_edit(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk_partition: Partition = space_pk.into();
    let answer_sk = EntityType::SpaceQuizAnswer(quiz_id.to_string());

    let answer = SpaceQuizAnswer::get(cli, &space_pk_partition, Some(answer_sk))
        .await?
        .ok_or(Error::NotFound("Quiz answer not found".into()))?;

    Ok(answer.into())
}
