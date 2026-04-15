use super::super::dto::{AddTeamMemberRequest, AddTeamMemberResponse};
use super::super::*;

use crate::features::posts::models::Team;
use std::collections::HashSet;

#[post("/api/teams/:team_pk/members", user: crate::features::auth::User, team: Team, role: crate::features::social::pages::member::dto::TeamRole)]
pub async fn add_team_member_handler(
    team_pk: TeamPartition,
    body: AddTeamMemberRequest,
) -> Result<AddTeamMemberResponse> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    if body.user_pks.len() > 100 {
        return Err(MemberError::TooManyInvitations.into());
    }

    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    let user_team_sk = EntityType::UserTeam(team.pk.to_string());

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

    // Batch-fetch existing UserTeam memberships to skip duplicates.
    let membership_keys: Vec<(Partition, EntityType)> = found_users
        .iter()
        .map(|u| (u.pk.clone(), user_team_sk.clone()))
        .collect();
    let existing_memberships =
        crate::features::auth::UserTeam::batch_get(cli, membership_keys).await?;
    let already_member_pks: HashSet<String> = existing_memberships
        .iter()
        .map(|ut| ut.pk.to_string())
        .collect();

    let new_members: Vec<_> = found_users
        .into_iter()
        .filter(|u| !already_member_pks.contains(&u.pk.to_string()))
        .collect();
    let success_count = new_members.len() as i64;

    // Build UserTeam upsert items. upsert (no condition check) avoids
    // ConditionalCheckFailedException from races between our batch-read and
    // this write.
    let transact_items: Vec<aws_sdk_dynamodb::types::TransactWriteItem> = new_members
        .iter()
        .map(|u| {
            let user_team = crate::features::auth::UserTeam::new(
                u.pk.clone(),
                team.pk.clone(),
                team.display_name.clone(),
                team.profile_url.clone(),
                team.username.clone(),
                team.dao_address.clone(),
                body.role,
            );
            user_team.upsert_transact_write_item()
        })
        .collect();

    crate::transact_write_all_items!(cli, transact_items);

    Ok(AddTeamMemberResponse {
        total_added: success_count,
        failed_pks,
    })
}
