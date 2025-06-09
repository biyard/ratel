use crate::utils::users::extract_user_id;

use by_axum::auth::Authorization;

use by_axum::axum::{
    Extension, Json,
    extract::{Path, Query, State},
    routing::{ post, get },
};
use by_types::QueryResponse;
use sqlx::postgres::PgRow;

use dto::*;


#[derive(Clone, Debug)]
pub struct MynetworkController {
    repo: MynetworkRepository,

    pool: sqlx::Pool<sqlx::Postgres>,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]

pub struct MynetworkPath {
    pub id: i64,
}

impl MynetworkController {

    async fn follow(&self, to_be_followed: i64, auth: Option<Authorization>) -> Result<Mynetwork> {
        let follower_id = extract_user_id(&self.pool, auth.clone()).await?;

        // check if the user is already following

        if follower_id == to_be_followed {
            return Err(Error::BadRequest);
        }

        let existing_follower = Mynetwork::query_builder()
            .follower_id_equals(follower_id)
            .following_id_equals(to_be_followed)
            .query()
            .map(Mynetwork::from)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("failed to check existing follower: {:?}", e);

                Error::DatabaseException(e.to_string())
            })?;

        if existing_follower.is_some() {
            return Err(Error::AlreadyFollowing);
        }

        // Create the network relationship

        let network = self.repo.insert(follower_id, to_be_followed).await?;

        Ok(network)
    }

    async fn unfollow(&self, user_id: i64, auth: Option<Authorization>) -> Result<Mynetwork> {
        let connection = Mynetwork::query_builder()
            .following_id_equals(user_id)
            .follower_id_equals(extract_user_id(&self.pool, auth.clone()).await?)
            .query()
            .map(Mynetwork::from)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("failed to get a network connection: {:?}", e);

                Error::DatabaseException(e.to_string())
            })?;

        match connection {
            Some(item) => Ok(self.repo.delete(item.id).await?),

            None => return Err(Error::NotFound),
        }
    }

    

        
}

