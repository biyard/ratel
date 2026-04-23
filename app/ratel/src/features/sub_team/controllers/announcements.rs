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
const ANNOUNCEMENT_SK_PREFIX: &str = "SUB_TEAM_ANNOUNCEMENT";
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
    let items: Vec<SubTeamAnnouncementResponse> = items.into_iter().map(Into::into).collect();
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
    Ok(ann.into())
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

    let ann = SubTeamAnnouncement::new_draft(
        team.pk.clone(),
        body.title,
        body.body,
        user.pk.to_string(),
    );
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
    SubTeamAnnouncement::updater(&team.pk, &sk)
        .with_status(SubTeamAnnouncementStatus::Published)
        .with_published_at(now)
        .with_updated_at(now)
        .execute(cli)
        .await
        .map_err(|e| {
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
