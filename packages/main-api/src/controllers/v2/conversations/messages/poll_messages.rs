use bdk::prelude::*;
use dto::{
    ConversationParticipant, Error, Message, Result,
    by_axum::{
        auth::Authorization,
        axum::{
            Extension, Json,
            extract::{Path, Query, State},
        },
    },
    sqlx::PgPool,
};
use serde::{Deserialize, Serialize};
use tokio::time::{Duration, timeout};

use crate::utils::users::extract_user_id;

#[derive(Debug, Clone, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct ConversationPath {
    #[schemars(description = "Conversation ID")]
    pub conversation_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct PollMessagesQuery {
    #[schemars(description = "Last message ID received (get messages with ID greater than this)")]
    pub since_id: Option<i64>,

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
    Path(ConversationPath { conversation_id }): Path<ConversationPath>,
    Query(query): Query<PollMessagesQuery>,
) -> Result<Json<PollMessagesResponse>> {
    let user_id = extract_user_id(&pool, auth).await?;

    tracing::debug!(
        "Polling messages for conversation {} by user {}, since_id: {:?}",
        conversation_id,
        user_id,
        query.since_id
    );

    // Verify that the user is a participant in this conversation
    let participant_count = ConversationParticipant::query_builder()
        .conversation_id_equals(conversation_id)
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
            conversation_id
        );
        return Err(Error::Unauthorized);
    }

    let since_id = query.since_id.unwrap_or(0);

    // Set timeout
    let timeout_duration = Duration::from_secs(query.timeout_seconds.unwrap_or(30));

    // Implement proper timeout with polling
    let start_time = tokio::time::Instant::now();
    let poll_result = timeout(timeout_duration, async {
        loop {
            let mut messages_query =
                Message::query_builder().conversation_id_equals(conversation_id);

            if since_id > 0 {
                messages_query = messages_query.id_greater_than(since_id);
            }

            let new_messages = messages_query
                .query()
                .map(Message::from)
                .fetch_all(&pool)
                .await?;

            if !new_messages.is_empty() {
                tracing::debug!(
                    "Found {} new messages for conversation {}",
                    new_messages.len(),
                    conversation_id
                );
                return Ok::<PollMessagesResponse, Error>(PollMessagesResponse {
                    messages: new_messages,
                    has_new_messages: true,
                });
            }

            // Check if we should continue polling
            if start_time.elapsed() >= timeout_duration {
                break;
            }

            // Wait before checking again, but don't exceed timeout
            let remaining_time = timeout_duration.saturating_sub(start_time.elapsed());
            let sleep_duration = Duration::from_millis(500).min(remaining_time);

            if sleep_duration.is_zero() {
                break;
            }

            tokio::time::sleep(sleep_duration).await;
        }

        // Return empty response if no new messages found within timeout
        Ok::<PollMessagesResponse, Error>(PollMessagesResponse {
            messages: vec![],
            has_new_messages: false,
        })
    })
    .await;

    match poll_result {
        Ok(response) => Ok(Json(response?)),
        Err(_timeout_elapsed) => {
            // Timeout occurred, return empty response
            tracing::debug!(
                "Polling timeout elapsed for conversation {} user {}",
                conversation_id,
                user_id
            );
            Ok(Json(PollMessagesResponse {
                messages: vec![],
                has_new_messages: false,
            }))
        }
    }
}
