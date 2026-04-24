use crate::common::models::space::{SpaceUser, SpaceCommon};
use crate::features::spaces::pages::actions::actions::quiz::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct RespondQuizRequest {
    pub answers: Vec<Answer>,
}

#[mcp_tool(name = "respond_quiz", description = "Submit answers to a quiz. Requires participant role. Returns score.")]
#[post(
    "/api/spaces/{space_pk}/quizzes/{quiz_id}/respond",
    role: SpaceUserRole,
    member: SpaceUser,
    user: crate::features::auth::User,
    space: SpaceCommon
)]
pub async fn respond_quiz(
    #[mcp(description = "Space partition key")]
    space_pk: SpacePartition,
    #[mcp(description = "Quiz sort key (e.g. 'SpaceQuiz#<uuid>')")]
    quiz_id: SpaceQuizEntityType,
    #[mcp(description = "Quiz answers. Each answer: {\"answer_type\": \"single_choice\", \"answer\": <index>} or {\"answer_type\": \"multiple_choice\", \"answer\": [<indices>]}")]
    req: RespondQuizRequest,
) -> Result<()> {
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
        &CompositePartition(space_id.clone(), quiz_id.clone()),
        Some(EntityType::SpaceAction),
    )
    .await?
    .ok_or(Error::SpaceActionNotFound)?;

    let deps_met = crate::features::spaces::pages::actions::services::dependency::dependencies_met(
        cli,
        &space,
        &space_action,
        &member.pk,
    )
    .await?;

    if !crate::features::spaces::pages::actions::can_execute_space_action(
        role,
        space_action.prerequisite,
        space.status,
        space_action.status.as_ref(),
        deps_met,
        space.join_anytime,
    ) {
        return Err(SpaceActionQuizError::NotAvailableInCurrentStatus.into());
    }

    let answer_sk = EntityType::SpaceQuizAnswer(quiz_id.to_string());
    let correct = SpaceQuizAnswer::get(cli, &space_pk, Some(answer_sk))
        .await?
        .ok_or(Error::NotFound("Quiz answer not found".into()))?;

    if !crate::features::spaces::pages::actions::actions::poll::types::validate_answers(
        quiz.questions.clone(),
        req.answers.clone(),
    ) {
        return Err(SpaceActionQuizError::AnswersMismatch.into());
    }

    let total_allowed = quiz.retry_count.saturating_add(1).min(MAX_TOTAL_ATTEMPTS);
    let limit: i32 = total_allowed as i32;
    let attempts = SpaceQuizAttempt::list_by_quiz_user(cli, &quiz_id, &member.pk, limit).await?;
    if attempts.len() as i64 >= total_allowed {
        return Err(SpaceActionQuizError::NoRemainingAttempts.into());
    }
    let score = calculate_score(&quiz.questions, &correct.answers, &req.answers)?;
    let attempt = SpaceQuizAttempt::new(space_id.clone(), quiz_id.clone(), member.clone(), req.answers, score, quiz.pass_score);
    attempt.create(cli).await?;

    if attempts.is_empty() {
        SpaceQuiz::updater(&space_pk, &quiz_sk)
            .increase_user_response_count(1)
            .execute(cli)
            .await?;
    }

    // Reward payout + XP recording run on EventBridge via SPACE_QUIZ_ATTEMPT#
    // INSERT → handle_quiz_xp. See features/activity/services/handle_xp_event.rs.
    let _ = attempts; // kept above for aggregate side-effects only
    let _ = score;

    crate::features::spaces::space_common::services::bump_participant_activity(
        cli, &space_pk, &member.pk,
    )
    .await;

    Ok(())
}

fn calculate_score(
    questions: &[Question],
    correct: &[QuizCorrectAnswer],
    answers: &[Answer],
) -> Result<i64> {
    if questions.len() != correct.len() || questions.len() != answers.len() {
        return Err(SpaceActionQuizError::AnswersMismatch.into());
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
