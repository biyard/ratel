use crate::features::auth::User;
use crate::features::posts::models::*;
use crate::features::posts::types::*;
use crate::features::posts::*;

#[cfg(feature = "server")]
use crate::features::posts::utils::validator::{validate_content, validate_title};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(untagged)]
pub enum UpdatePostRequest {
    Publish {
        title: String,
        content: String,
        image_urls: Option<Vec<String>>,
        publish: bool,
        visibility: Option<Visibility>,
        categories: Option<Vec<String>>,
    },
    PostType {
        r#type: PostType,
    },
    Writing {
        title: String,
        content: String,
        categories: Option<Vec<String>>,
    },
    Image {
        images: Vec<String>,
    },
    Info {
        visibility: Visibility,
    },
    ArtworkMetadata {
        metadata: Vec<PostArtworkMetadata>,
    },
}

#[mcp_tool(
    name = "update_post",
    description = "Update a post (publish, edit content, change visibility)."
)]
#[put("/api/posts/:post_id", user: User)]
pub async fn update_post_handler(post_id: FeedPartition, req: UpdatePostRequest) -> Result<Post> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    let post_pk: Partition = post_id.into();

    tracing::debug!(
        "update_post_handler: user = {:?}, post_pk = {:?}, req = {:?}",
        user,
        post_pk,
        req
    );
    let (mut post, has_permission) =
        Post::has_permission(cli, &post_pk, Some(&user.pk), TeamGroupPermission::PostEdit).await?;
    if !has_permission {
        return Err(Error::Unauthorized("No permission".into()));
    }

    let now = chrono::Utc::now().timestamp_millis();

    let updater = Post::updater(&post.pk, &post.sk).with_updated_at(now);
    post.updated_at = now;

    let transacts = match req {
        UpdatePostRequest::Writing {
            title,
            content,
            categories,
        } => {
            post.title = title.clone();
            post.html_contents = content.clone();
            if let Some(ref cats) = categories {
                post.categories = cats.clone();
            }

            let mut updater = updater.with_title(title).with_html_contents(content);
            if let Some(cats) = categories {
                updater = updater.with_categories(cats);
            }
            vec![updater.transact_write_item()]
        }
        UpdatePostRequest::Image { images } => {
            post.urls = images.clone();
            vec![updater.with_urls(images).transact_write_item()]
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
            categories,
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

            let status = if publish {
                PostStatus::Published
            } else {
                PostStatus::Draft
            };
            post.status = status;
            post.title = title.clone();
            post.html_contents = content.clone();
            post.visibility = Some(visibility.clone());
            if let Some(ref cats) = categories {
                post.categories = cats.clone();
            }
            let mut updater = updater
                .with_status(status)
                .with_title(title)
                .with_html_contents(content)
                .with_visibility(visibility);
            if let Some(cats) = categories {
                updater = updater.with_categories(cats);
            }
            if let Some(image_urls) = image_urls {
                post.urls = image_urls.clone();
                vec![updater.with_urls(image_urls).transact_write_item()]
            } else {
                vec![updater.transact_write_item()]
            }
        }
        UpdatePostRequest::PostType { r#type } => {
            post.post_type = r#type.clone();
            vec![updater.with_post_type(r#type).transact_write_item()]
        }
        UpdatePostRequest::ArtworkMetadata {
            metadata: next_metadata,
        } => {
            let mut transacts = vec![];

            transacts.push(updater.transact_write_item());

            let artwork = PostArtwork::get(cli, &post.pk, Some(EntityType::PostArtwork)).await?;
            tracing::debug!("Existing artwork metadata: {:?}", artwork);
            if let Some(mut artwork) = artwork {
                let artwork_updater =
                    PostArtwork::updater(post.pk.clone(), EntityType::PostArtwork)
                        .with_metadata(next_metadata.clone());
                transacts.push(artwork_updater.transact_write_item());
                artwork.metadata = next_metadata;
            } else {
                let artwork = PostArtwork::new(post.pk.clone(), next_metadata);
                tracing::debug!("Creating new artwork metadata: {:?}", artwork);
                transacts.push(artwork.create_transact_write_item());
            };
            transacts
        }
    };

    crate::transact_write_items!(cli, transacts)?;

    Ok(post)
}
