//! Query-time aggregation for the sub-team activity dashboard (PR #5).
//!
//! Phase 1 uses on-the-fly counts — the 50-sub-team cap bounds the cost and
//! lets us skip a pre-aggregation pipeline (Phase 2). Every aggregation here
//! hard-caps pagination at `MAX_SCAN_PAGES` × `PAGE_SIZE` items per sub-team
//! (≈500) and excludes `Visibility::Private` posts plus non-`Public`
//! spaces to honor the privacy boundary (FR-6 #39 / AC-15).

use std::collections::{HashMap, HashSet};

use crate::common::*;
use crate::common::models::space::SpaceCommon;
use crate::common::types::{SpacePublishState, SpaceVisibility};
use crate::features::auth::{UserTeam, UserTeamQueryOption};
use crate::features::posts::models::{Post, TeamOwner};
use crate::features::posts::types::{PostStatus, Visibility};
use crate::features::sub_team::models::SubTeamLink;
use crate::features::sub_team::types::{
    ActivityCountsResponse, ActivityWindow, MemberActivityResponse, PrivacyNotice, SubTeamError,
    SubTeamSummaryResponse,
};

/// Phase 1 per-parent cap. Beyond this the list endpoint truncates to the
/// most-recently-approved 50 rows and sets `truncated: true` on the
/// envelope.
pub const MAX_SUB_TEAMS: usize = 50;

const PAGE_SIZE: i32 = 100;
/// Bounded pagination — ≤ 500 items per sub-team scanned per metric. If a
/// team authored more than this in a 30-day window, Phase 2 snapshotting
/// will take over.
const MAX_SCAN_PAGES: usize = 5;
/// Used to compute `last_activity_at` in the summary response. A posts
/// query more recent than this is a "stale" team.
const RECENT_WINDOW_MS: i64 = 30 * 86_400 * 1000;

/// Compute (range_start_ms, range_end_ms) for an `ActivityWindow`. End is
/// always "now"; start is now - window duration.
pub fn compute_range(window: ActivityWindow) -> (i64, i64) {
    let end = crate::common::utils::time::get_now_timestamp_millis();
    let start = end - window.duration_ms();
    (start, end)
}

/// List recognized sub-teams under `parent_pk`, up to `MAX_SUB_TEAMS`
/// ordered by `approved_at` DESC. The second tuple element is `true` when
/// the parent has more than the cap — the response envelope exposes this
/// as the `truncated` flag.
pub async fn list_sub_team_links(
    cli: &aws_sdk_dynamodb::Client,
    parent_pk: &Partition,
) -> Result<(Vec<SubTeamLink>, bool)> {
    // Query one beyond the cap so we can detect truncation without a second
    // round-trip.
    let opts = SubTeamLink::opt()
        .sk("SUB_TEAM_LINK".to_string())
        .limit((MAX_SUB_TEAMS as i32) + 1);
    let (mut items, _) = SubTeamLink::query(cli, parent_pk.clone(), opts)
        .await
        .map_err(|e| {
            crate::error!("list_sub_team_links query failed: {e}");
            SubTeamError::ActivityAggregationFailed
        })?;

    // Order newest-approved first so truncation drops the oldest rows.
    items.sort_by(|a, b| b.approved_at.cmp(&a.approved_at));

    let truncated = items.len() > MAX_SUB_TEAMS;
    if truncated {
        items.truncate(MAX_SUB_TEAMS);
    }
    Ok((items, truncated))
}

