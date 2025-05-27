#[allow(unused, dead_code)]
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

use crate::utils::users::extract_user_with_allowing_anonymous;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct FollowerPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct FollowerController {
    repo: FollowerRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl FollowerController {
    async fn query(
        &self,
        auth: Option<Authorization>,
        param: FollowerQuery,
    ) -> Result<QueryResponse<FollowerSummary>> {
        let _user = extract_user_with_allowing_anonymous(&self.pool, auth).await?;
        
        let mut total_count = 0;
        let items: Vec<FollowerSummary> = FollowerSummary::query_builder()
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

    async fn follow(
        &self,
        auth: Option<Authorization>,
        FollowerFollowRequest { user_id }: FollowerFollowRequest,
    ) -> Result<Follower> {
        let current_user = extract_user_with_allowing_anonymous(&self.pool, auth).await?;

        // Check if user is trying to follow themselves
        if current_user.id == user_id {
            return Err(Error::BadRequest);
        }

        // Check if the user exists
        let target_user = User::query_builder()
            .id_equals(user_id)
            .query()
            .map(User::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("failed to get user {user_id}: {e}");
                Error::NotFound
            })?;

        // Check if already following
        if let Ok(_existing) = Follower::query_builder()
            .user_id_equals(user_id)
            .query()
            .map(Follower::from)
            .fetch_one(&self.pool)
            .await
        {
            return Err(Error::BadRequest);
        }

        let res = self
            .repo
            .insert(
                target_user.profile_url.clone(),
                target_user.nickname.clone(),
                None, // description
                true, // followed
                user_id,
            )
            .await
            .map_err(|e| {
                tracing::error!("failed to follow user {user_id}: {:?}", e);
                Error::BadRequest
            })?;

        Ok(res)
    }

    // #[allow(clippy::unused_async)]
    // async fn unfollow(
    //     &self,
    //     id: i64,
    //     auth: Option<Authorization>,
    // ) -> Result<Follower> {
    //     let _current_user = extract_user_with_allowing_anonymous(&self.pool, auth).await?;

    //     // Verify the follower record exists
    //     let _follower = Follower::query_builder()
    //         .id_equals(id)
    //         .query()
    //         .map(Follower::from)
    //         .fetch_one(&self.pool)
    //         .await
    //         .map_err(|e| {
    //             tracing::error!("failed to get follower {id}: {e}");
    //             Error::NotFound
    //         })?;

    //     let res = self.repo.delete(id).await.map_err(|e| {
    //         tracing::error!("failed to unfollow follower {id}: {:?}", e);
    //         Error::BadRequest
    //     })?;

    //     Ok(res)
    // }

    async fn get_by_id(&self, id: i64, auth: Option<Authorization>) -> Result<Follower> {
        let _user = extract_user_with_allowing_anonymous(&self.pool, auth).await?;

        Ok(Follower::query_builder()
            .id_equals(id)
            .query()
            .map(Follower::from)
            .fetch_one(&self.pool)
            .await?)
    }

    async fn update(
        &self,
        id: i64,
        auth: Option<Authorization>,
        param: FollowerUpdateRequest,
    ) -> Result<Follower> {
        let _user = extract_user_with_allowing_anonymous(&self.pool, auth).await?;

        // Verify the follower record exists
        let _follower = Follower::query_builder()
            .id_equals(id)
            .query()
            .map(Follower::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("failed to get follower {id}: {e}");
                Error::NotFound
            })?;

        let res = self.repo.update(id, param.into()).await?;

        Ok(res)
    }

    async fn delete(&self, id: i64, auth: Option<Authorization>) -> Result<Follower> {
        let _user = extract_user_with_allowing_anonymous(&self.pool, auth).await?;

        let res = self.repo.delete(id).await?;

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
            .route("/:id", get(Self::get_follower_by_id).post(Self::act_follower_by_id))
            .with_state(self.clone())
            .route("/", post(Self::act_follower).get(Self::get_followers))
            .with_state(self.clone()))
    }

    pub async fn act_follower(
        State(ctrl): State<FollowerController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<FollowerAction>,
    ) -> Result<Json<Follower>> {
        tracing::debug!("act_follower {:?}", body);
        let follower = match body {
            FollowerAction::Follow(param) => ctrl.follow(auth, param).await?,
        };

        Ok(Json(follower))
    }

    pub async fn act_follower_by_id(
        State(ctrl): State<FollowerController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(FollowerPath { id }): Path<FollowerPath>,
        Json(body): Json<FollowerByIdAction>,
    ) -> Result<Json<Follower>> {
        tracing::debug!("act_follower_by_id {:?} {:?}", id, body);
        match body {
            FollowerByIdAction::Update(param) => {
                let res = ctrl.update(id, auth, param).await?;
                Ok(Json(res))
            }
            FollowerByIdAction::Delete(_) => {
                let res = ctrl.delete(id, auth).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn get_follower_by_id(
        State(ctrl): State<FollowerController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(FollowerPath { id }): Path<FollowerPath>,
    ) -> Result<Json<Follower>> {
        tracing::debug!("get_follower {:?}", id);

        let follower = ctrl.get_by_id(id, auth).await?;
        Ok(Json(follower))
    }

    pub async fn get_followers(
        State(ctrl): State<FollowerController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<FollowerParam>,
    ) -> Result<Json<FollowerGetResponse>> {
        tracing::debug!("list_followers {:?}", q);

        match q {
            FollowerParam::Query(param) => {
                Ok(Json(FollowerGetResponse::Query(ctrl.query(auth, param).await?)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{TestContext, setup};

    #[tokio::test]
    async fn test_follow_user() {
        let TestContext {
            pool, endpoint, ..
        } = setup().await.unwrap();

        // Create another user to follow
        let target_user = User::get_repository(pool.clone())
            .insert(
                "target@example.com".to_string(),
                "target@example.com".to_string(),
                "target@example.com".to_string(),
                "https://example.com/target".to_string(),
                true,
                true,
                UserType::Individual,
                None,
                "target_user".to_string(),
                "<p>Target user profile</p>".to_string(),
            )
            .await
            .unwrap();

        let res = Follower::get_client(&endpoint)
            .follow(target_user.id)
            .await;

        assert!(res.is_ok());

        let follower = res.unwrap();
        assert_eq!(follower.user_id, target_user.id);
        assert_eq!(follower.followed, true);
        assert_eq!(follower.title, target_user.nickname);
    }

    #[tokio::test]
    async fn test_follow_self_should_fail() {
        let TestContext {
            user, endpoint, ..
        } = setup().await.unwrap();

        let res = Follower::get_client(&endpoint)
            .follow(user.id)
            .await;

        assert!(res.is_err());
        assert_eq!(res, Err(Error::BadRequest));
    }

    #[tokio::test]
    async fn test_follow_nonexistent_user() {
        let TestContext { endpoint, .. } = setup().await.unwrap();

        let res = Follower::get_client(&endpoint)
            .follow(99999)
            .await;

        assert!(res.is_err());
        assert_eq!(res, Err(Error::NotFound));
    }
}