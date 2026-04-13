use super::super::dto::UpdateGroupRequest;
use super::super::*;
use crate::features::social::types::SocialError;

use crate::features::posts::models::TeamGroup;
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};

#[patch("/api/teams/:team_pk/groups/:group_sk", user: crate::features::auth::User, permissions: TeamGroupPermissions)]
pub async fn update_group_handler(
    team_pk: TeamPartition,
    group_sk: String,
    body: UpdateGroupRequest,
) -> Result<()> {
    let conf = super::super::config::get();
    let cli = conf.common.dynamodb();
    let team_pk: Partition = team_pk.into();

    let require_admin = body.permissions.is_some();
    let can_edit = if require_admin {
        permissions.contains(TeamGroupPermission::TeamAdmin)
    } else {
        permissions.contains(TeamGroupPermission::TeamAdmin)
            || permissions.contains(TeamGroupPermission::TeamEdit)
            || permissions.contains(TeamGroupPermission::GroupEdit)
    };

    if !can_edit {
        return Err(SocialError::SessionNotFound.into());
    }

    let mut updater = TeamGroup::updater(team_pk, EntityType::TeamGroup(group_sk));

    if let Some(name) = body.name {
        updater = updater.with_name(name);
    }
    if let Some(description) = body.description {
        updater = updater.with_description(description);
    }
    if let Some(permissions) = body.permissions {
        updater = updater.with_permissions(TeamGroupPermissions(permissions).into());
    }

    updater.execute(cli).await?;
    Ok(())
}
