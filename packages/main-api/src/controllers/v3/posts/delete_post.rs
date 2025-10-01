use std::str::FromStr;

use crate::{
    AppState, Error2,
    models::{feed::Post, user::User},
    types::Partition,
};
use aide::NoApi;
use axum::extract::Query;
use bdk::prelude::*;
use by_axum::axum::extract::{Path, State};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct DeletePostPathParams {
    pub post_pk: String,
}

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
    Path(params): Path<DeletePostPathParams>,
) -> Result<(), Error2> {
    let cli = &dynamo.client;

    let post_pk = Partition::from_str(&params.post_pk)?;
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

    if force.unwrap_or(false) {
        // TODO: delete all dependancies
    } else {
        unimplemented!()
    }

    Post::delete(cli, post_pk, None::<String>).await?;

    Ok(())
}
