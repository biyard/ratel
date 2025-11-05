#![allow(warnings)]
use crate::{
    AppState, Error,
    models::{feed::Post, team::Team, user::User},
    types::{Partition, TeamGroupPermission, author::Author},
};
use aide::NoApi;
use axum::extract::{Json, State};
use bdk::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, serde::Serialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreatePostRequest {
    pub team_pk: Partition,
}

#[derive(Debug, Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreatePostResponse {
    pub post_pk: Partition,
}

pub async fn create_post_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    req: Option<Json<CreatePostRequest>>,
) -> Result<Json<CreatePostResponse>, Error> {
    tracing::debug!("create_post_handler {:?}", req);
    let cli = &dynamo.client;
    let author: Author = if let Some(Json(CreatePostRequest { team_pk })) = req {
        tracing::info!(
            "Creating post under team: {:?} by user {:?}",
            team_pk,
            user.pk
        );
        Team::get_permitted_team(cli, team_pk, user.pk, TeamGroupPermission::PostWrite)
            .await?
            .into()
    } else {
        user.into()
    };

    tracing::info!("Creating post for author: {:?}", author);
    let post = Post::draft(author);
    post.create(cli).await?;

    Ok(Json(CreatePostResponse { post_pk: post.pk }))
}
