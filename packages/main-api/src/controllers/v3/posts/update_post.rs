use crate::models::feed::Post;
use crate::models::user::User;
use crate::types::sorted_visibility::SortedVisibility;
use crate::types::{Partition, PostStatus, TeamGroupPermission, Visibility};
use crate::utils::validator::{validate_content, validate_title};
use crate::{AppState, Error2};
use aide::NoApi;
use axum::extract::{Json, Path, State};
use bdk::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct UpdatePostPathParams {
    pub post_pk: Partition,
}

#[derive(Debug, Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
#[serde(untagged)]
pub enum UpdatePostRequest {
    Writing { title: String, content: String },
    Image { images: Vec<String> },
    Info { visibility: Visibility },
    Publish,
    // TODO: Artwork metadata
}

// #[derive(Debug, Serialize, Default, aide::OperationIo, JsonSchema)]
// pub struct UpdatePostResponse {}
pub type UpdatePostResponse = Post;

pub async fn update_post_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(UpdatePostPathParams { post_pk }): Path<UpdatePostPathParams>,
    Json(req): Json<UpdatePostRequest>,
) -> Result<Json<UpdatePostResponse>, Error2> {
    let cli = &dynamo.client;
    let (mut post, has_permission) =
        Post::has_permission(cli, &post_pk, Some(&user.pk), TeamGroupPermission::PostEdit).await?;
    if !has_permission {
        return Err(Error2::NoPermission);
    }

    let now = chrono::Utc::now().timestamp_micros();
    let updater = Post::updater(&post.pk, &post.sk).with_updated_at(now);
    post.updated_at = now;

    let req = match req {
        UpdatePostRequest::Writing { title, content } => {
            validate_title(&title)?;
            validate_content(&content)?;

            post.title = title.clone();
            post.html_contents = content.clone();

            updater.with_title(title).with_html_contents(content)
        }
        UpdatePostRequest::Image { images } => {
            post.urls = images.clone();
            updater.with_urls(images)
        }
        UpdatePostRequest::Info { visibility } => {
            let sorted_visibility = match visibility {
                Visibility::TeamOnly(..) => {
                    SortedVisibility::team_only(post.user_pk.clone(), post.created_at)?
                }
                Visibility::Public => SortedVisibility::public(post.created_at),
            };

            post.visibility = Some(visibility.clone());
            post.sorted_visibility = sorted_visibility.clone();

            updater
                .with_visibility(visibility)
                .with_sorted_visibility(sorted_visibility)
        }
        UpdatePostRequest::Publish => {
            post.status = PostStatus::Published;

            updater.with_status(PostStatus::Published)
        }
    };

    req.execute(cli).await?;

    Ok(Json(post))
}
