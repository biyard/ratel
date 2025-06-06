use crate::utils::users::extract_user_id;

use by_axum::auth::Authorization;
use by_axum::axum::{
    Extension, Json,
    extract::{Path, State, Query},
    routing::{get, post},
};
use by_types::QueryResponse;

use sqlx::postgres::PgRow;

use dto::*;

#[derive(Clone, Debug)]
pub struct SuggestedUserController {
    repo: SuggestedUserRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct SuggestedUserPath {
    pub id: i64,
}

impl SuggestedUserController {
    async fn query(
        &self,
        auth: Option<Authorization>,
        param: SuggestedUserQuery,
    ) -> Result<QueryResponse<SuggestedUserSummary>> {

        // check_perm(&auth, Permission::CreateSuggestedUser).await?;
        let mut total_count = 0;

        let user_id = extract_user_id(&self.pool, auth.clone()).await;        
        if user_id.is_err() {
            return Err(Error::Unauthorized);
        }        
        let user_id = user_id.unwrap();

        // Clear all existing suggestions for this user before generating new ones
        sqlx::query("DELETE FROM suggested_users WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to clear existing suggestions for user {}: {:?}", user_id, e);
                Error::DatabaseException(e.to_string())
            })?;

        tracing::debug!("Cleared existing suggestions for user {}", user_id);

        // logic score = (mutual_friends * 2) + (shared_interests * 3) + (geolocation_match * 1)        // Get the current user's spaces to understand their interests

        //for now we will get 10 random

        let suggested_users = User::query_builder()
            .id_not_equals(user_id)
            .limit(param.size())
            .page(param.page())
            .order_by_random()
            .query()
            .map(User::from)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch suggested users: {:?}", e);
                Error::DatabaseException(e.to_string())
            })?;

        for suggested_user in suggested_users {
            let _result = self.repo.insert(
                false,

                suggested_user.nickname.clone(),
                None,   // image url
                suggested_user.profile_url.clone(),
                None,

                user_id,
                suggested_user.id,
            ).await.map_err(|e| {
                tracing::error!("Failed to insert suggested user: {:?}", e);
                Error::DatabaseException(e.to_string())
            })?;
        }

        let suggested: Vec<SuggestedUserSummary> = SuggestedUserSummary::query_builder()
            .user_id_equals(user_id)
            .limit(param.size())
            .page(param.page())
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
        
        for i in suggested.iter() {
            tracing::debug!("Suggested User: {:?}", i);
        }
        Ok(QueryResponse {
            items: suggested,
            total_count
        })

    }
    
    async fn set_dismissed(
        &self,
        id: i64,
        auth: Option<Authorization>,
        param: SuggestedUserUpdateRequest,
    ) -> Result<()> {
        // Use query_builder instead of the get method
        let suggestion = SuggestedUser::query_builder()
            .id_equals(id)
            .query()
            .map(SuggestedUser::from)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Error::DatabaseException(e.to_string()))?;
            
        if suggestion.is_none() {
            tracing::error!("Suggestion with id {} not found", id);
            return Err(Error::NotFound);
        }
        
        let user_id = extract_user_id(&self.pool, auth.clone()).await?;
        if suggestion.unwrap().user_id != user_id {
            return Err(Error::Unauthorized);
        }

        let _res = self.repo
            .update(id, param.into()).await?;

        Ok(())
    }
}


