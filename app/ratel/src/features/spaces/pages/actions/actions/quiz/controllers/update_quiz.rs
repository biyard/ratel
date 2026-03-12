use crate::features::spaces::pages::actions::actions::quiz::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[post("/api/spaces/{space_pk}/quizzes/{quiz_id}", role: SpaceUserRole)]
pub async fn update_quiz(
    space_pk: SpacePartition,
    quiz_id: SpaceQuizEntityType,
    req: UpdateQuizRequest,
) -> Result<String> {
    SpaceQuiz::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_pk.into();
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
        return Err(Error::BadRequest(
            "Quiz cannot be edited after responses exist".into(),
        ));
    }

    let now = crate::common::utils::time::get_now_timestamp_millis();
    let mut updater = SpaceQuiz::updater(&space_pk, &quiz_sk).with_updated_at(now);

    if let Some(title) = req.title {
        updater = updater.with_title(title);
    }

    if let Some(description) = req.description {
        updater = updater.with_description(description);
    }

    if req.started_at.is_some() || req.ended_at.is_some() {
        let started_at = req
            .started_at
            .ok_or(Error::BadRequest("started_at is required".into()))?;
        let ended_at = req
            .ended_at
            .ok_or(Error::BadRequest("ended_at is required".into()))?;
        if started_at >= ended_at {
            return Err(Error::BadRequest("Invalid time range".into()));
        }
        updater = updater.with_started_at(started_at).with_ended_at(ended_at);
    }

    if let Some(retry_count) = req.retry_count {
        if retry_count < 0 {
            return Err(Error::BadRequest("Retry count must be >= 0".into()));
        }
        updater = updater.with_retry_count(retry_count);
    }

    let mut questions_for_answers = None;
    if let Some(questions) = req.questions {
        if questions.is_empty() {
            return Err(Error::BadRequest("Questions cannot be empty".into()));
        }
        if questions
            .iter()
            .any(|q| !matches!(q, Question::SingleChoice(_) | Question::MultipleChoice(_)))
        {
            return Err(Error::BadRequest(
                "Quiz only supports choice questions".into(),
            ));
        }
        let description = questions
            .first()
            .map(|q| q.title().to_string())
            .unwrap_or_default();
        updater = updater
            .with_questions(questions.clone())
            .with_description(description);
        questions_for_answers = Some(questions);
    }

    let questions_for_validation = questions_for_answers
        .as_ref()
        .unwrap_or(&existing.questions);

    if let Some(pass_score) = req.pass_score {
        if pass_score < 0 {
            return Err(Error::BadRequest("Pass score must be >= 0".into()));
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

    if let Some(answers) = req.answers {
        let questions = questions_for_answers.unwrap_or_else(|| existing.questions.clone());

        if questions.len() != answers.len() {
            return Err(Error::BadRequest("Answers length mismatch".into()));
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
                    return Err(Error::BadRequest("Invalid single answer index".into()));
                }
            }
            Ok(())
        }
        (Question::MultipleChoice(q), QuizCorrectAnswer::Multiple { answers }) => {
            if answers
                .iter()
                .any(|idx| *idx < 0 || (*idx as usize) >= q.options.len())
            {
                return Err(Error::BadRequest("Invalid multiple answer index".into()));
            }
            Ok(())
        }
        _ => Err(Error::BadRequest(
            "Answer type does not match question".into(),
        )),
    }
}
