//! Direct-to-one-sub-team announcement endpoint (read side).
//!
//! Powers the history rows under the "이 하위팀에만 공지" card on the
//! parent's sub-team detail page. The send side now goes through the
//! shared broadcast composer (`create_announcement_handler` +
//! `publish_announcement_handler` with `target_child_team_id` set) so
//! direct messages share the editor, attachments, Space attachment,
//! and Draft → Publish 2-step lifecycle.

use crate::common::*;
use crate::features::posts::models::Team;
use crate::features::social::pages::member::dto::TeamRole;
use crate::features::sub_team::types::{SubTeamAnnouncementResponse, SubTeamError};

#[cfg(feature = "server")]
use crate::features::sub_team::models::{SubTeamAnnouncement, SubTeamAnnouncementStatus};

#[cfg(feature = "server")]
// Trailing `#` keeps the begins_with filter from accidentally matching
// `SUB_TEAM_ANNOUNCEMENT_FANOUT#...` marker rows that live in the same
// partition. See `announcements.rs` for the full rationale.
const ANNOUNCEMENT_SK_PREFIX: &str = "SUB_TEAM_ANNOUNCEMENT#";
#[cfg(feature = "server")]
const PAGE_LIMIT: i32 = 100;

// ── GET /api/teams/:team_pk/sub-teams/:sub_team_id/direct-messages ───────
//
// History of direct messages this parent has sent to one specific
// sub-team. Powers the "history" rows beneath the compose card on the
// detail page. Bookmarkable for pagination, newest first.
#[get(
    "/api/teams/:team_pk/sub-teams/:sub_team_id/direct-messages?bookmark",
    user: crate::features::auth::User,
    team: Team,
    role: TeamRole
)]
pub async fn list_direct_messages_handler(
    team_pk: TeamPartition,
    sub_team_id: String,
    bookmark: Option<String>,
) -> Result<ListResponse<SubTeamAnnouncementResponse>> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let opts = SubTeamAnnouncement::opt_with_bookmark(bookmark)
        .sk(ANNOUNCEMENT_SK_PREFIX.to_string())
        .limit(PAGE_LIMIT);
    let (items, next) = SubTeamAnnouncement::query(cli, team.pk.clone(), opts)
        .await
        .map_err(|e| {
            crate::error!("list_direct_messages query failed: {e}");
            SubTeamError::AnnouncementNotFound
        })?;

    let mut items: Vec<_> = items
        .into_iter()
        .filter(|a| {
            a.target_child_team_id.as_deref() == Some(sub_team_id.as_str())
                && !matches!(a.status, SubTeamAnnouncementStatus::Deleted)
        })
        .collect();
    items.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    // Anchor Post pk for every direct message is deterministic
    // (`Feed(announcement_id)`) in the new model, so we can fill in
    // `target_post_pk` directly without a feed walk. Old rows that
    // didn't carry the field still resolve correctly.
    for a in items.iter_mut() {
        if a.target_post_pk.is_none() {
            a.target_post_pk = Some(Partition::Feed(a.announcement_id.clone()).to_string());
        }
    }

    let response: Vec<SubTeamAnnouncementResponse> = items.into_iter().map(Into::into).collect();
    Ok((response, next).into())
}
