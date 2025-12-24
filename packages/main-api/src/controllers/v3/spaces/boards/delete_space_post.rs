#![allow(warnings)]
use crate::{
    AppState, Error, Permissions,
    controllers::v3::spaces::{SpacePath, SpacePathParam, SpacePostPath, SpacePostPathParam},
    features::spaces::{
        boards::models::{space_category::SpaceCategory, space_post::SpacePost},
        files::{FileLink, FileLinkTarget},
    },
    models::{SpaceCommon, feed::Post, team::Team, user::User},
    types::{EntityType, Partition, TeamGroupPermission, author::Author},
};
use aide::NoApi;
use axum::extract::{Json, Path, State};
use bdk::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct DeleteSpacePostResponse {
    pub space_post_pk: Partition,
}

pub async fn delete_space_post_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePostPathParam {
        space_pk,
        space_post_pk,
    }): SpacePostPath,
) -> Result<Json<DeleteSpacePostResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceEdit) {
        return Err(Error::NoPermission);
    }

    let space_post_id = match space_post_pk {
        Partition::SpacePost(v) => v.to_string(),
        _ => "".to_string(),
    };

    let post = SpacePost::get(
        &dynamo.client,
        space_pk.clone(),
        Some(EntityType::SpacePost(space_post_id.clone())),
    )
    .await?;

    SpacePost::delete(
        &dynamo.client,
        space_pk.clone(),
        Some(EntityType::SpacePost(space_post_id.clone())),
    )
    .await?;

    if let Some(post) = post {
        if let Some(files) = post.files {
            let file_urls: Vec<String> = files.iter().filter_map(|f| f.url.clone()).collect();
            if !file_urls.is_empty() {
                FileLink::remove_link_targets_batch(
                    &dynamo.client,
                    &space_pk,
                    file_urls,
                    &FileLinkTarget::Board(space_post_id.clone()),
                )
                .await
                .ok();
            }
        }
    }

    Ok(Json(DeleteSpacePostResponse {
        space_post_pk: Partition::SpacePost(space_post_id),
    }))
}
