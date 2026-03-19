use crate::features::auth::OptionalUser;
use crate::features::posts::controllers::dto::*;
use crate::features::posts::models::*;
use crate::features::posts::types::*;
use crate::features::posts::*;

#[cfg(feature = "server")]
use crate::features::timeline::models::TimelineEntry;

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

    let (posts, bookmark) = if let Some(ref user) = user {
        fetch_timeline_posts(cli, user, bookmark).await?
    } else {
        fetch_global_posts(cli, bookmark).await?
    };

    let likes = match (&user, posts.is_empty()) {
        (Some(user), false) => {
            let mut seen_likes = std::collections::HashSet::new();
            let like_keys: Vec<_> = posts
                .iter()
                .filter(|post| seen_likes.insert(post.pk.to_string()))
                .map(|post| PostLike::keys(&post.pk, &user.pk))
                .collect();
            PostLike::batch_get(cli, like_keys).await?
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
            PostResponse::from((user.clone(), post)).with_like(liked)
        })
        .collect();

    Ok(ListItemsResponse { items, bookmark })
}

/// Fetch posts from the user's personalized timeline.
/// Falls back to global feed if the timeline is empty.
#[cfg(feature = "server")]
async fn fetch_timeline_posts(
    cli: &aws_sdk_dynamodb::Client,
    user: &crate::features::auth::User,
    bookmark: Option<String>,
) -> Result<(Vec<Post>, Option<String>)> {
    let user_id = match &user.pk {
        Partition::User(id) => id.clone(),
        _ => return fetch_global_posts(cli, bookmark).await,
    };

    let opt = TimelineEntry::opt_with_bookmark(bookmark)
        .sk(EntityType::TimelineEntry(String::default()).to_string())
        .limit(10)
        .scan_index_forward(false);

    let (entries, next_bookmark) =
        TimelineEntry::query(cli, Partition::Timeline(user_id), opt).await?;

    if entries.is_empty() && next_bookmark.is_none() {
        // No timeline entries yet — fall back to global feed
        return fetch_global_posts(cli, None).await;
    }

    if entries.is_empty() {
        return Ok((vec![], next_bookmark));
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

    Ok((ordered_posts, next_bookmark))
}

/// Fetch posts from the global public feed (GSI6).
///
/// Space posts are only included when they meet the popularity threshold
/// or when the associated space has 5+ participants.
#[cfg(feature = "server")]
async fn fetch_global_posts(
    cli: &aws_sdk_dynamodb::Client,
    bookmark: Option<String>,
) -> Result<(Vec<Post>, Option<String>)> {
    let mut query_options = Post::opt().limit(10);

    if let Some(bk) = bookmark {
        query_options = query_options.bookmark(bk);
    }

    let (posts, bookmark) = Post::find_by_visibility(
        cli,
        format!("{}#{}", Visibility::Public, PostStatus::Published),
        query_options,
    )
    .await?;

    // Batch-fetch SpaceCommon for all space posts to check participant count.
    let mut seen_space_pks = std::collections::HashSet::new();
    let space_keys: Vec<(Partition, EntityType)> = posts
        .iter()
        .filter_map(|p| p.space_pk.clone())
        .filter(|pk| seen_space_pks.insert(pk.to_string()))
        .map(|pk| (pk, EntityType::SpaceCommon))
        .collect();

    let space_participants: std::collections::HashMap<String, i64> =
        if space_keys.is_empty() {
            std::collections::HashMap::new()
        } else {
            crate::common::models::space::SpaceCommon::batch_get(cli, space_keys)
                .await
                .unwrap_or_default()
                .into_iter()
                .map(|s| (s.pk.to_string(), s.participants))
                .collect()
        };

    let posts = posts
        .into_iter()
        .filter(|post| {
            let Some(ref space_pk) = post.space_pk else {
                return true;
            };
            if crate::features::timeline::services::is_popular(
                post.likes,
                post.comments,
                post.shares,
            ) {
                return true;
            }
            let participants = space_participants
                .get(&space_pk.to_string())
                .copied()
                .unwrap_or(0);
            participants >= 5
        })
        .collect();

    Ok((posts, bookmark))
}
