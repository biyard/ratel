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
    repo: VoteRepository,
}

impl VoteControllerV1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Vote::get_repository(pool);

        let ctrl = VoteControllerV1 { repo };

        Ok(by_axum::axum::Router::new()
            .route(
                "/:id",
                get(Self::get_vote), // .post(Self::act_vote_by_id)
            )
            .with_state(ctrl.clone())
            .route("/", post(Self::act_vote).get(Self::list_vote))
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
        State(_ctrl): State<VoteControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path((parent_id, id)): Path<(String, String)>,
    ) -> Result<Json<Vote>> {
        tracing::debug!("get_vote {} {:?}", parent_id, id);
        Ok(Json(Vote::default()))
    }

    pub async fn list_vote(
        State(_ctrl): State<VoteControllerV1>,
        Path(parent_id): Path<String>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(q): Query<VoteParam>,
    ) -> Result<Json<VoteGetResponse>> {
        tracing::debug!("list_vote {} {:?}", parent_id, q);

        Ok(Json(VoteGetResponse::Query(QueryResponse::default())))
    }
}

impl VoteControllerV1 {
    async fn vote(&self, _parent_id: String, _body: VoteVotingRequest) -> Result<Json<Vote>> {
        // self.repo.insert(body.amount, ).await?;

        Ok(Json(Vote::default()))
    }
}
