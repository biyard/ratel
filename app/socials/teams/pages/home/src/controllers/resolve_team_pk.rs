use crate::*;

#[get("/api/teams/:teamname/pk")]
pub async fn resolve_team_pk_handler(teamname: String) -> Result<String> {
    let cli = crate::config::get().dynamodb();

    use ratel_post::models::Team;
    let opt = Team::opt().limit(1);
    let (teams, _): (Vec<Team>, _) =
        Team::find_by_username_prefix(cli, &teamname, opt).await?;

    let team = teams
        .into_iter()
        .next()
        .ok_or(common::Error::NotFound(format!(
            "Team not found: {}",
            teamname
        )))?;

    Ok(team.pk.to_string())
}
