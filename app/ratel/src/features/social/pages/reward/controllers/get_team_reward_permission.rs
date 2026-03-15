use super::super::{dto::TeamRewardPermission, *};

use crate::features::posts::models::Team;
use crate::features::posts::types::TeamGroupPermissions;

#[get("/api/teams/:username/rewards/permission", user: crate::features::auth::OptionalUser, team: Team, permissions: TeamGroupPermissions)]
pub async fn get_team_reward_permission_handler(username: String) -> Result<TeamRewardPermission> {
    Ok(TeamRewardPermission {
        team_pk: team.pk.into(),
        permissions: permissions.into(),
        team_name: team.display_name,
    })
}
