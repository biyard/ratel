use crate::features::social::controllers::dto::TeamResponse;
use crate::features::social::*;

use crate::features::auth::*;
use crate::features::posts::models::Team;

#[get("/api/teams/find?username", user: OptionalUser)]
pub async fn find_team_handler(username: String) -> Result<TeamResponse> {
    let conf = crate::features::social::config::get();
    let cli = conf.dynamodb();

    let gsi2_sk_prefix = Team::compose_gsi2_sk(String::default());
    let team_query_option = Team::opt().sk(gsi2_sk_prefix);

    let (teams, _) =
        Team::find_by_username_prefix(cli, username.clone(), team_query_option).await?;

    let team = teams
        .into_iter()
        .find(|team| team.username == username)
        .ok_or(Error::NotFound("Team not found".to_string()))?;

    let user: Option<User> = user.into();

    let role = if let Some(user) = user {
        Team::get_user_role(cli, &team.pk, &user.pk)
            .await
            .unwrap_or(None)
    } else {
        None
    };

    Ok(TeamResponse::from((team, role)))
}
