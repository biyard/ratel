use crate::features::auth::User;
use crate::features::posts::models::*;
use crate::features::posts::types::*;
use crate::features::posts::*;
use std::collections::HashMap;

use crate::features::cross_posting::models::PostSyndicationDirective;
use crate::features::cross_posting::types::SocialPlatform;

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
        /// Per-post platform selection (cross-posting). Phase 1: defaults
        /// to all connected + auto_post_enabled when omitted; an explicit
        /// empty Vec means "Ratel-only" (FR-4 #27). Stage 1 factory only
        /// runs when this resolves to non-empty AND visibility = Public.
        #[serde(default)]
        enabled_platforms: Option<Vec<SocialPlatform>>,
        /// Per-platform body overrides (cross-posting v1.5 readiness). Phase
        /// 1: always empty — UI does not expose. The field is accepted now
        /// so v1.5 client/server are wire-compatible without a DTO change.
        #[serde(default)]
        platform_overrides: Option<HashMap<SocialPlatform, String>>,
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
            enabled_platforms,
            platform_overrides,
        } => {
            validate_title(&title)?;
            validate_content(&content)?;

            tracing::debug!(
                "Publish request: publish = {}, title = {}, content = [REDACTED]",
                publish,
                title
            );
            let visibility = visibility.unwrap_or_default();
            let is_public_publish = publish && matches!(visibility, Visibility::Public);

            post.visibility = Some(visibility.clone());

            let status = if publish {
                PostStatus::Published
            } else {
                PostStatus::Draft
            };
            post.status = status;
            post.title = title.clone();
            post.html_contents = content.clone();
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
            let mut transacts = if let Some(image_urls) = image_urls {
                post.urls = image_urls.clone();
                vec![updater.with_urls(image_urls).transact_write_item()]
            } else {
                vec![updater.transact_write_item()]
            };

            // Cross-posting Stage 1 factory hook (design doc → "Atomic
            // write — reuse existing infrastructure"). Append a
            // PostSyndicationDirective to the SAME transact batch so
            // either both Post + directive land or neither does. Stage 1
            // Lambda triggers on the Post Draft→Published MODIFY and reads
            // this directive to bake SyndicationJob rows; absence of the
            // directive is the kill switch (no syndication).
            if is_public_publish && enabled_platforms.as_ref().is_some_and(|v| !v.is_empty()) {
                let directive = PostSyndicationDirective {
                    pk: post.pk.clone(),
                    sk: EntityType::SyndicationDirective,
                    enabled_platforms: enabled_platforms.unwrap_or_default(),
                    platform_overrides: platform_overrides.unwrap_or_default(),
                    author_user_id: user.pk.clone(),
                    created_at: now,
                };
                transacts.push(directive.create_transact_write_item());
            }

            transacts
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
