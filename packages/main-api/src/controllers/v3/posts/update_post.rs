use crate::models::feed::Post;
use crate::models::user::User;
use crate::models::{PostArtwork, PostArtworkMetadata};
use crate::types::{EntityType, PostStatus, PostType, TeamGroupPermission, Visibility};
use crate::utils::time::get_now_timestamp_millis;
use crate::utils::validator::{validate_content, validate_title};
use crate::{AppState, Error, transact_write_items};
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
        image_urls: Option<Vec<String>>,
        publish: bool,
        visibility: Option<Visibility>,
    },
    PostType {
        r#type: PostType,
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
    WritingArtwork {
        title: String,
        content: String,
        metadata: Vec<PostArtworkMetadata>,
    },
}

pub async fn update_post_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(super::dto::PostPathParam { post_pk }): super::dto::PostPath,
    Json(req): Json<UpdatePostRequest>,
) -> Result<Json<Post>, Error> {
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
        return Err(Error::NoPermission);
    }

    let now = get_now_timestamp_millis();
    let updater = Post::updater(&post.pk, &post.sk).with_updated_at(now);
    post.updated_at = now;

    let transacts = match req {
        UpdatePostRequest::Writing { title, content } => {
            post.title = title.clone();
            post.html_contents = content.clone();

            vec![
                updater
                    .with_title(title)
                    .with_html_contents(content)
                    .transact_write_item(),
            ]
        }
        UpdatePostRequest::Image { images } => {
            post.urls = images.clone();
            vec![
                updater
                    .with_urls(images)
                    .with_updated_at(now)
                    .transact_write_item(),
            ]
        }

        UpdatePostRequest::Info { visibility } => {
            post.visibility = Some(visibility.clone());
            vec![updater.with_visibility(visibility).transact_write_item()]
        }
        UpdatePostRequest::Publish {
            publish,
            content,
            title,
            visibility,
            image_urls,
        } => {
            validate_title(&title)?;
            validate_content(&content)?;

            tracing::debug!(
                "Publish request: publish = {}, title = {}, content = [REDACTED]",
                publish,
                title
            );
            let visibility = visibility.unwrap_or_default();

            post.visibility = Some(visibility.clone());

            if !publish {
                // TODO: support unpublish if no dependencies
                return Err(Error::NotSupported(
                    "it does not support unpublished now".into(),
                ));
            }
            validate_title(&title)?;
            validate_content(&content)?;

            let image_urls = image_urls.unwrap_or_default();
            post.urls = image_urls.clone();
            post.status = PostStatus::Published;
            post.title = title.clone();
            post.html_contents = content.clone();
            post.visibility = Some(visibility.clone());
            post.status = PostStatus::Published;
            vec![
                updater
                    .with_status(PostStatus::Published)
                    .with_title(title)
                    .with_html_contents(content)
                    .with_visibility(visibility)
                    .with_urls(image_urls)
                    .transact_write_item(),
            ]
        }
        UpdatePostRequest::PostType { r#type } => {
            post.post_type = r#type.clone();
            vec![updater.with_post_type(r#type).transact_write_item()]
        }
        UpdatePostRequest::WritingArtwork {
            title,
            content,
            metadata,
        } => {
            validate_title(&title)?;
            validate_content(&content)?;

            let mut transacts = vec![];
            post.title = title.clone();
            post.html_contents = content.clone();
            transacts.push(
                updater
                    .with_title(title)
                    .with_html_contents(content)
                    .transact_write_item(),
            );

            let next_metadata =
                PostArtwork::get(cli, &post.pk, Some(EntityType::PostArtwork)).await?;
            let _metadata = if let Some(mut meta) = next_metadata {
                let artwork_updater =
                    PostArtwork::updater(post.pk.clone(), EntityType::PostArtwork)
                        .with_metadata(metadata.clone());
                transacts.push(artwork_updater.transact_write_item());
                meta.metadata = metadata;
                meta
            } else {
                let next_metadata = PostArtwork::new(post.pk.clone(), metadata);
                transacts.push(next_metadata.create_transact_write_item());
                next_metadata
            };
            transacts
        }
    };

    transact_write_items!(cli, transacts)?;

    Ok(Json(post))
}
