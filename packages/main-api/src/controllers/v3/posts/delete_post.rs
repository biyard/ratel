use crate::{
    AppState, Error2,
    models::{feed::Post, user::User},
    types::{EntityType, Partition},
};
use aide::NoApi;
use axum::extract::Query;
use bdk::prelude::*;
use by_axum::axum::extract::{Json, Path, State};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, serde::Serialize, aide::OperationIo, JsonSchema, Validate)]
pub struct DeletePostParams {
    pub force: Option<bool>,
}

#[derive(Debug, Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct DeletePostResponse {
    pub dependancies: Vec<Partition>,
}

pub async fn delete_post_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Query(DeletePostParams { force }): Query<DeletePostParams>,
    Path(super::dto::PostPathParam { post_pk }): super::dto::PostPath,
) -> Result<Json<Post>, Error2> {
    let cli = &dynamo.client;

    if !Post::has_permission(
        cli,
        &post_pk,
        Some(&user.pk),
        crate::types::TeamGroupPermission::PostDelete,
    )
    .await?
    .1
    {
        return Err(Error2::NoPermission);
    }

    // TODO: Check dependancies
    let dependancies = vec![];

    let force = force.unwrap_or(false);

    if force {
        // TODO: delete all dependancies
        tracing::warn!("Force delete is not implemented yet");
    } else if !dependancies.is_empty() {
        return Err(Error2::HasDependencies(dependancies));
    }

    let post = Post::delete(cli, post_pk, Some(EntityType::Post)).await?;

    Ok(Json(post))
}
