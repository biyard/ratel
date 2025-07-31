use crate::utils::users::extract_user_id;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, State},
        routing::{get, post},
    },
};
use by_types::QueryResponse;
use dto::*;
use sqlx::postgres::PgRow;
use std::collections::HashSet;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct SpaceIdPath {
    pub space_id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct SubmitAnswersRequest {
    pub answers: Vec<NoticeQuestionWithAnswer>,
}

#[derive(Clone, Debug)]
pub struct SpaceNoticeQuizAttemptController {
    // repo: NoticeQuizAttemptRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl SpaceNoticeQuizAttemptController {
    async fn get_attempts(
        &self,
        space_id: i64,
        auth: Option<Authorization>,
    ) -> Result<QueryResponse<NoticeQuizAttempt>> {
        let mut total_count = 0;
        let user_id = extract_user_id(&self.pool, auth.clone())
            .await
            .map_err(|_| Error::Unauthorized)?;

        let items = NoticeQuizAttempt::query_builder()
            .space_id_equals(space_id)
            .user_id_equals(user_id)
            .order_by_created_at_desc()
            .with_count()
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;
                total_count = row.try_get("total_count").unwrap_or_default();
                NoticeQuizAttempt::from(row)
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }
}

impl SpaceNoticeQuizAttemptController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/", get(Self::query_attempts))
            .route("/submit", post(Self::submit_answers))
            .with_state(self.clone())
    }

    pub async fn query_attempts(
        State(ctrl): State<SpaceNoticeQuizAttemptController>,
        Path(SpaceIdPath { space_id }): Path<SpaceIdPath>,
        Extension(auth): Extension<Option<Authorization>>,
    ) -> Result<Json<QueryResponse<NoticeQuizAttempt>>> {
        tracing::debug!("get_latest space_id: {}", space_id);

        let auth = auth.ok_or(Error::Unauthorized)?;

        Ok(Json(
            ctrl.get_attempts(space_id, Some(auth)).await?,
        ))
    }

    pub async fn submit_answers(
        State(ctrl): State<SpaceNoticeQuizAttemptController>,
        Path(SpaceIdPath { space_id }): Path<SpaceIdPath>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<SubmitAnswersRequest>,
    ) -> Result<Json<NoticeQuizAttempt>> {
        tracing::debug!(
            "submit_answers space_id: {}, answers: {:?}",
            space_id,
            body.answers
        );

        let auth = auth.ok_or(Error::Unauthorized)?;
        let user_id = extract_user_id(&ctrl.pool, Some(auth.clone())).await?;

        // Check if user already has 3 failed attempts
        let existing_attempts = NoticeQuizAttempt::query_builder()
            .space_id_equals(space_id)
            .user_id_equals(user_id)
            .order_by_created_at_desc()
            .query()
            .map(NoticeQuizAttempt::from)
            .fetch_all(&ctrl.pool)
            .await?;

        // Count only failed attempts for the limit check
        let failed_attempts_count = existing_attempts
            .iter()
            .filter(|attempt| !attempt.is_successful)
            .count();
        if failed_attempts_count >= 3 {
            return Err(Error::Unauthorized);
        }

        // Check if the user's last attempt was successful
        // If so, don't allow another submission since they already got the correct answer
        if let Some(last_attempt) = existing_attempts.first() {
            if last_attempt.is_successful {
                return Err(Error::Unauthorized);
            }
        }

        // Get the quiz with correct answers from NoticeQuizAnswer table
        let correct_quiz = NoticeQuizAnswer::query_builder()
            .space_id_equals(space_id)
            .query()
            .map(NoticeQuizAnswer::from)
            .fetch_optional(&ctrl.pool)
            .await?
            .ok_or(Error::NotFound)?;

        // Use HashSet for O(1) lookup of correct answers
        let mut correct_answers_map: std::collections::HashMap<String, HashSet<String>> =
            std::collections::HashMap::new();

        for (idx, question) in correct_quiz.notice_quiz.iter().enumerate() {
            let mut correct_options = HashSet::new();
            for option in &question.options {
                if option.is_correct {
                    correct_options.insert(option.content.clone());
                }
            }
            correct_answers_map.insert(idx.to_string(), correct_options);
        }

        // Validate user answers and calculate score
        let mut correct_count = 0;
        let total_questions = body.answers.len() as i32;

        for (idx, user_question) in body.answers.iter().enumerate() {
            if let Some(correct_options) = correct_answers_map.get(&idx.to_string()) {
                let user_correct_options: HashSet<String> = user_question
                    .options
                    .iter()
                    .filter(|opt| opt.is_correct)
                    .map(|opt| opt.content.clone())
                    .collect();

                if user_correct_options == *correct_options {
                    correct_count += 1;
                }
            }
        }

        let score = if total_questions > 0 {
            (correct_count as f64) / (total_questions as f64) * 100.0
        } else {
            0.0
        };

        let is_successful = score == 100.0; // Assuming 100% is passing

        // Create the attempt record using the repository pattern
        let repo = NoticeQuizAttempt::get_repository(ctrl.pool.clone());
        let mut tx = ctrl.pool.begin().await?;

        let attempt = repo
            .insert_with_tx(
                &mut *tx,
                space_id,
                user_id,
                body.answers,
                is_successful,
            )
            .await?;

        tx.commit().await?;

        Ok(Json(attempt.unwrap_or_default()))
    }
}
