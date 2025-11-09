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
    pub category: Option<String>,
}

pub async fn list_space_posts_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Query(ListSpacePostQueryParams { bookmark, category }): Query<ListSpacePostQueryParams>,
) -> Result<Json<ListSpacePostsResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    if user.is_none() {
        return Ok(Json(ListSpacePostsResponse {
            posts: vec![],
            bookmark: None,
        }));
    }

    let mut query_options = SpacePostQueryOption::builder()
        .limit(50)
        .scan_index_forward(false);

    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    // FIXME: fix to enhance this logic
    let (responses, bookmark) = if category.is_none() {
        SpacePost::find_by_space_ordered(&dynamo.client, space_pk.clone(), query_options).await?
    } else {
        let (posts, bookmark) = SpacePost::find_by_cagetory(
            &dynamo.client,
            category.clone().unwrap_or_default(),
            SpacePostQueryOption::builder().limit(50),
        )
        .await?;

        let mut posts = posts
            .iter()
            .filter(|v| v.pk == space_pk.clone())
            .map(|v| v.clone())
            .collect();

        (posts, bookmark)
    };

    let mut posts = vec![];

    for response in responses {
        let post: SpacePostResponse = response.clone().into();
        posts.push(post);
    }

    Ok(Json(ListSpacePostsResponse { posts, bookmark }))
}
