use crate::utils::users::extract_user_id;

use dto::{
    Result,
    by_axum::{
        auth::Authorization,
        axum::{Extension, Json, extract::State},
    },
    sqlx::{PgPool},
    sqlx
};

pub async fn mark_all_notifications_read_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
) -> Result<Json<()>> {
    let user_id = extract_user_id(&pool, auth).await?;
    
    // Update all notifications to read=true for this user
    sqlx::query(
        "UPDATE notifications SET read = true, updated_at = $1 WHERE user_id = $2 AND read = false"
    )
    .bind(chrono::Utc::now().timestamp())
    .bind(user_id)
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to mark all notifications as read: {:?}", e);
        dto::Error::DatabaseException(e.to_string())
    })?;

    Ok(Json(()))
}
