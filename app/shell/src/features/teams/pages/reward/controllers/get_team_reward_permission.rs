use super::super::{dto::TeamRewardPermission, *};

use ratel_post::models::Team;
use ratel_post::types::TeamGroupPermissions;

#[get("/api/teams/:teamname/rewards/permission", user: ratel_auth::OptionalUser, team: Team, permissions: TeamGroupPermissions)]
pub async fn get_team_reward_permission_handler(teamname: String) -> Result<TeamRewardPermission> {
    Ok(TeamRewardPermission {
        team_pk: team.pk.into(),
        permissions: permissions.into(),
        team_name: team.display_name,
    })
}
