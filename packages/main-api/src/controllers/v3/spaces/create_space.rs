use crate::models::feed::Post;
use crate::models::space::SpaceCommon;
use crate::models::user::User;
use crate::types::{Partition, SpaceType, TeamGroupPermission};
use crate::{AppState, Error2, transact_write};
use aide::NoApi;
use axum::extract::{Json, State};
use bdk::prelude::*;

use serde::{Deserialize, Serialize};

// #[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
// pub struct CreateSpacePathParams {
//     post_pk: Partition,
// }

#[derive(Debug, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreateSpaceRequest {
    pub(crate) space_type: SpaceType,
    pub(crate) post_pk: Partition,
}

#[derive(Debug, Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreateSpaceResponse {
    pub space_pk: Partition,
}

pub async fn create_space_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Json(CreateSpaceRequest {
        space_type,
        post_pk,
    }): Json<CreateSpaceRequest>,
) -> Result<Json<CreateSpaceResponse>, Error2> {
    let (post, has_perm) = Post::has_permission(
        &dynamo.client,
        &post_pk,
        Some(&user.pk),
        TeamGroupPermission::PostEdit,
    )
    .await?;
    if !has_perm {
        return Err(Error2::NoPermission);
    }

    let space = SpaceCommon::new(post_pk, user);

    let post_updater = Post::updater(&post.pk, &post.sk)
        .with_space_pk(space.pk.clone())
        .with_space_type(space_type);

    // let space_tx = space.create_transact_write_item();
    // let post_tx = post_updater.transact_write_item();

    transact_write!(
        dynamo.client,
        space.create_transact_write_item(),
        post_updater.transact_write_item()
    )?;
    // dynamo
    //     .client
    //     .transact_write_items()
    //     .set_transact_items(Some(vec![space_tx, post_tx]))
    //     .send()
    //     .await
    //     .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

    Ok(Json(CreateSpaceResponse { space_pk: space.pk }))
}
