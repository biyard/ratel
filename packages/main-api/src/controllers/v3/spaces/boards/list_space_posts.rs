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
    types::{EntityType, Partition, Permissions, TeamGroupPermission, author::Author},
};
use aide::NoApi;
use axum::extract::{Json, Path, Query, State};
use bdk::prelude::*;
use by_axum::axum::Extension;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct ListSpacePostQueryParams {
    pub bookmark: Option<String>,
    pub category: Option<String>,
}

pub async fn list_space_posts_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Extension(space): Extension<SpaceCommon>,
    Query(ListSpacePostQueryParams { bookmark, category }): Query<ListSpacePostQueryParams>,
) -> Result<Json<ListSpacePostsResponse>, Error> {
    let now = chrono::Utc::now().timestamp() * 1000;
    let is_owner = permissions.contains(TeamGroupPermission::SpaceEdit);

    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    if !is_owner && space.status != Some(crate::types::SpaceStatus::InProgress) {
        return Ok(Json(ListSpacePostsResponse {
            posts: vec![],
            bookmark: None,
        }));
    }

    if user.is_none() {
        return Ok(Json(ListSpacePostsResponse {
            posts: vec![],
            bookmark: None,
        }));
    }

    let mut query_options = SpacePostQueryOption::builder()
        .limit(10)
        .scan_index_forward(false);

    if let Some(bookmark) = bookmark.clone() {
        query_options = query_options.bookmark(bookmark);
    }

    // FIXME: fix to enhance this logic
    let (responses, bookmark) = if category.is_none() {
        SpacePost::find_by_space_ordered(&dynamo.client, space_pk.clone(), query_options).await?
    } else {
        let mut cat_opt = SpacePostQueryOption::builder()
            .limit(10)
            .scan_index_forward(false);

        if let Some(bm) = bookmark.clone() {
            cat_opt = cat_opt.bookmark(bm);
        }

        let (posts, bookmark) = SpacePost::find_by_cagetory(
            &dynamo.client,
            category.clone().unwrap_or_default(),
            cat_opt,
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

        if is_owner || (post.started_at <= now && now <= post.ended_at) {
            posts.push(post);
        }
    }

    Ok(Json(ListSpacePostsResponse { posts, bookmark }))
}
