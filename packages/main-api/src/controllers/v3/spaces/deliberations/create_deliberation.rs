use crate::{
    AppState, Error2,
    models::space::{
        DeliberationDetailResponse, DeliberationMetadata, DeliberationSpace, SpaceCommon,
    },
    utils::dynamo_extractor::extract_user,
};
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension,
        extract::{Json, State},
    },
};
use dto::{JsonSchema, aide, schemars};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema, Validate)]
pub struct CreateDeliberationRequest {
    #[schemars(description = "Post ID")]
    pub feed_id: String,
}

#[derive(Debug, Clone, Serialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreateDeliberationResponse {
    pub metadata: DeliberationDetailResponse,
}

pub async fn create_deliberation_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Json(req): Json<CreateDeliberationRequest>,
) -> Result<Json<CreateDeliberationResponse>, Error2> {
    let user = extract_user(&dynamo.client, auth).await?;

    let deliberation = DeliberationSpace::new(user);
    deliberation.create(&dynamo.client).await?;

    let common = SpaceCommon::new(
        deliberation.pk.clone(),
        crate::types::Partition::Feed(req.feed_id),
    );
    common.create(&dynamo.client).await?;

    let metadata = DeliberationMetadata::query(&dynamo.client, deliberation.pk.clone()).await?;

    let metadata: DeliberationDetailResponse = metadata.into();
    Ok(Json(CreateDeliberationResponse { metadata }))
}
