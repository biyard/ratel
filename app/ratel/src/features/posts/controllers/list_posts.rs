use crate::features::auth::OptionalUser;
use crate::features::posts::controllers::dto::*;
use crate::features::posts::models::*;
use crate::features::posts::types::*;
use crate::features::posts::*;

#[cfg(feature = "server")]
use crate::features::timeline::models::TimelineEntry;
#[cfg(feature = "server")]
use base64::Engine;

#[cfg(feature = "server")]
fn serializable_post_error(context: &str, err: Error) -> Error {
    match err {
        Error::Aws(ref inner) => {
            crate::error!("{context}: {inner}");
            PostError::ListFailed.into()
        }
        Error::Session(ref inner) => {
            crate::error!("{context}: {inner}");
            PostError::ListFailed.into()
        }
        other => other,
    }
}

#[cfg(feature = "server")]
const TIMELINE_BOOKMARK_PREFIX: &str = "timeline:";
#[cfg(feature = "server")]
const GLOBAL_BOOKMARK_PREFIX: &str = "global:";

#[cfg(feature = "server")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PostFeedSource {
    Timeline,
    Global,
}

#[cfg(feature = "server")]
fn parse_post_feed_bookmark(bookmark: Option<String>) -> (PostFeedSource, Option<String>) {
    match bookmark {
        Some(bookmark) if bookmark.starts_with(TIMELINE_BOOKMARK_PREFIX) => (
            PostFeedSource::Timeline,
            Some(bookmark[TIMELINE_BOOKMARK_PREFIX.len()..].to_string()),
        ),
        Some(bookmark) if bookmark.starts_with(GLOBAL_BOOKMARK_PREFIX) => (
            PostFeedSource::Global,
            Some(bookmark[GLOBAL_BOOKMARK_PREFIX.len()..].to_string()),
        ),
        Some(bookmark) => {
            let inferred_source = base64::engine::general_purpose::URL_SAFE_NO_PAD
                .decode(&bookmark)
                .ok()
                .and_then(|decoded| String::from_utf8(decoded).ok())
                .map(|decoded| {
                    if decoded.contains("gsi6_pk") {
                        PostFeedSource::Global
                    } else {
                        PostFeedSource::Timeline
                    }
                })
                .unwrap_or(PostFeedSource::Timeline);

            (inferred_source, Some(bookmark))
        }
        None => (PostFeedSource::Timeline, None),
    }
}

#[cfg(feature = "server")]
fn format_post_feed_bookmark(source: PostFeedSource, bookmark: Option<String>) -> Option<String> {
    bookmark.map(|bookmark| match source {
        PostFeedSource::Timeline => format!("{TIMELINE_BOOKMARK_PREFIX}{bookmark}"),
        PostFeedSource::Global => format!("{GLOBAL_BOOKMARK_PREFIX}{bookmark}"),
    })
}

#[mcp_tool(name = "list_posts", description = "List posts from the feed.")]
#[get("/api/posts?bookmark", user: OptionalUser)]
pub async fn list_posts_handler(
    bookmark: Option<String>,
) -> Result<ListItemsResponse<PostResponse>> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    let user: Option<crate::features::auth::User> = user.into();

    tracing::debug!(
        "list_posts_handler: user = {:?} bookmark = {:?}",
        user,
        bookmark
    );

    let (bookmark_source, bookmark) = parse_post_feed_bookmark(bookmark);

    let (feed_source, posts, bookmark) = if let Some(ref user) = user {
        match bookmark_source {
            PostFeedSource::Timeline => fetch_timeline_posts(cli, user, bookmark)
                .await
                .map_err(|err| serializable_post_error("Failed to load personalized feed", err))?,
            PostFeedSource::Global => {
                let (posts, bookmark) = fetch_global_posts(cli, bookmark)
                    .await
                    .map_err(|err| serializable_post_error("Failed to load public feed", err))?;
                (PostFeedSource::Global, posts, bookmark)
            }
        }
    } else {
        let (posts, bookmark) = fetch_global_posts(cli, bookmark)
            .await
            .map_err(|err| serializable_post_error("Failed to load public feed", err))?;
        (PostFeedSource::Global, posts, bookmark)
    };

    let likes = match (&user, posts.is_empty()) {
        (Some(user), false) => {
            let mut seen_likes = std::collections::HashSet::new();
            let like_keys: Vec<_> = posts
                .iter()
                .filter(|post| seen_likes.insert(post.pk.to_string()))
                .map(|post| PostLike::keys(&post.pk, &user.pk))
                .collect();
            PostLike::batch_get(cli, like_keys)
                .await
                .map_err(|err| serializable_post_error("Failed to load post likes", err))?
        }
        _ => vec![],
    };

    tracing::debug!("list_posts_handler: returning {} items", posts.len());
    let items: Vec<PostResponse> = posts
        .into_iter()
        .map(|post| {
            let post_like_pk = post
                .pk
                .clone()
                .to_post_like_key()
                .expect("to_post_like_key");
            let liked = likes.iter().any(|like| like.pk == post_like_pk);
            PostResponse::from(post).with_like(liked)
        })
        .collect();

    Ok(ListItemsResponse {
        items,
        bookmark: format_post_feed_bookmark(feed_source, bookmark),
    })
}

