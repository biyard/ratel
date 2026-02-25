use crate::controllers::dto::{TeamResponse, UpdateTeamRequest};
use crate::*;

use ratel_post::models::Team;
use ratel_post::types::{TeamGroupPermission, TeamGroupPermissions};

#[patch("/api/teams/:teamname/settings", user: ratel_auth::User)]
pub async fn update_team_handler(
    teamname: String,
    body: UpdateTeamRequest,
) -> Result<TeamResponse> {
    let conf = crate::config::get();
    let cli = conf.common.dynamodb();

    let gsi2_sk_prefix = Team::compose_gsi2_sk(String::default());
    let team_query_option = Team::opt().sk(gsi2_sk_prefix).limit(1);

    let (teams, _) =
        Team::find_by_username_prefix(cli, teamname.clone(), team_query_option).await?;

    let mut team = teams
        .into_iter()
        .find(|t| t.username == teamname)
        .ok_or(Error::NotFound("Team not found".to_string()))?;

    let permissions = Team::get_permissions_by_team_pk(cli, &team.pk, &user.pk)
        .await
        .unwrap_or_else(|_| TeamGroupPermissions::empty());
    let can_edit = permissions.contains(TeamGroupPermission::TeamEdit)
        || permissions.contains(TeamGroupPermission::TeamAdmin);
    if !can_edit {
        return Err(Error::Unauthorized(
            "You don't have permission to edit this team.".to_string(),
        ));
    }

    let mut need_update_user_team = false;
    let mut updater = Team::updater(&team.pk, EntityType::Team);

    if let Some(nickname) = body.nickname {
        updater = updater.with_display_name(nickname.clone());
        team.display_name = nickname;
        need_update_user_team = true;
    }
    if let Some(description) = body.description {
        updater = updater.with_description(description.clone());
        team.description = description;
        need_update_user_team = true;
    }
    if let Some(profile_url) = body.profile_url {
        updater = updater.with_profile_url(profile_url.clone());
        team.profile_url = profile_url;
        need_update_user_team = true;
    }
    if let Some(dao_address) = body.dao_address {
        updater = updater.with_dao_address(dao_address.clone());
        team.dao_address = Some(dao_address);
        need_update_user_team = true;
    }

    updater.execute(cli).await?;

    if need_update_user_team {
        let mut bookmark: Option<String> = None;
        let user_team_sk = EntityType::UserTeam(team.pk.to_string());
        loop {
            let mut option = ratel_auth::UserTeamQueryOption::builder();
            if let Some(b) = &bookmark {
                option = option.bookmark(b.clone());
            }
            option = option.limit(50);
            let (user_teams, next) =
                ratel_auth::UserTeam::find_by_team(cli, &user_team_sk, option).await?;
            for user_team in user_teams {
                let mut user_team_updater =
                    ratel_auth::UserTeam::updater(&user_team.pk, &user_team.sk)
                        .with_display_name(team.display_name.clone())
                        .with_profile_url(team.profile_url.clone());
                if let Some(ref dao_address) = team.dao_address {
                    user_team_updater = user_team_updater.with_dao_address(dao_address.clone());
                }
                user_team_updater.execute(cli).await?;
            }
            if next.is_none() {
                break;
            }
            bookmark = next;
        }
    }

    Ok(TeamResponse::from((team, permissions.into())))
}
