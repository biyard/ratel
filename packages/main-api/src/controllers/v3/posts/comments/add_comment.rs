use crate::{
    AppState, Error2,
    models::feed::{Post, PostComment},
    types::{EntityType, Partition, TeamGroupPermission},
    utils::{
        dynamo_extractor::extract_user,
        security::{RatelResource, check_permission},
    },
};
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
pub struct AddCommentPathParams {
    pub post_pk: String,
}

#[derive(Debug, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct AddCommentRequest {
    pub content: String,
}

#[derive(Debug, Serialize, Default, aide::OperationIo, JsonSchema)]
pub struct AddCommentResponse {
    pub comment_pk: String,
}

pub async fn add_comment_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Path(params): Path<AddCommentPathParams>,
    Json(req): Json<AddCommentRequest>,
) -> Result<Json<AddCommentResponse>, Error2> {
    let user = extract_user(&dynamo.client, auth.clone()).await?;
    let post = Post::get(&dynamo.client, &params.post_pk, Some(EntityType::Post))
        .await?
        .ok_or(Error2::BadRequest("Invalid post ID".to_string()))?;
    match post.user_pk {
        Partition::Team(_) => {
            check_permission(
                &dynamo.client,
                auth.clone(),
                RatelResource::Team {
                    team_pk: post.user_pk.to_string(),
                },
                vec![TeamGroupPermission::PostRead],
            )
            .await?;
        }
        _ => {}
    }

    let comment = PostComment::new(post.pk.clone(), req.content, user);
    comment.create(&dynamo.client).await?;
    Post::updater(&post.pk, &post.sk)
        .increase_comments(1)
        .execute(&dynamo.client)
        .await?;

    Ok(Json(AddCommentResponse {
        comment_pk: comment.pk.to_string(),
    }))
}
