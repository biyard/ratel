use bdk::prelude::*;
use dto::{
    ConversationParticipant, Error, Message, MessageStatus,
    Result,
    by_axum::{
        auth::Authorization,
        axum::{Extension, Json, extract::{Path, State}},
    },
    sqlx::PgPool,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::utils::users::extract_user_id;

#[derive(Debug, Clone, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct ConversationPath {
    #[schemars(description = "Conversation ID")]
    pub conversation_id: i64,
}

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema, Validate,
)]
pub struct AddMessageRequest {
    #[validate(length(min = 1, max = 10000))]
    #[schemars(description = "HTML content of the message")]
    pub html_contents: String,

    #[schemars(description = "Recipient user ID (only used for direct messages when no conversation exists)")]
    pub recipient_id: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct AddMessageResponse {
    pub message: Message,
}

pub async fn add_message_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Path(ConversationPath { conversation_id }): Path<ConversationPath>,
    Json(req): Json<AddMessageRequest>,
) -> Result<Json<AddMessageResponse>> {
    let user_id = extract_user_id(&pool, auth).await?;

    // Validate request
    req.validate().map_err(|_| Error::BadRequest)?;

    tracing::debug!("Adding message to conversation {} for user {}", conversation_id, user_id);

    // Start transaction
    let mut tx = pool.begin().await?;

    // Verify user is a participant in the conversation
    let participant_count = ConversationParticipant::query_builder()
        .conversation_id_equals(conversation_id)
        .user_id_equals(user_id)
        .query()
        .map(ConversationParticipant::from)
        .fetch_all(&mut *tx)
        .await?
        .len();

    if participant_count == 0 {
        return Err(Error::Unauthorized);
    }

    // Lock the conversation to prevent race conditions on seq_id
    sqlx::query("SELECT id FROM conversations WHERE id = $1 FOR UPDATE")
        .bind(conversation_id)
        .execute(&mut *tx)
        .await?;

    // Get next seq_id for this conversation
    let next_seq_id: i64 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(seq_id), 0) + 1 FROM messages WHERE conversation_id = $1",
    )
    .bind(conversation_id)
    .fetch_one(&mut *tx)
    .await?;

    // Insert the message
    let message_id: i64 = sqlx::query_scalar(
        r#"
        INSERT INTO messages (html_contents, status, sender_id, conversation_id, seq_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, EXTRACT(EPOCH FROM NOW())::bigint * 1000, EXTRACT(EPOCH FROM NOW())::bigint * 1000)
        RETURNING id
        "#,
    )
    .bind(&req.html_contents)
    .bind(MessageStatus::Sent as i32)
    .bind(user_id)
    .bind(conversation_id)
    .bind(next_seq_id)
    .fetch_one(&mut *tx)
    .await?;

    tracing::debug!(
        "Successfully created message with ID: {} in conversation: {}",
        message_id,
        conversation_id
    );

    // Fetch the created message for response
    let message = Message::query_builder()
        .id_equals(message_id)
        .query()
        .map(Message::from)
        .fetch_one(&mut *tx)
        .await?;

    tx.commit().await?;

    tracing::debug!(
        "Successfully created message with ID: {} in conversation: {}",
        message.id,
        conversation_id
    );

    Ok(Json(AddMessageResponse { message }))
}
