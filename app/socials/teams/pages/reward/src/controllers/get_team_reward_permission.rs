use crate::{dto::TeamRewardPermission, *};

use ratel_post::models::Team;
use ratel_post::types::TeamGroupPermissions;

#[get("/api/teams/:teamname/rewards/permission", user: ratel_auth::OptionalUser)]
pub async fn get_team_reward_permission_handler(teamname: String) -> Result<TeamRewardPermission> {
    let conf = crate::config::get();
    let cli = conf.common.dynamodb();

    let gsi2_sk_prefix = Team::compose_gsi2_sk(String::default());
    let team_query_option = Team::opt().sk(gsi2_sk_prefix).limit(1);

    let (teams, _) =
        Team::find_by_username_prefix(cli, teamname.clone(), team_query_option).await?;

    let team = teams
        .into_iter()
        .find(|t| t.username == teamname)
        .ok_or(Error::NotFound("Team not found".to_string()))?;

    let user: Option<ratel_auth::User> = user.into();
    let permissions: i64 = if let Some(user) = user {
        let permissions = Team::get_permissions_by_team_pk(cli, &team.pk, &user.pk)
            .await
            .unwrap_or_else(|_| TeamGroupPermissions::empty());
        permissions.into()
    } else {
        0
    };

    Ok(TeamRewardPermission {
        team_pk: team.pk.into(),
        permissions,
        team_name: team.display_name,
    })
}
