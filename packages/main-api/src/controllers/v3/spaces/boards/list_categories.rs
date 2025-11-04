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
            space_category::{SpaceCategory, SpaceCategoryQueryOption},
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

pub async fn list_categories_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> Result<Json<Vec<String>>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    let mut categories: Vec<String> = vec![];
    let mut bookmark = None::<String>;

    loop {
        let (responses, new_bookmark) = SpaceCategory::query(
            &dynamo.client,
            space_pk.clone(),
            if let Some(b) = &bookmark {
                SpaceCategoryQueryOption::builder()
                    .sk("SPACE_CATEGORY#".into())
                    .bookmark(b.clone())
            } else {
                SpaceCategoryQueryOption::builder().sk("SPACE_CATEGORY#".into())
            },
        )
        .await?;

        for response in responses {
            categories.push(response.name);
        }

        match new_bookmark {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    Ok(Json(categories))
}
