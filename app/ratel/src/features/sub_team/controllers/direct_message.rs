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
// Trailing `#` keeps the begins_with filter from accidentally matching
// `SUB_TEAM_ANNOUNCEMENT_FANOUT#...` marker rows that live in the same
// partition. See `announcements.rs` for the full rationale.
const ANNOUNCEMENT_SK_PREFIX: &str = "SUB_TEAM_ANNOUNCEMENT#";
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

    let mut announcement = SubTeamAnnouncement::new_direct(
        team.pk.clone(),
        sub_team_id.clone(),
        body.title.clone(),
        body.body.clone(),
        user.pk.to_string(),
    );

    // Write the anchor Post BEFORE the announcement row. Anchor pk is
    // deterministic (`Feed(announcement_id)`), and we want the row that
    // children's `/posts/{id}` deep-links land on to exist by the time
    // any inbox notification fires.
    //
    // Anchor `user_pk` is the **target child team** (not the parent),
    // so the Post surfaces naturally in that child's
    // `Post::find_by_user_and_status` wall query and stays off the
    // parent's wall. Author metadata still records the parent team so
    // the rendered card shows "parent team posted in your wall".
    use crate::features::posts::models::Post;
    use crate::features::posts::types::{PostStatus, PostType};

    let parent_team_id_str = match &team.pk {
        Partition::Team(id) => id.clone(),
        _ => String::new(),
    };
    let child_team_pk = Partition::Team(sub_team_id.clone());
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let anchor_post = Post {
        pk: Partition::Feed(announcement.announcement_id.clone()),
        sk: EntityType::Post,
        created_at: now,
        updated_at: now,
        title: body.title.clone(),
        body: ContentBody::html(body.body.clone()),
        post_type: PostType::Post,
        status: PostStatus::Published,
        // Direct messages share the same audience-gate as broadcasts:
        // resolved at read time by `broadcast_access::can_view_broadcast_post`
        // (parent's members + the one target child's members).
        visibility: Some(crate::features::posts::types::Visibility::Broadcast),
        shares: 0,
        likes: 0,
        comments: 0,
        reports: 0,
        user_pk: child_team_pk,
        author_display_name: team.display_name.clone(),
        author_profile_url: team.profile_url.clone(),
        author_username: team.username.clone(),
        author_type: crate::common::types::UserType::Team,
        space_pk: None,
        space_type: None,
        space_visibility: None,
        booster: None,
        rewards: None,
        urls: vec![],
        attachments: vec![],
        categories: vec![],
        announcement_id: Some(announcement.announcement_id.clone()),
        announcement_parent_team_id: Some(parent_team_id_str),
        pinned_as_announcement: false,
    };
    anchor_post.create(cli).await.map_err(|e| {
        crate::error!("send_direct_message anchor post create failed: {e}");
        SubTeamError::AnnouncementPublishFailed
    })?;

    // Pre-populate `target_post_pk` on the announcement row so the
    // parent's direct-message history can render deep links without
    // a backfill. Stream-driven fanout will overwrite this with the
    // same value later, but writing it now keeps the row consistent
    // even if the stream is delayed.
    announcement.target_post_pk = Some(anchor_post.pk.to_string());

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

    // Anchor Post pk for every direct message is deterministic
    // (`Feed(announcement_id)`) in the new model, so we can fill in
    // `target_post_pk` directly without a feed walk. Old rows that
    // didn't carry the field still resolve correctly.
    for a in items.iter_mut() {
        if a.target_post_pk.is_none() {
            a.target_post_pk = Some(Partition::Feed(a.announcement_id.clone()).to_string());
        }
    }

    let response: Vec<SubTeamAnnouncementResponse> =
        items.into_iter().map(Into::into).collect();
    Ok((response, next).into())
}
