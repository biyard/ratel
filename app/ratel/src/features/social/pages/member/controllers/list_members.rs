use super::super::dto::*;
use super::super::*;

use crate::features::posts::models::TeamOwner;
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};
use std::collections::{HashMap, HashSet};

#[get("/api/teams/:team_pk/members?bookmark&limit", user: crate::features::auth::OptionalUser, permissions: TeamGroupPermissions)]
pub async fn list_members_handler(
    team_pk: TeamPartition,
    bookmark: Option<String>,
    limit: Option<i32>,
) -> Result<ListItemsResponse<TeamMemberResponse>> {
    let conf = super::super::config::get();
    let cli = conf.common.dynamodb();
    let team_pk: Partition = team_pk.into();

    let user: Option<crate::features::auth::User> = user.into();
    let Some(_user) = user else {
        return Err(Error::NoPermission);
    };

    // Only team members (users with any permission) can view the member list
    if permissions.0.is_empty() {
        return Err(Error::NoPermission);
    }

    let is_first_page = bookmark.is_none();
    let page_limit = limit.unwrap_or(50).min(100);
    let mut query_options = crate::features::auth::UserTeamGroupQueryOption::builder().limit(page_limit);
    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    let (all_user_team_groups, next_bookmark) =
        crate::features::auth::UserTeamGroup::find_by_team_pk(cli, team_pk.clone(), query_options).await?;

    let user_keys: Vec<_> = {
        let mut seen = HashSet::new();
        all_user_team_groups
            .iter()
            .filter_map(|utg| {
                let key = utg.pk.to_string();
                if seen.insert(key) {
                    Some((utg.pk.clone(), EntityType::User))
                } else {
                    None
                }
            })
            .collect()
    };

    let users = if !user_keys.is_empty() {
        crate::features::auth::User::batch_get(cli, user_keys).await?
    } else {
        Vec::new()
    };

    let user_map: HashMap<String, crate::features::auth::User> =
        users.into_iter().map(|u| (u.pk.to_string(), u)).collect();

    let mut members_map: HashMap<String, TeamMemberResponse> = HashMap::new();

    for utg in all_user_team_groups {
        let user_pk_str = utg.pk.to_string();

        let Some(user) = user_map.get(&user_pk_str) else {
            continue;
        };

        let perms: TeamGroupPermissions = utg.team_group_permissions.into();
        let is_admin = perms.contains(TeamGroupPermission::TeamAdmin);
        let role = if is_admin { TeamRole::Admin } else { TeamRole::Member };

        let entry = members_map
            .entry(user_pk_str.clone())
            .or_insert_with(|| TeamMemberResponse {
                user_id: user_pk_str.clone(),
                username: user.username.clone(),
                display_name: user.display_name.clone(),
                profile_url: user.profile_url.clone(),
                role,
                is_owner: false,
            });

        // Admin wins if user has multiple groups
        if is_admin {
            entry.role = TeamRole::Admin;
        }
    }

    let owner_member = if is_first_page {
        match TeamOwner::get(cli, &team_pk, Some(&EntityType::TeamOwner)).await? {
            Some(team_owner) => {
                let owner_pk_str = team_owner.user_pk.to_string();
                let mut entry = members_map.remove(&owner_pk_str).unwrap_or_else(|| TeamMemberResponse {
                    user_id: owner_pk_str,
                    username: team_owner.username.clone(),
                    display_name: team_owner.display_name.clone(),
                    profile_url: team_owner.profile_url.clone(),
                    role: TeamRole::Admin,
                    is_owner: true,
                });
                entry.is_owner = true;
                Some(entry)
            }
            None => None,
        }
    } else {
        None
    };

    let mut members: Vec<TeamMemberResponse> = members_map.into_values().collect();
    members.sort_by(|a, b| a.username.cmp(&b.username));

    if let Some(owner) = owner_member {
        members.insert(0, owner);
    }

    Ok(ListItemsResponse {
        items: members,
        bookmark: next_bookmark,
    })
}
