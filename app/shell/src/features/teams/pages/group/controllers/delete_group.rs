use super::super::dto::DeleteGroupResponse;
use super::super::*;

#[cfg(feature = "server")]
use aws_sdk_dynamodb::types::TransactWriteItem;
use crate::features::posts::models::TeamGroup;
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};

#[delete("/api/teams/:team_pk/groups/:group_sk", user: ratel_auth::User, permissions: TeamGroupPermissions)]
pub async fn delete_group_handler(
    team_pk: TeamPartition,
    group_sk: String,
) -> Result<DeleteGroupResponse> {
    let conf = super::super::config::get();
    let cli = conf.common.dynamodb();
    let team_pk: Partition = team_pk.into();

    let can_edit = permissions.contains(TeamGroupPermission::TeamAdmin)
        || permissions.contains(TeamGroupPermission::TeamEdit)
        || permissions.contains(TeamGroupPermission::GroupEdit);

    if !can_edit {
        return Err(Error::Unauthorized(
            "You don't have permission to delete groups.".to_string(),
        ));
    }

    let mut target_group: Option<TeamGroup> = None;
    let mut bookmark: Option<String> = None;
    loop {
        let mut option = TeamGroup::opt()
            .limit(50)
            .sk(EntityType::TeamGroup(String::default()).to_string());
        if let Some(b) = &bookmark {
            option = option.bookmark(b.clone());
        }
        let (groups, next) = TeamGroup::query(cli, team_pk.clone(), option).await?;
        if let Some(found) = groups.into_iter().find(|group| match &group.sk {
            EntityType::TeamGroup(id) => id == &group_sk,
            _ => false,
        }) {
            target_group = Some(found);
            break;
        }
        if next.is_none() {
            break;
        }
        bookmark = next;
    }

    let target_group = target_group.ok_or(Error::NotFound("Group not found".into()))?;

    let mut removed_members = 0usize;
    let mut bookmark: Option<String> = None;
    let mut transact_items: Vec<TransactWriteItem> = Vec::new();
    loop {
        let mut option = ratel_auth::UserTeamGroupQueryOption::builder().limit(50);
        if let Some(b) = &bookmark {
            option = option.bookmark(b.clone());
        }
        let (user_team_groups, next) =
            ratel_auth::UserTeamGroup::find_by_team_pk(cli, &team_pk, option).await?;

        for utg in user_team_groups {
            if let EntityType::UserTeamGroup(utg_group_sk) = &utg.sk {
                if *utg_group_sk == target_group.sk.to_string() {
                    transact_items.push(ratel_auth::UserTeamGroup::delete_transact_write_item(
                        utg.pk, utg.sk,
                    ));
                    removed_members += 1;
                }
            }
        }

        if next.is_none() {
            break;
        }
        bookmark = next;
    }

    transact_items.push(TeamGroup::delete_transact_write_item(
        target_group.pk,
        target_group.sk,
    ));

    for chunk in transact_items.chunks(25) {
        cli.transact_write_items()
            .set_transact_items(Some(chunk.to_vec()))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to delete group entities: {}", e);
                Error::InternalServerError("Failed to delete group".into())
            })?;
    }

    Ok(DeleteGroupResponse {
        message: "Group has been successfully deleted.".to_string(),
        removed_members,
    })
}
