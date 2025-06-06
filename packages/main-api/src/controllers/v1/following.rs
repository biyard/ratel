use crate::utils::users::{extract_user_id, extract_user};

use by_axum::auth::Authorization;
use by_axum::axum::{
    Extension, Json,
    extract::{Path, Query, State},
    routing::{get},
};
use by_types::QueryResponse;

use sqlx::postgres::PgRow;

use dto::*;

#[derive(Clone, Debug)]
pub struct FollowingController {
    repo: FollowerRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct FollowingPath {
    pub id: i64,
}

impl FollowingController {
    async fn query(
        &self,
        user_id: i64,
        _auth: Option<Authorization>,
        param: FollowerQuery,
    ) -> Result<QueryResponse<FollowerSummary>> {
        let mut total_count = 0;
        let items: Vec<FollowerSummary> = FollowerSummary::query_builder()
            .following_id_equals(user_id)
            .limit(param.size())
            .page(param.page())
            .order_by_created_at_desc()
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;
                total_count = row.try_get("total_count").unwrap_or_default();
                row.into()
            })
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch suggested users for user {}: {:?}", user_id, e);
                Error::DatabaseException(e.to_string())
            })?;

        Ok(
            QueryResponse {
                items,
                total_count,
            }
        )
    }

    async fn follow(
        &self,
        follower_id: i64,
        auth: Option<Authorization>,
    ) -> Result<Follower> {

        let following_id = extract_user_id(&self.pool, auth.clone()).await?;
        let follower = extract_user(&self.pool, auth.clone()).await?;
        if follower_id == following_id {
            return Err(Error::BadRequest);
        }

        // check if the user is already following
        let existing_follower = Follower::query_builder()
            .follower_id_equals(follower_id)
            .following_id_equals(following_id)
            .query()
            .map(Follower::from)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("failed to check existing follower: {:?}", e);
                Error::DatabaseException(e.to_string())
            })?;

        if existing_follower.is_some() {
            return Err(Error::AlreadyFollowing);
        }   

        let following = User::query_builder()
            .id_equals(following_id)
            .query()
            .map(User::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("failed to get a following user: {:?}", e);
                Error::InvalidUser
            })?;            

        let follower = self.repo.insert(
            follower.nickname,
            follower.profile_url,
            None, // follower_profile_image
            None,
            following.nickname,
            following.profile_url,
            None, // following_profile_image
            None, // following_description
            follower_id,
            following_id,
        ).await?;

        Ok(follower)
    }

    async fn unfollow(
        &self,
        user_id: i64,
        auth: Option<Authorization>,
    ) -> Result<Follower> {
        let connection = Follower::query_builder()
            .follower_id_equals(user_id)
            .following_id_equals(extract_user_id(&self.pool, auth.clone()).await?)
            .query()
            .map(Follower::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("failed to get a follower: {:?}", e);
                Error::DatabaseException(e.to_string())
            })?;
        
        let res = self.repo.delete(connection.id).await?;

        Ok(res)
        
    }

}

