// use bdk::prelude::by_axum::auth::Authorization;
use bdk::prelude::*;
use dto::*;

pub async fn send_notification(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i64,
    title: Option<String>,
    message: String,
    image_url: Option<String>,
) -> Result<Notification> {

    let res = Notification::get_repository(pool.clone())
        .insert(
            user_id,
            title,
            message,
            image_url,
            NotificationStatus::Unread
        ).await
        .map_err(|e| {
            tracing::error!("failed to insert notification: {:?}", e);
            Error::DatabaseException(e.to_string())
        });

    res
}