use std::collections::HashSet;

use crate::common::models::auth::UserFollow;
use crate::features::timeline::*;
use crate::features::timeline::models::{TimelineEntry, TimelineReason};

/// Fan out a published post to all relevant timelines.
///
/// Strategies applied:
/// 1. **Follower fan-out** — delivers to all followers of the author + the author themselves
/// 2. **Team member fan-out** — if the author is a Team, delivers to all team members
/// 3. Deduplicates across strategies so each user gets at most one entry
pub async fn fan_out_timeline_entries(
    cli: &aws_sdk_dynamodb::Client,
    post_pk: &Partition,
    author_pk: &Partition,
    created_at: i64,
) -> Result<()> {
    let mut seen = HashSet::new();
    let mut entries: Vec<(Partition, TimelineReason)> = Vec::new();

    // 1. Author themselves
    seen.insert(author_pk.to_string());
    entries.push((author_pk.clone(), TimelineReason::Following));

    // 2. All followers of the author
    collect_followers(cli, author_pk, &mut seen, &mut entries).await?;

    // 3. If the author is a Team, all team members
    if matches!(author_pk, Partition::Team(_)) {
        collect_team_members(cli, author_pk, &mut seen, &mut entries).await?;
    }

    tracing::info!(
        "fan_out_timeline_entries: writing {} timeline entries for post {}",
        entries.len(),
        post_pk
    );

    write_timeline_entries(cli, post_pk, author_pk, created_at, &entries).await
}

/// Fan out a popular post to a broader audience.
///
/// Called asynchronously when a post crosses popularity thresholds (likes, comments).
/// Delivers to followers-of-followers (2nd degree) who don't already have the entry.
pub async fn fan_out_popular_post(
    cli: &aws_sdk_dynamodb::Client,
    post_pk: &Partition,
    author_pk: &Partition,
    created_at: i64,
) -> Result<()> {
    let mut seen = HashSet::new();
    let mut entries: Vec<(Partition, TimelineReason)> = Vec::new();

    // Collect direct followers first (to mark as seen — they already have it)
    seen.insert(author_pk.to_string());
    collect_follower_pks(cli, author_pk, &mut seen).await?;

    // For each direct follower, collect their followers (2nd-degree)
    let direct_followers = seen.clone();
    for follower_pk_str in &direct_followers {
        if follower_pk_str == &author_pk.to_string() {
            continue;
        }
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

// ── Internal helpers ─────────────────────────────────────────────────

/// Collect all follower user_pks of `target_pk`, adding them to `entries` with reason Following.
async fn collect_followers(
    cli: &aws_sdk_dynamodb::Client,
    target_pk: &Partition,
    seen: &mut HashSet<String>,
    entries: &mut Vec<(Partition, TimelineReason)>,
) -> Result<()> {
    collect_followers_with_reason(cli, target_pk, seen, entries, TimelineReason::Following).await
}

/// Collect all follower user_pks of `target_pk`, adding them to `entries` with a custom reason.
async fn collect_followers_with_reason(
    cli: &aws_sdk_dynamodb::Client,
    target_pk: &Partition,
    seen: &mut HashSet<String>,
    entries: &mut Vec<(Partition, TimelineReason)>,
    reason: TimelineReason,
) -> Result<()> {
    let mut bookmark: Option<String> = None;
    loop {
        let mut opt = UserFollow::opt()
            .sk(EntityType::Follower(String::default()).to_string())
            .limit(100);
        if let Some(bk) = bookmark {
            opt = opt.bookmark(bk);
        }

        let (follows, next_bookmark) = UserFollow::query(cli, target_pk.clone(), opt).await?;
        for follow in follows {
            let key = follow.user_pk.to_string();
            if seen.insert(key) {
                entries.push((follow.user_pk, reason.clone()));
            }
        }

        bookmark = next_bookmark;
        if bookmark.is_none() {
            break;
        }
    }
    Ok(())
}

/// Collect follower PKs into `seen` without creating entries (for dedup purposes).
async fn collect_follower_pks(
    cli: &aws_sdk_dynamodb::Client,
    target_pk: &Partition,
    seen: &mut HashSet<String>,
) -> Result<()> {
    let mut bookmark: Option<String> = None;
    loop {
        let mut opt = UserFollow::opt()
            .sk(EntityType::Follower(String::default()).to_string())
            .limit(100);
        if let Some(bk) = bookmark {
            opt = opt.bookmark(bk);
        }

        let (follows, next_bookmark) = UserFollow::query(cli, target_pk.clone(), opt).await?;
        for follow in follows {
            seen.insert(follow.user_pk.to_string());
        }

        bookmark = next_bookmark;
        if bookmark.is_none() {
            break;
        }
    }
    Ok(())
}

/// Collect all team member user_pks via UserTeamGroup GSI2.
async fn collect_team_members(
    cli: &aws_sdk_dynamodb::Client,
    team_pk: &Partition,
    seen: &mut HashSet<String>,
    entries: &mut Vec<(Partition, TimelineReason)>,
) -> Result<()> {
    let mut bookmark: Option<String> = None;
    loop {
        let mut opt = crate::features::auth::UserTeamGroup::opt().limit(100);
        if let Some(bk) = bookmark {
            opt = opt.bookmark(bk);
        }

        let (members, next_bookmark) =
            crate::features::auth::UserTeamGroup::find_by_team_pk(cli, team_pk.clone(), opt)
                .await?;

        for member in members {
            let key = member.pk.to_string();
            if seen.insert(key) {
                entries.push((member.pk, TimelineReason::TeamMember));
            }
        }

        bookmark = next_bookmark;
        if bookmark.is_none() {
            break;
        }
    }
    Ok(())
}

/// Write timeline entries in transact_write_items batches of 25.
async fn write_timeline_entries(
    cli: &aws_sdk_dynamodb::Client,
    post_pk: &Partition,
    author_pk: &Partition,
    created_at: i64,
    entries: &[(Partition, TimelineReason)],
) -> Result<()> {
    for chunk in entries.chunks(25) {
        let transacts: Vec<aws_sdk_dynamodb::types::TransactWriteItem> = chunk
            .iter()
            .map(|(user_pk, reason)| {
                let entry =
                    TimelineEntry::new(user_pk, post_pk, author_pk, created_at, reason.clone());
                entry.create_transact_write_item()
            })
            .collect();

        cli.transact_write_items()
            .set_transact_items(Some(transacts))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to write timeline entries: {}", e);
                Error::InternalServerError("Failed to write timeline entries".into())
            })?;
    }

    Ok(())
}
