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
    bill_id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct VoteMemberPath {
    bill_id: i64,
    member_id: i64,
}

#[derive(Clone, Debug)]
pub struct VoteController {
    pool: sqlx::Pool<sqlx::Postgres>,
    repo: VoteRepository,
    user: UserRepository,
}

impl VoteController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo: VoteRepository = Vote::get_repository(pool.clone());
        let user: UserRepository = User::get_repository(pool.clone());
        Self { pool, repo, user }
    }
    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/", get(Self::list_vote).post(Self::my_vote))
            .with_state(self.clone())
            .route(
                "/:member_id",
                post(Self::act_vote).get(Self::list_vote_by_member),
            )
            .with_state(self.clone())
    }

    pub async fn act_vote(
        State(ctrl): State<VoteController>,
        Path(VoteMemberPath { bill_id, member_id }): Path<VoteMemberPath>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<VoteAction>,
    ) -> Result<Json<Vote>> {
        tracing::debug!("act_vote {} {} {:?}", bill_id, member_id, body);

        match body {
            VoteAction::Voting(req) => ctrl.vote(req, bill_id, member_id).await,
        }
    }

    pub async fn my_vote(
        State(ctrl): State<VoteController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(VotePath { bill_id }): Path<VotePath>,
    ) -> Result<Json<Vote>> {
        tracing::debug!("my_vote {}", bill_id);

        Ok(ctrl.get_my_result(bill_id).await?)
    }

    pub async fn list_vote(
        State(ctrl): State<VoteController>,
        Path(VotePath { bill_id }): Path<VotePath>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(param): Query<VoteParam>,
    ) -> Result<Json<VoteGetResponse>> {
        tracing::debug!("list_vote {} {:?}", bill_id, param);

        match param {
            VoteParam::Query(q) => ctrl.list_votes(q, bill_id).await,
            VoteParam::Read(_) => todo!(),
        }
    }

    pub async fn list_vote_by_member(
        State(ctrl): State<VoteController>,
        Path(VoteMemberPath { bill_id, member_id }): Path<VoteMemberPath>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(param): Query<VoteParam>,
    ) -> Result<Json<VoteGetResponse>> {
        tracing::debug!("list_vote_by_member {} {} {:?}", bill_id, member_id, param);

        match param {
            VoteParam::Query(q) => ctrl.list_votes_by_member(q, bill_id, member_id).await,
            VoteParam::Read(_) => todo!(),
        }
    }
}

impl VoteController {
    async fn vote(
        &self,
        body: VoteVotingRequest,
        bill_id: i64,
        member_id: i64,
    ) -> Result<Json<Vote>> {
        let user = self
            .user
            .find_one(&UserReadAction::new().user_info())
            .await?;

        match Vote::query_builder()
            .user_id_equals(user.id)
            .bill_id_equals(bill_id)
            .query()
            .map(|r: sqlx::postgres::PgRow| Into::<Vote>::into(r))
            .fetch_one(&self.pool)
            .await
        {
            Ok(_) => Err(ServiceError::UniqueViolation("Already voted".to_string())),
            Err(_) => {
                let vote = self
                    .repo
                    .insert(body.selected, bill_id, member_id, user.id)
                    .await?;
                Ok(Json(vote))
            }
        }
    }

    async fn list_votes(&self, query: VoteQuery, bill_id: i64) -> Result<Json<VoteGetResponse>> {
        let mut total_count: i64 = 0;
        let votes: Vec<VoteSummary> = Vote::query_builder()
            .limit(query.size())
            .page(query.page())
            .bill_id_equals(bill_id)
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

    async fn get_my_result(&self, bill_id: i64) -> Result<Json<Vote>> {
        let user = self
            .user
            .find_one(&UserReadAction::new().user_info())
            .await?;

        let vote: Vote = Vote::query_builder()
            .user_id_equals(user.id)
            .id_equals(bill_id)
            .query()
            .map(|r: sqlx::postgres::PgRow| r.into())
            .fetch_one(&self.pool)
            .await?;

        Ok(Json(vote))
    }

    async fn list_votes_by_member(
        &self,
        query: VoteQuery,
        bill_id: i64,
        member_id: i64,
    ) -> Result<Json<VoteGetResponse>> {
        let mut total_count: i64 = 0;
        let votes: Vec<VoteSummary> = Vote::query_builder()
            .limit(query.size())
            .page(query.page())
            .bill_id_equals(bill_id)
            .member_id_equals(member_id)
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::tests::{TestContext, setup};

//     #[tokio::test]
//     #[allow(unreachable_code)]
//     #[allow(unused)]
//     async fn test_vote() {
//         let TestContext { user, endpoint, .. } = setup().await.unwrap();

//         let cli = Vote::get_client(&endpoint);

//         // FIXME: complete test
//         return;
//         // "Unknown": "error returned from database: insert or update on table \"votes\" violates foreign key constraint \"votes_member_id_fkey\""
//         // I think member_id is not necessary @hackartist
//         // Err(Unknown("error decoding response body"))
//         let res = cli.voting(1, VoteOption::Supportive, 35, user.id).await;
//         tracing::debug!("{:?}", res);
//         assert!(res.is_ok());
//         let res = res.unwrap();
//         assert_eq!(res.selected, VoteOption::Supportive);

//         let res = cli.voting(1, VoteOption::Against, 35, user.id).await;
//         assert!(res.is_ok());
//         let res = res.unwrap();
//         assert_eq!(res.selected, VoteOption::Supportive);

//         let rst = cli.get_my_result(1, 1).await.unwrap();
//         assert_eq!(rst.selected, VoteOption::Supportive);
//     }
// }
