#![allow(warnings)]
use crate::File;
use crate::models::email_template::email_template::EmailTemplate;
use crate::services::fcm_notification::FCMService;
use crate::utils::html::create_space_post_html;
use crate::{
    AppState, Error, Permissions,
    controllers::v3::spaces::{SpacePath, SpacePathParam},
    features::spaces::{
        boards::models::{space_category::SpaceCategory, space_post::SpacePost},
        files::{FileLink, FileLinkTarget, SpaceFile},
        members::{SpaceInvitationMember, SpaceInvitationMemberQueryOption},
    },
    models::{SpaceCommon, feed::Post, team::Team, user::User},
    types::{EntityType, Partition, TeamGroupPermission, UserType, author::Author},
    utils::aws::{DynamoClient, SesClient},
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
    pub files: Vec<File>,
    pub started_at: i64,
    pub ended_at: i64,
}

#[derive(Debug, Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreateSpacePostResponse {
    pub space_post_pk: Partition,
}

pub async fn create_space_post_handler(
    State(AppState { dynamo, ses, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<CreateSpacePostRequest>,
) -> Result<Json<CreateSpacePostResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceEdit) {
        return Err(Error::NoPermission);
    }

    let common = SpaceCommon::get(&dynamo.client, &space_pk, Some(&EntityType::SpaceCommon))
        .await?
        .ok_or(Error::NotFoundSpace)?;

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
        space_pk.clone(),
        req.title.clone(),
        req.html_contents.clone(),
        req.category_name.clone(),
        req.urls.clone(),
        Some(req.files.clone()),
        req.started_at,
        req.ended_at,
        user.clone(),
    );
    post.create(&dynamo.client).await?;

    // Link files to both Files tab and Board
    let post_id = match &post.sk {
        EntityType::SpacePost(v) => v.to_string(),
        _ => "".to_string(),
    };

    tracing::info!("Linking {} files for post {}", req.files.len(), post_id);

    // Add files to SpaceFile entity (for Files tab)
    if !req.files.is_empty() {
        SpaceFile::add_files(&dynamo.client, space_pk.clone(), req.files.clone()).await?;
    }

    // Link files: Batch add both Files and Board targets to all file URLs
    let file_urls: Vec<String> = req.files.iter().filter_map(|f| f.url.clone()).collect();
    if !file_urls.is_empty() {
        FileLink::add_link_targets_batch(
            &dynamo.client,
            space_pk.clone(),
            file_urls.clone(),
            vec![
                FileLinkTarget::Files,
                FileLinkTarget::Board(post_id.clone()),
            ],
        )
        .await
        .ok();
    }

    let _ =
        send_create_post_alerm(&dynamo, &ses, &common, req.title, req.html_contents, user).await?;

    Ok(Json(CreateSpacePostResponse {
        space_post_pk: Partition::SpacePost(post_id),
    }))
}

async fn send_create_post_alerm(
    dynamo: &DynamoClient,
    ses: &SesClient,
    space: &SpaceCommon,
    title: String,
    html_contents: String,
    user: User,
) -> Result<Json<()>, Error> {
    let mut bookmark = None::<String>;
    let mut emails: Vec<String> = Vec::new();
    let mut user_pks: Vec<Partition> = Vec::new();

    loop {
        let (responses, new_bookmark) = SpaceInvitationMember::query(
            &dynamo.client,
            space.pk.clone(),
            if let Some(b) = &bookmark {
                SpaceInvitationMemberQueryOption::builder()
                    .sk("SPACE_INVITATION_MEMBER#".into())
                    .bookmark(b.clone())
            } else {
                SpaceInvitationMemberQueryOption::builder().sk("SPACE_INVITATION_MEMBER#".into())
            },
        )
        .await?;

        for response in responses {
            emails.push(response.email);
            user_pks.push(response.user_pk);
        }

        match new_bookmark {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    if emails.is_empty() {
        return Ok(Json(()));
    }

    let _ = SpacePost::send_email(
        dynamo,
        ses,
        emails,
        space.clone(),
        title.clone(),
        html_contents,
        user,
    )
    .await?;

    // FIXME: fix to one call code
    let mut fcm = FCMService::new().await?;
    let _ = SpacePost::send_notification(&dynamo, &mut fcm, space, title, user_pks).await?;

    Ok(Json(()))
}
