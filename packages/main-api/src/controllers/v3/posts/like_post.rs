use std::str::FromStr;

use crate::models::feed::{Post, PostLike};
use crate::types::{EntityType, Partition, TeamGroupPermission, Visibility};
use crate::utils::dynamo_extractor::extract_user;
use crate::utils::security::{RatelResource, check_permission};
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
    let user = extract_user(&dynamo.client, auth.clone()).await?;

    let post = Post::get(&dynamo.client, &params.post_pk, Some(EntityType::Post))
        .await?
        .ok_or(Error2::NotFound("Post not found".to_string()))?;
    if let Some(Visibility::TeamOnly(team_pk)) = post.visibility.clone() {
        check_permission(
            &dynamo.client,
            auth,
            RatelResource::Team { team_pk },
            vec![TeamGroupPermission::PostRead],
        )
        .await?;
    }

    let pk = Partition::from_str(&params.post_pk)?;
    let like_sk = EntityType::PostLike(user.pk.to_string());
    let already_liked = PostLike::get(&dynamo.client, &params.post_pk, Some(like_sk.clone()))
        .await?
        .is_some();

    if req.like && !already_liked {
        PostLike::new(pk.clone(), user)
            .create(&dynamo.client)
            .await?;
        Post::updater(&pk, EntityType::Post)
            .increase_likes(1)
            .execute(&dynamo.client)
            .await?;
    } else if !req.like && already_liked {
        PostLike::delete(&dynamo.client, &params.post_pk, Some(like_sk)).await?;
        Post::updater(&pk, EntityType::Post)
            .decrease_likes(1)
            .execute(&dynamo.client)
            .await?;
    }

    Ok(Json(LikePostResponse { like: req.like }))
}
