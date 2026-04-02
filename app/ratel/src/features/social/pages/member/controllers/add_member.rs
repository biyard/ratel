use super::super::dto::{AddTeamMemberRequest, AddTeamMemberResponse, TeamRole};
use super::super::*;

use crate::features::posts::models::Team;
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};
use std::collections::HashSet;

#[post("/api/teams/:team_pk/members", user: crate::features::auth::User, team: Team, permissions: TeamGroupPermissions)]
pub async fn add_team_member_handler(
    team_pk: TeamPartition,
    body: AddTeamMemberRequest,
) -> Result<AddTeamMemberResponse> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let _team_pk: Partition = team_pk.into();

    let can_edit = permissions.contains(TeamGroupPermission::TeamAdmin)
        || permissions.contains(TeamGroupPermission::TeamEdit)
        || permissions.contains(TeamGroupPermission::GroupEdit);
    if !can_edit {
        return Err(Error::UnauthorizedAccess);
    }

    let group_permissions: i64 = match body.role {
        TeamRole::Admin => TeamGroupPermissions::all().into(),
        TeamRole::Member => TeamGroupPermissions::member().into(),
    };

    // Deduplicate requested user pks.
    let mut seen = HashSet::new();
    let mut failed_pks = vec![];
    let unique_pks: Vec<String> = body
        .user_pks
        .iter()
        .filter_map(|pk| {
            if seen.insert(pk.clone()) {
                Some(pk.clone())
            } else {
                failed_pks.push(pk.clone());
                None
            }
        })
        .collect();

    if unique_pks.is_empty() {
        return Ok(AddTeamMemberResponse {
            total_added: 0,
            failed_pks,
        });
    }

    // Batch-fetch users to verify they exist.
    let user_keys: Vec<(Partition, EntityType)> = unique_pks
        .iter()
        .filter_map(|pk| pk.parse::<Partition>().ok().map(|p| (p, EntityType::User)))
        .collect();
    let found_users = crate::features::auth::User::batch_get(cli, user_keys).await?;
    let found_user_pks: HashSet<String> = found_users.iter().map(|u| u.pk.to_string()).collect();

    for pk in &unique_pks {
        if !found_user_pks.contains(pk.as_str()) {
            failed_pks.push(pk.clone());
        }
    }

    // Batch-fetch existing UserTeam memberships to skip already-members.
    let membership_keys: Vec<(Partition, EntityType)> = found_users
        .iter()
        .map(|u| (u.pk.clone(), EntityType::UserTeam(team.pk.to_string())))
        .collect();
    let existing_memberships =
        crate::features::auth::UserTeam::batch_get(cli, membership_keys).await?;
    let already_member_pks: HashSet<String> = existing_memberships
        .iter()
        .map(|ut| ut.pk.to_string())
        .collect();

    // Collect users who need to be added.
    let new_members: Vec<_> = found_users
        .into_iter()
        .filter(|u| !already_member_pks.contains(&u.pk.to_string()))
        .collect();

    let team_group_sk = EntityType::TeamGroup(body.role.to_string());
    let success_count = new_members.len() as i64;

    // Build 2 write items per user (UserTeam + UserTeamGroup).
    // upsert (no condition check) avoids ConditionalCheckFailedException from
    // races between our batch-read and this write.
    let transact_items: Vec<aws_sdk_dynamodb::types::TransactWriteItem> = new_members
        .iter()
        .flat_map(|u| {
            let user_team = crate::features::auth::UserTeam::new(
                u.pk.clone(),
                team.pk.clone(),
                team.display_name.clone(),
                team.profile_url.clone(),
                team.username.clone(),
                team.dao_address.clone(),
            );
            let user_team_group = crate::features::auth::UserTeamGroup::new(
                u.pk.clone(),
                team_group_sk.clone(),
                group_permissions,
                team.pk.clone(),
            );
            [
                user_team.upsert_transact_write_item(),
                user_team_group.upsert_transact_write_item(),
            ]
        })
        .collect();

    // transact_write_all_items! auto-chunks into batches of 100 and propagates errors via ?.
    crate::transact_write_all_items!(cli, transact_items);

    Ok(AddTeamMemberResponse {
        total_added: success_count,
        failed_pks,
    })
}
