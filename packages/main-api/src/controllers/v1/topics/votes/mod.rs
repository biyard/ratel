#![allow(dead_code)]
use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use by_types::QueryResponse;
use dto::*;

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
        Path(parent_id): Path<String>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<VoteAction>,
    ) -> Result<Json<Vote>> {
        tracing::debug!("act_vote {} {:?}", parent_id, body);

        match body {
            VoteAction::Voting(req) => ctrl.vote(parent_id, req).await,
        }
    }

    pub async fn get_vote(
        State(ctrl): State<VoteControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(parent_id): Path<String>,
    ) -> Result<Json<Vote>> {
        tracing::debug!("get_vote {}", parent_id);

        let id = parent_id.parse::<i64>()?;

        let vote: Vote = Vote::query_builder()
            .id_equals(id)
            .query()
            .map(|r: sqlx::postgres::PgRow| r.into())
            .fetch_one(&ctrl.pool)
            .await?;

        Ok(Json(vote))
    }

    pub async fn list_vote(
        State(ctrl): State<VoteControllerV1>,
        Path(parent_id): Path<String>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(param): Query<VoteParam>,
    ) -> Result<Json<VoteGetResponse>> {
        tracing::debug!("list_vote {} {:?}", parent_id, param);

        match param {
            // VoteParam::Query(q) => Ok(Json(VoteGetResponse::Query(ctrl.repo.find(&q).await?))),
            VoteParam::Query(_) => ctrl.list_votes(parent_id).await,
        }
    }

    pub async fn get_final_result(
        State(ctrl): State<VoteControllerV1>,
        Path(parent_id): Path<String>,
        Extension(_auth): Extension<Option<Authorization>>,
    ) -> Result<Json<VoteResultSummary>> {
        tracing::debug!("get_final_result {}", parent_id);

        ctrl.vote_result_summary(parent_id).await
    }
}

impl VoteControllerV1 {
    async fn vote(&self, parent_id: String, body: VoteVotingRequest) -> Result<Json<Vote>> {
        if body.amount < 0 {
            return Err(ServiceError::BadRequest);
        }

        let topic_id = parent_id.parse::<i64>()?;
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

    async fn vote_result_summary(&self, parent_id: String) -> Result<Json<VoteResultSummary>> {
        let topic_id = parent_id.parse::<i64>()?;

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

    async fn list_votes(&self, parent_id: String) -> Result<Json<VoteGetResponse>> {
        let topic_id = parent_id.parse::<i64>()?;

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
