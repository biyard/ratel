use crate::controllers::dto::UpdateGroupRequest;
use crate::*;

use ratel_post::models::{Team, TeamGroup};
use ratel_post::types::{TeamGroupPermission, TeamGroupPermissions};

#[patch("/api/teams/:team_pk/groups/:group_sk", user: ratel_auth::User)]
pub async fn update_group_handler(
    team_pk: TeamPartition,
    group_sk: String,
    body: UpdateGroupRequest,
) -> Result<()> {
    let conf = crate::config::get();
    let cli = conf.common.dynamodb();
    let team_pk: Partition = team_pk.into();

    let permissions = Team::get_permissions_by_team_pk(cli, &team_pk, &user.pk)
        .await
        .unwrap_or_else(|_| TeamGroupPermissions::empty());

    let require_admin = body.permissions.is_some();
    let can_edit = if require_admin {
        permissions.contains(TeamGroupPermission::TeamAdmin)
    } else {
        permissions.contains(TeamGroupPermission::TeamAdmin)
            || permissions.contains(TeamGroupPermission::TeamEdit)
            || permissions.contains(TeamGroupPermission::GroupEdit)
    };

    if !can_edit {
        return Err(Error::Unauthorized(
            "You don't have permission to update groups.".to_string(),
        ));
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
