use std::str::FromStr;

use crate::{
    AppState, Error2,
    models::{
        feed::{Post, PostDetailResponse, PostMetadata},
        user::User,
    },
    types::Partition,
};
use dto::{JsonSchema, aide, schemars};
use dto::{
    aide::NoApi,
    by_axum::axum::{
        Json,
        extract::{Path, State},
    },
};
use serde::Deserialize;

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct GetPostPathParams {
    pub post_pk: String,
}

pub type GetPostResponse = PostDetailResponse;

pub async fn get_post_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(GetPostPathParams { post_pk }): Path<GetPostPathParams>,
) -> Result<Json<GetPostResponse>, Error2> {
    let cli = &dynamo.client;
    let post_pk = Partition::from_str(&post_pk)?;

    if !Post::has_permission(
        cli,
        &post_pk,
        if let Some(ref user) = user {
            Some(&user.pk)
        } else {
            None
        },
        crate::types::TeamGroupPermission::PostRead,
    )
    .await?
    .1
    {
        return Err(Error2::NoPermission);
    };

    let post_metadata = PostMetadata::query(cli, &post_pk).await?;

    Ok(Json(post_metadata.into()))
}
