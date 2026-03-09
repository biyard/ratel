use super::dto::TeamDraftPermission;
use super::super::*;

use crate::features::posts::models::Team;
use crate::features::posts::types::TeamGroupPermissions;

#[get("/api/teams/:teamname/drafts/permission", user: ratel_auth::OptionalUser, team: Team, permissions: TeamGroupPermissions)]
pub async fn get_team_drafts_permission_handler(teamname: String) -> Result<TeamDraftPermission> {
    Ok(TeamDraftPermission {
        team_pk: team.pk.into(),
        permissions: permissions.into(),
    })
}
