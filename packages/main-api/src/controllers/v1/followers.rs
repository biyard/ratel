use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, Query, State},
        routing::{get, post},
    },
};
use by_types::QueryResponse;
use dto::*;
use sqlx::postgres::PgRow;

use crate::utils::users::extract_user_id;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct UserPath {
    pub user_id: i64,
}

#[derive(Clone, Debug)]
pub struct FollowerController {
    repo: FollowerRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl FollowerController {
    async fn query_user_followings(
        &self,
        user_id: i64,
        _auth: Option<Authorization>,
        param: FollowerQuery,
    ) -> Result<QueryResponse<FollowerSummary>> {
        let mut total_count = 0;

        let user_exists = User::query_builder()
            .id_equals(user_id)
            .query()
            .map(User::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("User not found {user_id}: {e}");
                Error::NotFound
            })?;

        let items: Vec<FollowerSummary> = FollowerSummary::query_builder()
            .followed_id_equals(user_id)
            .limit(param.size())
            .page(param.page())
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;

                total_count = row.try_get("total_count").unwrap_or_default();
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }
    
    async fn query_user_followers(
        &self,
        user_id: i64,
        _auth: Option<Authorization>,
        param: FollowerQuery,
    ) -> Result<QueryResponse<FollowerSummary>> {
        let mut total_count = 0;

        let user_exists = User::query_builder()
            .id_equals(user_id)
            .query()
            .map(User::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("User not found {user_id}: {e}");
                Error::NotFound
            })?;

        let items: Vec<FollowerSummary> = FollowerSummary::query_builder()
            .follower_id_equals(user_id)
            .limit(param.size())
            .page(param.page())
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;

                total_count = row.try_get("total_count").unwrap_or_default();
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }

    async fn query_followings(
        &self,
        auth: Option<Authorization>,
        param: FollowerQuery,
    ) -> Result<QueryResponse<FollowerSummary>> {
        let mut total_count = 0;
        let user_id = extract_user_id(&self.pool, auth).await?;

        let items: Vec<FollowerSummary> = FollowerSummary::query_builder()
            .follower_id_equals(user_id)
            .limit(param.size())
            .page(param.page())
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;

                total_count = row.try_get("total_count").unwrap_or_default();
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }

    async fn query_followers(
        &self,
        auth: Option<Authorization>,
        param: FollowerQuery,
    ) -> Result<QueryResponse<FollowerSummary>> {
        let mut total_count = 0;
        let user_id = extract_user_id(&self.pool, auth).await?;

        let items: Vec<FollowerSummary> = FollowerSummary::query_builder()
            .followed_id_equals(user_id)
            .limit(param.size())
            .page(param.page())
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;

                total_count = row.try_get("total_count").unwrap_or_default();
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }

    async fn create(
        &self,
        auth: Option<Authorization>,
        req: FollowerFollowRequest
    ) -> Result<Follower> {
        let user_id = extract_user_id(&self.pool, auth).await?;
        if let Ok(_already_following) = Follower::query_builder()
            .follower_id_equals(user_id)
            .followed_id_equals(req.followed_id)
            .query()
            .map(Follower::from)
            .fetch_one(&self.pool)
            .await
        {
            return Err(Error::AlreadyFollowing);
        }
        
        let follower = self.repo.insert(user_id, req.followed_id).await?;
        Ok(follower)
    }

    async fn delete(&self, user_id: i64, auth: Option<Authorization>) -> Result<Follower> {
        if auth.is_none() {
            return Err(Error::Unauthorized);
        }
        let auth_user_id = extract_user_id(&self.pool, auth).await?;

        if let is_following = Follower::query_builder()
            .follower_id_equals(user_id)
            .followed_id_equals(auth_user_id)
            .query()
            .map(Follower::from)
            .fetch_one(&self.pool)
            .await{
                return Err(Error::InternalServerError);
            }

        if !is_following {
            return Err(Error::NotFollowingUser);
        }
        let res = self.repo.delete(user_id).await?;

        Ok(res)
    }
}

