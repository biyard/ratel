use super::super::*;
use super::dto::TeamResponse;

use crate::features::auth::OptionalUser;
use crate::features::posts::models::Team;
use crate::features::posts::types::TeamGroupPermissions;

#[get("/api/teams/:username/settings", user: OptionalUser)]
pub async fn get_team_settings_handler(username: String) -> Result<TeamResponse> {
    let conf = super::super::config::get();
    let cli = conf.dynamodb();

    let gsi2_sk_prefix = Team::compose_gsi2_sk(String::default());
    let team_query_option = Team::opt().sk(gsi2_sk_prefix);

    let (teams, _) =
        Team::find_by_username_prefix(cli, username.clone(), team_query_option).await?;

    let team = teams
        .into_iter()
        .find(|t| t.username == username)
        .ok_or(Error::NotFound("Team not found".to_string()))?;

    let user: Option<crate::features::auth::User> = user.into();

    let permissions: i64 = if let Some(user) = user {
        Team::get_permissions_by_team_pk(cli, &team.pk, &user.pk)
            .await
            .unwrap_or_else(|_| TeamGroupPermissions::empty())
            .into()
    } else {
        0
    };

    Ok(TeamResponse::from((team, permissions)))
}
