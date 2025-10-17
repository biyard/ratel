// TODO: this controller will be migrated to individual tab
use crate::{
    AppState, Error2,
    models::{
        User,
        space::{DeliberationDetailResponse, DeliberationMetadata, SpaceCommon},
    },
    types::{Partition, TeamGroupPermission},
};
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;

use aide::NoApi;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationGetPath {
    #[serde(deserialize_with = "crate::types::path_param_string_to_partition")]
    pub space_pk: Partition,
}

pub async fn get_deliberation_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(DeliberationGetPath { space_pk }): Path<DeliberationGetPath>,
) -> Result<Json<DeliberationDetailResponse>, Error2> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error2::NotFoundDeliberationSpace);
    }

    let (_, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        user.as_ref().map(|u| &u.pk),
        TeamGroupPermission::SpaceRead,
    )
    .await?;
    if !has_perm {
        return Err(Error2::NoPermission);
    }

    let metadata = DeliberationMetadata::query(&dynamo.client, space_pk.clone()).await?;
    tracing::debug!("Deliberation metadata retrieved: {:?}", metadata);
    let metadata: DeliberationDetailResponse = metadata.into();

    tracing::debug!("DeliberationDetailResponse formed: {:?}", metadata);

    Ok(Json(metadata))
}