/// Summary row for the sub-teams list endpoint. Queries the child team's
/// last 30 days of posts to populate `last_activity_at`, and joins member
/// count via UserTeam's find_by_team GSI.
pub async fn build_sub_team_summary(
    cli: &aws_sdk_dynamodb::Client,
    link: &SubTeamLink,
) -> Result<SubTeamSummaryResponse> {
    let child_pk = Partition::Team(link.child_team_id.clone());

    // Load the child team for identity fields.
    let team = crate::features::posts::models::Team::get(cli, &child_pk, Some(EntityType::Team))
        .await
        .map_err(|e| {
            crate::error!("build_sub_team_summary team load failed: {e}");
            SubTeamError::ActivityAggregationFailed
        })?
        .ok_or_else(|| {
            crate::error!("build_sub_team_summary: child team not found {}", link.child_team_id);
            SubTeamError::ActivityAggregationFailed
        })?;

    let member_count = count_team_members(cli, &child_pk).await.unwrap_or(0);

    // last_activity_at = max(updated_at) across public posts in the last 30
    // days. Bounded scan — if nothing matches we return None.
    let cutoff = crate::common::utils::time::get_now_timestamp_millis() - RECENT_WINDOW_MS;
    let last_activity_at = max_post_updated_at(cli, &child_pk, cutoff).await.unwrap_or(None);

    Ok(SubTeamSummaryResponse {
        sub_team_id: link.child_team_id.clone(),
        display_name: team.display_name,
        profile_url: team.profile_url,
        username: team.username,
        recognized_at: link.approved_at,
        member_count,
        last_activity_at,
    })
}

/// Compute aggregated activity counts over the given window. Excludes
/// `Visibility::Private` posts, only `PostStatus::Published` count, and
/// excludes non-`Public` spaces.
pub async fn compute_activity_counts(
    cli: &aws_sdk_dynamodb::Client,
    sub_team_pk: &Partition,
    window: ActivityWindow,
) -> Result<ActivityCountsResponse> {
    let (range_start_ms, range_end_ms) = compute_range(window);

    let posts = list_team_posts_in_range(cli, sub_team_pk, range_start_ms, range_end_ms).await?;
    let post_count = posts.len() as i64;

    // Active members = distinct user_pks that authored a Post (not the
    // team's own pk — when a team account is the author the sub-team
    // itself is technically "active", but FR-6 #36 wants individual
    // members). Use the post's user_pk when it's a User, else attribute
    // to the team and keep it as "team post".
    let mut active: HashSet<String> = HashSet::new();
    for p in &posts {
        match &p.user_pk {
            Partition::User(_) => {
                active.insert(p.user_pk.to_string());
            }
            _ => {
                // Team-authored post counts the team-level author itself;
                // record it as a single "member" entry keyed on the team
                // pk so we don't inflate the number with duplicates.
                active.insert(p.user_pk.to_string());
            }
        }
    }
    let active_member_count = active.len() as i64;

    let space_count = count_public_spaces_in_range(cli, sub_team_pk, range_start_ms, range_end_ms)
        .await
        .unwrap_or(0);

    let total_member_count = count_team_members(cli, sub_team_pk).await.unwrap_or(0);

    Ok(ActivityCountsResponse {
        window,
        range_start_ms,
        range_end_ms,
        post_count,
        space_count,
        active_member_count,
        total_member_count,
        privacy_notice: PrivacyNotice::default_notice(),
    })
}

/// Drill-down: per-member activity aggregated over the window. Sorted by
/// `post_count DESC` then `last_active_at DESC`. Spaces-participated is
/// derived from spaces authored by the user where visibility is Public —
/// a best-effort approximation per OQ-A (Phase 1: counts only).
pub async fn compute_member_activity(
    cli: &aws_sdk_dynamodb::Client,
    sub_team_pk: &Partition,
    window: ActivityWindow,
) -> Result<Vec<MemberActivityResponse>> {
    let (range_start_ms, _) = compute_range(window);

    // 1. Load every member user_pk of the sub-team.
    let members = resolve_team_member_pks(cli, sub_team_pk).await?;
    if members.is_empty() {
        return Ok(vec![]);
    }

    // 2. Fetch the team's posts in range (team-authored posts attributed
    //    to the team account itself — already in `list_team_posts_in_range`).
    //    Then also pull each member's personal posts in the window so the
    //    drill-down reflects individual contribution.
    let mut per_user_posts: HashMap<String, Vec<Post>> = HashMap::new();
    for user_pk in &members {
        let posts = list_user_posts_in_range(cli, user_pk, range_start_ms).await?;
        per_user_posts.insert(user_pk.to_string(), posts);
    }

    let mut rows: Vec<MemberActivityResponse> = Vec::with_capacity(members.len());
    for user_pk in &members {
        let posts = per_user_posts
            .get(&user_pk.to_string())
            .cloned()
            .unwrap_or_default();
        let post_count = posts.len() as i64;
        let last_active_at = posts.iter().map(|p| p.updated_at).max();

        // Identity — best-effort lookup; if the user row is missing we
        // still emit a row with a placeholder so the count is not lost.
        let (handle, display_name, profile_url) = load_user_identity(cli, user_pk).await;

        let space_count_participated =
            count_user_public_spaces_in_range(cli, user_pk, range_start_ms)
                .await
                .unwrap_or(0);

        rows.push(MemberActivityResponse {
            user_id: user_id_from(user_pk),
            handle,
            display_name,
            profile_url,
            post_count,
            space_count_participated,
            last_active_at,
        });
    }

    rows.sort_by(|a, b| {
        b.post_count
            .cmp(&a.post_count)
            .then(b.last_active_at.cmp(&a.last_active_at))
    });
    Ok(rows)
}

