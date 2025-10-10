use bdk::prelude::*;

use crate::{
    AppState, Error2,
    controllers::v3::posts::PostPath,
    models::{
        feed::{Post, PostComment},
        team::Team,
        user::User,
    },
    types::{EntityType, Partition, TeamGroupPermission},
};
use aide::NoApi;
use by_axum::axum::extract::{Json, Path, State};
use serde::{Deserialize, Serialize};

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
    NoApi(user): NoApi<User>,
    Path(params): PostPath,
    Json(req): Json<AddCommentRequest>,
) -> Result<Json<PostComment>, Error2> {
    let post = Post::get(&dynamo.client, &params.post_pk, Some(EntityType::Post))
        .await?
        .ok_or(Error2::PostNotFound)?;
    match &post.user_pk {
        team_pk if matches!(team_pk, &Partition::Team(_)) => {
            Team::has_permission(
                &dynamo.client,
                &team_pk,
                &user.pk,
                TeamGroupPermission::PostRead,
            )
            .await?;
        }
        _ => {}
    }

    let comment = Post::comment(&dynamo.client, post.pk.clone(), req.content, user).await?;

    Ok(Json(comment))
}
