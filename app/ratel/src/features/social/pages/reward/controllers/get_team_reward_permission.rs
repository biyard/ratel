use super::super::{dto::TeamRewardPermission, *};

use crate::features::posts::models::Team;

#[get("/api/teams/:username/rewards/permission", user: crate::features::auth::OptionalUser, team: Team)]
pub async fn get_team_reward_permission_handler(username: String) -> Result<TeamRewardPermission> {
    let conf = super::super::config::get();
    let cli = conf.common.dynamodb();
    let user: Option<crate::features::auth::User> = user.into();
    let role = match user {
        Some(u) => Team::get_user_role(cli, &team.pk, &u.pk).await?,
        None => crate::features::social::pages::member::dto::TeamRole::Member,
    };
    Ok(TeamRewardPermission {
        team_pk: team.pk.into(),
        permissions: role.to_legacy_permissions(),
        team_name: team.display_name,
        role,
    })
}
