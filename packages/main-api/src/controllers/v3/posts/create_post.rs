use crate::{
    AppState, Error2,
    models::{feed::Post, team::Team, user::User},
    types::{Partition, TeamGroupPermission, author::Author},
};
use aide::NoApi;
use axum::extract::{Json, State};
use bdk::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, serde::Serialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreatePostRequest {
    pub team_pk: Option<Partition>,
}

#[derive(Debug, Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreatePostResponse {
    pub post_pk: Partition,
}

pub async fn create_post_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Json(req): Json<CreatePostRequest>,
) -> Result<Json<CreatePostResponse>, Error2> {
    let cli = &dynamo.client;
    let author: Author = if let Some(team_pk) = req.team_pk {
        Team::get_permitted_team(cli, team_pk, user.pk, TeamGroupPermission::PostWrite)
            .await?
            .into()
    } else {
        user.into()
    };

    let post = Post::draft(author);
    post.create(cli).await?;

    Ok(Json(CreatePostResponse { post_pk: post.pk }))
}
