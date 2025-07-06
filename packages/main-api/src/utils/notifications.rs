use by_axum::auth::Authorization;
use dto::*;

use crate::utils::users::extract_user_id;

pub async fn send_notification(
    pool: &sqlx::Pool<sqlx::Postgres>,
    auth: Option<Authorization>,
    user_id: i64,
    content: NotificationData,
) -> Result<Notification> {

    let _verified = extract_user_id(&pool, auth).await?;

    // Check if the target user exists
    User::query_builder()
        .id_equals(user_id)
        .query()
        .map(User::from)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Target user not found: {:?}", e);
            Error::NotFound
        })?;

    let notification_type = match content {
        NotificationData::BoostingSpace { .. } => NotificationType::BoostingSpace,
        NotificationData::ConnectNetwork { .. } => NotificationType::ConnectNetwork,
        NotificationData::InviteDiscussion { .. } => NotificationType::InviteDiscussion,
        NotificationData::InviteTeam { .. } => NotificationType::InviteTeam,
        NotificationData::None => NotificationType::Unknown,
    };


    Notification::get_repository(pool.clone())
        .insert(
            user_id,
            content,
            notification_type,
            false
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to insert notification: {:?}", e);
            Error::DatabaseException(e.to_string())
        })
}