use bdk::prelude::*;
use dto::{
    Conversation, ConversationParticipant, ConversationType, Error, Message, MessageStatus,
    ParticipantRole, Result,
    by_axum::{
        auth::Authorization,
        axum::{Extension, Json, extract::State},
    },
    sqlx::PgPool,
};
use validator::Validate;

use crate::utils::users::extract_user_id;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema, Validate,
)]
pub struct AddMessageRequest {
    #[validate(length(min = 1, max = 10000))]
    #[schemars(description = "HTML content of the message")]
    pub html_content: String,

    #[schemars(description = "Conversation ID to send message to (optional for direct messages)")]
    pub conversation_id: Option<i64>,

    #[schemars(description = "Recipient user ID (required only for new direct messages)")]
    pub recipient_id: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct AddMessageResponse {
    pub message: Message,
}

pub async fn add_message_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Json(req): Json<AddMessageRequest>,
) -> Result<Json<AddMessageResponse>> {
    let user_id = extract_user_id(&pool, auth).await?;

    // Validate request
    req.validate().map_err(|_| Error::BadRequest)?;

    tracing::debug!("Adding message for user {}", user_id);

    // Start transaction
    let mut tx = pool.begin().await?;

    let conversation_id = match req.conversation_id {
        Some(conv_id) => {
            // Verify user is a participant in the existing conversation
            let participant_count = ConversationParticipant::query_builder()
                .conversation_id_equals(conv_id)
                .user_id_equals(user_id)
                .query()
                .map(ConversationParticipant::from)
                .fetch_all(&mut *tx)
                .await?
                .len();

            if participant_count == 0 {
                return Err(Error::Unauthorized);
            }

            conv_id
        }
        None => {
            // Need to create or find direct message conversation
            let recipient_id = req.recipient_id.ok_or(Error::BadRequest)?;

            if recipient_id == user_id {
                return Err(Error::BadRequest); // Can't send message to yourself
            }

            // Check if recipient user exists
            let recipient_exists = sqlx::query("SELECT id FROM users WHERE id = $1")
                .bind(recipient_id)
                .fetch_optional(&mut *tx)
                .await?;

            if recipient_exists.is_none() {
                return Err(Error::NotFound);
            }

            // Check if direct conversation already exists between these users
            let existing_conversations = Conversation::query_builder()
                .conversation_type_equals(ConversationType::Direct)
                .query()
                .map(Conversation::from)
                .fetch_all(&mut *tx)
                .await?;

            // Find a conversation with exactly these two participants
            let mut existing_conversation_id = None;
            for conv in existing_conversations {
                let participants = ConversationParticipant::query_builder()
                    .conversation_id_equals(conv.id)
                    .query()
                    .map(ConversationParticipant::from)
                    .fetch_all(&mut *tx)
                    .await?;

                if participants.len() == 2 {
                    let participant_ids: Vec<i64> =
                        participants.iter().map(|p| p.user_id).collect();
                    if participant_ids.contains(&user_id) && participant_ids.contains(&recipient_id)
                    {
                        existing_conversation_id = Some(conv.id);
                        break;
                    }
                }
            }

            match existing_conversation_id {
                Some(conv_id) => conv_id,
                None => {
                    // Create new direct conversation using ORM
                    let conversation_repo = Conversation::get_repository(pool.clone());
                    let new_conversation = conversation_repo
                        .insert_with_tx(
                            &mut *tx,
                            None::<String>, // title
                            None::<String>, // description
                            ConversationType::Direct,
                        )
                        .await?
                        .ok_or(Error::ServerError(
                            "Failed to create conversation".to_string(),
                        ))?;

                    // Add both users as participants using ORM
                    let participant_repo = ConversationParticipant::get_repository(pool.clone());
                    participant_repo
                        .insert_with_tx(
                            &mut *tx,
                            new_conversation.id,
                            user_id,
                            ParticipantRole::Member,
                        )
                        .await?;

                    participant_repo
                        .insert_with_tx(
                            &mut *tx,
                            new_conversation.id,
                            recipient_id,
                            ParticipantRole::Member,
                        )
                        .await?;

                    tracing::debug!(
                        "Created new direct conversation with ID: {}",
                        new_conversation.id
                    );

                    new_conversation.id
                }
            }
        }
    };

    // Generate next sequence ID using MAX + 1 since delete only clears content (keeps seq_id)
    // This ensures we never reuse sequence IDs of "deleted" (content-cleared) messages

    // For new conversations (empty), we can safely start with seq_id = 1
    // For existing conversations, we need to lock the conversation first
    // Use a simpler approach to prevent race conditions

    // First lock the conversation to prevent concurrent message insertions
    sqlx::query("SELECT 1 FROM conversations WHERE id = $1 FOR UPDATE")
        .bind(conversation_id)
        .execute(&mut *tx)
        .await?;

    // Now get the next sequence ID without FOR UPDATE on the aggregate query
    let next_seq_id_opt: Option<i64> =
        sqlx::query_scalar("SELECT MAX(seq_id) FROM messages WHERE conversation_id = $1")
            .bind(conversation_id)
            .fetch_one(&mut *tx)
            .await?;

    let next_seq_id = next_seq_id_opt.unwrap_or(0) + 1;

    // Then insert the message
    let message_id: i64 = sqlx::query_scalar(
        r#"
        INSERT INTO messages (html_content, status, sender_id, conversation_id, seq_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, EXTRACT(EPOCH FROM NOW())::bigint * 1000, EXTRACT(EPOCH FROM NOW())::bigint * 1000)
        RETURNING id
        "#,
    )
    .bind(&req.html_content)
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
