use super::super::dto::{AddTeamMemberRequest, AddTeamMemberResponse};
use super::super::*;

use crate::features::posts::models::Team;
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};
use std::collections::HashSet;

#[post("/api/teams/:team_pk/members", user: crate::features::auth::User, team: Team, permissions: TeamGroupPermissions)]
pub async fn add_team_member_handler(
    team_pk: TeamPartition,
    body: AddTeamMemberRequest,
) -> Result<AddTeamMemberResponse> {
    let conf = super::super::config::get();
    let cli = conf.common.dynamodb();
    let team_pk: Partition = team_pk.into();

    let can_edit = permissions.contains(TeamGroupPermission::TeamAdmin)
        || permissions.contains(TeamGroupPermission::TeamEdit)
        || permissions.contains(TeamGroupPermission::GroupEdit);
    if !can_edit {
        return Err(Error::UnauthorizedAccess);
    }

    let group_permissions: i64 = match body.role.as_str() {
        "admin" => TeamGroupPermissions::all().into(),
        _ => TeamGroupPermissions::member().into(),
    };

    let mut success_count = 0i64;
    let mut failed_pks = vec![];
    let mut seen = HashSet::new();

    for member_pk in &body.user_pks {
        if !seen.insert(member_pk.as_str()) {
            failed_pks.push(member_pk.to_string());
            continue;
        }

        let user = crate::features::auth::User::get(cli, member_pk, Some(EntityType::User)).await?;
        let Some(user) = user else {
            failed_pks.push(member_pk.to_string());
            continue;
        };

        let user_team_sk = EntityType::UserTeam(team.pk.to_string());
        if crate::features::auth::UserTeam::get(cli, &user.pk, Some(&user_team_sk))
            .await?
            .is_none()
        {
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

        let team_group_sk = EntityType::TeamGroup(body.role.clone());
        let user_team_group_sk = EntityType::UserTeamGroup(team_group_sk.to_string());
        if crate::features::auth::UserTeamGroup::get(cli, &user.pk, Some(&user_team_group_sk))
            .await?
            .is_some()
        {
            continue;
        }

        crate::features::auth::UserTeamGroup::new(
            user.pk.clone(),
            team_group_sk,
            group_permissions,
            team.pk.clone(),
        )
        .create(cli)
        .await?;
        success_count += 1;
    }

    Ok(AddTeamMemberResponse {
        total_added: success_count,
        failed_pks,
    })
}
