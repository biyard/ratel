use std::str::FromStr;

use crate::models::feed::{Post, PostLike};
use crate::types::{EntityType, Partition};
use crate::utils::dynamo_extractor::extract_user;
use crate::{AppState, Error2};
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension,
        extract::{Json, Path, State},
    },
};
use dto::{JsonSchema, aide, schemars};
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
    Extension(auth): Extension<Option<Authorization>>,
    Path(params): Path<LikePostPathParams>,
    Json(req): Json<LikePostRequest>,
) -> Result<Json<LikePostResponse>, Error2> {
    let user = extract_user(&dynamo.client, auth).await?;

    Post::get(&dynamo.client, &params.post_pk, Some(EntityType::Post))
        .await?
        .ok_or(Error2::NotFound("Post not found".to_string()))?;
    let pk = Partition::from_str(&params.post_pk)?;
    if req.like {
        PostLike::new(pk.clone(), user)
            .create(&dynamo.client)
            .await?;
        Post::updater(&pk, EntityType::Post)
            .increase_likes(1)
            .execute(&dynamo.client)
            .await?;
    } else {
        PostLike::delete(
            &dynamo.client,
            &params.post_pk,
            Some(EntityType::PostLike(user.pk.to_string())),
        )
        .await?;
        Post::updater(&pk, EntityType::Post)
            .decrease_likes(1)
            .execute(&dynamo.client)
            .await?;
    }

    Ok(Json(LikePostResponse { like: req.like }))
}
