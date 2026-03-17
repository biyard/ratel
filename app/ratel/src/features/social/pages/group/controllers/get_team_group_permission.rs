use super::super::*;

use crate::features::posts::models::Team;
use crate::features::posts::types::TeamGroupPermissions;

#[get("/api/teams/:username/groups/permission", user: crate::features::auth::OptionalUser, team: Team, permissions: TeamGroupPermissions)]
pub async fn get_team_group_permission_handler(
    username: String,
) -> Result<super::super::dto::TeamGroupPermission> {
    Ok(super::super::dto::TeamGroupPermission {
        team_pk: team.pk.into(),
        permissions: permissions.into(),
    })
}
