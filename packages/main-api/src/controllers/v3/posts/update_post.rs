use crate::models::feed::Post;
use crate::models::user::User;
use crate::types::{PostStatus, TeamGroupPermission, Visibility};
use crate::utils::validator::{validate_content, validate_title};
use crate::{AppState, Error2};
use aide::NoApi;
use axum::extract::{Json, Path, State};
use bdk::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
#[serde(untagged)]
pub enum UpdatePostRequest {
    Publish {
        title: String,
        content: String,
        publish: bool,
        visibility: Option<Visibility>,
    },
    Writing {
        title: String,
        content: String,
    },
    Image {
        images: Vec<String>,
    },
    Info {
        visibility: Visibility,
    },
    // TODO: Artwork metadata
}

pub async fn update_post_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(super::dto::PostPathParam { post_pk }): super::dto::PostPath,
    Json(req): Json<UpdatePostRequest>,
) -> Result<Json<Post>, Error2> {
    tracing::debug!(
        "update_post_handler: user = {:?}, post_pk = {:?}, req = {:?}",
        user,
        post_pk,
        req
    );
    let cli = &dynamo.client;
    let (mut post, has_permission) =
        Post::has_permission(cli, &post_pk, Some(&user.pk), TeamGroupPermission::PostEdit).await?;
    if !has_permission {
        return Err(Error2::NoPermission);
    }

    let now = chrono::Utc::now().timestamp();
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
            post.visibility = Some(visibility.clone());
            updater.with_visibility(visibility)
        }
        UpdatePostRequest::Publish {
            publish,
            content,
            title,
            visibility,
        } => {
            tracing::debug!(
                "Publish request: publish = {}, title = {}, content = [REDACTED]",
                publish,
                title
            );
            let visibility = visibility.unwrap_or_default();

            post.visibility = Some(visibility.clone());

            if !publish {
                // TODO: support unpublish if no dependencies
                return Err(Error2::NotSupported(
                    "it does not support unpublished now".into(),
                ));
            }
            let av: aws_sdk_dynamodb::types::AttributeValue =
                serde_dynamo::to_attribute_value(&PostStatus::Published)
                    .expect("failed to serialize field");

            tracing::debug!("Publishing post with AV: {:?}", av);

            post.status = PostStatus::Published;

            updater
                .with_status(PostStatus::Published)
                .with_title(title)
                .with_html_contents(content)
                .with_visibility(visibility)
        }
    };

    req.execute(cli).await?;

    Ok(Json(post))
}
