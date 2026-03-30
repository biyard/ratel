use super::super::*;
use super::dto::{TeamResponse, UpdateTeamRequest};

use crate::features::posts::models::Team;
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};

#[patch("/api/teams/:username/settings", team: Team, permissions: TeamGroupPermissions)]
pub async fn update_team_handler(
    #[allow(unused_variables)] username: String,
    body: UpdateTeamRequest,
) -> Result<TeamResponse> {
    let conf = super::super::config::get();
    let cli = conf.common.dynamodb();
    let mut team = team;
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
    if let Some(thumbnail_url) = body.thumbnail_url {
        updater = updater.with_thumbnail_url(thumbnail_url.clone());
        team.thumbnail_url = Some(thumbnail_url);
    }
    if let Some(allow_invite) = body.allow_invite {
        updater = updater.with_allow_invite(allow_invite);
        team.allow_invite = allow_invite;
    }
    if let Some(allow_create_space) = body.allow_create_space {
        updater = updater.with_allow_create_space(allow_create_space);
        team.allow_create_space = allow_create_space;
    }

    updater.execute(cli).await?;

    if need_update_user_team {
        let mut bookmark: Option<String> = None;
        let user_team_sk = EntityType::UserTeam(team.pk.to_string());
        loop {
            let mut option = crate::features::auth::UserTeamQueryOption::builder();
            if let Some(b) = &bookmark {
                option = option.bookmark(b.clone());
            }
            option = option.limit(50);
            let (user_teams, next) =
                crate::features::auth::UserTeam::find_by_team(cli, &user_team_sk, option).await?;
            for user_team in user_teams {
                let mut user_team_updater =
                    crate::features::auth::UserTeam::updater(&user_team.pk, &user_team.sk)
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
