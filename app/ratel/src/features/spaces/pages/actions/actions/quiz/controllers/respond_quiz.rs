use crate::features::spaces::pages::actions::actions::quiz::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RespondQuizRequest {
    pub answers: Vec<Answer>,
}

#[post(
    "/api/spaces/{space_pk}/quizzes/{quiz_id}/respond",
    role: SpaceUserRole,
    user: crate::features::auth::User
)]
pub async fn respond_quiz(
    space_pk: SpacePartition,
    quiz_id: SpaceQuizEntityType,
    req: RespondQuizRequest,
) -> Result<String> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_pk.into();
    let quiz_sk: EntityType = quiz_id.clone().into();

    let quiz = SpaceQuiz::get(cli, &space_pk, Some(quiz_sk.clone()))
        .await?
        .ok_or(Error::NotFound("Quiz not found".into()))?;

    quiz.can_respond(&role)?;

    let answer_sk = EntityType::SpaceQuizAnswer(quiz_id.to_string());
    let correct = SpaceQuizAnswer::get(cli, &space_pk, Some(answer_sk))
        .await?
        .ok_or(Error::NotFound("Quiz answer not found".into()))?;

    if !crate::features::spaces::pages::actions::actions::poll::types::validate_answers(quiz.questions.clone(), req.answers.clone()) {
        return Err(Error::BadRequest("Answers do not match questions".into()));
    }

    let attempts =
        SpaceQuizAttempt::list_by_quiz_user(cli, &quiz_id, &user.pk, quiz.retry_count as i32)
            .await?;
    if attempts.len() as i64 >= quiz.retry_count {
        return Err(Error::BadRequest("No remaining submissions".into()));
    }

    let score = calculate_score(&quiz.questions, &correct.answers, &req.answers)?;
    let attempt = SpaceQuizAttempt::new(quiz_id.clone(), user.pk.clone(), req.answers, score);
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
