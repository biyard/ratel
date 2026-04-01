use super::super::dto::{RemoveMemberRequest, RemoveMemberResponse};
use super::super::*;

use crate::features::posts::models::Team;
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};

#[delete("/api/teams/:team_pk/groups/:group_sk/member", user: crate::features::auth::User, team: Team, permissions: TeamGroupPermissions)]
pub async fn remove_member_handler(
    team_pk: TeamPartition,
    group_sk: String,
    body: RemoveMemberRequest,
) -> Result<RemoveMemberResponse> {
    let conf = super::super::config::get();
    let cli = conf.common.dynamodb();
    let team_pk: Partition = team_pk.into();
    let can_edit = permissions.contains(TeamGroupPermission::TeamAdmin)
        || permissions.contains(TeamGroupPermission::TeamEdit)
        || permissions.contains(TeamGroupPermission::GroupEdit);
    if !can_edit {
        return Err(Error::Unauthorized(
            "You don't have permission to remove members.".to_string(),
        ));
    }

    let user_team_group_sk =
        EntityType::UserTeamGroup(EntityType::TeamGroup(group_sk.clone()).to_string());

    let mut success_count = 0;
    let mut failed_pks = vec![];

    for member in &body.user_pks {
        let user = crate::features::auth::User::get(cli, member, Some(EntityType::User)).await?;
        let Some(user) = user else {
            failed_pks.push(member.to_string());
            continue;
        };

        crate::features::auth::UserTeamGroup::delete(
            cli,
            &user.pk,
            Some(user_team_group_sk.clone()),
        )
        .await?;

        let opt = crate::features::auth::UserTeamGroupQueryOption::builder()
            .sk(user.pk.to_string())
            .limit(1);
        let (user_team_groups, _) =
            crate::features::auth::UserTeamGroup::find_by_team_pk(cli, team.pk.clone(), opt).await?;
        if user_team_groups.is_empty() {
            crate::features::auth::UserTeam::delete(
                cli,
                &user.pk,
                Some(EntityType::UserTeam(team.pk.to_string())),
            )
            .await?;
        }

        success_count += 1;
    }

    Ok(RemoveMemberResponse {
        total_removed: success_count,
        failed_pks,
    })
}
