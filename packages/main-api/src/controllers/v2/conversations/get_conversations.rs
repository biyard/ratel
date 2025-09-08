use bdk::prelude::*;
use dto::{
    Conversation, ConversationParticipant, Result,
    by_axum::{
        auth::Authorization,
        axum::{
            Extension, Json,
            extract::{Query, State},
        },
    },
    sqlx::PgPool,
};

use crate::utils::users::extract_user_id;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct GetConversationsQuery {
    #[schemars(description = "Maximum number of conversations to return (default: 20, max: 100)")]
    pub limit: Option<i64>,

    #[schemars(description = "Number of conversations to skip for pagination (default: 0)")]
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct GetConversationsResponse {
    pub conversations: Vec<Conversation>,
    pub total_count: i64,
    pub has_more: bool,
}

pub async fn get_conversations_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Query(GetConversationsQuery { limit, offset }): Query<GetConversationsQuery>,
) -> Result<Json<GetConversationsResponse>> {
    let user_id = extract_user_id(&pool, auth).await?;

    let limit = limit.unwrap_or(20).min(100); // Default 20, max 100
    let offset = offset.unwrap_or(0);

    tracing::debug!(
        "Getting conversations for user {} with limit {} offset {}",
        user_id,
        limit,
        offset
    );

    // Get conversation IDs that the user participates in using ORM
    let user_participants = ConversationParticipant::query_builder()
        .user_id_equals(user_id)
        .query()
        .map(ConversationParticipant::from)
        .fetch_all(&pool)
        .await?;

    if user_participants.is_empty() {
        return Ok(Json(GetConversationsResponse {
            conversations: vec![],
            total_count: 0,
            has_more: false,
        }));
    }

    // Extract conversation IDs
    let conversation_ids: Vec<i64> = user_participants
        .iter()
        .map(|p| p.conversation_id)
        .collect();

    // Get total count for pagination
    let total_count = conversation_ids.len() as i64;

    // Apply pagination to conversation IDs
    let paginated_conversation_ids: Vec<i64> = conversation_ids
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect();

    // Fetch conversations using BDK query builder, ordered by updated_at DESC
    let mut conversations = Vec::new();
    for conversation_id in paginated_conversation_ids {
        let conversation = Conversation::query_builder()
            .id_equals(conversation_id)
            .query()
            .map(Conversation::from)
            .fetch_one(&pool)
            .await?;

        conversations.push(conversation);
    }

    // Sort by updated_at DESC (most recent first)
    conversations.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

    let has_more = offset + limit < total_count;

    tracing::debug!(
        "Found {} conversations for user {} (total: {})",
        conversations.len(),
        user_id,
        total_count
    );

    Ok(Json(GetConversationsResponse {
        conversations,
        total_count,
        has_more,
    }))
}