impl FollowerController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Follower::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            // .route("/:id", get(Self::get_news_by_id).post(Self::act_news_by_id))
            // .with_state(self.clone())
            .route("/followings/:user_id", get(Self::get_user_followings))
            .route("/followers/:user_id", get(Self::get_user_followers))
            .route("/followings", get(Self::get_followings))
            .route("/:user_id", get(Self::act_unfollow))
            .with_state(self.clone())
            .route("/", post(Self::act_follower)
            .get(Self::get_followers))
            .with_state(self.clone()))
    }

    pub async fn act_follower(
        State(ctrl): State<FollowerController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<FollowerAction>,
    ) -> Result<Json<Follower>> {
        tracing::debug!("act_follower {:?}", body);
        match body {
            FollowerAction::Follow(param) => {
                let res = ctrl.create(auth, param).await?;
                Ok(Json(res))
            },
            FollowerAction::Unfollow(_) => todo!(),
        }
    }

    pub async fn act_unfollow(
        State(ctrl): State<FollowerController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(UserPath { user_id }): Path<UserPath>,
        Json(body): Json<FollowerByIdAction>,
    ) -> Result<Json<Follower>> {
        tracing::debug!("act_unfollow {:?}", body);
        match body {
            FollowerByIdAction::Delete(_) => {
                let res = ctrl.delete(user_id, auth).await?;
                Ok(Json(res))
            }
        }
    }

    // pub async fn act_news_by_id(
    //     State(ctrl): State<FollowerController>,
    //     Extension(auth): Extension<Option<Authorization>>,
    //     Path(NewsPath { id }): Path<NewsPath>,
    //     Json(body): Json<NewsByIdAction>,
    // ) -> Result<Json<News>> {
    //     tracing::debug!("act_news_by_id {:?} {:?}", id, body);
    //     match body {
    //         NewsByIdAction::Update(param) => {
    //             let res = ctrl.update(id, auth, param).await?;
    //             Ok(Json(res))
    //         }
    //         NewsByIdAction::Delete(_) => {
    //             let res = ctrl.delete(id, auth).await?;
    //             Ok(Json(res))
    //         }
    //     }
    // }

    // Fetch the all followers of the authenticated user
    pub async fn get_followers(
        State(ctrl): State<FollowerController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<FollowerParam>,
    ) -> Result<Json<FollowerGetResponse>> {
        tracing::debug!("list_followers {:?}", q);

        match q {
            FollowerParam::Query(param) => {
                Ok(Json(FollowerGetResponse::Query(ctrl.query_followers(auth, param).await?)))
            }
        }
    }

    // Fetch the all followings of the authenticated user
    pub async fn get_followings(
        State(ctrl): State<FollowerController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<FollowerParam>,
    ) -> Result<Json<FollowerGetResponse>> {
        tracing::debug!("list_followings {:?}", q);

        match q {
            FollowerParam::Query(param) => {
                Ok(Json(FollowerGetResponse::Query(ctrl.query_followings(auth, param).await?)))
            }
        }
    }

    pub async fn get_user_followings(
        State(ctrl): State<FollowerController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<FollowerParam>,
        Path(UserPath { user_id }): Path<UserPath>,
    ) -> Result<Json<FollowerGetResponse>> {
        tracing::debug!("list_users_followings {:?}", q);

        match q {
            FollowerParam::Query(param) => {
                Ok(Json(FollowerGetResponse::Query(ctrl.query_user_followings(user_id, auth, param).await?)))
            }
        }
    }

    pub async fn get_user_followers(
        State(ctrl): State<FollowerController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<FollowerParam>,
        Path(UserPath { user_id }): Path<UserPath>,
    ) -> Result<Json<FollowerGetResponse>> {
        tracing::debug!("list_users_followings {:?}", q);

        match q {
            FollowerParam::Query(param) => {
                Ok(Json(FollowerGetResponse::Query(ctrl.query_user_followers(user_id, auth, param).await?)))
            }
        }
    }
}
