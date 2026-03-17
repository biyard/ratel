use crate::features::posts::controllers::dto::PostResponse;
use crate::features::posts::models::*;
use crate::features::timeline::controllers::dto::*;
use crate::features::timeline::models::*;
use crate::features::timeline::*;

/// Query a single timeline category.
///
/// `GET /api/timeline?category=following&bookmark=...`
#[get("/api/timeline?category&bookmark", user: crate::features::auth::User)]
pub async fn list_timeline_handler(
    category: String,
    bookmark: Option<String>,
) -> Result<TimelineCategoryRow> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    let reason: TimelineReason = category
        .parse()
        .map_err(|e: String| Error::BadRequest(e))?;

    let user_id = match &user.pk {
        Partition::User(id) => id.clone(),
        _ => {
            return Err(Error::BadRequest("Invalid user".into()));
        }
    };

    let category_key = format!("{}#{}", user_id, reason);

    fetch_category_row(cli, &category_key, &reason, &user, bookmark, 10).await
}

/// Query all timeline categories with a preview of posts for each.
///
/// `GET /api/timeline/feed?preview_count=...`
///
/// Returns a Netflix-style layout with multiple rows, each containing
/// a preview of posts from a different category.
#[get("/api/timeline/feed?preview_count", user: crate::features::auth::User)]
pub async fn list_timeline_feed_handler(
    preview_count: Option<i32>,
) -> Result<TimelineFeedResponse> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    let user_id = match &user.pk {
        Partition::User(id) => id.clone(),
        _ => {
            return Err(Error::BadRequest("Invalid user".into()));
        }
    };

    let limit = preview_count.unwrap_or(5).min(20);
    let mut categories = Vec::new();

    for reason in TIMELINE_CATEGORIES {
        let category_key = format!("{}#{}", user_id, reason);
        match fetch_category_row(cli, &category_key, reason, &user, None, limit).await {
            Ok(row) if !row.items.is_empty() => {
                categories.push(row);
            }
            Ok(_) => {} // empty category, skip
            Err(e) => {
                tracing::warn!("Failed to fetch category {}: {}", reason, e);
            }
        }
    }

    Ok(TimelineFeedResponse { categories })
}

#[cfg(feature = "server")]
async fn fetch_category_row(
    cli: &aws_sdk_dynamodb::Client,
    category_key: &str,
    reason: &TimelineReason,
    user: &crate::features::auth::User,
    bookmark: Option<String>,
    limit: i32,
) -> Result<TimelineCategoryRow> {
    let mut opt = TimelineEntry::opt()
        .limit(limit)
        .scan_index_forward(false);

    if let Some(bk) = bookmark {
        opt = opt.bookmark(bk);
    }

    let (entries, next_bookmark) =
        TimelineEntry::find_by_category(cli, category_key.to_string(), opt).await?;

    if entries.is_empty() {
        return Ok(TimelineCategoryRow {
            category: reason.to_string(),
            items: vec![],
            bookmark: next_bookmark.clone(),
            has_more: next_bookmark.is_some(),
        });
    }

    // Batch get the actual posts
    let post_keys: Vec<(Partition, EntityType)> = entries
        .iter()
        .map(|entry| (entry.post_pk.clone(), EntityType::Post))
        .collect();

    let posts = Post::batch_get(cli, post_keys).await?;

    // Maintain timeline ordering
    let post_map: std::collections::HashMap<String, Post> = posts
        .into_iter()
        .map(|p| (p.pk.to_string(), p))
        .collect();

    // Get like status for the user
    let ordered_posts: Vec<Post> = entries
        .iter()
        .filter_map(|entry| post_map.get(&entry.post_pk.to_string()).cloned())
        .collect();

    let likes = if !ordered_posts.is_empty() {
        PostLike::batch_get(
            cli,
            ordered_posts
                .iter()
                .map(|post| PostLike::keys(&post.pk, &user.pk))
                .collect(),
        )
        .await
        .unwrap_or_default()
    } else {
        vec![]
    };

    let items: Vec<PostResponse> = ordered_posts
        .into_iter()
        .map(|post| {
            let post_like_pk = post
                .pk
                .clone()
                .to_post_like_key()
                .expect("to_post_like_key");
            let liked = likes.iter().any(|like| like.pk == post_like_pk);
            PostResponse::from((Some(user.clone()), post)).with_like(liked)
        })
        .collect();

    Ok(TimelineCategoryRow {
        category: reason.to_string(),
        items,
        bookmark: next_bookmark.clone(),
        has_more: next_bookmark.is_some(),
    })
}
