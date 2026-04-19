use super::dto::TeamDraftPermission;
use super::super::*;

use crate::features::posts::models::Team;

#[get("/api/teams/:username/drafts/permission", user: crate::features::auth::OptionalUser, team: Team)]
pub async fn get_team_drafts_permission_handler(username: String) -> Result<TeamDraftPermission> {
    let conf = super::super::config::get();
    let cli = conf.common.dynamodb();
    let user: Option<crate::features::auth::User> = user.into();
    let role = match user {
        Some(u) => Team::get_user_role(cli, &team.pk, &u.pk)
            .await?
            .unwrap_or_default(),
        None => crate::features::social::pages::member::dto::TeamRole::default(),
    };
    Ok(TeamDraftPermission {
        team_pk: team.pk.clone().into(),
        permissions: role.to_legacy_permissions(),
        role,
    })
}
