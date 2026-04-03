use std::collections::HashSet;

use crate::features::posts::models::Post;
use crate::features::timeline::models::TimelineReason;
use crate::features::timeline::*;

use super::common::{collect_follower_pks, collect_followers_with_reason, write_timeline_entries};

/// Fan out a popular post to a broader audience.
///
/// Called asynchronously when a post crosses popularity thresholds (likes, comments).
/// Delivers to followers-of-followers (2nd degree) who don't already have the entry.
pub async fn fan_out_popular_post(post: Post) -> Result<()> {
    if !is_popular(post.likes, post.comments, post.shares)
        || !post.is_published()
        || !post.is_public()
    {
        return Ok(());
    }

    tracing::info!(
        "Popular post fan-out: post_pk={}, likes={}, comments={}, shares={}",
        post.pk,
        post.likes,
        post.comments,
        post.shares
    );

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let post_pk = &post.pk;
    let author_pk = &post.user_pk;
    let created_at = post.created_at;
    let mut seen = HashSet::new();
    let mut entries: Vec<(Partition, TimelineReason)> = Vec::new();

    // Collect direct followers first (to mark as seen -- they already have it)
    let author_pk_str = author_pk.to_string();
    seen.insert(author_pk_str.clone());
    collect_follower_pks(cli, author_pk, &mut seen).await?;

    // For each direct follower, collect their followers (2nd-degree)
    // Build a stable list of direct follower PKs to iterate over, avoiding a full HashSet clone.
    let direct_followers: Vec<String> = seen
        .iter()
        .filter(|pk| *pk != &author_pk_str)
        .cloned()
        .collect();
    for follower_pk_str in &direct_followers {
        let follower_pk: Partition = match follower_pk_str.parse() {
            Ok(pk) => pk,
            Err(_) => continue,
        };
        collect_followers_with_reason(
            cli,
            &follower_pk,
            &mut seen,
            &mut entries,
            TimelineReason::Popular,
        )
        .await?;
    }

    if entries.is_empty() {
        return Ok(());
    }

    tracing::info!(
        "fan_out_popular_post: writing {} timeline entries for popular post {}",
        entries.len(),
        post_pk
    );

    write_timeline_entries(cli, post_pk, author_pk, created_at, &entries).await
}

/// Returns true if a post meets the popularity threshold for broader distribution.
pub fn is_popular(likes: i64, comments: i64, shares: i64) -> bool {
    // A post is considered popular if it meets any of these criteria:
    // - 10+ likes
    // - 5+ comments
    // - 3+ shares
    // - Combined engagement score >= 15
    likes >= 10 || comments >= 5 || shares >= 3 || (likes + comments * 2 + shares * 3) >= 15
}
