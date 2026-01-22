use crate::models::{
    dynamo_tables::main::user::user_evm_address::UserEvmAddress,
    dynamo_tables::main::user::user_team_group::{UserTeamGroup, UserTeamGroupQueryOption},
    team::{Team, TeamGroup, TeamGroupQueryOption, TeamMetadata, TeamOwner},
    user::User,
};
use crate::types::{EntityType, list_items_response::ListItemsResponse};
use crate::{AppState, Error};
use bdk::prelude::*;
use by_axum::{
    aide::NoApi,
    axum::{
        Json,
        extract::{Path, Query, State},
    },
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct MemberGroup {
    #[schemars(description = "Group ID")]
    pub group_id: String,
    #[schemars(description = "Group name")]
    pub group_name: String,
    #[schemars(description = "Group description")]
    pub description: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct TeamMember {
    #[schemars(description = "User ID")]
    pub user_id: String,
    #[schemars(description = "Username")]
    pub username: String,
    #[schemars(description = "Display name")]
    pub display_name: String,
    #[schemars(description = "Profile URL")]
    pub profile_url: String,
    #[schemars(description = "Groups the user belongs to in this team")]
    pub groups: Vec<MemberGroup>,
    #[schemars(description = "Whether the user is the team owner")]
    pub is_owner: bool,
    #[schemars(description = "User's EVM address if registered")]
    pub evm_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct ListMembersQueryParams {
    #[schemars(description = "Pagination bookmark")]
    pub bookmark: Option<String>,
    #[schemars(description = "Number of items to return (default: 50, max: 100)")]
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct TeamMemberResponse {
    #[schemars(description = "List of team members")]
    pub members: Vec<TeamMember>,
    #[schemars(description = "Total member count")]
    pub total_count: usize,
    #[schemars(description = "Pagination bookmark for next page")]
    pub bookmark: Option<String>,
}

pub async fn list_members_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(team_username): Path<String>,
    Query(ListMembersQueryParams { bookmark, limit }): Query<ListMembersQueryParams>,
) -> Result<Json<ListItemsResponse<TeamMember>>, Error> {
    tracing::info!("list_members_handler called - team_username: {}, bookmark: {:?}, limit: {:?}",
        team_username, bookmark, limit);

    // 1. Check if user is authenticated
    let auth_user = user.ok_or(Error::Unauthorized("Authentication required".into()))?;
    tracing::info!("User authenticated: {}", auth_user.pk);

    // 2. Get team by username
    tracing::info!("Finding team by username: {}", team_username);
    let team_results =
        Team::find_by_username_prefix(&dynamo.client, team_username.clone(), Default::default())
            .await?;

    let team = team_results
        .0
        .into_iter()
        .find(|t| t.username == team_username)
        .ok_or(Error::NotFound("Team not found".into()))?;

    let team_partition = team.pk.clone();
    let team_pk_str = team_partition.to_string();
    tracing::info!("Team found - pk: {}", team_pk_str);

    // 3. Get team owner
    tracing::info!("Getting team owner for team: {}", team_pk_str);
    let team_owner =
        TeamOwner::get(&dynamo.client, &team_pk_str, Some(&EntityType::TeamOwner)).await?;
    let is_auth_user_owner = team_owner
        .as_ref()
        .map(|owner| owner.user_pk == auth_user.pk)
        .unwrap_or(false);
    tracing::info!("Is auth user owner: {}", is_auth_user_owner);

    // 4. Set up pagination and get team members
    let page_limit = limit.unwrap_or(50).min(100);
    let mut query_options = UserTeamGroupQueryOption::builder().limit(page_limit);

    if let Some(bookmark_str) = bookmark {
        query_options = query_options.bookmark(bookmark_str);
    }

    tracing::info!("Querying UserTeamGroup for team: {}", team_partition);
    let (all_user_team_groups, next_bookmark) =
        UserTeamGroup::find_by_team_pk(&dynamo.client, team_partition.clone(), query_options)
            .await?;
    tracing::info!("Found {} user team groups", all_user_team_groups.len());

    // 5. Check permission: authenticated user must be owner or in the member list
    let is_auth_user_member = all_user_team_groups
        .iter()
        .any(|utg| utg.pk == auth_user.pk);
    tracing::info!("Is auth user member: {}", is_auth_user_member);

    if !is_auth_user_owner && !is_auth_user_member {
        tracing::warn!("User {} is not authorized to view members", auth_user.pk);
        return Err(Error::Unauthorized(
            "You must be a member of this team to view its members".into(),
        ));
    }

    // 6. Get all team groups (directly query TeamGroup, not TeamMetadata)
    tracing::info!("Querying TeamGroup for team: {}", team_partition);
    let (team_groups, _) = TeamGroup::query(
        &dynamo.client,
        team_partition.clone(),
        TeamGroupQueryOption::builder()
            .sk(EntityType::TeamGroup(String::default()).to_string())
            .limit(100),
    )
    .await?;
    tracing::info!("Found {} team groups", team_groups.len());

    // Create group map
    let group_map: HashMap<String, TeamGroup> = team_groups
        .into_iter()
        .map(|group| {
            let key = match &group.sk {
                EntityType::TeamGroup(uuid) => format!("TEAM_GROUP#{}", uuid),
                _ => group.sk.to_string(),
            };
            (key, group)
        })
        .collect();

    // 7. Batch get all user information (instead of N queries)
    let user_keys: Vec<_> = all_user_team_groups
        .iter()
        .map(|utg| (utg.pk.clone(), EntityType::User))
        .collect();

    tracing::info!("Batch getting {} users", user_keys.len());
    let users = if !user_keys.is_empty() {
        User::batch_get(&dynamo.client, user_keys).await?
    } else {
        Vec::new()
    };
    tracing::info!("Retrieved {} users", users.len());

    // Create user map for quick lookup
    let user_map: HashMap<String, User> = users
        .into_iter()
        .map(|u| (u.pk.to_string(), u))
        .collect();

    // 8. Batch get all EVM addresses
    let evm_keys: Vec<_> = all_user_team_groups
        .iter()
        .map(|utg| (utg.pk.clone(), EntityType::UserEvmAddress))
        .collect();

    tracing::info!("Batch getting {} EVM addresses", evm_keys.len());
    let evm_addresses = if !evm_keys.is_empty() {
        UserEvmAddress::batch_get(&dynamo.client, evm_keys)
            .await
            .unwrap_or_default()
    } else {
        Vec::new()
    };
    tracing::info!("Retrieved {} EVM addresses", evm_addresses.len());

    // Create EVM address map
    let evm_map: HashMap<String, String> = evm_addresses
        .into_iter()
        .map(|e| (e.pk.to_string(), e.evm_address))
        .collect();

    // 9. Build members map
    tracing::info!("Building members map from {} user team groups", all_user_team_groups.len());
    let mut members_map: HashMap<String, TeamMember> = HashMap::new();

    for utg in all_user_team_groups {
        let user_pk = utg.pk.clone();
        let user_pk_str = user_pk.to_string();

        // Extract group SK
        let group_sk_string = if let EntityType::UserTeamGroup(inner) = &utg.sk {
            inner.clone()
        } else {
            continue;
        };

        // Get user from map
        if let Some(user) = user_map.get(&user_pk_str) {
            let entry = members_map.entry(user_pk_str.clone()).or_insert_with(|| {
                TeamMember {
                    user_id: user_pk_str.clone(),
                    username: user.username.clone(),
                    display_name: user.display_name.clone(),
                    profile_url: user.profile_url.clone(),
                    groups: Vec::new(),
                    is_owner: false,
                    evm_address: evm_map.get(&user_pk_str).cloned(),
                }
            });

            // Add group information
            if let Some(group) = group_map.get(&group_sk_string) {
                let group_id = if let EntityType::TeamGroup(uuid) = &group.sk {
                    uuid.clone()
                } else {
                    group.sk.to_string()
                };

                entry.groups.push(MemberGroup {
                    group_id,
                    group_name: group.name.clone(),
                    description: group.description.clone(),
                });
            }
        }
    }

    // 10. Add team owner if not already included and set owner flags
    if let Some(owner) = team_owner {
        let owner_pk = owner.user_pk.clone();
        let owner_pk_str = owner_pk.to_string();
        tracing::info!("Adding/updating team owner: {}", owner_pk_str);

        // Get or create owner entry (EVM address already fetched in batch)
        let owner_entry = members_map
            .entry(owner_pk_str.clone())
            .or_insert_with(|| TeamMember {
                user_id: owner_pk_str.clone(),
                username: owner.username.clone(),
                display_name: owner.display_name.clone(),
                profile_url: owner.profile_url.clone(),
                groups: Vec::new(),
                is_owner: true,
                evm_address: evm_map.get(&owner_pk_str).cloned(),
            });

        owner_entry.is_owner = true;
    }

    // 11. Sort and return
    let mut members: Vec<TeamMember> = members_map.into_values().collect();
    tracing::info!("Total members before sorting: {}", members.len());

    members.sort_by(|a, b| {
        // Sort by owner first, then by username
        match (a.is_owner, b.is_owner) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.username.cmp(&b.username),
        }
    });

    tracing::info!("Returning {} members with bookmark: {:?}", members.len(), next_bookmark);
    Ok(Json(ListItemsResponse {
        items: members,
        bookmark: next_bookmark,
    }))
}
