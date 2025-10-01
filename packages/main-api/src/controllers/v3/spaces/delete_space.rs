use crate::models::space::SpaceCommon;
use crate::types::{EntityType, Partition};
use crate::utils::dynamo_extractor::extract_user_from_session;
use crate::{AppState, Error2};
use dto::by_axum::axum::{
    Extension,
    extract::{Path, State},
};
use dto::{JsonSchema, aide, schemars};
use serde::Deserialize;

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct DeleteSpacePathParams {
    #[schemars(description = "Space PK to be deleted")]
    #[serde(deserialize_with = "crate::types::path_param_string_to_partition")]
    pub space_pk: Partition,
}

pub async fn delete_space_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<tower_sessions::Session>,
    Path(DeleteSpacePathParams { space_pk }): Path<DeleteSpacePathParams>,
) -> Result<(), Error2> {
    let _user = extract_user_from_session(&dynamo.client, &session).await?;
    // FIXME: ADD PERMISSION CHECK
    let space_common = SpaceCommon::get(&dynamo.client, &space_pk, Some(EntityType::SpaceCommon))
        .await?
        .ok_or(Error2::NotFoundSpace)?;

    SpaceCommon::delete(&dynamo.client, &space_common.pk, Some(space_common.sk)).await?;

    Ok(())
}
