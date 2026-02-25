use crate::controllers::dto::{RemoveMemberRequest, RemoveMemberResponse};
use crate::*;

use ratel_post::models::{Team, TeamGroup};
use ratel_post::types::{TeamGroupPermission, TeamGroupPermissions};

#[delete("/api/teams/:team_pk/groups/:group_sk/member", user: ratel_auth::User)]
pub async fn remove_member_handler(
    team_pk: Partition,
    group_sk: String,
    body: RemoveMemberRequest,
) -> Result<RemoveMemberResponse> {
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
            "You don't have permission to remove members.".to_string(),
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

    for member in &body.user_pks {
        let user = ratel_auth::User::get(cli, member, Some(EntityType::User)).await?;
        let Some(user) = user else {
            failed_pks.push(member.to_string());
            continue;
        };

        ratel_auth::UserTeamGroup::delete(
            cli,
            &user.pk,
            Some(EntityType::UserTeamGroup(team_group.sk.to_string())),
        )
        .await?;

        let opt = ratel_auth::UserTeamGroup::opt()
            .sk(user.pk.to_string())
            .limit(1);
        let (user_team_groups, _) =
            ratel_auth::UserTeamGroup::find_by_team_pk(cli, team.pk.clone(), opt).await?;
        if user_team_groups.is_empty() {
            ratel_auth::UserTeam::delete(
                cli,
                &user.pk,
                Some(EntityType::UserTeam(team.pk.to_string())),
            )
            .await?;
        }

        success_count += 1;
    }

    if success_count > 0 {
        TeamGroup::updater(team_group.pk, team_group.sk)
            .decrease_members(success_count)
            .execute(cli)
            .await?;
    }

    Ok(RemoveMemberResponse {
        total_removed: success_count,
        failed_pks,
    })
}
