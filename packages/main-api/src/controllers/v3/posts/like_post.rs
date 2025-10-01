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
    NoApi(user): NoApi<User>,
    Path(params): Path<LikePostPathParams>,
    Json(req): Json<LikePostRequest>,
) -> Result<Json<LikePostResponse>, Error2> {
    let pk = Partition::from_str(&params.post_pk)?;
    PostLike::new(pk, user).create(&dynamo.client).await?;

    Ok(Json(LikePostResponse { like: req.like }))
}
