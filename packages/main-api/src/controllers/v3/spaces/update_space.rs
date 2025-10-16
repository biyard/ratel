use crate::controllers::v3::spaces::dto::*;
use crate::models::space::SpaceCommon;

use crate::models::user::User;
use crate::types::{SpacePublishState, SpaceVisibility, TeamGroupPermission};
use crate::{AppState, Error2};
use aide::NoApi;
use axum::extract::{Json, Path, State};
use bdk::prelude::*;

use serde::Deserialize;
#[derive(Debug, Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
#[serde(untagged)]
pub enum UpdateSpaceRequest {
    Publish {
        publish: bool,
        visibility: SpaceVisibility,
    },
    Visibility {
        visibility: SpaceVisibility,
    },
}

pub async fn update_space_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): Path<SpacePathParam>,
    Json(req): Json<UpdateSpaceRequest>,
) -> Result<Json<SpaceCommonResponse>, Error2> {
    let (mut space, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        Some(&user.pk),
        TeamGroupPermission::PostEdit,
    )
    .await?;
    if !has_perm {
        return Err(Error2::NoPermission);
    }

    match req {
        UpdateSpaceRequest::Publish {
            publish,
            visibility,
        } => {
            if !publish {
                return Err(Error2::NotSupported(
                    "it does not support unpublished now".into(),
                ));
            }
            space.publish_state = SpacePublishState::Published;
            let mut updater: crate::models::space::SpaceCommonUpdater =
                SpaceCommon::updater(&space.pk, &space.sk)
                    .with_publish_state(SpacePublishState::Published);

            updater = updater.with_visibility(visibility.clone());
            space.visibility = visibility;
            updater.execute(&dynamo.client).await?;

            Ok(Json(SpaceCommonResponse::from(space)))
        }
        UpdateSpaceRequest::Visibility { visibility } => {
            let updater =
                SpaceCommon::updater(&space.pk, &space.sk).with_visibility(visibility.clone());
            updater.execute(&dynamo.client).await?;

            space.visibility = visibility;
            Ok(Json(SpaceCommonResponse::from(space)))
        }
    }
}
