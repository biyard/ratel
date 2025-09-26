use crate::{
    AppState, Error2,
    models::space::{DeliberationDetailResponse, DeliberationMetadata},
    types::Partition,
};
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension,
        extract::{Json, Path, State},
    },
};
use dto::{aide, schemars};

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationGetPath {
    pub id: String,
}

pub async fn get_deliberation_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(_auth): Extension<Option<Authorization>>,
    Path(DeliberationGetPath { id }): Path<DeliberationGetPath>,
) -> Result<Json<DeliberationDetailResponse>, Error2> {
    let metadata =
        DeliberationMetadata::query(&dynamo.client, Partition::DeliberationSpace(id.to_string()))
            .await?;

    let metadata: DeliberationDetailResponse = metadata.into();

    Ok(Json(metadata))
}
