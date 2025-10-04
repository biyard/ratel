use crate::models::feed::Post;
use crate::models::user::User;
use crate::{AppState, Error2};
use aide::NoApi;
use bdk::prelude::*;
use by_axum::axum::extract::{Json, Path, State};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct LikePostRequest {
    pub like: bool,
}

#[derive(Debug, Serialize, Default, aide::OperationIo, JsonSchema)]
pub struct LikePostResponse {
    pub like: bool,
}

pub async fn like_post_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(User { pk: user_pk, .. }): NoApi<User>,
    Path(super::dto::PostPathParam { post_pk }): super::dto::PostPath,
    Json(req): Json<LikePostRequest>,
) -> Result<Json<LikePostResponse>, Error2> {
    let cli = &dynamo.client;

    if req.like {
        Post::like(cli, post_pk, user_pk).await?;
        Ok(Json(LikePostResponse { like: true }))
    } else {
        Post::unlike(cli, post_pk, user_pk).await?;
        Ok(Json(LikePostResponse { like: false }))
    }
}
