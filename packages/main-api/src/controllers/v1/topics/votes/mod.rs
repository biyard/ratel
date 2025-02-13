#![allow(dead_code)]
use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, State},
        routing::{get, post},
        Extension, Json,
    },
};
// use by_types::QueryResponse;
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
                "/:id",
                post(Self::act_vote).get(Self::get_vote), // .post(Self::act_vote_by_id)
            )
            .with_state(ctrl.clone())
            .route("/:id/result", get(Self::get_result))
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

    // pub async fn act_vote_by_id(
    //     State(_ctrl): State<VoteControllerV1>,
    //     Extension(_auth): Extension<Option<Authorization>>,
    //     Path((parent_id, id)): Path<(String, String)>,
    //     Json(body): Json<VoteByIdAction>,
    // ) -> Result<Json<Vote>> {
    //     tracing::debug!("act_vote_by_id {} {:?} {:?}", parent_id, id, body);
    //     Ok(Json(Vote::default()))
    // }

    pub async fn get_vote(
        State(ctrl): State<VoteControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(parent_id): Path<String>,
    ) -> Result<Json<Vote>> {
        tracing::debug!("get_vote {}", parent_id);

        let id = parent_id.parse::<i64>()?;

        let user = ctrl
            .user
            .find_one(&UserReadAction::new().user_info())
            .await?;

        let vote = ctrl
            .repo
            .find_one(&VoteReadAction::new().find_by_id(user.id, id))
            .await?;

        Ok(Json(vote))
    }

    // pub async fn list_vote(
    //     State(_ctrl): State<VoteControllerV1>,
    //     Path(parent_id): Path<String>,
    //     Extension(_auth): Extension<Option<Authorization>>,
    //     Query(param): Query<VoteParam>,
    // ) -> Result<Json<VoteGetResponse>> {
    //     tracing::debug!("list_vote {} {:?}", parent_id, param);

    //     match param {
    //         // VoteParam::Query(q) => ctrl.repo.list_by_user_id(q).await,
    //         // VoteParam::Read(r) => ctrl.vote_result_summary(r, parent_id).await,
    //         _ => Err(ServiceError::BadRequest), // TODO: Unimplemented
    //     }
    // }

    pub async fn get_result(
        State(ctrl): State<VoteControllerV1>,
        Path(parent_id): Path<String>,
        Extension(_auth): Extension<Option<Authorization>>,
    ) -> Result<Json<VoteResultSummary>> {
        tracing::debug!("get_result {}", parent_id);

        ctrl.vote_result_summary(parent_id).await
    }
}

impl VoteControllerV1 {
    async fn vote(&self, parent_id: String, body: VoteVotingRequest) -> Result<Json<Vote>> {
        let topic_id = parent_id.parse::<i64>()?;
        let user = self
            .user
            .find_one(&UserReadAction::new().user_info())
            .await?;

        match self
            .repo
            .find_one(&VoteReadAction::new().find_by_id(user.id, topic_id))
            .await
        {
            Ok(vote) => {
                let vote = self
                    .repo
                    .update(
                        vote.id,
                        VoteRepositoryUpdateRequest {
                            vote: Some(body.vote),
                            amount: Some(body.amount),
                            user_id: Some(user.id),
                            topic_id: Some(topic_id),
                        },
                    )
                    .await?;
                return Ok(Json(vote));
            }
            Err(_) => {
                let vote = self
                    .repo
                    .insert(body.vote, body.amount, user.id, topic_id)
                    .await?;
                return Ok(Json(vote));
            }
        }
    }

    async fn vote_result_summary(&self, parent_id: String) -> Result<Json<VoteResultSummary>> {
        let topic_id = parent_id.parse::<i64>()?;

        let query = VoteSummary::base_sql_with("where topic_id = $1");
        tracing::debug!("vote_result_summary query: {}", query);

        let items: Vec<VoteSummary> = sqlx::query(&query)
            .bind(topic_id)
            .map(|r: sqlx::postgres::PgRow| {
                // use sqlx::Row;
                r.into()
            })
            .fetch_all(&self.pool)
            .await?;

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
}
