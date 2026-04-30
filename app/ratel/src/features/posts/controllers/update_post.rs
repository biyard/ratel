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
    Author {
        team_id: Option<TeamPartition>,
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
        return Err(PostError::NotAccessible.into());
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
        UpdatePostRequest::Author { team_id } => {
            let author: Author = if let Some(team_id) = team_id {
                let team_pk: Partition = team_id.into();
                Team::get_permitted_team(
                    cli,
                    team_pk,
                    user.pk.clone(),
                    TeamGroupPermission::PostWrite,
                )
                .await?
                .into()
            } else {
                let user_pk = user.pk.clone();
                let full_user = crate::features::auth::User::get(
                    cli,
                    user_pk,
                    Some(EntityType::User),
                )
                .await?
                .ok_or::<Error>(PostError::NotAccessible.into())?;
                full_user.into()
            };

            post.user_pk = author.pk.clone();
            post.author_display_name = author.display_name.clone();
            post.author_profile_url = author.profile_url.clone();
            post.author_username = author.username.clone();
            post.author_type = author.user_type.clone();

            vec![
                updater
                    .with_user_pk(author.pk)
                    .with_author_display_name(author.display_name)
                    .with_author_profile_url(author.profile_url)
                    .with_author_username(author.username)
                    .with_author_type(author.user_type)
                    .transact_write_item(),
            ]
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

    // Essence re-indexing happens via the DynamoDB Stream pipeline.

    Ok(post)
}
