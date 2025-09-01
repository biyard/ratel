use bdk::prelude::*;
use dto::{
    Conversation, ConversationParticipant, ConversationType, Error, ParticipantRole, Result, User,
    by_axum::{
        auth::Authorization,
        axum::{Extension, Json, extract::State},
    },
    sqlx::PgPool,
};
use validator::Validate;

use crate::utils::users::extract_user_id;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    aide::OperationIo,
    JsonSchema,
    Validate,
)]
pub struct CreateConversationRequest {
    #[validate(length(min = 1, max = 255))]
    #[schemars(description = "Title of the conversation (Group or Channel only)")]
    pub title: String,

    #[validate(length(max = 1000))]
    #[schemars(description = "Description of the conversation (optional)")]
    pub description: Option<String>,

    #[schemars(description = "Type of conversation (Group or Channel only)")]
    pub conversation_type: ConversationType,

    #[validate(length(min = 1))]
    #[schemars(description = "List of participant user IDs to add to the conversation")]
    pub participant_ids: Vec<i64>,
}

pub async fn create_conversation_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Json(req): Json<CreateConversationRequest>,
) -> Result<Json<Conversation>> {
    let user_id = extract_user_id(&pool, auth).await?;

    // Validate request
    req.validate().map_err(|_| Error::BadRequest)?;

    // Only allow Group and Channel conversation types (no Direct messages)
    if req.conversation_type == ConversationType::Direct {
        return Err(Error::BadRequest);
    }

    tracing::debug!(
        "Creating {} conversation for user {} with {} participants",
        match req.conversation_type {
            ConversationType::Group => "group",
            ConversationType::Channel => "channel",
            ConversationType::Direct => "direct", // Won't reach here due to check above
        },
        user_id,
        req.participant_ids.len()
    );

    // Start transaction
    let mut tx = pool.begin().await?;

    // Create the conversation using ORM
    let conversation_repo = Conversation::get_repository(pool.clone());
    let conversation = conversation_repo
        .insert_with_tx(
            &mut *tx,
            Some(req.title.clone()),
            req.description.clone(),
            req.conversation_type,
        )
        .await?
        .ok_or(Error::ServerError(
            "Failed to create conversation".to_string(),
        ))?;

    // Add creator as participant using ORM
    let participant_repo = ConversationParticipant::get_repository(pool.clone());
    participant_repo
        .insert_with_tx(
            &mut *tx,
            conversation.id,
            user_id,
            ParticipantRole::Admin, // Creator gets admin role
        )
        .await?;

    // Add other participants using ORM
    for participant_id in req.participant_ids {
        // Skip if same as creator
        if participant_id == user_id {
            continue;
        }

        // Verify participant user exists using ORM query
        let user_exists = User::query_builder()
            .id_equals(participant_id)
            .query()
            .map(User::from)
            .fetch_optional(&mut *tx)
            .await?;

        if user_exists.is_none() {
            return Err(Error::NotFound);
        }

        // Add participant using ORM
        participant_repo
            .insert_with_tx(
                &mut *tx,
                conversation.id,
                participant_id,
                ParticipantRole::Member,
            )
            .await?;
    }

    // Commit transaction
    tx.commit().await?;

    tracing::debug!(
        "Successfully created conversation with ID: {}",
        conversation.id
    );

    Ok(Json(conversation))
}
