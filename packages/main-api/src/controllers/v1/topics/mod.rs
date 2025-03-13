#![allow(dead_code)]
pub mod comments;
pub mod votes;
use bdk::prelude::*;

use by_axum::{
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, Query, State},
        routing::{get, post},
    },
};
use dto::*;

#[cfg(test)]
mod tests;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct TopicPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct TopicControllerV1 {
    pool: sqlx::Pool<sqlx::Postgres>,
    repo: TopicRepository,
    user: UserRepository,
    like: TopicLikeRepository,
}

impl TopicControllerV1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Topic::get_repository(pool.clone());
        let user = User::get_repository(pool.clone());
        let like = TopicLike::get_repository(pool.clone());
        let ctrl = TopicControllerV1 {
            pool: pool.clone(),
            repo,
            user,
            like,
        };

        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_topic).get(Self::list_topic))
            .with_state(ctrl.clone())
            .route(
                "/:topic_id",
                get(Self::get_topic).post(Self::act_topic_by_id),
            )
            .with_state(ctrl.clone())
            .nest(
                "/:topic_id/comments",
                comments::CommentControllerV1::route(pool.clone())?,
            )
            .nest("/:topic_id/votes", votes::VoteControllerV1::route(pool)?))
    }

    pub async fn act_topic(
        State(ctrl): State<TopicControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<TopicAction>,
    ) -> Result<Json<Topic>> {
        tracing::debug!("act_topic {:?}", body);
        match body {
            TopicAction::Create(req) => ctrl.create_topic(req).await,
        }
    }

    pub async fn act_topic_by_id(
        State(ctrl): State<TopicControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(TopicPath { id }): Path<TopicPath>,
        Json(body): Json<TopicByIdAction>,
    ) -> Result<Json<Topic>> {
        tracing::debug!("act_topic_by_id {:?}", id);

        match body {
            TopicByIdAction::Like(_) => ctrl.like(id).await,
            TopicByIdAction::Unlike(_) => ctrl.unlike(id).await,
        }
    }

    pub async fn get_topic(
        State(ctrl): State<TopicControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(TopicPath { id }): Path<TopicPath>,
    ) -> Result<Json<Topic>> {
        tracing::debug!("get_topic {:?}", id);
        let user = ctrl
            .user
            .find_one(&UserReadAction::new().user_info())
            .await?;

        let topic: Topic = Topic::query_builder(user.id)
            .id_equals(id)
            .query()
            .map(|r: sqlx::postgres::PgRow| r.into())
            .fetch_one(&ctrl.pool)
            .await?;

        Ok(Json(topic))
    }

    pub async fn list_topic(
        State(ctrl): State<TopicControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(params): Query<TopicParam>,
    ) -> Result<Json<TopicGetResponse>> {
        tracing::debug!("list_topic {:?}", params);

        match params {
            TopicParam::Query(q) => Ok(Json(TopicGetResponse::Query(ctrl.repo.find(&q).await?))),
        }
    }
}

impl TopicControllerV1 {
    pub async fn create_topic(&self, body: TopicCreateRequest) -> Result<Json<Topic>> {
        tracing::debug!("create_topic {:?}", body);

        if body.title.is_empty() || body.content.is_empty() {
            return Err(ServiceError::BadRequest);
        }

        if body.ended_at < chrono::Utc::now().timestamp() {
            return Err(ServiceError::BadRequest);
        }

        match body.status {
            TopicStatus::Ongoing | TopicStatus::Finished | TopicStatus::Cancelled => {
                return Err(ServiceError::BadRequest);
            }
            _ => {}
        }

        let topic = self
            .repo
            .insert(
                body.ended_at,
                // body.user_id,
                body.title,
                body.content,
                None,
                TopicResult::default(),
                body.status,
                body.legislation_link,
                body.solutions,
                body.discussions,
                body.additional_resources,
            )
            .await?;

        Ok(Json(topic))
    }

    async fn like(&self, id: i64) -> Result<Json<Topic>> {
        let user = self
            .user
            .find_one(&UserReadAction::new().user_info())
            .await?;

        let topic: Topic = Topic::query_builder(user.id)
            .id_equals(id)
            .query()
            .map(|r: sqlx::postgres::PgRow| r.into())
            .fetch_one(&self.pool)
            .await?;

        self.like.insert(user.id, topic.id).await?;

        let topic = Topic::query_builder(user.id)
            .id_equals(id)
            .query()
            .map(|r: sqlx::postgres::PgRow| r.into())
            .fetch_one(&self.pool)
            .await?;

        Ok(Json(topic))
    }

    async fn unlike(&self, id: i64) -> Result<Json<Topic>> {
        self.like.delete(id).await?;

        let user = self
            .user
            .find_one(&UserReadAction::new().user_info())
            .await?;

        let topic = Topic::query_builder(user.id)
            .id_equals(id)
            .query()
            .map(|r: sqlx::postgres::PgRow| r.into())
            .fetch_one(&self.pool)
            .await?;

        Ok(Json(topic))
    }
}
