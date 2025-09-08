use bdk::prelude::*;
use by_types::QueryResponse;
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
use sqlx::postgres::PgRow;

use crate::utils::users::extract_user_id;

#[derive(Debug, Clone, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct ConversationPath {
    #[schemars(description = "Conversation ID")]
    pub conversation_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct GetMessagesQuery {
    #[serde(default = "default_size")]
    #[schemars(description = "Number of messages per page (default: 50, max: 100)")]
    pub size: i32,

    #[serde(default = "default_page")]
    #[schemars(description = "Page number (default: 1)")]
    pub page: i32,
}

fn default_size() -> i32 {
    50
}
fn default_page() -> i32 {
    1
}

impl GetMessagesQuery {
    pub fn size(&self) -> i32 {
        if self.size > 100 {
            100
        } else {
            self.size.max(1)
        }
    }

    pub fn page(&self) -> i32 {
        self.page.max(1)
    }
}

pub async fn get_messages_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Path(ConversationPath { conversation_id }): Path<ConversationPath>,
    Query(query): Query<GetMessagesQuery>,
) -> Result<Json<QueryResponse<Message>>> {
    let user_id = extract_user_id(&pool, auth).await?;

    tracing::debug!(
        "Getting messages for user {} in conversation {} (size: {}, page: {})",
        user_id,
        conversation_id,
        query.size(),
        query.page()
    );

    // Verify user is a participant in the conversation
    let participant_count = ConversationParticipant::query_builder()
        .conversation_id_equals(conversation_id)
        .user_id_equals(user_id)
        .query()
        .map(ConversationParticipant::from)
        .fetch_all(&pool)
        .await?
        .len();

    if participant_count == 0 {
        return Err(Error::Unauthorized);
    }

    let mut total_count = 0;

    // Get messages ordered by created_at DESC (newest first) with pagination
    let items: Vec<Message> = Message::query_builder()
        .conversation_id_equals(conversation_id)
        .limit(query.size())
        .page(query.page())
        .order_by_created_at_desc()
        .with_count()
        .query()
        .map(|row: PgRow| {
            use sqlx::Row;
            total_count = row.try_get("total_count").unwrap_or_default();
            Message::from(row)
        })
        .fetch_all(&pool)
        .await?;

    tracing::debug!(
        "Retrieved {} messages out of {} total for conversation {}",
        items.len(),
        total_count,
        conversation_id
    );

    Ok(Json(QueryResponse { total_count, items }))
}
