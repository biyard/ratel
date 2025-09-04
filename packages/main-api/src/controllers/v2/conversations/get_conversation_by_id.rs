use bdk::prelude::*;
use dto::{
    Conversation, ConversationParticipant, Error, Result,
    by_axum::{
        auth::Authorization,
        axum::{
            Extension, Json,
            extract::{Path, State},
        },
    },
    sqlx::PgPool,
};

use crate::utils::users::extract_user_id;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, aide::OperationIo, JsonSchema,
)]
pub struct ConversationPath {
    #[schemars(description = "Conversation ID")]
    pub conversation_id: i64,
}

pub async fn get_conversation_by_id_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Path(ConversationPath { conversation_id }): Path<ConversationPath>,
) -> Result<Json<Conversation>> {
    let user_id = extract_user_id(&pool, auth).await?;

    tracing::debug!(
        "Getting conversation {} for user {}",
        conversation_id,
        user_id
    );

    // Verify that the user is a participant in this conversation using ORM
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
            "User {} attempted to access conversation {} they don't participate in",
            user_id,
            conversation_id
        );
        return Err(Error::NotFound);
    }

    // Fetch the conversation using BDK query builder
    let conversation = Conversation::query_builder()
        .id_equals(conversation_id)
        .query()
        .map(Conversation::from)
        .fetch_one(&pool)
        .await
        .map_err(|_| {
            tracing::error!(
                "Failed to fetch conversation {} for user {}",
                conversation_id,
                user_id
            );
            Error::NotFound
        })?;

    tracing::debug!(
        "Successfully retrieved conversation {} for user {}",
        conversation_id,
        user_id
    );

    Ok(Json(conversation))
}
