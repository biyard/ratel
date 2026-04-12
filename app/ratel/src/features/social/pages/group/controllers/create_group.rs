use super::super::dto::{CreateGroupRequest, CreateGroupResponse};
use super::super::*;
use crate::features::social::types::SocialError;

use crate::features::posts::models::{Team, TeamGroup};
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};

#[post("/api/teams/:team_pk/groups", user: crate::features::auth::User, team: Team, permissions: TeamGroupPermissions)]
pub async fn create_group_handler(
    team_pk: TeamPartition,
    body: CreateGroupRequest,
) -> Result<CreateGroupResponse> {
    let conf = super::super::config::get();
    let cli = conf.common.dynamodb();

    let need_admin = body
        .permissions
        .iter()
        .any(|p| matches!(p, TeamGroupPermission::TeamAdmin));

    let can_edit = if need_admin {
        permissions.contains(TeamGroupPermission::TeamAdmin)
    } else {
        permissions.contains(TeamGroupPermission::TeamAdmin)
            || permissions.contains(TeamGroupPermission::TeamEdit)
    };

    if !can_edit {
        return Err(SocialError::SessionNotFound.into());
    }

    let group = TeamGroup::new(
        team.pk.clone(),
        body.name,
        body.description,
        TeamGroupPermissions(body.permissions),
    );

    group.create(cli).await?;
    let group_pk = group.pk.clone();
    let group_sk = group.sk.clone();

    let user_team_sk = EntityType::UserTeam(team.pk.to_string());
    let existing_user_team = crate::features::auth::UserTeam::get(cli, &user.pk, Some(&user_team_sk)).await?;
    if existing_user_team.is_none() {
        crate::features::auth::UserTeam::new(
            user.pk.clone(),
            team.pk.clone(),
            team.display_name.clone(),
            team.profile_url.clone(),
            team.username.clone(),
            team.dao_address.clone(),
        )
        .create(cli)
        .await?;
    }

    crate::features::auth::UserTeamGroup::new(
        user.pk.clone(),
        group_sk.clone(),
        group.permissions,
        team.pk.clone(),
    )
    .create(cli)
    .await?;

    Ok(CreateGroupResponse { group_pk, group_sk })
}
