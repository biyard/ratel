use crate::common::*;
use crate::features::posts::models::Team;
use crate::features::social::pages::member::dto::TeamRole;
use crate::features::sub_team::models::SubTeamAnnouncementStatus;
use crate::features::sub_team::types::{
    CreateSubTeamAnnouncementRequest, SubTeamAnnouncementResponse, SubTeamError,
    UpdateSubTeamAnnouncementRequest,
};

#[cfg(feature = "server")]
use crate::features::sub_team::models::SubTeamAnnouncement;
#[cfg(feature = "server")]
use crate::features::sub_team::services::announcement_fanout::{
    count_recognized_sub_teams, MAX_RECOGNIZED_SUB_TEAMS,
};

#[cfg(feature = "server")]
// Trailing `#` is required: without it the begins_with query would also
// match `SUB_TEAM_ANNOUNCEMENT_FANOUT#...` marker rows in the same
// partition (when a team is both a parent broadcasting and a recognized
// child of someone else), and the deserializer would then try to read
// a `SubTeamAnnouncement` out of a marker row and fail on a missing
// `title` field — aborting the whole list. The `#` narrows the prefix to
// the announcement-id segment only.
const ANNOUNCEMENT_SK_PREFIX: &str = "SUB_TEAM_ANNOUNCEMENT#";
#[cfg(feature = "server")]
const PAGE_LIMIT: i32 = 50;

// ── GET /api/teams/:team_pk/sub-teams/announcements ─────────────────
#[get("/api/teams/:team_pk/sub-teams/announcements?bookmark&status", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn list_announcements_handler(
    team_pk: TeamPartition,
    bookmark: Option<String>,
    status: Option<SubTeamAnnouncementStatus>,
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
            crate::error!("list_announcements query failed: {e}");
            SubTeamError::AnnouncementNotFound
        })?;

    // Default: exclude Deleted. If a specific status is requested, return only
    // rows matching that status. Newest first.
    let mut items: Vec<_> = items
        .into_iter()
        .filter(|a| match status {
            Some(s) => a.status == s,
            None => !matches!(a.status, SubTeamAnnouncementStatus::Deleted),
        })
        .collect();
    items.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    // Comments aggregate: each announcement's anchor Post lives at
    // `Partition::Feed(announcement_id)` (created by
    // `publish_announcement_handler`). Batch-get them all and map
    // pk → comments so the response surfaces the real count instead
    // of the hardcoded 0 placeholder.
    use crate::features::posts::models::Post;
    use std::collections::HashMap;
    let anchor_keys: Vec<(Partition, EntityType)> = items
        .iter()
        .filter(|a| matches!(a.status, SubTeamAnnouncementStatus::Published))
        .map(|a| (Partition::Feed(a.announcement_id.clone()), EntityType::Post))
        .collect();
    let comment_counts: HashMap<String, i64> = if anchor_keys.is_empty() {
        HashMap::new()
    } else {
        Post::batch_get(cli, anchor_keys)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|p| (p.pk.to_string(), p.comments))
            .collect()
    };

    let items: Vec<SubTeamAnnouncementResponse> = items
        .into_iter()
        .map(|a| {
            let pk_key = Partition::Feed(a.announcement_id.clone()).to_string();
            let mut resp: SubTeamAnnouncementResponse = a.into();
            if let Some(c) = comment_counts.get(&pk_key) {
                resp.comments_count = (*c) as i32;
            }
            resp
        })
        .collect();
    Ok((items, next).into())
}

// ── GET /api/teams/:team_pk/sub-teams/announcements/:announcement_id ─
#[get("/api/teams/:team_pk/sub-teams/announcements/:announcement_id", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn get_announcement_handler(
    team_pk: TeamPartition,
    announcement_id: String,
) -> Result<SubTeamAnnouncementResponse> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let sk = EntityType::SubTeamAnnouncement(announcement_id);

    let ann = SubTeamAnnouncement::get(cli, &team.pk, Some(sk))
        .await
        .map_err(|e| {
            crate::error!("get_announcement get failed: {e}");
            SubTeamError::AnnouncementNotFound
        })?
        .ok_or(SubTeamError::AnnouncementNotFound)?;
    let mut resp: SubTeamAnnouncementResponse = ann.clone().into();
    // Anchor Post lives at Feed(announcement_id); pull its comment count.
    if matches!(ann.status, SubTeamAnnouncementStatus::Published) {
        let anchor_pk = Partition::Feed(ann.announcement_id.clone());
        if let Ok(Some(p)) = crate::features::posts::models::Post::get(
            cli,
            &anchor_pk,
            Some(EntityType::Post),
        )
        .await
        {
            resp.comments_count = p.comments as i32;
        }
    }
    Ok(resp)
}

