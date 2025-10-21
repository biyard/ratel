use crate::controllers::v3::spaces::dto::*;
use crate::models::space::SpaceCommon;

use crate::models::Post;
use crate::models::user::User;
use crate::types::{EntityType, SpacePublishState, SpaceVisibility, TeamGroupPermission};
use crate::{AppState, Error2, transact_write};
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
    Content {
        content: String,
    },
    Title {
        title: String,
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

    let now = chrono::Utc::now().timestamp_millis();
    let mut su = SpaceCommon::updater(&space.pk, &space.sk).with_updated_at(now);
    let mut pu =
        Post::updater(space.pk.clone().to_post_key()?, EntityType::Post).with_updated_at(now);

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
            // FIXME: check validation if it is well designed to be published.
            su = su
                .with_publish_state(SpacePublishState::Published)
                .with_visibility(visibility.clone());

            pu = pu.with_space_visibility(SpaceVisibility::Public);

            space.publish_state = SpacePublishState::Published;
            space.visibility = visibility;
        }
        UpdateSpaceRequest::Visibility { visibility } => {
            su = su.with_visibility(visibility.clone());

            space.visibility = visibility;
        }
        UpdateSpaceRequest::Content { content } => {
            su = su.with_content(content.clone());

            space.content = content;
        }
        UpdateSpaceRequest::Title { title } => {
            pu = pu.with_title(title.clone());
        }
    }

    transact_write!(
        dynamo.client,
        su.transact_write_item(),
        pu.transact_write_item()
    )?;

    Ok(Json(SpaceCommonResponse::from(space)))
}
