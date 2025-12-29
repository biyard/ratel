use crate::controllers::v3::spaces::dto::*;
use crate::features::spaces::files::{
    FileLink, FileLinkInfo, GetFilesByTargetRequest, GetFilesByTargetResponse, ListFileLinksResponse,
};
use crate::types::{Partition, TeamGroupPermission};
use crate::{AppState, Error, Permissions};
use aide::NoApi;
use axum::extract::{Json, Path, Query, State};
use bdk::prelude::*;

/// Get all files linked to a specific target
pub async fn get_files_by_target_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Query(req): Query<GetFilesByTargetRequest>,
) -> Result<Json<GetFilesByTargetResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundDeliberationSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceRead) {
        return Err(Error::NoPermission);
    }

    let file_urls =
        FileLink::get_files_by_target(&dynamo.client, &space_pk, &req.target).await?;

    Ok(Json(GetFilesByTargetResponse {
        target: req.target,
        file_urls,
    }))
}

/// List all file links in a space
pub async fn list_file_links_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> Result<Json<ListFileLinksResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundDeliberationSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceRead) {
        return Err(Error::NoPermission);
    }

    let file_links = FileLink::list_by_space(&dynamo.client, &space_pk).await?;

    let file_link_infos: Vec<FileLinkInfo> = file_links
        .into_iter()
        .map(|link| FileLinkInfo {
            file_url: link.file_url,
            link_target: link.link_target,
            created_at: link.created_at,
            updated_at: link.updated_at,
        })
        .collect();

    Ok(Json(ListFileLinksResponse {
        file_links: file_link_infos,
    }))
}
