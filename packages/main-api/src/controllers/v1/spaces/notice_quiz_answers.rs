use crate::utils::users::extract_user_id;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, State},
        routing::get,
    },
};
use dto::*;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct SpaceIdPath {
    pub space_id: i64,
}

#[derive(Clone, Debug)]
pub struct SpaceNoticeQuizAnswersController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl SpaceNoticeQuizAnswersController {
    async fn get_answers(
        &self,
        space_id: i64,
        auth: Option<Authorization>,
    ) -> Result<NoticeQuizAnswer> {
        let user_id = extract_user_id(&self.pool, auth.clone())
            .await
            .map_err(|_| Error::Unauthorized)?;

        let space = Space::query_builder(user_id)
            .id_equals(space_id)
            .query()
            .map(Space::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| Error::NotFound)?;

        if space.owner_id != user_id {
            return Err(Error::Unauthorized);
        }

        let item = NoticeQuizAnswer::query_builder()
            .space_id_equals(space_id)
            .query()
            .map(NoticeQuizAnswer::from)
            .fetch_one(&self.pool)
            .await?;

        Ok(item)
    }
}

impl SpaceNoticeQuizAnswersController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/", get(Self::query_attempts))
            .with_state(self.clone())
    }

    pub async fn query_attempts(
        State(ctrl): State<SpaceNoticeQuizAnswersController>,
        Path(SpaceIdPath { space_id }): Path<SpaceIdPath>,
        Extension(auth): Extension<Option<Authorization>>,
    ) -> Result<Json<NoticeQuizAnswer>> {
        tracing::debug!("get_latest space_id: {}", space_id);

        let auth = auth.ok_or(Error::Unauthorized)?;

        Ok(Json(ctrl.get_answers(space_id, Some(auth)).await?))
    }
}
