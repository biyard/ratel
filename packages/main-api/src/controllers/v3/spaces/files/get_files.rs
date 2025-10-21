use crate::controllers::v3::spaces::dto::*;
use crate::models::space::SpaceCommon;

use crate::models::user::User;
use crate::types::{EntityType, Partition, TeamGroupPermission};
use crate::{AppState, Error2};

use aide::NoApi;
use serde::{Deserialize, Serialize};

use crate::models::file::SpaceFile;
use crate::types::File;
use crate::types::space_file_feature_type::SpaceFileFeatureType;
use axum::extract::{Json, Path, Query, State};
use bdk::prelude::*;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct GetSpaceFileResponse {
    pub files: Vec<File>,
}

#[derive(Debug, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct GetSpaceFileQueryParams {
    pub feature_type: SpaceFileFeatureType,
}

pub async fn get_files_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Query(GetSpaceFileQueryParams { feature_type }): Query<GetSpaceFileQueryParams>,
) -> Result<Json<GetSpaceFileResponse>, Error2> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error2::NotFoundDeliberationSpace);
    }

    let (_, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        user.as_ref().map(|u| &u.pk),
        TeamGroupPermission::SpaceEdit,
    )
    .await?;
    if !has_perm {
        return Err(Error2::NoPermission);
    }

    let files = SpaceFile::get(
        &dynamo.client,
        &space_pk.clone(),
        Some(EntityType::SpaceFile(feature_type.to_string())),
    )
    .await?;

    let files = files.unwrap_or_default();

    Ok(Json(GetSpaceFileResponse {
        files: files.clone().files,
    }))
}
