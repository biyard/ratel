use crate::features::spaces::pages::actions::actions::quiz::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct UpdateQuizRequest {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub started_at: Option<i64>,
    #[serde(default)]
    pub ended_at: Option<i64>,
    #[serde(default)]
    pub retry_count: Option<i64>,
    #[serde(default)]
    pub pass_score: Option<i64>,
    #[serde(default)]
    pub questions: Option<Vec<Question>>,
    #[serde(default)]
    pub answers: Option<Vec<QuizCorrectAnswer>>,
    #[serde(default)]
    pub files: Option<Vec<File>>,
}

#[mcp_tool(name = "update_quiz", description = "Update a quiz (title, description, time, questions, answers, pass_score, retry_count, files). Requires creator role.")]
#[post("/api/spaces/{space_pk}/quizzes/{quiz_id}", role: SpaceUserRole, space: crate::common::models::space::SpaceCommon)]
pub async fn update_quiz(
    #[mcp(description = "Space partition key")]
    space_pk: SpacePartition,
    #[mcp(description = "Quiz sort key (e.g. 'SpaceQuiz#<uuid>')")]
    quiz_id: SpaceQuizEntityType,
    #[mcp(description = "Quiz update data as JSON. Fields: title, description, started_at, ended_at, retry_count, pass_score, questions, answers, files (all optional)")]
    req: UpdateQuizRequest,
) -> Result<String> {
    SpaceQuiz::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_id = space_pk;
    let space_pk: Partition = space_id.clone().into();
    let quiz_sk: EntityType = quiz_id.clone().into();

    let existing = SpaceQuiz::get(cli, &space_pk, Some(quiz_sk.clone()))
        .await?
        .ok_or(Error::NotFound("Quiz not found".into()))?;
    let updates_locked_fields = req.started_at.is_some()
        || req.ended_at.is_some()
        || req.retry_count.is_some()
        || req.pass_score.is_some()
        || req.questions.is_some()
        || req.answers.is_some();

    if existing.user_response_count > 0 && updates_locked_fields {
        return Err(SpaceActionQuizError::CannotEditAfterResponses.into());
    }

    let now = crate::common::utils::time::get_now_timestamp_millis();

    // Lock all quiz edits once the action has started. UI already
    // disables inputs; defend the API surface here too.
    let action_pk_for_check = CompositePartition(space_id.clone(), quiz_id.to_string());
    let space_action_check =
        crate::features::spaces::pages::actions::models::SpaceAction::get(
            cli,
            &action_pk_for_check,
            Some(EntityType::SpaceAction),
        )
        .await
        .map_err(|e| Error::InternalServerError(format!("Failed to get space action: {e:?}")))?
        .ok_or(Error::NotFound("Space action not found".into()))?;
    if crate::features::spaces::pages::actions::is_action_locked(
        space.status.clone(),
        space_action_check.started_at,
    ) {
        return Err(Error::BadRequest(
            "Quiz cannot be edited after the action has started".into(),
        ));
    }
    let mut updater = SpaceQuiz::updater(&space_pk, &quiz_sk).with_updated_at(now);
    let action_pk = CompositePartition(space_id, quiz_id.to_string());
    let action_sk = EntityType::SpaceAction;
    let mut action_updater = crate::features::spaces::pages::actions::models::SpaceAction::updater(
        &action_pk, &action_sk,
    )
    .with_updated_at(now);
    let mut should_update_action = false;

    if let Some(title) = req.title {
        action_updater = action_updater.with_title(title);
        should_update_action = true;
    }

    if let Some(description) = req.description {
        action_updater = action_updater.with_description(description);
        should_update_action = true;
    }

    if req.started_at.is_some() || req.ended_at.is_some() {
        let started_at = req
            .started_at
            .ok_or(Error::SpaceActionQuiz(SpaceActionQuizError::StartedAtRequired))?;
        let ended_at = req
            .ended_at
            .ok_or(Error::SpaceActionQuiz(SpaceActionQuizError::EndedAtRequired))?;
        if started_at >= ended_at {
            return Err(SpaceActionQuizError::InvalidTimeRange.into());
        }
        action_updater = action_updater
            .with_started_at(started_at)
            .with_ended_at(ended_at);
        should_update_action = true;
    }

    if let Some(retry_count) = req.retry_count {
        if retry_count < 0 {
            return Err(SpaceActionQuizError::InvalidRetryCount.into());
        }
        if retry_count.saturating_add(1) > MAX_TOTAL_ATTEMPTS {
            return Err(SpaceActionQuizError::RetryCountExceedsMax.into());
        }
        updater = updater.with_retry_count(retry_count);
    }

    let mut questions_for_answers = None;
    if let Some(questions) = req.questions {
        if questions.is_empty() {
            return Err(SpaceActionQuizError::EmptyQuestions.into());
        }
        if questions
            .iter()
            .any(|q| !matches!(q, Question::SingleChoice(_) | Question::MultipleChoice(_)))
        {
            return Err(SpaceActionQuizError::UnsupportedQuestionType.into());
        }
        updater = updater.with_questions(questions.clone());
        questions_for_answers = Some(questions);
    }

    let questions_for_validation = questions_for_answers
        .as_ref()
        .unwrap_or(&existing.questions);

    if let Some(pass_score) = req.pass_score {
        if pass_score < 0 {
            return Err(SpaceActionQuizError::InvalidPassScore.into());
        }
        updater = updater.with_pass_score(pass_score);
    }

    if let Some(mut files) = req.files {
        for file in &mut files {
            if file.id.is_empty() {
                file.id = crate::common::uuid::Uuid::now_v7().to_string();
            }
        }
        updater = updater.with_files(files);
    }

    updater.execute(cli).await?;
    if should_update_action {
        action_updater.execute(cli).await?;
    }

    if let Some(answers) = req.answers {
        let questions = questions_for_answers.unwrap_or_else(|| existing.questions.clone());

        if questions.len() != answers.len() {
            return Err(SpaceActionQuizError::AnswersLengthMismatch.into());
        }
        for (question, answer) in questions.iter().zip(answers.iter()) {
            validate_quiz_answer(question, answer)?;
        }

        let answer_sk = EntityType::SpaceQuizAnswer(quiz_id.to_string());
        let answer_updater = SpaceQuizAnswer::updater(&space_pk, &answer_sk)
            .with_created_at(now)
            .with_updated_at(now)
            .with_space_pk(space_pk.clone())
            .with_answers(answers);
        answer_updater.execute(cli).await?;
    }

    Ok("success".to_string())
}

fn validate_quiz_answer(question: &Question, answer: &QuizCorrectAnswer) -> Result<()> {
    match (question, answer) {
        (Question::SingleChoice(q), QuizCorrectAnswer::Single { answer }) => {
            if let Some(idx) = answer {
                if *idx < 0 || (*idx as usize) >= q.options.len() {
                    return Err(SpaceActionQuizError::InvalidSingleAnswerIndex.into());
                }
            }
            Ok(())
        }
        (Question::MultipleChoice(q), QuizCorrectAnswer::Multiple { answers }) => {
            if answers
                .iter()
                .any(|idx| *idx < 0 || (*idx as usize) >= q.options.len())
            {
                return Err(SpaceActionQuizError::InvalidMultipleAnswerIndex.into());
            }
            Ok(())
        }
        _ => Err(SpaceActionQuizError::AnswerTypeMismatch.into()),
    }
}