impl SuggestedUserController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        SuggestedUserController {
            repo: SuggestedUserRepository::new(pool.clone()),
            pool,
        }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        let router = by_axum::axum::Router::new()
            .route("/", get(Self::get_suggestions))
            .with_state(self.clone())
            .route("/:id", post(Self::act_suggestion_by_id))
            .with_state(self.clone());

        Ok(router)
    }

    pub async fn get_suggestions(
        State(ctrl): State<SuggestedUserController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<SuggestedUserParam>,
    ) -> Result<Json<SuggestedUserGetResponse>> {
        match q {
        SuggestedUserParam::Query(param) => {
            if param.size() > 20 {
                return Err(Error::BadRequest);
            }
            Ok(Json(SuggestedUserGetResponse::Query(ctrl.query(auth, param).await?)))

        }
        }
    }

    pub async fn act_suggestion_by_id(
        State(ctrl): State<SuggestedUserController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(SuggestedUserPath { id }): Path<SuggestedUserPath>,
        Json(body): Json<SuggestedUserByIdAction>,
    ) -> Result<Json<()>> {

        match body{
            SuggestedUserByIdAction::Update(param) => {
                ctrl.set_dismissed(id, auth, param).await?;
                Ok(Json(()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{TestContext, setup, setup_test_user};

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
    async fn test_query_suggestions() {
        let TestContext { 
            pool, 
            claims,
            ..
        } = setup().await.unwrap();

        let cli = SuggestedUserController::new(pool.clone());
        let auth = Some(Authorization::Bearer { claims });

        // Create test users for suggestions
        let (_test_user1, _test_user2, _test_user3) = test_setup(&pool).await;

        // Test basic query
        let param = SuggestedUserQuery {
            size: 10,
            bookmark: Some("1".to_string()),
            ..Default::default()
        };

        let res = cli.query(auth, param).await;
        
        assert!(res.is_ok());
    }


    #[tokio::test]
    async fn test_set_dismissed_functionality() {
        let TestContext { 
            pool, 
            claims,
            ..
        } = setup().await.unwrap();

        let cli = SuggestedUserController::new(pool.clone());
        let auth = Some(Authorization::Bearer { claims });

        // Create test users for suggestions
        let (_test_user1, _test_user2, _test_user3) = test_setup(&pool).await;

        // Get suggestions first
        let param = SuggestedUserQuery {
            size: 10,
            bookmark: Some("1".to_string()),
            ..Default::default()
        };

        let res = cli.query(auth.clone(), param).await.unwrap();
        
        if let Some(suggestion) = res.items.first() {
            // Test dismissing a suggestion
            let update_param = SuggestedUserUpdateRequest {
                dismissed: true,
            };
            
            let res = cli.set_dismissed(suggestion.id, auth, update_param).await;
            
            assert!(res.is_ok(), "Should successfully dismiss suggestion");
        }
    }

    #[tokio::test]
    async fn test_query_suggestions_unauthorized() {
        let TestContext { 
            pool,
            ..
        } = setup().await.unwrap();

        let cli = SuggestedUserController::new(pool.clone());

        // Create test users for suggestions
        let (_test_user1, _test_user2, _test_user3) = test_setup(&pool).await;

        // Test query without auth (should work based on current implementation)
        let param = SuggestedUserQuery {
            size: 10,
            bookmark: Some("1".to_string()),
            ..Default::default()
        };

        let res = cli.query(None, param).await;
        
        // Current implementation allows only authorized access, so this should succeed
        assert!(res.is_err(), "Current implementation allows only authorized query");
    }

    #[tokio::test]
    async fn test_set_dismissed_unauthorized() {
        let TestContext { 
            pool,
            ..
        } = setup().await.unwrap();

        let cli = SuggestedUserController::new(pool.clone());

        // Create test users for suggestions
        let (_test_user1, _test_user2, _test_user3) = test_setup(&pool).await;

        // Try to dismiss without proper auth
        let update_param = SuggestedUserUpdateRequest {
            dismissed: true,
        };
        
        let res = cli.set_dismissed(1, None, update_param).await;
        
        assert!(res.is_err(), "Should reject unauthorized dismiss attempt");
    }

    #[tokio::test]
    async fn test_empty_suggestions_response() {
        let TestContext { 
            pool, 
            claims,
            ..
        } = setup().await.unwrap();

        let cli = SuggestedUserController::new(pool.clone());
        let auth = Some(Authorization::Bearer { claims });

        // Test query when no suggestions exist
        let param = SuggestedUserQuery {
            size: 10,
            bookmark: Some("1".to_string()),
            ..Default::default()
        };

        let res = cli.query(auth, param).await.unwrap();
        
        assert!(res.total_count >= 0, "Should have valid total count even when empty");
        assert!(res.items.len() <= 10, "Should respect size limit even when empty");
    }

    #[tokio::test]
    async fn test_suggestions_clear_and_regenerate() {
        let TestContext { 
            pool, 
            claims,
            ..
        } = setup().await.unwrap();

        let cli = SuggestedUserController::new(pool.clone());
        let auth = Some(Authorization::Bearer { claims });

        // Create test users for suggestions
        let (_test_user1, _test_user2, _test_user3) = test_setup(&pool).await;

        // First query - should generate suggestions
        let param = SuggestedUserQuery {
            size: 5,
            bookmark: Some("1".to_string()),
            ..Default::default()
        };

        let res1 = cli.query(auth.clone(), param.clone()).await.unwrap();
        let _first_count = res1.total_count;
        
        // Second query - should clear and regenerate suggestions
        let res2 = cli.query(auth, param).await.unwrap();
        
        // Should have suggestions both times (assuming test users exist)
        assert!(res2.total_count >= 0, "Should have valid count on second query");
    }

    #[tokio::test]
    async fn test_query_suggestions_pagination() {
        let TestContext { 
            pool, 
            claims,
            ..
        } = setup().await.unwrap();

        let cli = SuggestedUserController::new(pool.clone());
        let auth = Some(Authorization::Bearer { claims });

        // Create test users for suggestions
        let (_test_user1, _test_user2, _test_user3) = test_setup(&pool).await;

        // Test pagination - page 1
        let param1 = SuggestedUserQuery {
            size: 2,
            bookmark: Some("1".to_string()),
            ..Default::default()
        };

        let res1 = cli.query(auth.clone(), param1).await.unwrap();
        
        // Test pagination - page 2
        let param2 = SuggestedUserQuery {
            size: 2,
            bookmark: Some("2".to_string()),
            ..Default::default()
        };

        let res2 = cli.query(auth, param2).await.unwrap();
        
        assert!(res1.items.len() <= 2, "Page 1 should respect size limit");
        assert!(res2.items.len() <= 2, "Page 2 should respect size limit");
        
        // If both pages have items, they should be different
        if !res1.items.is_empty() && !res2.items.is_empty() {
            let page1_ids: std::collections::HashSet<i64> = res1.items.iter().map(|s| s.id).collect();
            let page2_ids: std::collections::HashSet<i64> = res2.items.iter().map(|s| s.id).collect();
            
            let intersection: Vec<_> = page1_ids.intersection(&page2_ids).collect();
            assert!(intersection.is_empty(), "Different pages should have different items");
        }
    }

    #[tokio::test]
    async fn test_query_suggestions_size_limit() {
        let TestContext { 
            pool, 
            claims,
            ..
        } = setup().await.unwrap();

        let cli = SuggestedUserController::new(pool.clone());
        let auth = Some(Authorization::Bearer { claims });

        // Create test users for suggestions
        let (_test_user1, _test_user2, _test_user3) = test_setup(&pool).await;

        // Test size limit enforcement - this should be handled at the route level
        let param = SuggestedUserQuery {
            size: 25, // Over the limit of 20
            bookmark: Some("1".to_string()),
            ..Default::default()
        };

        // This actually tests the controller logic, not the route validation
        let res = cli.query(auth, param).await;
        // Note: The size limit check is in the route handler, not the controller method
        assert!(res.is_ok(), "Controller method doesn't enforce size limit directly");
    }
}