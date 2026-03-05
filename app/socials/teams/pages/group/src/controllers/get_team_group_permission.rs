use crate::*;

use ratel_post::models::Team;
use ratel_post::types::TeamGroupPermissions;

#[get("/api/teams/:teamname/groups/permission", user: ratel_auth::OptionalUser, team: Team, permissions: TeamGroupPermissions)]
pub async fn get_team_group_permission_handler(
    teamname: String,
) -> Result<crate::dto::TeamGroupPermission> {
    Ok(crate::dto::TeamGroupPermission {
        team_pk: team.pk.into(),
        permissions: permissions.into(),
    })
}
