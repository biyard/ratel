//! Direct-to-one-sub-team announcement endpoints.
//!
//! Renders the "이 하위팀에만 공지 · Direct announcement" card on the
//! parent's sub-team detail page. Single-step send (no draft), no
//! post-publish edits — the trade is that fanout writes ONE post to the
//! target child's feed and demotes any prior direct-message post from
//! this parent so only the latest stays pinned.
//!
//! Companion list endpoint surfaces the history under the same card.

use crate::common::*;
use crate::features::posts::models::Team;
use crate::features::social::pages::member::dto::TeamRole;
use crate::features::sub_team::types::{
    SendDirectMessageRequest, SubTeamAnnouncementResponse, SubTeamError,
};

#[cfg(feature = "server")]
use crate::features::sub_team::models::{
    SubTeamAnnouncement, SubTeamAnnouncementStatus, SubTeamLink,
};

#[cfg(feature = "server")]
const ANNOUNCEMENT_SK_PREFIX: &str = "SUB_TEAM_ANNOUNCEMENT";
#[cfg(feature = "server")]
const PAGE_LIMIT: i32 = 100;

// ── POST /api/teams/:team_pk/sub-teams/:sub_team_id/direct-message ───────
//
// Parent-side action. Sends a direct announcement to a single recognized
// sub-team. Creates a `SubTeamAnnouncement` row with
// `target_child_team_id = Some(sub_team_id)` and `status = Published`
// (skipping the Draft step). The Stream → fanout chain writes one Post
// to the target child's feed; older direct-message Posts from this
// parent are demoted so only the newest stays pinned.
#[post(
    "/api/teams/:team_pk/sub-teams/:sub_team_id/direct-message",
    user: crate::features::auth::User,
    team: Team,
    role: TeamRole
)]
pub async fn send_direct_message_handler(
    team_pk: TeamPartition,
    sub_team_id: String,
    body: SendDirectMessageRequest,
) -> Result<SubTeamAnnouncementResponse> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    if body.title.trim().is_empty() {
        return Err(SubTeamError::AnnouncementPublishFailed.into());
    }

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    // Verify the target is a recognized sub-team of this parent. Stops
    // direct-messaging arbitrary teams via guessed ids.
    let link_sk = EntityType::SubTeamLink(sub_team_id.clone());
    let link = SubTeamLink::get(cli, &team.pk, Some(link_sk))
        .await
        .map_err(|e| {
            crate::error!("send_direct_message link lookup failed: {e}");
            SubTeamError::SubTeamLinkNotFound
        })?
        .ok_or(SubTeamError::SubTeamLinkNotFound)?;
    let _ = link;

    let announcement = SubTeamAnnouncement::new_direct(
        team.pk.clone(),
        sub_team_id,
        body.title,
        body.body,
        user.pk.to_string(),
    );
    announcement.create(cli).await.map_err(|e| {
        crate::error!("send_direct_message create failed: {e}");
        SubTeamError::AnnouncementPublishFailed
    })?;

    Ok(announcement.into())
}

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

    // Backfill `target_post_pk` for rows fanned out before the field
    // existed. We can't query Posts by `announcement_id` directly (no
    // GSI), so walk the child team's feed once and index by
    // announcement_id, then attach.
    let needs_backfill = items.iter().any(|a| a.target_post_pk.is_none());
    if needs_backfill {
        use crate::features::posts::models::Post;
        let child_team_pk = Partition::Team(sub_team_id.clone());
        let mut bookmark: Option<String> = None;
        let mut by_ann: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        for _ in 0..10 {
            let mut opt = Post::opt().limit(100);
            if let Some(bm) = bookmark.as_ref() {
                opt = opt.bookmark(bm.clone());
            }
            let (posts, next_bm) = Post::find_by_user_pk(cli, &child_team_pk, opt)
                .await
                .map_err(|e| {
                    crate::error!("list_direct_messages backfill find_by_user_pk failed: {e}");
                    SubTeamError::AnnouncementNotFound
                })?;
            for p in posts {
                if let Some(ann_id) = &p.announcement_id {
                    by_ann.entry(ann_id.clone()).or_insert_with(|| p.pk.to_string());
                }
            }
            match next_bm {
                Some(b) => bookmark = Some(b),
                None => break,
            }
        }
        for a in items.iter_mut() {
            if a.target_post_pk.is_none() {
                if let Some(post_pk) = by_ann.get(&a.announcement_id) {
                    a.target_post_pk = Some(post_pk.clone());
                }
            }
        }
    }

    let response: Vec<SubTeamAnnouncementResponse> =
        items.into_iter().map(Into::into).collect();
    Ok((response, next).into())
}
