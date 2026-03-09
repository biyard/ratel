use super::super::*;

use crate::features::posts::models::Team;

#[get("/api/teams/:teamname/pk", team: Team)]
pub async fn resolve_team_pk_handler(teamname: String) -> Result<String> {
    Ok(team.pk.to_string())
}
