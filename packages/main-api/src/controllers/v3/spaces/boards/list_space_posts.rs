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
    Path(SpacePathParam { space_pk }): SpacePath,
    Query(ListSpacePostQueryParams { bookmark, category }): Query<ListSpacePostQueryParams>,
) -> Result<Json<ListSpacePostsResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    let opt = SpacePost::opt_with_bookmark(bookmark);

    let (responses, bookmark) = if let Some(category) = category {
        SpacePost::find_by_category(&dynamo.client, format!("{}#{}", space_pk, category), opt)
            .await?
    } else {
        SpacePost::find_by_space_ordered(&dynamo.client, space_pk.clone(), opt).await?
    };

    Ok(Json(ListSpacePostsResponse {
        posts: responses.into_iter().map(|p| p.into()).collect(),
        bookmark,
    }))
}
