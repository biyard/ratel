use dto::*;

pub async fn send_notification(
    pool: &sqlx::Pool<sqlx::Postgres>,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    content: NotificationData,
) -> Result<Notification> {
    
    let repo = Notification::get_repository(pool.clone());
    repo.insert_with_tx(
            &mut **tx,
            user_id,
            content,
            false
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to insert notification: {:?}", e);
            Error::DatabaseException(e.to_string())
        })?
        .ok_or_else(|| {
            tracing::error!("Insert operation returned None");
            Error::DatabaseException("Insert operation failed".to_string())
        })
}
