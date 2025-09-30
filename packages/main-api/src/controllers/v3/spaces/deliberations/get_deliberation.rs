use crate::{
    AppState, Error2,
    models::space::{DeliberationDetailResponse, DeliberationMetadata},
};
use dto::by_axum::axum::{
    Extension,
    extract::{Json, Path, State},
};
use dto::{aide, schemars};
use tower_sessions::Session;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationGetPath {
    pub space_pk: String,
}

pub async fn get_deliberation_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(_session): Extension<Session>,
    Path(DeliberationGetPath { space_pk }): Path<DeliberationGetPath>,
) -> Result<Json<DeliberationDetailResponse>, Error2> {
    let space_pk = space_pk.replace("%23", "#");
    tracing::debug!(
        "get_deliberation_handler called with space_pk: {}",
        space_pk
    );
    let metadata = DeliberationMetadata::query(&dynamo.client, space_pk.clone()).await?;

    tracing::debug!("Deliberation metadata retrieved: {:?}", metadata);
    let metadata: DeliberationDetailResponse = metadata.into();

    tracing::debug!("DeliberationDetailResponse formed: {:?}", metadata);

    Ok(Json(metadata))
}
