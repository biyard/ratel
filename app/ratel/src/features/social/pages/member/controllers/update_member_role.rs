use super::super::dto::{TeamMemberResponse, UpdateMemberRoleRequest};
use super::super::*;

use crate::features::posts::models::{Team, TeamOwner};

#[patch("/api/teams/:team_pk/members/role", user: crate::features::auth::User, team: Team, role: crate::features::social::pages::member::dto::TeamRole)]
pub async fn update_member_role_handler(
    team_pk: TeamPartition,
    body: UpdateMemberRoleRequest,
) -> Result<TeamMemberResponse> {
    let conf = super::super::config::get();
    let cli = conf.common.dynamodb();
    let team_pk: Partition = team_pk.into();
    let _ = team;

    if !role.is_admin_or_owner() {
        return Err(Error::NoPermission);
    }

    // Cannot change own role.
    if body.user_pk == user.pk.to_string() {
        return Err(MemberError::CannotChangeOwnRole.into());
    }

    // Cannot change owner's role.
    if let Some(team_owner) = TeamOwner::get(cli, &team_pk, Some(&EntityType::TeamOwner)).await? {
        if team_owner.user_pk.to_string() == body.user_pk {
            return Err(MemberError::CannotChangeOwnerRole.into());
        }
    }

    // Verify target user exists.
    let target_user = crate::features::auth::User::get(cli, &body.user_pk, Some(EntityType::User))
        .await?
        .ok_or(MemberError::UserNotFound)?;

    let user_team_sk = EntityType::UserTeam(team_pk.to_string());
    let existing = crate::features::auth::UserTeam::get(cli, &target_user.pk, Some(&user_team_sk))
        .await?
        .ok_or(MemberError::UserNotFound)?;

    // No-op if role is already what was requested.
    if existing.role == body.role {
        return Ok(TeamMemberResponse {
            user_id: target_user.pk.to_string(),
            username: target_user.username.clone(),
            display_name: target_user.display_name.clone(),
            profile_url: target_user.profile_url.clone(),
            role: body.role,
            is_owner: false,
        });
    }

    crate::features::auth::UserTeam::updater(&existing.pk, &existing.sk)
        .with_role(body.role)
        .execute(cli)
        .await
        .map_err(|e| {
            crate::error!("update_member_role failed: {e}");
            MemberError::RoleChangeFailed
        })?;

    Ok(TeamMemberResponse {
        user_id: target_user.pk.to_string(),
        username: target_user.username.clone(),
        display_name: target_user.display_name.clone(),
        profile_url: target_user.profile_url.clone(),
        role: body.role,
        is_owner: false,
    })
}
