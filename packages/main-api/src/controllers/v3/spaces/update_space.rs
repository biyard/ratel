use crate::controllers::v3::spaces::dto::*;
use crate::models::space::SpaceCommon;

use crate::models::Post;
use crate::models::user::User;
use crate::types::{EntityType, SpacePublishState, SpaceVisibility, TeamGroupPermission};
use crate::{AppState, Error2, transact_write_items};
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
            let updater = SpaceCommon::updater(&space.pk, &space.sk)
                .with_publish_state(SpacePublishState::Published)
                .with_visibility(visibility.clone());

            let mut tx = vec![updater.transact_write_item()];

            if space.visibility == SpaceVisibility::Public {
                tx.push(
                    Post::updater(space.pk.clone().to_post_key()?, EntityType::Post)
                        .with_updated_at(chrono::Utc::now().timestamp_millis())
                        .with_space_visibility(SpaceVisibility::Public)
                        .transact_write_item(),
                );
            }

            transact_write_items!(dynamo.client, tx)?;

            space.publish_state = SpacePublishState::Published;
            space.visibility = visibility;

            Ok(Json(SpaceCommonResponse::from(space)))
        }
        UpdateSpaceRequest::Visibility { visibility } => {
            let updater =
                SpaceCommon::updater(&space.pk, &space.sk).with_visibility(visibility.clone());
            updater.execute(&dynamo.client).await?;

            space.visibility = visibility;
            Ok(Json(SpaceCommonResponse::from(space)))
        }
        UpdateSpaceRequest::Content { content } => {
            let updater = SpaceCommon::updater(&space.pk, &space.sk).with_content(content.clone());
            updater.execute(&dynamo.client).await?;

            space.content = content;
            Ok(Json(SpaceCommonResponse::from(space)))
        }
        UpdateSpaceRequest::Title { title } => {
            let post_pk = space.pk.clone().to_post_key()?;
            let updater = Post::updater(&post_pk, EntityType::Post).with_title(title.clone());
            updater.execute(&dynamo.client).await?;

            Ok(Json(SpaceCommonResponse::from(space)))
        }
    }
}
