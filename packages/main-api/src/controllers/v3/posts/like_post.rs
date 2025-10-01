use std::str::FromStr;

use crate::models::feed::PostLike;
use crate::models::user::User;
use crate::types::Partition;
use crate::{AppState, Error2};
use aide::NoApi;
use bdk::prelude::*;
use by_axum::axum::extract::{Json, Path, State};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct LikePostPathParams {
    pub post_pk: String,
}

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
    Path(params): Path<LikePostPathParams>,
    Json(req): Json<LikePostRequest>,
) -> Result<Json<LikePostResponse>, Error2> {
    let cli = &dynamo.client;
    let post_pk = Partition::from_str(&params.post_pk)?;

    if req.like {
        PostLike::like(cli, post_pk, user_pk).await?;
        Ok(Json(LikePostResponse { like: true }))
    } else {
        PostLike::unlike(cli, post_pk, user_pk).await?;
        Ok(Json(LikePostResponse { like: false }))
    }
}
