use std::collections::HashSet;

use crate::features::timeline::models::TimelineReason;
use crate::features::timeline::*;

use super::common::{collect_follower_pks, collect_followers_with_reason, write_timeline_entries};

/// Fan out a popular space to a broader audience.
///
/// Called asynchronously when a space crosses the popularity threshold (>5 participants).
/// Delivers the space's associated post to followers of the space author and their
/// 2nd-degree network who don't already have the entry.
pub async fn fan_out_popular_space(space: crate::common::models::space::SpaceCommon) -> Result<()> {
    if !is_popular_space(space.participants) || !space.is_public() || !space.is_published() {
        return Ok(());
    }

    tracing::info!(
        "Popular space fan-out: space_pk={}, participants={}",
        space.pk,
        space.participants
    );

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let space_pk = &space.pk;
    let post_pk = &space.post_pk;
    let author_pk = &space.user_pk;
    let created_at = space.created_at;

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
            TimelineReason::PopularSpace,
        )
        .await?;
    }

    if entries.is_empty() {
        return Ok(());
    }

    tracing::info!(
        "fan_out_popular_space: writing {} timeline entries for popular space {} (post {})",
        entries.len(),
        space_pk,
        post_pk
    );

    write_timeline_entries(cli, post_pk, author_pk, created_at, &entries).await
}

/// Returns true if a space meets the popularity threshold for broader distribution.
pub fn is_popular_space(participants: i64) -> bool {
    participants > 5
}
