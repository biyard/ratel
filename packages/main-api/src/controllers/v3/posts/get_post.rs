use crate::{
    AppState, Error2,
    models::feed::{Post, PostDetailResponse, PostMetadata},
    types::{EntityType, TeamGroupPermission, Visibility},
    utils::security::{RatelResource, check_permission},
};
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, State},
    },
};
use dto::{JsonSchema, aide, schemars};
use serde::Deserialize;

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct GetPostPathParams {
    pub post_pk: String,
}

pub type GetPostResponse = PostDetailResponse;

pub async fn get_post_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Path(path): Path<GetPostPathParams>,
) -> Result<Json<GetPostResponse>, Error2> {
    let post = Post::get(&dynamo.client, &path.post_pk, Some(EntityType::Post))
        .await?
        .ok_or(Error2::NotFound("Post not found".to_string()))?;
    if let Some(Visibility::TeamOnly(team_pk)) = post.visibility {
        check_permission(
            &dynamo.client,
            auth,
            RatelResource::Team { team_pk },
            vec![TeamGroupPermission::PostRead],
        )
        .await?;
    }

    let post_metadata = PostMetadata::query(&dynamo.client, &path.post_pk).await?;

    let post_response: PostDetailResponse = post_metadata.into();

    Ok(Json(post_response))
}
