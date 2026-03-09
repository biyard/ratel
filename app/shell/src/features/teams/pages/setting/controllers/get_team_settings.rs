use super::dto::TeamResponse;
use super::super::*;

use ratel_post::models::Team;
use ratel_post::types::TeamGroupPermissions;

#[get("/api/teams/:teamname/settings", user: ratel_auth::OptionalUser, team: Team, permissions: TeamGroupPermissions)]
pub async fn get_team_settings_handler(teamname: String) -> Result<TeamResponse> {
    Ok(TeamResponse::from((team, permissions.into())))
}
