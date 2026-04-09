use crate::common::models::space::{SpaceUser, SpaceCommon};
use crate::features::spaces::pages::actions::actions::quiz::*;
#[cfg(feature = "server")]
use crate::features::spaces::space_common::models::space_reward::SpaceReward;

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
    let quiz_action_id = quiz_id.to_string(); // UUID only, matches SpaceReward action_id
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

    if !crate::features::spaces::pages::actions::can_execute_space_action_legacy(
        role,
        space_action.prerequisite,
        space.status,
        space.join_anytime,
    ) {
        return Err(SpaceActionQuizError::NotAvailableInCurrentStatus.into());
    }

    let now = crate::common::utils::time::get_now_timestamp_millis();
    if now < space_action.started_at || now > space_action.ended_at {
        return Err(SpaceActionQuizError::NotInProgress.into());
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
    let attempt = SpaceQuizAttempt::new(quiz_id.clone(), member.clone(), req.answers, score);
    attempt.create(cli).await?;

    if attempts.is_empty() {
        SpaceQuiz::updater(&space_pk, &quiz_sk)
            .increase_user_response_count(1)
            .execute(cli)
            .await?;
    }

    let already_passed = attempts.iter().any(|a| a.score >= quiz.pass_score);
    if score >= quiz.pass_score && !already_passed {
        let activity_user_pk = user.pk.clone();
        let activity_user_name = member.display_name.clone();
        let activity_user_avatar = member.profile_url.clone();

        match SpaceReward::get_by_action(
            cli,
            space_id.clone(),
            quiz_action_id.clone(),
            RewardUserBehavior::QuizAnswer,
        )
        .await
        {
            Ok(space_reward) => {
                if let Err(e) =
                    SpaceReward::award(cli, &space_reward, user.pk, Some(space.user_pk.clone()))
                        .await
                {
                    tracing::error!(
                        space_pk = %space_id,
                        action_id = %quiz_sk,
                        error = %e,
                        "Failed to award quiz reward"
                    );
                }
            }
            Err(e) => {
                tracing::warn!(
                    space_pk = %space_id,
                    action_id = %quiz_sk,
                    error = %e,
                    "SpaceReward not found for quiz action"
                );
            }
        }

        {
            if let Err(e) = crate::features::activity::controllers::record_activity(
                cli,
                space_id.clone(),
                crate::features::activity::types::AuthorPartition::from(activity_user_pk),
                quiz_action_id.clone(),
                crate::features::spaces::pages::actions::types::SpaceActionType::Quiz,
                space_action.activity_score,
                space_action.additional_score,
                crate::features::activity::types::SpaceActivityData::Quiz {
                    quiz_id: quiz_id.to_string(),
                    passed: true,
                    correct_count: score as u32,
                    pass_threshold: quiz.pass_score as u32,
                },
                activity_user_name,
                activity_user_avatar,
            ).await {
                tracing::error!(error = %e, "Failed to record quiz activity");
            }
        }
    }
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
