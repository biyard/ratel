//! Parent-admin activity dashboard endpoints.
//!
//! All routes nest under `/api/teams/:team_pk/sub-teams/...` where the
//! `team_pk` path segment is the PARENT team. Admin/Owner role required.
//!
//! Phase 1 aggregation is query-time — see
//! `sub_team::services::activity_aggregation` for the cost/cap model. All
//! endpoints exclude private posts and non-public spaces (FR-6 #39 / AC-15)
//! and return the fixed privacy notice inline (AC-20).

use crate::common::*;
use crate::features::posts::models::Team;
use crate::features::social::pages::member::dto::TeamRole;
use crate::features::sub_team::types::{
    ActivityCountsResponse, ActivityWindow, MemberActivityResponse, PrivacyNotice,
    SubTeamDetailResponse, SubTeamError, SubTeamListResponse, SubTeamSummaryResponse,
};

#[cfg(feature = "server")]
use crate::features::sub_team::models::SubTeamLink;
#[cfg(feature = "server")]
use crate::features::sub_team::services::activity_aggregation::{
    build_sub_team_summary, compute_activity_counts, compute_member_activity, list_sub_team_links,
};

// ── GET /api/teams/:team_pk/sub-teams ─────────────────────────────
//
// Lists recognized sub-teams under the parent team. Admin/Owner only. The
// envelope's `truncated` flag is set when the parent has > 50 sub-teams —
// Phase 1 cap, see MAX_SUB_TEAMS in activity_aggregation.
#[get("/api/teams/:team_pk/sub-teams?bookmark", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn list_sub_teams_handler(
    team_pk: TeamPartition,
    bookmark: Option<String>,
) -> Result<SubTeamListResponse> {
    let _ = team_pk;
    let _ = user;
    let _ = bookmark; // Phase 1: single page; bookmark reserved for Phase 2.
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let (links, truncated) = list_sub_team_links(cli, &team.pk).await?;

    let mut items: Vec<SubTeamSummaryResponse> = Vec::with_capacity(links.len());
    for link in &links {
        match build_sub_team_summary(cli, link).await {
            Ok(row) => items.push(row),
            Err(e) => {
                crate::error!(
                    "list_sub_teams: summary failed for child={}: {e}",
                    link.child_team_id
                );
            }
        }
    }

    Ok(SubTeamListResponse {
        items,
        bookmark: None,
        truncated,
    })
}

// ── GET /api/teams/:team_pk/sub-teams/:sub_team_id ────────────────
//
// Sub-team overview — identity + counts aggregated over the default
// Monthly window. `privacy_notice` is included so the overview UI never
// needs a second call.
#[get("/api/teams/:team_pk/sub-teams/:sub_team_id?window", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn get_sub_team_detail_handler(
    team_pk: TeamPartition,
    sub_team_id: String,
    window: Option<ActivityWindow>,
) -> Result<SubTeamDetailResponse> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    // Verify the parent→child link exists so a parent admin can't query
    // arbitrary teams' activity just by URL-guessing.
    let link_sk = EntityType::SubTeamLink(sub_team_id.clone());
    let link = SubTeamLink::get(cli, &team.pk, Some(link_sk))
        .await
        .map_err(|e| {
            crate::error!("get_sub_team_detail link fetch failed: {e}");
            SubTeamError::ActivityAggregationFailed
        })?
        .ok_or(SubTeamError::SubTeamLinkNotFound)?;

    let sub_team_pk = Partition::Team(sub_team_id.clone());
    let child = Team::get(cli, &sub_team_pk, Some(EntityType::Team))
        .await
        .map_err(|e| {
            crate::error!("get_sub_team_detail team fetch failed: {e}");
            SubTeamError::ActivityAggregationFailed
        })?
        .ok_or(SubTeamError::ActivityAggregationFailed)?;

    let window = window.unwrap_or(ActivityWindow::Monthly);
    let counts = compute_activity_counts(cli, &sub_team_pk, window).await?;

    Ok(SubTeamDetailResponse {
        sub_team_id,
        display_name: child.display_name,
        profile_url: child.profile_url,
        username: child.username,
        recognized_at: link.approved_at,
        window,
        post_count: counts.post_count,
        space_count: counts.space_count,
        active_member_count: counts.active_member_count,
        total_member_count: counts.total_member_count,
        privacy_notice: PrivacyNotice::default_notice(),
    })
}

// ── GET /api/teams/:team_pk/sub-teams/:sub_team_id/activity ───────
//
// Numeric-only counts for a given window (default Monthly). The detail
// page shows these next to the headline metrics.
#[get("/api/teams/:team_pk/sub-teams/:sub_team_id/activity?window", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn get_sub_team_activity_handler(
    team_pk: TeamPartition,
    sub_team_id: String,
    window: Option<ActivityWindow>,
) -> Result<ActivityCountsResponse> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    // Authorize: link must exist.
    let link_sk = EntityType::SubTeamLink(sub_team_id.clone());
    let _link = SubTeamLink::get(cli, &team.pk, Some(link_sk))
        .await
        .map_err(|e| {
            crate::error!("get_sub_team_activity link fetch failed: {e}");
            SubTeamError::ActivityAggregationFailed
        })?
        .ok_or(SubTeamError::SubTeamLinkNotFound)?;

    let sub_team_pk = Partition::Team(sub_team_id);
    let window = window.unwrap_or(ActivityWindow::Monthly);
    compute_activity_counts(cli, &sub_team_pk, window).await
}

// ── GET /api/teams/:team_pk/sub-teams/:sub_team_id/member-activity ─
//
// Drill-down. Returns per-member rows sorted by post_count DESC then
// last_active_at DESC. Bookmark is currently `None` — Phase 1 returns
// the full member set (teams are ≤ 200 members per roadmap constraint).
#[get("/api/teams/:team_pk/sub-teams/:sub_team_id/member-activity?window&bookmark", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn get_sub_team_member_activity_handler(
    team_pk: TeamPartition,
    sub_team_id: String,
    window: Option<ActivityWindow>,
    bookmark: Option<String>,
) -> Result<ListResponse<MemberActivityResponse>> {
    let _ = team_pk;
    let _ = user;
    let _ = bookmark; // Phase 1: unpaginated; reserved for Phase 2.
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    // Authorize: link must exist.
    let link_sk = EntityType::SubTeamLink(sub_team_id.clone());
    let _link = SubTeamLink::get(cli, &team.pk, Some(link_sk))
        .await
        .map_err(|e| {
            crate::error!("get_sub_team_member_activity link fetch failed: {e}");
            SubTeamError::ActivityAggregationFailed
        })?
        .ok_or(SubTeamError::SubTeamLinkNotFound)?;

    let sub_team_pk = Partition::Team(sub_team_id);
    let window = window.unwrap_or(ActivityWindow::Monthly);
    let rows = compute_member_activity(cli, &sub_team_pk, window).await?;
    Ok((rows, None).into())
}
