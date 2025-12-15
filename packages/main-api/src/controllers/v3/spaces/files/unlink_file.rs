use crate::controllers::v3::spaces::dto::*;
use crate::features::spaces::files::{FileLink, UnlinkFileRequest, UnlinkFileResponse};
use crate::types::{Partition, TeamGroupPermission};
use crate::{AppState, Error, Permissions};
use aide::NoApi;
use axum::extract::{Json, Path, State};
use bdk::prelude::*;

/// Unlink a file from specified targets
pub async fn unlink_file_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<UnlinkFileRequest>,
) -> Result<Json<UnlinkFileResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundDeliberationSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceEdit) {
        return Err(Error::NoPermission);
    }

    let mut remaining_targets = vec![];

    // Remove each target
    for target in req.targets {
        if let Some(file_link) = FileLink::remove_link_target(
            &dynamo.client,
            &space_pk,
            &req.file_url,
            &target,
        )
        .await?
        {
            remaining_targets = file_link.link_targets;
        }
    }

    Ok(Json(UnlinkFileResponse {
        file_url: req.file_url,
        remaining_targets,
    }))
}