// ── POST /api/teams/:team_pk/sub-teams/announcements ────────────────
#[post("/api/teams/:team_pk/sub-teams/announcements", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn create_announcement_handler(
    team_pk: TeamPartition,
    body: CreateSubTeamAnnouncementRequest,
) -> Result<SubTeamAnnouncementResponse> {
    let _ = team_pk;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let mut ann = SubTeamAnnouncement::new_draft(
        team.pk.clone(),
        body.title,
        body.body,
        user.pk.to_string(),
    );
    ann.html_contents = body.html_contents;
    ann.tags = body.tags;
    ann.attachments = body.attachments;
    ann.space_enabled = body.space_enabled;
    ann.space_type = body.space_type;
    ann.create(cli).await.map_err(|e| {
        crate::error!("create_announcement create failed: {e}");
        SubTeamError::AnnouncementPublishFailed
    })?;

    Ok(ann.into())
}

// ── PATCH /api/teams/:team_pk/sub-teams/announcements/:announcement_id ─
#[patch("/api/teams/:team_pk/sub-teams/announcements/:announcement_id", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn update_announcement_handler(
    team_pk: TeamPartition,
    announcement_id: String,
    body: UpdateSubTeamAnnouncementRequest,
) -> Result<SubTeamAnnouncementResponse> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let sk = EntityType::SubTeamAnnouncement(announcement_id);

    let mut existing = SubTeamAnnouncement::get(cli, &team.pk, Some(sk.clone()))
        .await
        .map_err(|e| {
            crate::error!("update_announcement get failed: {e}");
            SubTeamError::AnnouncementNotFound
        })?
        .ok_or(SubTeamError::AnnouncementNotFound)?;

    if !matches!(existing.status, SubTeamAnnouncementStatus::Draft) {
        return Err(SubTeamError::AnnouncementNotInDraft.into());
    }

    let now = crate::common::utils::time::get_now_timestamp_millis();
    let mut updater =
        SubTeamAnnouncement::updater(&team.pk, &sk).with_updated_at(now);
    existing.updated_at = now;
    let mut changed = false;

    if let Some(title) = body.title {
        updater = updater.with_title(title.clone());
        existing.title = title;
        changed = true;
    }

    if let Some(body_str) = body.body {
        updater = updater.with_body(body_str.clone());
        existing.body = body_str;
        changed = true;
    }

    if let Some(html) = body.html_contents {
        updater = updater.with_html_contents(html.clone());
        existing.html_contents = html;
        changed = true;
    }

    if let Some(tags) = body.tags {
        updater = updater.with_tags(tags.clone());
        existing.tags = tags;
        changed = true;
    }

    if let Some(attachments) = body.attachments {
        updater = updater.with_attachments(attachments.clone());
        existing.attachments = attachments;
        changed = true;
    }

    if let Some(se) = body.space_enabled {
        updater = updater.with_space_enabled(se);
        existing.space_enabled = se;
        changed = true;
    }

    if let Some(st) = body.space_type {
        updater = updater.with_space_type(st);
        existing.space_type = Some(st);
        changed = true;
    }

    if changed {
        updater.execute(cli).await.map_err(|e| {
            crate::error!("update_announcement execute failed: {e}");
            SubTeamError::AnnouncementPublishFailed
        })?;
    }

    Ok(existing.into())
}

