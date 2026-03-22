use crate::common::models::space::{SpaceAuthor, SpaceCommon};
use crate::features::spaces::pages::actions::actions::quiz::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RespondQuizRequest {
    pub answers: Vec<Answer>,
}

#[post(
    "/api/spaces/{space_pk}/quizzes/{quiz_id}/respond",
    role: SpaceUserRole,
    author: SpaceAuthor,
    space: SpaceCommon
)]
pub async fn respond_quiz(
    space_pk: SpacePartition,
    quiz_id: SpaceQuizEntityType,
    req: RespondQuizRequest,
) -> Result<String> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_id = space_pk;
    let space_pk: Partition = space_id.clone().into();
    let quiz_sk: EntityType = quiz_id.clone().into();

    let quiz = SpaceQuiz::get(cli, &space_pk, Some(quiz_sk.clone()))
        .await?
        .ok_or(Error::NotFound("Quiz not found".into()))?;

    let space_action = crate::features::spaces::pages::actions::models::SpaceAction::get(
        cli,
        &CompositePartition(space_id, quiz_id.clone()),
        Some(EntityType::SpaceAction),
    )
    .await?
    .ok_or(Error::SpaceActionNotFound)?;

    if !crate::features::spaces::pages::actions::can_execute_space_action(
        role,
        space_action.prerequisite,
        space.status,
    ) {
        return Err(Error::BadRequest(
            "Quiz is not available in the current space status".into(),
        ));
    }

    let now = crate::common::utils::time::get_now_timestamp_millis();
    if now < space_action.started_at || now > space_action.ended_at {
        return Err(Error::BadRequest("Quiz is not in progress".into()));
    }

    let answer_sk = EntityType::SpaceQuizAnswer(quiz_id.to_string());
    let correct = SpaceQuizAnswer::get(cli, &space_pk, Some(answer_sk))
        .await?
        .ok_or(Error::NotFound("Quiz answer not found".into()))?;

    if !crate::features::spaces::pages::actions::actions::poll::types::validate_answers(
        quiz.questions.clone(),
        req.answers.clone(),
    ) {
        return Err(Error::BadRequest("Answers do not match questions".into()));
    }

    let total_allowed = quiz.retry_count.saturating_add(1);
    let limit: i32 = total_allowed.try_into().unwrap_or(i32::MAX);
    let attempts =
        SpaceQuizAttempt::list_by_quiz_user(cli, &quiz_id, &author.pk, limit)
            .await?;
    if attempts.len() as i64 >= total_allowed {
        return Err(Error::BadRequest("No remaining attempts".into()));
    }

    let score = calculate_score(&quiz.questions, &correct.answers, &req.answers)?;
    let attempt = SpaceQuizAttempt::new(quiz_id.clone(), author, req.answers, score);
    attempt.create(cli).await?;

    if attempts.is_empty() {
        SpaceQuiz::updater(&space_pk, &quiz_sk)
            .increase_user_response_count(1)
            .execute(cli)
            .await?;
    }

    Ok("success".to_string())
}

fn calculate_score(
    questions: &[Question],
    correct: &[QuizCorrectAnswer],
    answers: &[Answer],
) -> Result<i64> {
    if questions.len() != correct.len() || questions.len() != answers.len() {
        return Err(Error::BadRequest("Answers do not match questions".into()));
    }

    let mut score = 0;
    for ((question, correct), answer) in questions.iter().zip(correct).zip(answers) {
        let is_correct = match (question, correct, answer) {
            (
                Question::SingleChoice(_),
                QuizCorrectAnswer::Single { answer: expected },
                Answer::SingleChoice { answer: actual, .. },
            ) => expected.is_some() && expected == actual,
            (
                Question::MultipleChoice(_),
                QuizCorrectAnswer::Multiple { answers: expected },
                Answer::MultipleChoice { answer: actual, .. },
            ) => {
                let mut expected = expected.clone();
                expected.sort_unstable();
                expected.dedup();

                let mut actual = actual.clone().unwrap_or_default();
                actual.sort_unstable();
                actual.dedup();

                expected == actual && !expected.is_empty()
            }
            _ => false,
        };

        if is_correct {
            score += 1;
        }
    }

    Ok(score)
}
