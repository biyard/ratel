#![allow(warnings)]
use crate::{
    AppState, Error,
    controllers::v3::spaces::{SpacePath, SpacePathParam, SpacePostPath, SpacePostPathParam},
    features::spaces::boards::{
        dto::{
            list_space_posts_response::ListSpacePostsResponse,
            space_post_response::SpacePostResponse,
        },
        models::{
            space_category::SpaceCategory,
            space_post::{SpacePost, SpacePostQueryOption},
        },
    },
    models::{SpaceCommon, feed::Post, team::Team, user::User},
    types::{EntityType, Partition, TeamGroupPermission, author::Author},
};
use aide::NoApi;
use axum::extract::{Json, Path, Query, State};
use bdk::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct ListSpacePostQueryParams {
    pub bookmark: Option<String>,
}

pub async fn list_space_posts_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Query(ListSpacePostQueryParams { bookmark }): Query<ListSpacePostQueryParams>,
) -> Result<Json<ListSpacePostsResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    let mut query_options = SpacePostQueryOption::builder()
        .sk("SPACE_POST#".into())
        .limit(10);

    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    let (responses, bookmark) =
        SpacePost::query(&dynamo.client, space_pk.clone(), query_options).await?;

    let mut posts = vec![];

    for response in responses {
        let post: SpacePostResponse = response.clone().into();
        posts.push(post);
    }

    Ok(Json(ListSpacePostsResponse { posts, bookmark }))
}
