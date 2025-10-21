use crate::controllers::v3::spaces::dto::*;
use crate::models::space::SpaceCommon;

use crate::models::user::User;
use crate::types::{Partition, TeamGroupPermission};
use crate::{AppState, Error2};

use aide::NoApi;

use crate::features::spaces::files::SpaceFile;
use crate::types::File;
use axum::extract::{Json, Path, State};
use bdk::prelude::*;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct UpdateSpaceFileRequest {
    #[schemars(description = "Space Files")]
    pub files: Vec<File>,
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

    let (pk, sk) = SpaceFile::keys(&space_pk);

    let files = SpaceFile::get(&dynamo.client, &pk.clone(), Some(sk.clone())).await?;

    if files.is_some() {
        SpaceFile::updater(&pk.clone(), sk.clone())
            .with_files(req.files)
            .execute(&dynamo.client)
            .await?;
    } else {
        let files = SpaceFile::new(space_pk.clone(), req.files);

        files.create(&dynamo.client).await?;
    }

    let files = SpaceFile::get(&dynamo.client, &pk.clone(), Some(sk.clone())).await?;

    let files = files.unwrap();

    Ok(Json(UpdateSpaceFileResponse {
        files: files.clone().files,
    }))
}
