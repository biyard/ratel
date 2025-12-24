use crate::controllers::v3::spaces::dto::*;

use crate::types::{Partition, TeamGroupPermission};
use crate::{AppState, Error, Permissions};

use aide::NoApi;

use crate::features::spaces::files::{FileLink, FileLinkTarget, SpaceFile};
use crate::types::File;
use axum::extract::{Json, Path, State};
use bdk::prelude::*;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct UpdateSpaceFileRequest {
    #[schemars(description = "Space Files")]
    pub files: Vec<File>,

    #[schemars(description = "Optional: Link these files to additional targets")]
    #[serde(default)]
    pub link_targets: Vec<FileLinkTarget>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct UpdateSpaceFileResponse {
    pub files: Vec<File>,
}

//FIXME: implement with dynamodb upsert method
pub async fn update_files_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<UpdateSpaceFileRequest>,
) -> Result<Json<UpdateSpaceFileResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundDeliberationSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceEdit) {
        return Err(Error::NoPermission);
    }

    let (pk, sk) = SpaceFile::keys(&space_pk);

    let files = SpaceFile::get(&dynamo.client, &pk.clone(), Some(sk.clone())).await?;

    if files.is_some() {
        SpaceFile::updater(&pk.clone(), sk.clone())
            .with_files(req.files.clone())
            .execute(&dynamo.client)
            .await?;
    } else {
        let files = SpaceFile::new(space_pk.clone(), req.files.clone());

        files.create(&dynamo.client).await?;
    }

    // Create file links if targets are specified
    if !req.link_targets.is_empty() {
        let file_urls: Vec<String> = req.files.iter().filter_map(|f| f.url.clone()).collect();
        if !file_urls.is_empty() {
            FileLink::add_link_targets_batch(
                &dynamo.client,
                space_pk.clone(),
                file_urls,
                req.link_targets.clone(),
            )
            .await?;
        }
    }

    let files = SpaceFile::get(&dynamo.client, &pk.clone(), Some(sk.clone())).await?;

    let files = files.unwrap();

    Ok(Json(UpdateSpaceFileResponse {
        files: files.clone().files,
    }))
}
