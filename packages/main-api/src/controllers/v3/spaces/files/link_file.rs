use crate::controllers::v3::spaces::dto::*;
use crate::features::spaces::files::{FileLink, LinkFileRequest, LinkFileResponse};
use crate::types::{Partition, TeamGroupPermission};
use crate::{AppState, Error, Permissions};
use aide::NoApi;
use axum::extract::{Json, Path, State};
use bdk::prelude::*;

/// Link a file to additional targets (Overview, Board, etc.)
pub async fn link_file_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<LinkFileRequest>,
) -> Result<Json<LinkFileResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundDeliberationSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceEdit) {
        return Err(Error::NoPermission);
    }

    let mut linked_targets = vec![];

    // Add each target
    for target in req.targets {
        let file_link = FileLink::add_link_target(
            &dynamo.client,
            space_pk.clone(),
            req.file_url.clone(),
            target.clone(),
        )
        .await?;

        linked_targets = file_link.link_targets;
    }

    Ok(Json(LinkFileResponse {
        file_url: req.file_url,
        linked_targets,
    }))
}
