use crate::common::*;
use crate::features::posts::models::Team;
use crate::features::social::pages::member::dto::TeamRole;
use crate::features::sub_team::types::{
    DeregisterRequest, LeaveParentRequest, SubTeamError, TerminationAck,
};

#[cfg(feature = "server")]
use crate::features::sub_team::models::SubTeamLink;
#[cfg(feature = "server")]
use crate::features::sub_team::services::termination::{
    build_parent_sub_teams_url, build_sub_team_parent_url, detach_sub_team_link,
    notify_team_admins,
};

// ── POST /api/teams/:team_pk/sub-teams/:sub_team_id/deregister ──────
//
// Parent-side action. The admin/owner of the parent removes the link to a
// recognized sub-team. Child team content (posts, members, spaces, messages)
// stays intact — only `parent_team_id` and the `SubTeamLink` row are touched.
#[post(
    "/api/teams/:team_pk/sub-teams/:sub_team_id/deregister",
    user: crate::features::auth::User,
    team: Team,
    role: TeamRole
)]
pub async fn deregister_sub_team_handler(
    team_pk: TeamPartition,
    sub_team_id: String,
    body: DeregisterRequest,
) -> Result<TerminationAck> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    if body.reason.trim().is_empty() {
        return Err(SubTeamError::ApplicationStateMismatch.into());
    }

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    // 1. Confirm SubTeamLink exists under parent pk.
    let link_sk = EntityType::SubTeamLink(sub_team_id.clone());
    let link = SubTeamLink::get(cli, &team.pk, Some(link_sk.clone()))
        .await
        .map_err(|e| {
            crate::error!("deregister_sub_team link lookup failed: {e}");
            SubTeamError::SubTeamLinkNotFound
        })?
        .ok_or(SubTeamError::SubTeamLinkNotFound)?;
    let _ = link;

    let child_pk: Partition = Partition::Team(sub_team_id.clone());

    // 2. Transact-write: delete link + clear child parent_team_id.
    detach_sub_team_link(cli, &team.pk, &child_pk, &sub_team_id).await?;

    // 3. Notify former sub-team admins.
    let former_parent_team_id = match &team.pk {
        Partition::Team(id) => id.clone(),
        _ => String::new(),
    };
    let former_parent_team_name = team.display_name.clone();
    let reason = body.reason.clone();
    let cta_url = build_sub_team_parent_url(&sub_team_id);
    notify_team_admins(cli, &child_pk, move || {
        InboxPayload::SubTeamDeregistered {
            former_parent_team_id: former_parent_team_id.clone(),
            former_parent_team_name: former_parent_team_name.clone(),
            sub_team_id: sub_team_id.clone(),
            reason: reason.clone(),
            cta_url: cta_url.clone(),
        }
    })
    .await;

    Ok(TerminationAck { ok: true })
}

// ── POST /api/teams/:team_pk/parent/leave ───────────────────────────
//
// Child-side action. The admin/owner of the applying team severs its
// relationship with its recognized parent. Content stays intact; the child
// returns to standalone.
#[post(
    "/api/teams/:team_pk/parent/leave",
    user: crate::features::auth::User,
    team: Team,
    role: TeamRole
)]
pub async fn leave_parent_handler(
    team_pk: TeamPartition,
    body: LeaveParentRequest,
) -> Result<TerminationAck> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    // 1. Must currently be a recognized sub-team.
    let parent_team_id = team
        .parent_team_id
        .clone()
        .ok_or(SubTeamError::NotASubTeam)?;

    let sub_team_id = match &team.pk {
        Partition::Team(id) => id.clone(),
        _ => {
            crate::error!("leave_parent_handler: unexpected child pk");
            return Err(SubTeamError::ApplicationStateMismatch.into());
        }
    };
    let parent_pk: Partition = Partition::Team(parent_team_id.clone());

    // 2. Best-effort: confirm SubTeamLink exists — if missing, this is data
    //    drift, but we still want to clear the child's dangling parent_team_id.
    let link_sk = EntityType::SubTeamLink(sub_team_id.clone());
    let maybe_link = SubTeamLink::get(cli, &parent_pk, Some(link_sk))
        .await
        .unwrap_or(None);

    if maybe_link.is_some() {
        detach_sub_team_link(cli, &parent_pk, &team.pk, &sub_team_id).await?;
    } else {
        // Clear the orphan parent_team_id on the child without touching a link.
        let now = crate::common::utils::time::get_now_timestamp_millis();
        Team::updater(&team.pk, EntityType::Team)
            .remove_parent_team_id()
            .with_updated_at(now)
            .execute(cli)
            .await
            .map_err(|e| {
                crate::error!("leave_parent orphan-clear failed: {e}");
                Error::from(SubTeamError::ApplicationStateMismatch)
            })?;
    }

    // 3. Notify former parent admins.
    let former_sub_team_id = sub_team_id.clone();
    let former_sub_team_name = team.display_name.clone();
    let former_parent_team_id = parent_team_id.clone();
    let reason = body.reason.clone();
    let cta_url = build_parent_sub_teams_url(&parent_team_id);
    notify_team_admins(cli, &parent_pk, move || {
        InboxPayload::SubTeamLeftParent {
            former_parent_team_id: former_parent_team_id.clone(),
            former_sub_team_id: former_sub_team_id.clone(),
            former_sub_team_name: former_sub_team_name.clone(),
            reason: reason.clone(),
            cta_url: cta_url.clone(),
        }
    })
    .await;

    Ok(TerminationAck { ok: true })
}

// Silence unused-import warnings when body types live only here.
#[cfg(feature = "server")]
fn _unused_silencer() {
    let _ = DeregisterRequest::default();
    let _ = LeaveParentRequest::default();
    let _ = TerminationAck::default();
}
