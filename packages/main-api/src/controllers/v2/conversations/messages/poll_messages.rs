use bdk::prelude::*;
use dto::{
    ConversationParticipant, Error, Message, Result,
    by_axum::{
        auth::Authorization,
        axum::{
            Extension, Json,
            extract::{Query, State},
        },
    },
    sqlx::PgPool,
};
use serde::{Deserialize, Serialize};
use tokio::time::{Duration, timeout};

use crate::utils::users::extract_user_id;

#[derive(Debug, Clone, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct PollMessagesQuery {
    #[schemars(description = "Conversation ID to poll messages for")]
    pub conversation_id: i64,

    #[schemars(description = "Timestamp to get messages since (in milliseconds)")]
    pub since: Option<i64>,

    #[schemars(description = "Maximum time to wait for new messages (seconds, default: 30)")]
    pub timeout_seconds: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct PollMessagesResponse {
    pub messages: Vec<Message>,
    pub has_new_messages: bool,
}

pub async fn poll_messages_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Query(query): Query<PollMessagesQuery>,
) -> Result<Json<PollMessagesResponse>> {
    let user_id = extract_user_id(&pool, auth).await?;

    tracing::debug!(
        "Polling messages for conversation {} by user {}",
        query.conversation_id,
        user_id
    );

    // Verify that the user is a participant in this conversation
    let participant_count = ConversationParticipant::query_builder()
        .conversation_id_equals(query.conversation_id)
        .user_id_equals(user_id)
        .query()
        .map(ConversationParticipant::from)
        .fetch_all(&pool)
        .await?
        .len();

    if participant_count == 0 {
        tracing::warn!(
            "User {} attempted to poll messages from conversation {} they don't participate in",
            user_id,
            query.conversation_id
        );
        return Err(Error::Unauthorized);
    }

    let since_timestamp = query.since.unwrap_or(0);
    let timeout_duration = Duration::from_secs(query.timeout_seconds.unwrap_or(30));

    // For now, implement a simple polling mechanism
    // In a production system, you'd want to use more sophisticated real-time updates
    let poll_result = timeout(timeout_duration, async {
        // Simple polling loop - check for new messages every second
        loop {
            let messages = Message::query_builder()
                .conversation_id_equals(query.conversation_id)
                .query()
                .map(Message::from)
                .fetch_all(&pool)
                .await?;

            let new_messages: Vec<Message> = messages
                .into_iter()
                .filter(|msg| msg.created_at > since_timestamp)
                .collect();

            if !new_messages.is_empty() {
                return Ok::<PollMessagesResponse, Error>(PollMessagesResponse {
                    messages: new_messages,
                    has_new_messages: true,
                });
            }

            // Wait a second before checking again
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    })
    .await;

    match poll_result {
        Ok(response) => Ok(Json(response?)),
        Err(_) => {
            // Timeout occurred, return empty response
            Ok(Json(PollMessagesResponse {
                messages: vec![],
                has_new_messages: false,
            }))
        }
    }
}