impl MynetworkController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        MynetworkController {
            repo: MynetworkRepository::new(pool.clone()),
            pool,
        }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        let router = by_axum::axum::Router::new()
            .route("/", post(Self::act_follower_by_id))
            .with_state(self.clone())
            .route("/followings/:id", get(Self::get_followings))
            .route("/followers/:id", get(Self::get_followers))
            .with_state(self.clone());

        Ok(router)
    }

    pub async fn act_follower_by_id(
        State(ctrl): State<MynetworkController>,

        Extension(auth): Extension<Option<Authorization>>,

        Path(MynetworkPath { id }): Path<MynetworkPath>,

        Json(body): Json<MynetworkAction>,
    ) -> Result<Json<Mynetwork>> {
        tracing::debug!("act_follower_by_id {:?}", body);

        match body {
            MynetworkAction::Follow(_) => {
                let res = ctrl.follow(id, auth).await?;

                Ok(Json(res))
            }

            MynetworkAction::Unfollow(_) => {
                let res = ctrl.unfollow(id, auth).await?;

                Ok(Json(res))
            }
        }
    }

    pub async fn get_followings(
        State(ctrl): State<MynetworkController>,
        Extension(_): Extension<Option<Authorization>>,
        Path(MynetworkPath { id }): Path<MynetworkPath>,
        Query(param): Query<UserQuery>,
    ) -> Result<Json<QueryResponse<User>>> {
        // Get paginated list of users that the given user is following
        let mut total_count = 0;
        let following_ids: Vec<i64> = Mynetwork::query_builder()
            .follower_id_equals(id)
            .limit(param.size())
            .page(param.page())
            .order_by_created_at_desc()
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;
                total_count = row.try_get("total_count").unwrap_or_default();
                let network: Mynetwork = row.into();
                network.following_id
            })
            .fetch_all(&ctrl.pool)
            .await
            .map_err(|e| {
                tracing::error!("failed to get following relationships: {:?}", e);
                Error::DatabaseException(e.to_string())
            })?;

        // If no following relationships found, return empty result
        if following_ids.is_empty() {
            return Ok(Json(QueryResponse {
                items: vec![],
                total_count: 0,
            }));
        }

        // Get the actual user details for the following IDs
        let users: Vec<User> = if following_ids.is_empty() {
            vec![]
        } else {
            // Create OR conditions for multiple IDs using BitOr operator
            let mut combined_query = None;
            
            for following_id in following_ids {
                let single_query = User::query_builder().id_equals(following_id);
                match combined_query {
                    None => combined_query = Some(single_query),
                    Some(existing_query) => combined_query = Some(existing_query | single_query),
                }
            }
            
            if let Some(query) = combined_query {
                query
                    .order_by_created_at_desc()
                    .query()
                    .map(User::from)
                    .fetch_all(&ctrl.pool)
                    .await
                    .map_err(|e| {
                        tracing::error!("failed to get users: {:?}", e);
                        Error::DatabaseException(e.to_string())
                    })?
            } else {
                vec![]
            }
        };

        Ok(Json(QueryResponse {
            items: users,
            total_count,
        }))
    }

    pub async fn get_followers(
        State(ctrl): State<MynetworkController>,
        Extension(_): Extension<Option<Authorization>>,
        Path(MynetworkPath { id }): Path<MynetworkPath>,
        Query(param): Query<UserQuery>,
    ) -> Result<Json<QueryResponse<User>>> {
        // Get paginated list of users that are following the given user
        let mut total_count = 0;
        let follower_ids: Vec<i64> = Mynetwork::query_builder()
            .following_id_equals(id)
            .limit(param.size())
            .page(param.page())
            .order_by_created_at_desc()
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;
                total_count = row.try_get("total_count").unwrap_or_default();
                let network: Mynetwork = row.into();
                network.follower_id
            })
            .fetch_all(&ctrl.pool)
            .await
            .map_err(|e| {
                tracing::error!("failed to get follower relationships: {:?}", e);
                Error::DatabaseException(e.to_string())
            })?;

        // If no follower relationships found, return empty result
        if follower_ids.is_empty() {
            return Ok(Json(QueryResponse {
                items: vec![],
                total_count: 0,
            }));
        }

        // Get the actual user details for the follower IDs
        let users: Vec<User> = if follower_ids.is_empty() {
            vec![]
        } else {
            // Create OR conditions for multiple IDs using BitOr operator
            let mut combined_query = None;
            
            for follower_id in follower_ids {
                let single_query = User::query_builder().id_equals(follower_id);
                match combined_query {
                    None => combined_query = Some(single_query),
                    Some(existing_query) => combined_query = Some(existing_query | single_query),
                }
            }
            
            if let Some(query) = combined_query {
                query
                    .order_by_created_at_desc()
                    .query()
                    .map(User::from)
                    .fetch_all(&ctrl.pool)
                    .await
                    .map_err(|e| {
                        tracing::error!("failed to get users: {:?}", e);
                        Error::DatabaseException(e.to_string())
                    })?
            } else {
                vec![]
            }
        };

        Ok(Json(QueryResponse {
            items: users,
            total_count,
        }))
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

    async fn test_follow_and_unfollow() {
        let TestContext { pool, .. } = setup().await.unwrap();

        let cli = MynetworkController::new(pool.clone());

        // Create test users

        let (user1, user2, _) = test_setup(&pool).await;

        let claims = setup_jwt_token(user1.clone()).0;

        let auth = Some(Authorization::Bearer { claims });

        // Test follow operation

        let follow_result = cli.follow(user2.id, auth.clone()).await;

        assert!(follow_result.is_ok(), "Should be able to follow user2");

        // Test unfollow operation

        // let unfollow_result = cli.unfollow(user2.id, auth.clone()).await;

        // assert!(unfollow_result.is_ok(), "Should be able to unfollow user2");
    }

    #[tokio::test]

    async fn test_follow_error_cases() {
        let TestContext { pool, .. } = setup().await.unwrap();

        let cli = MynetworkController::new(pool.clone());

        // Create test users

        let (user1, _, user3) = test_setup(&pool).await;

        let claims = setup_jwt_token(user1.clone()).0;

        let auth = Some(Authorization::Bearer { claims });

        // Test follow yourself (should fail)

        let follow_self_result = cli.follow(user1.id, auth.clone()).await;

        assert!(
            follow_self_result.is_err(),
            "Should not be able to follow yourself"
        );

        let follow_result = cli.follow(user3.id, auth.clone()).await;

        assert!(follow_result.is_ok(), "Should be able to follow user3");

        let follow_again_result = cli.follow(user3.id, auth.clone()).await;

        assert!(
            follow_again_result.is_err(),
            "Should not be able to follow user3 twice"
        );

        let unfollow_result = cli.unfollow(user3.id, auth.clone()).await;

        assert!(unfollow_result.is_ok(), "Should be able to unfollow user3");

        if let Err(error) = follow_again_result {
            match error {
                Error::AlreadyFollowing => {} // This is the expected error

                _ => panic!("Expected AlreadyFollowing error, got: {:?}", error),
            }
        }
    }
}
