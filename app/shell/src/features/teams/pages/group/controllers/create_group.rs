use super::super::dto::{CreateGroupRequest, CreateGroupResponse};
use super::super::*;

use ratel_post::models::{Team, TeamGroup};
use ratel_post::types::{TeamGroupPermission, TeamGroupPermissions};

#[post("/api/teams/:team_pk/groups", user: ratel_auth::User, team: Team, permissions: TeamGroupPermissions)]
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
        return Err(Error::Unauthorized(
            "You don't have permission to create groups.".to_string(),
        ));
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
    let existing_user_team = ratel_auth::UserTeam::get(cli, &user.pk, Some(&user_team_sk)).await?;
    if existing_user_team.is_none() {
        ratel_auth::UserTeam::new(
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

    ratel_auth::UserTeamGroup::new(
        user.pk.clone(),
        group_sk.clone(),
        group.permissions,
        team.pk.clone(),
    )
    .create(cli)
    .await?;

    Ok(CreateGroupResponse { group_pk, group_sk })
}
