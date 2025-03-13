#![allow(dead_code)]
use bdk::prelude::*;
use by_axum::{
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, Query, State},
        routing::{get, post},
    },
};
use by_types::QueryResponse;
use dto::*;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct VotePath {
    topic_id: i64,
}

#[derive(Clone, Debug)]
pub struct VoteControllerV1 {
    pool: sqlx::Pool<sqlx::Postgres>,
    repo: VoteRepository,
    user: UserRepository,
}

impl VoteControllerV1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Vote::get_repository(pool.clone());
        let user = User::get_repository(pool.clone());
        let ctrl = VoteControllerV1 { pool, repo, user };

        Ok(by_axum::axum::Router::new()
            .route(
                "/",
                post(Self::act_vote).get(Self::get_vote), // .post(Self::act_vote_by_id)
            )
            .with_state(ctrl.clone())
            .route("/all", get(Self::list_vote))
            .with_state(ctrl.clone())
            .route("/result", get(Self::get_final_result))
            .with_state(ctrl.clone()))
    }

    pub async fn act_vote(
        State(ctrl): State<VoteControllerV1>,
        Path(VotePath { topic_id }): Path<VotePath>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<VoteAction>,
    ) -> Result<Json<Vote>> {
        tracing::debug!("act_vote {} {:?}", topic_id, body);

        match body {
            VoteAction::Voting(req) => ctrl.vote(topic_id, req).await,
        }
    }

    pub async fn get_vote(
        State(ctrl): State<VoteControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(VotePath { topic_id }): Path<VotePath>,
    ) -> Result<Json<Vote>> {
        tracing::debug!("get_vote {}", topic_id);

        let vote: Vote = Vote::query_builder()
            .id_equals(topic_id)
            .query()
            .map(|r: sqlx::postgres::PgRow| r.into())
            .fetch_one(&ctrl.pool)
            .await?;

        Ok(Json(vote))
    }

    pub async fn list_vote(
        State(ctrl): State<VoteControllerV1>,
        Path(VotePath { topic_id }): Path<VotePath>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(param): Query<VoteParam>,
    ) -> Result<Json<VoteGetResponse>> {
        tracing::debug!("list_vote {} {:?}", topic_id, param);

        match param {
            // VoteParam::Query(q) => Ok(Json(VoteGetResponse::Query(ctrl.repo.find(&q).await?))),
            VoteParam::Query(_) => ctrl.list_votes(topic_id).await,
        }
    }

    pub async fn get_final_result(
        State(ctrl): State<VoteControllerV1>,
        Path(VotePath { topic_id }): Path<VotePath>,
        Extension(_auth): Extension<Option<Authorization>>,
    ) -> Result<Json<VoteResultSummary>> {
        tracing::debug!("get_final_result {}", topic_id);

        ctrl.vote_result_summary(topic_id).await
    }
}

impl VoteControllerV1 {
    async fn vote(&self, topic_id: i64, body: VoteVotingRequest) -> Result<Json<Vote>> {
        if body.amount < 0 {
            return Err(ServiceError::BadRequest);
        }

        let user = self
            .user
            .find_one(&UserReadAction::new().user_info())
            .await?;

        let vote = self
            .repo
            .insert(body.vote, body.amount, user.id, topic_id)
            .await?;

        Ok(Json(vote))
    }

    async fn vote_result_summary(&self, topic_id: i64) -> Result<Json<VoteResultSummary>> {
        let items: Vec<VoteSummary> = Vote::query_builder()
            .topic_id_equals(topic_id)
            .query()
            .map(|r: sqlx::postgres::PgRow| r.into())
            .fetch_all(&self.pool)
            .await?;

        // FIXME: need conditional sum
        Ok(Json(VoteResultSummary {
            pros: items
                .iter()
                .filter(|r| r.vote == VoteResult::Supportive)
                .map(|r| r.amount) // `amount` 필드를 합산
                .sum::<i64>(),
            cons: items
                .iter()
                .filter(|r| r.vote == VoteResult::Against)
                .map(|r| r.amount) // `amount` 필드를 합산
                .sum::<i64>(),
            neutral: items
                .iter()
                .filter(|r| r.vote == VoteResult::Neutral)
                .map(|r| r.amount) // `amount` 필드를 합산
                .sum::<i64>(),
        }))
    }

    async fn list_votes(&self, topic_id: i64) -> Result<Json<VoteGetResponse>> {
        // FIXME: topic_id_equals not working @hackartist
        let mut total_count: i64 = 0;
        let votes: Vec<VoteSummary> = Vote::query_builder()
            .topic_id_equals(topic_id)
            .with_count()
            .query()
            .map(|r: sqlx::postgres::PgRow| {
                use sqlx::Row;
                total_count = r.get("total_count");
                r.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(Json(VoteGetResponse::Query(QueryResponse {
            items: votes,
            total_count,
        })))
    }
}
