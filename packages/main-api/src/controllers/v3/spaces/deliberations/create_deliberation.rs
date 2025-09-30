use crate::{
    AppState, Error2,
    models::space::{
        DeliberationDetailResponse, DeliberationMetadata, DeliberationSpace, SpaceCommon,
    },
    utils::dynamo_extractor::extract_user_from_session,
};
use dto::by_axum::axum::{
    Extension,
    extract::{Json, State},
};
use dto::{JsonSchema, aide, schemars};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema, Validate)]
pub struct CreateDeliberationRequest {
    #[schemars(description = "Post ID")]
    pub feed_pk: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreateDeliberationResponse {
    pub metadata: DeliberationDetailResponse,
}

pub async fn create_deliberation_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<Session>,
    Json(req): Json<CreateDeliberationRequest>,
) -> Result<Json<CreateDeliberationResponse>, Error2> {
    let feed_pk = req.feed_pk.replace("%23", "#");
    let feed_id = feed_pk.split('#').last().unwrap().to_string();

    tracing::debug!("create_deliberation_handler called with req: {:?}", req,);
    let user = extract_user_from_session(&dynamo.client, &session).await?;
    tracing::debug!("User extracted: {:?}", user);

    let deliberation = DeliberationSpace::new(user);
    deliberation.create(&dynamo.client).await?;

    let common = SpaceCommon::new(
        deliberation.pk.clone(),
        crate::types::Partition::Feed(feed_id),
    );
    common.create(&dynamo.client).await?;

    let metadata = DeliberationMetadata::query(&dynamo.client, deliberation.pk.clone()).await?;

    let metadata: DeliberationDetailResponse = metadata.into();
    Ok(Json(CreateDeliberationResponse { metadata }))
}
