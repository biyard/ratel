use crate::features::spaces::discussions::models::space_discussion::SpaceDiscussion;
use crate::features::spaces::files::SpaceFile;
use crate::features::spaces::polls::Poll;
use crate::features::spaces::recommendations::SpaceRecommendation;
use crate::models::Post;
use crate::models::space::SpaceCommon;
use crate::types::{EntityType, Partition};
use crate::utils::dynamo_extractor::extract_user_from_session;
use crate::{AppState, Error2};
use axum::{
    Extension,
    extract::{Path, State},
};
use bdk::prelude::*;

use serde::Deserialize;

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct DeleteSpacePathParams {
    #[schemars(description = "Space PK to be deleted")]
    #[serde(deserialize_with = "crate::types::path_param_string_to_partition")]
    pub space_pk: Partition,
}

pub async fn delete_space_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<tower_sessions::Session>,
    Path(DeleteSpacePathParams { space_pk }): Path<DeleteSpacePathParams>,
) -> Result<(), Error2> {
    let post_pk = space_pk.clone().to_post_key()?;

    let _user = extract_user_from_session(&dynamo.client, &session).await?;
    // FIXME: ADD PERMISSION CHECK
    let space_common = SpaceCommon::get(&dynamo.client, &space_pk, Some(EntityType::SpaceCommon))
        .await?
        .ok_or(Error2::SpaceNotFound)?;

    // FIXME: fix to metadata model
    SpaceDiscussion::delete_all(&dynamo.client, &space_common.pk).await?;
    SpaceCommon::delete(&dynamo.client, &space_common.pk, Some(space_common.sk)).await?;
    SpaceFile::delete_one(&dynamo.client, &space_common.pk).await?;
    Poll::delete_one(&dynamo.client, &space_common.pk).await?;
    SpaceRecommendation::delete_one(&dynamo.client, &space_common.pk).await?;

    let _ = Post::updater(post_pk, EntityType::Post)
        .remove_space_pk()
        .remove_space_type()
        .remove_space_visibility()
        .execute(&dynamo.client)
        .await?;

    Ok(())
}
