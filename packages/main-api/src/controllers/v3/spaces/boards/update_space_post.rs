#![allow(warnings)]
use crate::File;
use crate::{
    AppState, Error, Permissions,
    controllers::v3::spaces::{SpacePath, SpacePathParam, SpacePostPath, SpacePostPathParam},
    features::spaces::{
        boards::{
            dto::space_post_response::SpacePostResponse,
            models::{space_category::SpaceCategory, space_post::SpacePost},
        },
        files::{FileLink, FileLinkTarget, SpaceFile},
    },
    models::{SpaceCommon, feed::Post, team::Team, user::User},
    types::{EntityType, Partition, TeamGroupPermission, author::Author},
};
use aide::NoApi;
use axum::extract::{Json, Path, State};
use bdk::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, serde::Serialize, Default, aide::OperationIo, JsonSchema)]
pub struct UpdateSpacePostRequest {
    pub title: String,
    pub html_contents: String,
    pub category_name: String,
    pub urls: Vec<String>,
    pub files: Vec<File>,
    pub started_at: i64,
    pub ended_at: i64,
}

pub async fn update_space_post_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePostPathParam {
        space_pk,
        space_post_pk,
    }): SpacePostPath,
    Json(req): Json<UpdateSpacePostRequest>,
) -> Result<Json<SpacePostResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceEdit) {
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

    let (pk, sk) = SpacePost::keys(&space_pk, &space_post_pk);

    // Get existing post to compare files
    let existing_post = SpacePost::get(&dynamo.client, pk.clone(), Some(sk.clone())).await?;
    let old_file_urls: Vec<String> = existing_post
        .as_ref()
        .and_then(|p| p.files.as_ref())
        .map(|files| files.iter().filter_map(|f| f.url.clone()).collect())
        .unwrap_or_default();

    let v = SpacePost::updater(pk, sk)
        .with_title(req.title.clone())
        .with_html_contents(req.html_contents.clone())
        .with_category_name(req.category_name.clone())
        .with_urls(req.urls.clone())
        .with_files(req.files.clone())
        .with_started_at(req.started_at.clone())
        .with_ended_at(req.ended_at.clone())
        .execute(&dynamo.client)
        .await?;

    // Link files to both Files tab and Board
    let post_id = match &v.sk {
        EntityType::SpacePost(id) => id.to_string(),
        _ => "".to_string(),
    };

    // Add files to SpaceFile entity (for Files tab)
    if !req.files.is_empty() {
        SpaceFile::add_files(&dynamo.client, space_pk.clone(), req.files.clone()).await?;
    }

    // Link files: Batch add both Files and Board targets to all file URLs
    let new_file_urls: Vec<String> = req.files.iter().filter_map(|f| f.url.clone()).collect();
    if !new_file_urls.is_empty() {
        FileLink::add_link_targets_batch(
            &dynamo.client,
            space_pk.clone(),
            new_file_urls.clone(),
            vec![
                FileLinkTarget::Files,
                FileLinkTarget::Board(post_id.clone()),
            ],
        )
        .await?;
    }

    // Remove Board target from files that were removed
    let removed_urls: Vec<String> = old_file_urls
        .into_iter()
        .filter(|url| !new_file_urls.contains(url))
        .collect();
    if !removed_urls.is_empty() {
        FileLink::remove_link_targets_batch(
            &dynamo.client,
            &space_pk,
            removed_urls,
            &FileLinkTarget::Board(post_id.clone()),
        )
        .await
        .ok();
    }

    Ok(Json(v.into()))
}