// ── POST /api/teams/:team_pk/sub-teams/announcements/:announcement_id/publish ─
#[post("/api/teams/:team_pk/sub-teams/announcements/:announcement_id/publish", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn publish_announcement_handler(
    team_pk: TeamPartition,
    announcement_id: String,
) -> Result<SubTeamAnnouncementResponse> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let sk = EntityType::SubTeamAnnouncement(announcement_id);

    let mut existing = SubTeamAnnouncement::get(cli, &team.pk, Some(sk.clone()))
        .await
        .map_err(|e| {
            crate::error!("publish_announcement get failed: {e}");
            SubTeamError::AnnouncementNotFound
        })?
        .ok_or(SubTeamError::AnnouncementNotFound)?;

    if !matches!(existing.status, SubTeamAnnouncementStatus::Draft) {
        return Err(SubTeamError::AnnouncementNotInDraft.into());
    }

    let count = count_recognized_sub_teams(cli, &team.pk).await?;
    if count > MAX_RECOGNIZED_SUB_TEAMS {
        return Err(SubTeamError::BroadcastTooManySubTeams.into());
    }

    let now = crate::common::utils::time::get_now_timestamp_millis();

    // Build the Space FIRST (if enabled) so the anchor Post can be
    // created with `space_pk` already populated in a single Put. An
    // earlier version used `Post::updater(...).with_space_pk(...)` to
    // link the two after the Post existed, but the DynamoEntity updater
    // re-derives every GSI sort key from its default-initialised inner
    // struct — so `with_updated_at(now)` ended up writing
    // gsi5_sk=`"Draft##<now>"` on the live row and the anchor Post
    // vanished from the parent's feed (the `Published` prefix query
    // stopped matching it). Putting once with all fields correct
    // sidesteps the updater entirely.
    use crate::common::models::space::SpaceCommon;
    use crate::features::posts::models::Post;
    use crate::features::posts::types::{PostStatus, PostType};

    let parent_team_id_str = match &team.pk {
        Partition::Team(id) => id.clone(),
        _ => String::new(),
    };
    let anchor_post_body = if !existing.html_contents.is_empty() {
        existing.html_contents.clone()
    } else {
        existing.body.clone()
    };

    let space: Option<SpaceCommon> = if existing.space_enabled && existing.space_pk.is_none() {
        Some(SpaceCommon::new(
            FeedPartition(existing.announcement_id.clone()),
            team.pk.clone(),
            team.display_name.clone(),
            team.profile_url.clone(),
            team.username.clone(),
        ))
    } else {
        None
    };

    let anchor_post = Post {
        pk: Partition::Feed(existing.announcement_id.clone()),
        sk: EntityType::Post,
        created_at: now,
        updated_at: now,
        title: existing.title.clone(),
        body: ContentBody::html(anchor_post_body),
        post_type: PostType::Post,
        status: PostStatus::Published,
        // Sub-team broadcasts are NEVER publicly visible — `Visibility::Broadcast`
        // hands access control over to
        // `sub_team::services::broadcast_access::can_view_broadcast_post`
        // (parent team's members + every recognized child team's members).
        visibility: Some(crate::features::posts::types::Visibility::Broadcast),
        shares: 0,
        likes: 0,
        comments: 0,
        reports: 0,
        user_pk: team.pk.clone(),
        author_display_name: team.display_name.clone(),
        author_profile_url: team.profile_url.clone(),
        author_username: team.username.clone(),
        author_type: crate::common::types::UserType::Team,
        space_pk: space.as_ref().map(|s| s.pk.clone()),
        space_type: existing.space_type,
        space_visibility: None,
        booster: None,
        rewards: None,
        urls: vec![],
        attachments: existing.attachments.clone(),
        categories: existing.tags.clone(),
        announcement_id: Some(existing.announcement_id.clone()),
        announcement_parent_team_id: Some(parent_team_id_str),
        // Broadcasts pin to the top of every receiving team's wall —
        // parent + recognized children — so the announcement card sits
        // above the regular feed. `list_team_posts_handler` sorts on
        // this flag first, then created_at desc.
        pinned_as_announcement: true,
        ai_draft_used: false,
    };
    if let Err(e) = anchor_post.create(cli).await {
        crate::error!("publish_announcement: anchor post create failed: {e}");
        return Err(SubTeamError::AnnouncementPublishFailed.into());
    }

    if let Some(space) = space {
        let space_pk_str = space.pk.to_string();
        if let Err(e) = space.create(cli).await {
            crate::error!("publish_announcement: space create failed: {e}");
            return Err(SubTeamError::AnnouncementPublishFailed.into());
        }
        existing.space_pk = Some(space_pk_str);
    }

    let mut updater = SubTeamAnnouncement::updater(&team.pk, &sk)
        .with_status(SubTeamAnnouncementStatus::Published)
        .with_published_at(now)
        .with_updated_at(now);
    if let Some(space_pk) = existing.space_pk.clone() {
        updater = updater.with_space_pk(space_pk);
    }
    updater.execute(cli).await.map_err(|e| {
        crate::error!("publish_announcement execute failed: {e}");
        SubTeamError::AnnouncementPublishFailed
    })?;

    existing.status = SubTeamAnnouncementStatus::Published;
    existing.published_at = Some(now);
    existing.updated_at = now;

    // Fan-out happens asynchronously via the DynamoDB Stream → Pipe → Lambda
    // chain in deployed envs, or via the local-dev stream poller which
    // invokes `handle_announcement_published` from `common::stream_handler`.
    // Integration tests call the service directly.

    Ok(existing.into())
}

// ── DELETE /api/teams/:team_pk/sub-teams/announcements/:announcement_id ─
#[delete("/api/teams/:team_pk/sub-teams/announcements/:announcement_id", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn delete_announcement_handler(
    team_pk: TeamPartition,
    announcement_id: String,
) -> Result<String> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let sk = EntityType::SubTeamAnnouncement(announcement_id);

    // Soft-delete — keep any already-fanned-out Posts intact. Phase 1 does
    // NOT un-pin past fan-out Posts on delete.
    let existing = SubTeamAnnouncement::get(cli, &team.pk, Some(sk.clone()))
        .await
        .map_err(|e| {
            crate::error!("delete_announcement get failed: {e}");
            SubTeamError::AnnouncementNotFound
        })?
        .ok_or(SubTeamError::AnnouncementNotFound)?;

    let now = crate::common::utils::time::get_now_timestamp_millis();
    SubTeamAnnouncement::updater(&team.pk, &sk)
        .with_status(SubTeamAnnouncementStatus::Deleted)
        .with_updated_at(now)
        .execute(cli)
        .await
        .map_err(|e| {
            crate::error!("delete_announcement execute failed: {e}");
            SubTeamError::AnnouncementPublishFailed
        })?;

    let _ = existing;
    Ok(String::new())
}