/// Fetch posts from the user's personalized timeline.
/// Falls back to global feed if the timeline is empty.
#[cfg(feature = "server")]
async fn fetch_timeline_posts(
    cli: &aws_sdk_dynamodb::Client,
    user: &crate::features::auth::User,
    bookmark: Option<String>,
) -> Result<(PostFeedSource, Vec<Post>, Option<String>)> {
    let user_id = match &user.pk {
        Partition::User(id) => id.clone(),
        _ => {
            let (posts, bookmark) = fetch_global_posts(cli, bookmark).await?;
            return Ok((PostFeedSource::Global, posts, bookmark));
        }
    };

    let opt = TimelineEntry::opt_with_bookmark(bookmark)
        .sk(EntityType::TimelineEntry(String::default()).to_string())
        .limit(10)
        .scan_index_forward(false);

    let (entries, next_bookmark) =
        TimelineEntry::query(cli, Partition::Timeline(user_id), opt).await?;

    if entries.is_empty() && next_bookmark.is_none() {
        // No timeline entries yet — fall back to global feed
        let (posts, bookmark) = fetch_global_posts(cli, None).await?;
        return Ok((PostFeedSource::Global, posts, bookmark));
    }

    if entries.is_empty() {
        return Ok((PostFeedSource::Timeline, vec![], next_bookmark));
    }

    // Batch get the actual posts (deduplicate keys for BatchGetItem)
    let mut seen = std::collections::HashSet::new();
    let post_keys: Vec<(Partition, EntityType)> = entries
        .iter()
        .filter(|entry| seen.insert(entry.post_pk.to_string()))
        .map(|entry| (entry.post_pk.clone(), EntityType::Post))
        .collect();

    let posts = Post::batch_get(cli, post_keys).await?;

    // Maintain timeline ordering by mapping posts by pk
    let post_map: std::collections::HashMap<String, Post> =
        posts.into_iter().map(|p| (p.pk.to_string(), p)).collect();

    let mut seen_posts = std::collections::HashSet::new();
    let ordered_posts: Vec<Post> = entries
        .iter()
        .filter(|entry| seen_posts.insert(entry.post_pk.to_string()))
        .filter_map(|entry| post_map.get(&entry.post_pk.to_string()).cloned())
        .collect();

    Ok((PostFeedSource::Timeline, ordered_posts, next_bookmark))
}

/// Fetch posts from the global public feed (GSI6).
#[cfg(feature = "server")]
async fn fetch_global_posts(
    cli: &aws_sdk_dynamodb::Client,
    bookmark: Option<String>,
) -> Result<(Vec<Post>, Option<String>)> {
    let mut query_options = Post::opt().limit(10);

    if let Some(bk) = bookmark {
        query_options = query_options.bookmark(bk);
    }

    Post::find_by_visibility(
        cli,
        format!("{}#{}", Visibility::Public, PostStatus::Published),
        query_options,
    )
    .await
}
