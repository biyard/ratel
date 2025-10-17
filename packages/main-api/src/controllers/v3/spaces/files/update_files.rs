use crate::controllers::v3::spaces::dto::*;
use crate::models::space::SpaceCommon;

use crate::models::user::User;
use crate::types::{EntityType, Partition, TeamGroupPermission};
use crate::{AppState, Error2};

use aide::NoApi;

use crate::models::file::SpaceFile;
use crate::types::File;
use crate::types::space_file_feature_type::SpaceFileFeatureType;
use axum::extract::{Json, Path, State};
use bdk::prelude::*;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct UpdateSpaceFileRequest {
    #[schemars(description = "Space Files")]
    pub files: Vec<File>,
    #[schemars(description = "Space File Feature Type: (Overview, Recommendation)")]
    pub feature_type: SpaceFileFeatureType,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct UpdateSpaceFileResponse {
    pub files: Vec<File>,
}

//FIXME: implement with dynamodb upsert method
pub async fn update_files_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<UpdateSpaceFileRequest>,
) -> Result<Json<UpdateSpaceFileResponse>, Error2> {
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
        Some(EntityType::SpaceFile(req.feature_type.to_string())),
    )
    .await?;

    if files.is_some() {
        SpaceFile::updater(
            &space_pk.clone(),
            EntityType::SpaceFile(req.feature_type.to_string()),
        )
        .with_feature_type(req.feature_type.clone())
        .with_files(req.files)
        .execute(&dynamo.client)
        .await?;
    } else {
        let files = SpaceFile::new(space_pk.clone(), req.feature_type.clone(), req.files);

        files.create(&dynamo.client).await?;
    }

    let files = SpaceFile::get(
        &dynamo.client,
        &space_pk.clone(),
        Some(EntityType::SpaceFile(req.feature_type.to_string())),
    )
    .await?;

    let files = files.unwrap();

    Ok(Json(UpdateSpaceFileResponse {
        files: files.clone().files,
    }))
}
