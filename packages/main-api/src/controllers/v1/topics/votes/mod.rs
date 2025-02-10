#![allow(dead_code)]
use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
// use by_types::QueryResponse;
use dto::*;

#[derive(Clone, Debug)]
pub struct VoteControllerV1 {
    repo: VoteRepository,
    user: UserRepository,
}

impl VoteControllerV1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Vote::get_repository(pool.clone());
        let user = User::get_repository(pool.clone());
        let ctrl = VoteControllerV1 { repo, user };

        Ok(by_axum::axum::Router::new()
            .route(
                "/:id",
                post(Self::act_vote).get(Self::get_vote), // .post(Self::act_vote_by_id)
            )
            .with_state(ctrl.clone())
            .route("/", get(Self::list_vote))
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
        Query(param): Query<VoteParam>,
    ) -> Result<Json<VoteGetResponse>> {
        tracing::debug!("list_vote {} {:?}", parent_id, param);

        match param {
            // VoteParam::Query(q) => ctrl.repo.list_by_user_id(q).await,
            // VoteParam::Read(r) => ctrl.vote_result_summary(r, parent_id).await,
            _ => Err(ServiceError::BadRequest), // TODO: Unimplemented
        }
    }
}

impl VoteControllerV1 {
    async fn vote(&self, parent_id: String, body: VoteVotingRequest) -> Result<Json<Vote>> {
        let topic_id = parent_id.parse::<i64>()?;
        let user = self
            .user
            .find_one(&UserReadAction::new().user_info())
            .await?;

        // TODO: feat upsert vote
        let vote = self
            .repo
            .insert(body.vote, body.amount, user.id, topic_id)
            .await?;

        Ok(Json(vote))
    }

    // async fn list_by_user_id(&self, q: VoteQuery) -> Result<Json<VoteGetResponse>> {
    //     let user = self
    //         .user
    //         .find_one(&UserReadAction::new().user_info())
    //         .await?;

    //     let items = self.repo.list_by_user_id(user.id).await?;

    //     Ok(Json(VoteGetResponse::Query(items)))
    // }

    async fn vote_result_summary(
        &self,
        _q: VoteReadAction,
        _parent_id: String,
    ) -> Result<Json<VoteResultSummary>> {
        //     let topic_id = parent_id.parse::<i64>()?;

        //     let query = VoteSummary::base_sql_with("where topic_id = $1 limit $2 offset $3");
        //     tracing::debug!("vote_result_summary query: {}", query);

        //     // TODO: Using vote_result_summary
        //     let items: Vec<VoteSummary> = sqlx::query(&query)
        //         .bind(topic_id)
        //         .bind(q.size as i64)
        //         .bind(
        //             q.size as i64
        //                 * (q.bookmark
        //                     .unwrap_or("1".to_string())
        //                     .parse::<i64>()
        //                     .unwrap()
        //                     - 1),
        //         )
        //         .map(|r: sqlx::postgres::PgRow| {
        //             use sqlx::Row;
        //             total_count = r.get("total_count");
        //             r.into()
        //         })
        //         .fetch_all(&ctrl.pool)
        //         .await?;

        //     Ok(Json(VoteResultSummary {
        //         pros: items
        //             .iter()
        //             .filter(|r| r.vote == VoteResult::Supportive)
        //             .count() as i64,
        //         cons: items
        //             .iter()
        //             .filter(|r| r.vote == VoteResult::Against)
        //             .count() as i64,
        //     }))
        Ok(Json(VoteResultSummary { pros: 0, cons: 0 }))
    }
}
