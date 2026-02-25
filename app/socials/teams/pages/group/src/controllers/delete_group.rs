use crate::controllers::dto::DeleteGroupResponse;
use crate::*;

use ratel_post::models::{Team, TeamGroup, TeamGroupQueryOption};
use ratel_post::types::{TeamGroupPermission, TeamGroupPermissions};

#[delete("/api/teams/:teamname/groups/:group_id", user: ratel_auth::User)]
pub async fn delete_group_handler(
    teamname: String,
    group_id: String,
) -> Result<DeleteGroupResponse> {
    let conf = crate::config::get();
    let cli = conf.common.dynamodb();

    let gsi2_sk_prefix = Team::compose_gsi2_sk(String::default());
    let team_query_option = Team::opt().sk(gsi2_sk_prefix).limit(1);
    let (teams, _) =
        Team::find_by_username_prefix(cli, teamname.clone(), team_query_option).await?;

    let team = teams
        .into_iter()
        .find(|t| t.username == teamname)
        .ok_or(Error::NotFound("Team not found".to_string()))?;

    let permissions = Team::get_permissions_by_team_pk(cli, &team.pk, &user.pk)
        .await
        .unwrap_or_else(|_| TeamGroupPermissions::empty());

    let can_edit = permissions.contains(TeamGroupPermission::TeamAdmin)
        || permissions.contains(TeamGroupPermission::TeamEdit)
        || permissions.contains(TeamGroupPermission::GroupEdit);

    if !can_edit {
        return Err(Error::Unauthorized(
            "You don't have permission to delete groups.".to_string(),
        ));
    }

    let mut target_group: Option<TeamGroup> = None;
    let mut bookmark: Option<String> = None;
    loop {
        let mut option = TeamGroupQueryOption::builder()
            .limit(50)
            .sk(EntityType::TeamGroup(String::default()).to_string());
        if let Some(b) = &bookmark {
            option = option.bookmark(b.clone());
        }
        let (groups, next) = TeamGroup::query(cli, team.pk.clone(), option).await?;
        if let Some(found) = groups.into_iter().find(|group| match &group.sk {
            EntityType::TeamGroup(id) => id == &group_id,
            _ => false,
        }) {
            target_group = Some(found);
            break;
        }
        if next.is_none() {
            break;
        }
        bookmark = next;
    }

    let target_group = target_group.ok_or(Error::NotFound("Group not found".into()))?;

    let mut removed_members = 0usize;
    let mut utg_bookmark: Option<String> = None;
    loop {
        let mut option = ratel_auth::UserTeamGroupQueryOption::builder().limit(50);
        if let Some(b) = &utg_bookmark {
            option = option.bookmark(b.clone());
        }
        let (user_team_groups, next) =
            ratel_auth::UserTeamGroup::find_by_team_pk(cli, &team.pk, option).await?;

        for utg in user_team_groups {
            if let EntityType::UserTeamGroup(utg_group_sk) = &utg.sk {
                if *utg_group_sk == target_group.sk.to_string() {
                    ratel_auth::UserTeamGroup::delete(cli, utg.pk, Some(utg.sk)).await?;
                    removed_members += 1;
                }
            }
        }

        if next.is_none() {
            break;
        }
        utg_bookmark = next;
    }

    TeamGroup::delete(cli, target_group.pk, Some(target_group.sk)).await?;

    Ok(DeleteGroupResponse {
        message: format!(
            "Group '{}' has been successfully deleted from team '{}'",
            group_id, teamname
        ),
        removed_members,
    })
}
