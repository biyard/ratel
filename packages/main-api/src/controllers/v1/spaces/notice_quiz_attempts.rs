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
use dto::{*};
use sqlx::postgres::PgRow;

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
    pub answers: NoticeAnswer, // Use the new HashMap-based structure
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
        let correct_quiz_answer = NoticeQuizAnswer::query_builder()
            .space_id_equals(space_id)
            .query()
            .map(NoticeQuizAnswer::from)
            .fetch_optional(&ctrl.pool)
            .await?
            .ok_or(Error::NotFound)?;

        // Use the pre-computed HashMap for O(1) lookup of correct answers
        let correct_answers_map = &correct_quiz_answer.answers.answers;

        // Calculate the result by comparing user answers with correct answers
        let mut correct_count = 0;
        let total_questions = correct_answers_map.len();

        for (question_id, correct_options) in correct_answers_map {
            if let Some(user_options) = body.answers.answers.get(question_id) {
                // Check if user's selected options match exactly with correct options
                if user_options == correct_options {
                    correct_count += 1;
                }
            }
        }

        let is_successful = correct_count == total_questions;

        // Insert the quiz attempt with results using manual parameter approach
        let attempt_repo = NoticeQuizAttempt::get_repository(ctrl.pool.clone());
        let quiz_attempt = attempt_repo
            .insert(space_id, user_id, body.answers, is_successful)
            .await?;

        tracing::info!(
            "Quiz attempt created for user {} in space {} with result: {} correct out of {} questions",
            user_id,
            space_id,
            correct_count,
            total_questions
        );

        Ok(Json(quiz_attempt))
    }
}
