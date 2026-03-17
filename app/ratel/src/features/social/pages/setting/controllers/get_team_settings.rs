use super::dto::TeamResponse;
use super::super::*;

use crate::features::posts::models::Team;
use crate::features::posts::types::TeamGroupPermissions;

#[get("/api/teams/:username/settings", user: crate::features::auth::OptionalUser, team: Team, permissions: TeamGroupPermissions)]
pub async fn get_team_settings_handler(username: String) -> Result<TeamResponse> {
    Ok(TeamResponse::from((team, permissions.into())))
}
