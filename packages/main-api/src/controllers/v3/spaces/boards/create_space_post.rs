#![allow(warnings)]
use crate::{
    AppState, Error,
    controllers::v3::spaces::{SpacePath, SpacePathParam},
    features::spaces::boards::models::{space_category::SpaceCategory, space_post::SpacePost},
    models::{SpaceCommon, feed::Post, team::Team, user::User},
    types::{EntityType, Partition, TeamGroupPermission, author::Author},
};
use aide::NoApi;
use axum::extract::{Json, Path, State};
use bdk::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, serde::Serialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreateSpacePostRequest {
    pub title: String,
    pub html_contents: String,
    pub category_name: String,
    pub urls: Vec<String>,
}

#[derive(Debug, Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreateSpacePostResponse {
    pub space_post_pk: Partition,
}

pub async fn create_space_post_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<CreateSpacePostRequest>,
) -> Result<Json<CreateSpacePostResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    let (_, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        Some(&user.pk),
        TeamGroupPermission::SpaceEdit,
    )
    .await?;
    if !has_perm {
        return Err(Error::NoPermission);
    }

    let category_name = req.category_name.clone();
    let category = SpaceCategory::get(
        &dynamo.client,
        space_pk.clone(),
        Some(EntityType::SpaceCategory(category_name.clone())),
    )
    .await?;

    if category.is_none() {
        let category = SpaceCategory::new(space_pk.clone(), category_name.clone());
        category.create(&dynamo.client).await?;
    }

    let post = SpacePost::new(
        space_pk,
        req.title.clone(),
        req.html_contents.clone(),
        req.category_name.clone(),
        req.urls.clone(),
        user,
    );
    post.create(&dynamo.client).await?;

    // TODO: alert message to user with email

    let post_id = match post.sk {
        EntityType::SpacePost(v) => v.to_string(),
        _ => "".to_string(),
    };

    Ok(Json(CreateSpacePostResponse {
        space_post_pk: Partition::SpacePost(post_id),
    }))
}
