use super::super::dto::*;
use super::super::*;

use crate::features::posts::models::{TeamGroup, TeamOwner};
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
    let Some(user) = user else {
        return Err(Error::Unauthorized(
            "You don't have permission to view members.".to_string(),
        ));
    };

    let can_view = permissions.contains(TeamGroupPermission::TeamAdmin)
        || permissions.contains(TeamGroupPermission::TeamEdit)
        || permissions.contains(TeamGroupPermission::GroupEdit);
    if !can_view {
        return Err(Error::Unauthorized(
            "You don't have permission to view members.".to_string(),
        ));
    }

    let team_owner = TeamOwner::get(cli, &team_pk, Some(&EntityType::TeamOwner)).await?;

    let page_limit = limit.unwrap_or(50).min(100);
    let mut query_options = crate::features::auth::UserTeamGroupQueryOption::builder().limit(page_limit);
    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    let (all_user_team_groups, next_bookmark) =
        crate::features::auth::UserTeamGroup::find_by_team_pk(cli, team_pk.clone(), query_options).await?;

    let (team_groups, _) = TeamGroup::query(
        cli,
        team_pk.clone(),
        TeamGroup::opt()
            .limit(100)
            .sk(EntityType::TeamGroup(String::default()).to_string()),
    )
    .await?;

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

        let group_sk_string = if let EntityType::UserTeamGroup(inner) = &utg.sk {
            inner.clone()
        } else {
            continue;
        };

        let Some(user) = user_map.get(&user_pk_str) else {
            continue;
        };

        let entry = members_map
            .entry(user_pk_str.clone())
            .or_insert_with(|| TeamMemberResponse {
                user_id: user_pk_str.clone(),
                username: user.username.clone(),
                display_name: user.display_name.clone(),
                profile_url: user.profile_url.clone(),
                groups: Vec::new(),
                is_owner: false,
            });

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

    if let Some(owner) = team_owner {
        let owner_pk_str = owner.user_pk.to_string();
        let owner_entry =
            members_map
                .entry(owner_pk_str.clone())
                .or_insert_with(|| TeamMemberResponse {
                    user_id: owner_pk_str.clone(),
                    username: owner.username.clone(),
                    display_name: owner.display_name.clone(),
                    profile_url: owner.profile_url.clone(),
                    groups: Vec::new(),
                    is_owner: true,
                });
        owner_entry.is_owner = true;
    }

    let mut members: Vec<TeamMemberResponse> = members_map.into_values().collect();
    members.sort_by(|a, b| match (a.is_owner, b.is_owner) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.username.cmp(&b.username),
    });

    Ok(ListItemsResponse {
        items: members,
        bookmark: next_bookmark,
    })
}
