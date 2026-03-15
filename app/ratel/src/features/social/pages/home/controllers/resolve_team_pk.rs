use super::super::*;

use crate::features::posts::models::Team;

#[get("/api/teams/:username/pk", team: Team)]
pub async fn resolve_team_pk_handler(username: String) -> Result<String> {
    Ok(team.pk.to_string())
}
