use super::super::*;
use super::dto::TeamResponse;

use crate::features::posts::models::Team;
use crate::features::posts::types::TeamGroupPermissions;

#[get("/api/teams/:username/settings", team: Team, permissions: TeamGroupPermissions)]
pub async fn get_team_settings_handler(
    #[allow(unused_variables)] username: String,
) -> Result<TeamResponse> {
    Ok(TeamResponse::from((team, permissions.into())))
}
