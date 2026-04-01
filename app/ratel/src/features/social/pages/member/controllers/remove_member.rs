use super::super::dto::{RemoveMemberRequest, RemoveMemberResponse};
use super::super::*;

use crate::features::posts::models::Team;
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};

#[delete("/api/teams/:team_pk/members", user: crate::features::auth::User, team: Team, permissions: TeamGroupPermissions)]
pub async fn remove_team_member_handler(
    team_pk: TeamPartition,
    body: RemoveMemberRequest,
) -> Result<RemoveMemberResponse> {
    let conf = super::super::config::get();
    let cli = conf.common.dynamodb();
    let team_pk: Partition = team_pk.into();

    let can_edit = permissions.contains(TeamGroupPermission::TeamAdmin)
        || permissions.contains(TeamGroupPermission::TeamEdit)
        || permissions.contains(TeamGroupPermission::GroupEdit);
    if !can_edit {
        return Err(Error::NoPermission);
    }

    let mut success_count = 0i64;
    let mut failed_pks = vec![];

    for member_pk in &body.user_pks {
        let user = crate::features::auth::User::get(cli, member_pk, Some(EntityType::User)).await?;
        let Some(user) = user else {
            failed_pks.push(member_pk.to_string());
            continue;
        };

        let opt = crate::features::auth::UserTeamGroupQueryOption::builder()
            .sk(user.pk.to_string());
        let (user_team_groups, _) =
            crate::features::auth::UserTeamGroup::find_by_team_pk(cli, team_pk.clone(), opt)
                .await?;

        for utg in user_team_groups {
            crate::features::auth::UserTeamGroup::delete(cli, &user.pk, Some(utg.sk)).await?;
        }

        crate::features::auth::UserTeam::delete(
            cli,
            &user.pk,
            Some(EntityType::UserTeam(team_pk.to_string())),
        )
        .await?;

        success_count += 1;
    }

    Ok(RemoveMemberResponse {
        total_removed: success_count,
        failed_pks,
    })
}