impl FollowingController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        FollowingController {
            repo: FollowerRepository::new(pool.clone()),
            pool,
        }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        let router = by_axum::axum::Router::new()
            .route("/:id", get(Self::get_following).post(Self::act_follower_by_id))
            .with_state(self.clone());

        

        Ok(router)
    }

    pub async fn get_following(
        State(ctrl): State<FollowingController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(FollowingPath { id }): Path<FollowingPath>,
        Query(q): Query<FollowerParam>,
    ) -> Result<Json<FollowerGetResponse>> {
        match q {
            FollowerParam::Query(param) => {
                Ok(Json(FollowerGetResponse::Query(ctrl.query(id, auth, param).await?)))
            }
        }
    }

    pub async fn act_follower_by_id(
        State(ctrl): State<FollowingController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(FollowingPath { id }): Path<FollowingPath>,
        Json(body): Json<FollowerByIdAction>,
    ) -> Result<Json<Follower>> {
        tracing::debug!("act_feed_by_id {:?} {:?}", id, body);
        match body {
            FollowerByIdAction::Follow(_) => {
                let res = ctrl.follow(id, auth).await?;
                Ok(Json(res))
            },
            FollowerByIdAction::Unfollow(_) => {
                let res = ctrl.unfollow(id, auth).await?;
                Ok(Json(res))
            }
        }
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{TestContext, setup, setup_jwt_token, setup_test_user};

    async fn test_setup(pool: &sqlx::Pool<sqlx::Postgres>) -> (User, User, User) {
        let id1 = uuid::Uuid::new_v4().to_string();
        let id2 = uuid::Uuid::new_v4().to_string();
        let id3 = uuid::Uuid::new_v4().to_string();
        
        let user1 = setup_test_user(&id1, pool).await.unwrap();
        let user2 = setup_test_user(&id2, pool).await.unwrap();
        let user3 = setup_test_user(&id3, pool).await.unwrap();

        (user1, user2, user3)
    }

    #[tokio::test]
    async fn test_query_followers() {
        let TestContext { 
            pool, 
            claims,
            user,
            ..
        } = setup().await.unwrap();

        let cli = FollowingController::new(pool.clone());
        let auth = Some(Authorization::Bearer { claims });


        // Test basic query
        let param = FollowerQuery {
            size: 10,
            bookmark: Some("1".to_string()),
            ..Default::default()
        };

        let res = cli.query(user.id, auth, param).await;
        
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_follow_and_unfollow() {
        let TestContext { 
            pool, 
            ..
        } = setup().await.unwrap();

        let cli = FollowingController::new(pool.clone());

        // Create test users
        let (user1, user2, _) = test_setup(&pool).await;

        let claims = setup_jwt_token(user1.clone()).0;
        let auth = Some(Authorization::Bearer { claims });

        // Test follow operation
        let follow_result = cli.follow(user2.id, auth.clone()).await;
        assert!(follow_result.is_ok(), "Should be able to follow user2");
        
        // Verify follow operation worked by checking the query results
        let param = FollowerQuery {
            size: 10,
            bookmark: Some("1".to_string()),
            ..Default::default()
        };

        // Test unfollow operation
        let unfollow_result = cli.unfollow(user2.id, auth.clone()).await;
        assert!(unfollow_result.is_ok(), "Should be able to unfollow user2");
        

        // Verify unfollow operation worked
        let query_res_after = cli.query(user1.id, auth.clone(), param.clone()).await.unwrap();
        let following_ids_after: Vec<i64> = query_res_after.items.iter()
            .map(|f| f.following_id)
            .collect();
        assert!(!following_ids_after.contains(&user2.id), "User1 should no longer be following user2");
    }

    #[tokio::test]
    async fn test_follow_error_cases() {
        let TestContext { 
            pool,
            ..
        } = setup().await.unwrap();
        let cli = FollowingController::new(pool.clone());

        // Create test users
        let (user1, user2, _) = test_setup(&pool).await;

        let claims = setup_jwt_token(user1.clone()).0;
        let auth = Some(Authorization::Bearer { claims });

        // Test follow yourself (should fail)
        let follow_self_result = cli.follow(user1.id, auth.clone()).await;
        assert!(follow_self_result.is_err(), "Should not be able to follow yourself");
        
        // Test follow someone and then try to follow again (should fail with AlreadyFollowing)
        let follow_result = cli.follow(user2.id, auth.clone()).await;
        assert!(follow_result.is_ok(), "Should be able to follow user2");
        
        let follow_again_result = cli.follow(user2.id, auth.clone()).await;
        assert!(follow_again_result.is_err(), "Should not be able to follow user2 twice");
        
        if let Err(error) = follow_again_result {
            match error {
                Error::AlreadyFollowing => {}, // This is the expected error
                _ => panic!("Expected AlreadyFollowing error, got: {:?}", error),
            }
        }
    }
}
