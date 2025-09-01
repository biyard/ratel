use bdk::prelude::*;
use dto::{
    Error, Message, Result,
    by_axum::{
        auth::Authorization,
        axum::{Extension, Json, extract::State},
    },
    sqlx::PgPool,
};
use serde::{Deserialize, Serialize};

use crate::utils::users::extract_user_id;

#[derive(Debug, Clone, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct ClearMessageRequest {
    #[schemars(description = "Message ID to clear content")]
    pub message_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct ClearMessageResponse {
    pub success: bool,
    pub message: String,
}

pub async fn clear_message_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Json(req): Json<ClearMessageRequest>,
) -> Result<Json<ClearMessageResponse>> {
    let user_id = extract_user_id(&pool, auth).await?;

    tracing::debug!("Clearing message {} for user {}", req.message_id, user_id);

    // Start transaction
    let mut tx = pool.begin().await?;

    // Verify the message exists and user is the sender (owner)
    let message = Message::query_builder()
        .id_equals(req.message_id)
        .query()
        .map(Message::from)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(Error::NotFound)?;

    // Only allow the sender to clear their own messages
    if message.sender_id != user_id {
        return Err(Error::Unauthorized);
    }

    // Clear message content only, keeping seq_id and other data intact
    sqlx::query(
        r#"
        UPDATE messages 
        SET html_content = '', updated_at = EXTRACT(EPOCH FROM NOW())::bigint * 1000
        WHERE id = $1
        "#,
    )
    .bind(req.message_id)
    .execute(&mut *tx)
    .await?;

    // Commit transaction
    tx.commit().await?;

    tracing::debug!(
        "Successfully cleared content of message {} by sender {}",
        req.message_id,
        user_id
    );

    Ok(Json(ClearMessageResponse {
        success: true,
        message: "Message content cleared successfully".to_string(),
    }))
}
