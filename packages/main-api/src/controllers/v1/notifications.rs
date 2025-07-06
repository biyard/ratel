use crate::utils::users::extract_user_id;

use by_axum::auth::Authorization;

use by_axum::axum::{
    Extension, Json,
    extract::{Path, Query, State},
    routing::{post, get},
};

use by_types::QueryResponse;
use dto::*;
use sqlx::postgres::PgRow;

#[derive(Clone, Debug)]
pub struct NotificationController {
    repo: NotificationRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct NotificationPath {
    pub id: i64,
}

impl NotificationController {
    async fn query(
        &self,
        auth: Option<Authorization>,
        param: NotificationQuery,
    ) -> Result<QueryResponse<NotificationSummary>> {
        let mut total_count = 0;
        let user_id = extract_user_id(&self.pool, auth).await?;
        
        let items: Vec<NotificationSummary> = Notification::query_builder()
            .user_id_equals(user_id)
            .limit(param.size() as i32)
            .page(param.page())
            .order_by_created_at_desc()
            .with_count()
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;
                total_count = row.try_get("total_count").unwrap_or_default();
                NotificationSummary::from(row)
            })
            .fetch_all(&self.pool)
            .await?;
        
        tracing::debug!("query notification items: {:?}", items);
        Ok(QueryResponse { total_count, items })
    } 

    async fn dismiss_notification(
        &self,
        id: i64,
        auth: Option<Authorization>,
    ) -> Result<Notification> {
        let notification = Notification::query_builder()
            .id_equals(id)
            .query()
            .map(Notification::from)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("failed to fetch notification: {:?}", e);
                Error::DatabaseException(e.to_string())
            })?
            .ok_or(Error::NotFound)?;

        let user_id = extract_user_id(&self.pool, auth).await?;

        if user_id != notification.user_id {
            return Err(Error::Unauthorized);
        }

        let res = self.repo
            .delete(id)
            .await
            .map_err(|e| {
                tracing::error!("failed to dismiss notification: {:?}", e);
                Error::DatabaseException(e.to_string())
            })?;
        
        Ok(res)
    }

    async fn update_status_to_read(
        &self,
        id: i64,
        auth: Option<Authorization>,
    ) -> Result<Notification> {
        let notification = Notification::query_builder()
            .id_equals(id)
            .query()
            .map(Notification::from)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("failed to fetch notification: {:?}", e);
                Error::DatabaseException(e.to_string())
            })?
            .ok_or(Error::NotFound)?;

        let user_id = extract_user_id(&self.pool, auth).await?;

        if user_id != notification.user_id {
            return Err(Error::Unauthorized);
        }

        // Update the notification to mark as read using the repository
        let res = self.repo
            .update(id, NotificationRepositoryUpdateRequest {
                read: Some(true),
                ..Default::default()
            })
            .await
            .map_err(|e| {
                tracing::error!("failed to update notification read status: {:?}", e);
                Error::DatabaseException(e.to_string())
            })?;
        
        Ok(res)
    }

}

impl NotificationController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        NotificationController {
            repo: NotificationRepository::new(pool.clone()),

