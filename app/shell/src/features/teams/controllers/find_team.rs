use crate::features::teams::controllers::dto::TeamResponse;
use crate::features::teams::*;

use ratel_post::models::Team;
use ratel_post::types::TeamGroupPermissions;

#[get("/api/teams/find", user: OptionalUser)]
pub async fn find_team_handler(username: String) -> Result<TeamResponse> {
    let conf = crate::features::teams::config::get();
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

    let permissions: i64 = if let Some(user) = user {
        let permissions = Team::get_permissions_by_team_pk(cli, &team.pk, &user.pk)
            .await
            .unwrap_or_else(|_| TeamGroupPermissions::empty());
        permissions.into()
    } else {
        0
    };

    Ok(TeamResponse::from((team, permissions)))
}
