use super::super::*;

use super::super::dto::TeamMemberPermission;
use crate::features::posts::models::Team;
use crate::features::posts::types::TeamGroupPermissions;

#[get("/api/teams/:username/members/permission", user: crate::features::auth::OptionalUser, team: Team, permissions: TeamGroupPermissions)]
pub async fn get_team_member_permission_handler(username: String) -> Result<TeamMemberPermission> {
    Ok(TeamMemberPermission {
        team_pk: team.pk.into(),
        permissions: permissions.into(),
    })
}
