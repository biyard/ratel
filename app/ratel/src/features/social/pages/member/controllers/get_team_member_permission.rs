use super::super::*;

use super::super::dto::{TeamMemberPermission, TeamRole};
use crate::features::posts::models::Team;

#[get("/api/teams/:username/members/permission", user: crate::features::auth::OptionalUser, team: Team)]
pub async fn get_team_member_permission_handler(username: String) -> Result<TeamMemberPermission> {
    let conf = super::super::config::get();
    let cli = conf.common.dynamodb();
    let user: Option<crate::features::auth::User> = user.into();
    let role = match user {
        Some(u) => Team::get_user_role(cli, &team.pk, &u.pk).await?,
        None => TeamRole::Member,
    };
    Ok(TeamMemberPermission {
        team_pk: team.pk.clone().into(),
        permissions: role.to_legacy_permissions(),
        role,
    })
}