// ── Private helpers ────────────────────────────────────────────────

async fn count_team_members(
    cli: &aws_sdk_dynamodb::Client,
    team_pk: &Partition,
) -> Result<i64> {
    let user_team_sk = EntityType::UserTeam(team_pk.to_string());
    let mut count: i64 = 0;
    let mut bookmark: Option<String> = None;
    for _ in 0..MAX_SCAN_PAGES {
        let mut opt = UserTeamQueryOption::builder().limit(PAGE_SIZE);
        if let Some(bm) = bookmark.as_ref() {
            opt = opt.bookmark(bm.clone());
        }
        let (rows, next) = UserTeam::find_by_team(cli, &user_team_sk, opt)
            .await
            .map_err(|e| {
                crate::error!("count_team_members query failed: {e}");
                SubTeamError::ActivityAggregationFailed
            })?;
        count += rows.len() as i64;
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }
    if let Ok(Some(_)) = TeamOwner::get(cli, team_pk, Some(&EntityType::TeamOwner)).await {
        count += 1;
    }
    Ok(count)
}

async fn resolve_team_member_pks(
    cli: &aws_sdk_dynamodb::Client,
    team_pk: &Partition,
) -> Result<Vec<Partition>> {
    let mut seen: HashSet<String> = HashSet::new();

    if let Ok(Some(owner)) = TeamOwner::get(cli, team_pk, Some(&EntityType::TeamOwner)).await {
        seen.insert(owner.user_pk.to_string());
    }

    let user_team_sk = EntityType::UserTeam(team_pk.to_string());
    let mut bookmark: Option<String> = None;
    for _ in 0..MAX_SCAN_PAGES {
        let mut opt = UserTeamQueryOption::builder().limit(PAGE_SIZE);
        if let Some(bm) = bookmark.as_ref() {
            opt = opt.bookmark(bm.clone());
        }
        let (rows, next) = UserTeam::find_by_team(cli, &user_team_sk, opt)
            .await
            .map_err(|e| {
                crate::error!("resolve_team_member_pks query failed: {e}");
                SubTeamError::ActivityAggregationFailed
            })?;
        for row in rows {
            seen.insert(row.pk.to_string());
        }
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    Ok(seen
        .into_iter()
        .filter_map(|s| s.parse::<Partition>().ok())
        .collect())
}

/// Scan the team's posts (GSI1 find_by_user_pk) and keep only Published +
/// non-Private rows whose `created_at` falls within [start, end].
/// `filter_sk_prefix("POST")` drops other entity types sharing the
/// `USER_PK#…` GSI1 pk (e.g. `TeamOwner`, `SpaceCommon`) before the rows
/// hit the deserializer — otherwise a user's TeamOwner row would fail to
/// deserialize as a `Post` and break the whole query.
async fn list_team_posts_in_range(
    cli: &aws_sdk_dynamodb::Client,
    team_pk: &Partition,
    start_ms: i64,
    end_ms: i64,
) -> Result<Vec<Post>> {
    let mut out: Vec<Post> = Vec::new();
    let mut bookmark: Option<String> = None;
    for _ in 0..MAX_SCAN_PAGES {
        let opts = Post::opt_with_bookmark(bookmark.clone())
            .filter_sk_prefix("POST")
            .limit(PAGE_SIZE);
        let (items, next) = Post::find_by_user_pk(cli, team_pk, opts).await.map_err(|e| {
            crate::error!("list_team_posts_in_range query failed: {e}");
            SubTeamError::ActivityAggregationFailed
        })?;
        for p in items {
            if p.status != PostStatus::Published {
                continue;
            }
            // FR-6 #39 / AC-15 — exclude private posts.
            if matches!(p.visibility, Some(Visibility::Private)) {
                continue;
            }
            if p.created_at < start_ms || p.created_at > end_ms {
                continue;
            }
            out.push(p);
        }
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }
    Ok(out)
}

/// List a user's posts since `since_ms`, excluding Private and Draft.
/// See `list_team_posts_in_range` for the filter-sk-prefix rationale.
async fn list_user_posts_in_range(
    cli: &aws_sdk_dynamodb::Client,
    user_pk: &Partition,
    since_ms: i64,
) -> Result<Vec<Post>> {
    let mut out: Vec<Post> = Vec::new();
    let mut bookmark: Option<String> = None;
    for _ in 0..MAX_SCAN_PAGES {
        let opts = Post::opt_with_bookmark(bookmark.clone())
            .filter_sk_prefix("POST")
            .limit(PAGE_SIZE);
        let (items, next) = Post::find_by_user_pk(cli, user_pk, opts).await.map_err(|e| {
            crate::error!("list_user_posts_in_range query failed: {e}");
            SubTeamError::ActivityAggregationFailed
        })?;
        for p in items {
            if p.status != PostStatus::Published {
                continue;
            }
            if matches!(p.visibility, Some(Visibility::Private)) {
                continue;
            }
            if p.created_at < since_ms {
                continue;
            }
            out.push(p);
        }
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }
    Ok(out)
}

async fn max_post_updated_at(
    cli: &aws_sdk_dynamodb::Client,
    team_pk: &Partition,
    since_ms: i64,
) -> Result<Option<i64>> {
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let posts = list_team_posts_in_range(cli, team_pk, since_ms, now).await?;
    Ok(posts.iter().map(|p| p.updated_at).max())
}

async fn count_public_spaces_in_range(
    cli: &aws_sdk_dynamodb::Client,
    user_pk: &Partition,
    start_ms: i64,
    end_ms: i64,
) -> Result<i64> {
    let mut count: i64 = 0;
    let mut bookmark: Option<String> = None;
    for _ in 0..MAX_SCAN_PAGES {
        // `filter_sk_prefix("SPACE_COMMON")` drops non-SpaceCommon entity
        // types that share the `USER_PK#…` GSI1 pk (see
        // `list_team_posts_in_range` for rationale).
        let opts = SpaceCommon::opt_with_bookmark(bookmark.clone())
            .filter_sk_prefix("SPACE_COMMON")
            .limit(PAGE_SIZE);
        let (items, next) = SpaceCommon::find_by_user_pk(cli, user_pk, opts)
            .await
            .map_err(|e| {
                crate::error!("count_public_spaces_in_range query failed: {e}");
                SubTeamError::ActivityAggregationFailed
            })?;
        for s in items {
            if s.publish_state != SpacePublishState::Published {
                continue;
            }
            // Only Public spaces count — Private and Team-scoped spaces
            // are excluded from the parent-facing dashboard.
            if !matches!(s.visibility, SpaceVisibility::Public) {
                continue;
            }
            if s.created_at < start_ms || s.created_at > end_ms {
                continue;
            }
            count += 1;
        }
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }
    Ok(count)
}

async fn count_user_public_spaces_in_range(
    cli: &aws_sdk_dynamodb::Client,
    user_pk: &Partition,
    since_ms: i64,
) -> Result<i64> {
    let now = crate::common::utils::time::get_now_timestamp_millis();
    count_public_spaces_in_range(cli, user_pk, since_ms, now).await
}

async fn load_user_identity(
    cli: &aws_sdk_dynamodb::Client,
    user_pk: &Partition,
) -> (String, String, String) {
    use crate::common::models::auth::User;
    match User::get(cli, user_pk, Some(EntityType::User)).await {
        Ok(Some(u)) => (u.username, u.display_name, u.profile_url),
        _ => (String::new(), String::new(), String::new()),
    }
}

fn user_id_from(pk: &Partition) -> String {
    match pk {
        Partition::User(uid) => uid.clone(),
        other => other.to_string(),
    }
}
