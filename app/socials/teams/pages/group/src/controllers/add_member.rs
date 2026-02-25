use crate::controllers::dto::{AddMemberRequest, AddMemberResponse};
use crate::*;

use ratel_post::models::{Team, TeamGroup};
use ratel_post::types::{TeamGroupPermission, TeamGroupPermissions};
use std::collections::HashSet;

#[post("/api/teams/:team_pk/groups/:group_sk/member", user: ratel_auth::User)]
pub async fn add_member_handler(
    team_pk: Partition,
    group_sk: String,
    body: AddMemberRequest,
) -> Result<AddMemberResponse> {
    let conf = crate::config::get();
    let cli = conf.common.dynamodb();

    let permissions = Team::get_permissions_by_team_pk(cli, &team_pk, &user.pk)
        .await
        .unwrap_or_else(|_| TeamGroupPermissions::empty());
    let can_edit = permissions.contains(TeamGroupPermission::TeamAdmin)
        || permissions.contains(TeamGroupPermission::TeamEdit)
        || permissions.contains(TeamGroupPermission::GroupEdit);
    if !can_edit {
        return Err(Error::Unauthorized(
            "You don't have permission to invite members.".to_string(),
        ));
    }

    let team = Team::get(cli, &team_pk, Some(EntityType::Team))
        .await?
        .ok_or(Error::NotFound("Team not found".into()))?;

    let team_group = TeamGroup::get(
        cli,
        &team_pk,
        Some(EntityType::TeamGroup(group_sk.clone())),
    )
    .await?
    .ok_or(Error::NotFound("Team group not found".into()))?;

    let mut success_count = 0;
    let mut failed_pks = vec![];
    let mut seen = HashSet::new();

    for member in &body.user_pks {
        if !seen.insert(member.as_str()) {
            failed_pks.push(member.to_string());
            continue;
        }

        let user = ratel_auth::User::get(cli, member, Some(EntityType::User)).await?;
        let Some(user) = user else {
            failed_pks.push(member.to_string());
            continue;
        };

        let user_team_sk = EntityType::UserTeam(team.pk.to_string());
        let existing_user_team = ratel_auth::UserTeam::get(cli, &user.pk, Some(&user_team_sk))
            .await?;
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

        let user_team_group_sk = EntityType::UserTeamGroup(team_group.sk.to_string());
        let existing_user_team_group =
            ratel_auth::UserTeamGroup::get(cli, &user.pk, Some(&user_team_group_sk)).await?;
        if existing_user_team_group.is_some() {
            continue;
        }

        ratel_auth::UserTeamGroup::new(
            user.pk.clone(),
            team_group.sk.clone(),
            team_group.permissions,
            team.pk.clone(),
        )
        .create(cli)
        .await?;
        success_count += 1;
    }

    if success_count > 0 {
        TeamGroup::updater(team_group.pk, team_group.sk)
            .increase_members(success_count)
            .execute(cli)
            .await?;
    }

    Ok(AddMemberResponse {
        total_added: success_count,
        failed_pks,
    })
}