            pool,
        }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/:id", post(Self::act_notification_by_id))
            .route("/", get(Self::get_query))
            .with_state(self.clone()))
    }

    // pub async fn act_notification(
    //     State(ctrl): State<NotificationController>,
    //     Extension(auth): Extension<Option<Authorization>>,
    //     Json(body): Json<NotificationAction>,
    // ) -> Result<Json<Notification>> {
    //     match body {
    //         NotificationAction::Create (param) => {
    //             let notification = ctrl.create_notification(auth, param).await?;
    //             Ok(Json(notification))
    //         }
    //     }
    // }

    pub async fn act_notification_by_id(
        State(ctrl): State<NotificationController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(NotificationPath { id }): Path<NotificationPath>,
        Json(body): Json<NotificationByIdAction>,
    ) -> Result<Json<Notification>> {
        match body {
            NotificationByIdAction::Dismiss (_) => {
                let notification = ctrl.dismiss_notification(id, auth).await?;
                Ok(Json(notification))
            }
            NotificationByIdAction::UpdateStatusToRead (_) => {
                let notification = ctrl.update_status_to_read(id, auth).await?;
                Ok(Json(notification))
            }
        }
    }

    pub async fn get_query(
        State(ctrl): State<NotificationController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(param): Query<NotificationQuery>,
    ) -> Result<Json<QueryResponse<NotificationSummary>>> {
        let res = ctrl.query(auth, param).await.map_err(|e| {
            tracing::error!("failed to query notifications: {:?}", e);
            e
        })?;
        Ok(Json(res))
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{TestContext, setup, setup_jwt_token, setup_test_user};
    use crate::utils::notifications::send_notification;

    async fn test_setup(pool: &sqlx::Pool<sqlx::Postgres>) -> (User, User) {
        let id1 = uuid::Uuid::new_v4().to_string();
        let id2 = uuid::Uuid::new_v4().to_string();

        let user1 = setup_test_user(&id1, pool).await.unwrap();
        let user2 = setup_test_user(&id2, pool).await.unwrap();

        (user1, user2)
    }

    async fn create_test_notification(
        pool: &sqlx::Pool<sqlx::Postgres>,
        auth: Option<Authorization>,
        user_id: i64,
        title: String,
        _content: String,
    ) -> Result<Notification> {
        let test_notification = NotificationData::ConnectNetwork {
            requester_id: user_id,
            image_url: "https://example.com/profile.jpg".to_string(),
            description: title,
        };
        send_notification(pool, auth, user_id, test_notification).await
    }


    #[tokio::test]
    async fn test_update_status_to_read() {
        let TestContext { pool, now, .. } = setup().await.unwrap();
        let ctrl = NotificationController::new(pool.clone());

        let (user1, _) = test_setup(&pool).await;
        let claims = setup_jwt_token(user1.clone()).0;
        let auth = Some(Authorization::Bearer { claims });

        // Create a test notification
        let notification = create_test_notification(&pool, auth.clone(), user1.id, format!("Test {}", now), "Test message".to_string()).await.unwrap();
        
        assert_eq!(notification.read, false);

        // Update status to read
        let result = ctrl.update_status_to_read(notification.id, auth).await;
        
        assert!(result.is_ok(), "Should be able to update notification status");
        
        let updated_notification = result.unwrap();
        assert_eq!(updated_notification.read, true);
    }

    #[tokio::test]
    async fn test_update_status_unauthorized() {
        let TestContext { pool, now, .. } = setup().await.unwrap();
        let ctrl = NotificationController::new(pool.clone());

        let (user1, user2) = test_setup(&pool).await;
        let claims = setup_jwt_token(user2.clone()).0;
        let auth = Some(Authorization::Bearer { claims });

        // Create notification for user1
        let notification = create_test_notification(&pool, Some(Authorization::Bearer { claims: setup_jwt_token(user1.clone()).0 }), user1.id, format!("Test {}", now), "Test message".to_string()).await.unwrap();

        // Try to update with user2 credentials (should fail)
        let result = ctrl.update_status_to_read(notification.id, auth).await;
        
        assert!(result.is_err(), "Should fail when trying to update another user's notification");
        assert_eq!(result, Err(Error::Unauthorized));
    }

    #[tokio::test]
    async fn test_update_status_not_found() {
        let TestContext { pool, .. } = setup().await.unwrap();
        let ctrl = NotificationController::new(pool.clone());

        let (user1, _) = test_setup(&pool).await;
        let claims = setup_jwt_token(user1.clone()).0;
        let auth = Some(Authorization::Bearer { claims });

        // Try to update non-existent notification
        let result = ctrl.update_status_to_read(99999, auth).await;
        
        assert!(result.is_err(), "Should fail for non-existent notification");
        assert_eq!(result, Err(Error::NotFound));
    }

    #[tokio::test]
    async fn test_dismiss_notification() {
        let TestContext { pool, now, .. } = setup().await.unwrap();
        let ctrl = NotificationController::new(pool.clone());

        let (user1, _) = test_setup(&pool).await;
        let claims = setup_jwt_token(user1.clone()).0;
        let auth = Some(Authorization::Bearer { claims });

        // Create a test notification
        let notification = create_test_notification(&pool, auth.clone(), user1.id, format!("Test {}", now), "Test message".to_string()).await.unwrap();

        // Dismiss the notification
        let result = ctrl.dismiss_notification(notification.id, auth).await;
        
        assert!(result.is_ok(), "Should be able to dismiss notification");

        // Verify notification is deleted by trying to find it
        let query_result = Notification::query_builder()
            .id_equals(notification.id)
            .query()
            .map(Notification::from)
            .fetch_optional(&pool)
            .await;
        
        assert!(query_result.is_ok());
        assert!(query_result.unwrap().is_none(), "Notification should be deleted");
    }

    #[tokio::test]
    async fn test_dismiss_notification_unauthorized() {
        let TestContext { pool, now, .. } = setup().await.unwrap();
        let ctrl = NotificationController::new(pool.clone());

        let (user1, user2) = test_setup(&pool).await;
        let claims = setup_jwt_token(user2.clone()).0;
        let auth = Some(Authorization::Bearer { claims });

        // Create notification for user1
        let notification = create_test_notification(&pool, Some(Authorization::Bearer { claims: setup_jwt_token(user1.clone()).0 }), user1.id, format!("Test {}", now), "Test message".to_string()).await.unwrap();

        // Try to dismiss with user2 credentials (should fail)
        let result = ctrl.dismiss_notification(notification.id, auth).await;
        
        assert!(result.is_err(), "Should fail when trying to dismiss another user's notification");
        assert_eq!(result, Err(Error::Unauthorized));
    }

    #[tokio::test]
    async fn test_dismiss_notification_not_found() {
        let TestContext { pool, .. } = setup().await.unwrap();
        let ctrl = NotificationController::new(pool.clone());

        let (user1, _) = test_setup(&pool).await;
        let claims = setup_jwt_token(user1.clone()).0;
        let auth = Some(Authorization::Bearer { claims });

        // Try to dismiss non-existent notification
        let result = ctrl.dismiss_notification(99999, auth).await;
        
        assert!(result.is_err(), "Should fail for non-existent notification");
        assert_eq!(result, Err(Error::NotFound));
    }

    #[tokio::test]
    async fn test_dismiss_notification_without_auth() {
        let TestContext { pool, now, .. } = setup().await.unwrap();
        let ctrl = NotificationController::new(pool.clone());

        let (user1, _) = test_setup(&pool).await;

        // Create a test notification
        let notification = create_test_notification(&pool, Some(Authorization::Bearer { claims: setup_jwt_token(user1.clone()).0 }), user1.id, format!("Test {}", now), "Test message".to_string()).await.unwrap();

        // Try to dismiss without authentication
        let result = ctrl.dismiss_notification(notification.id, None).await;
        
        assert!(result.is_err(), "Should fail without authentication");
    }


}