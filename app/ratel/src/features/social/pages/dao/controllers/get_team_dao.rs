use super::super::dto::{EligibleAdminResponse, TeamDao, TeamDaoTeamResponse};
use super::super::*;

use crate::features::posts::models::{Team, TeamOwner};
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};
use std::collections::{HashMap, HashSet};

#[get("/api/teams/:username/dao/context", user: crate::features::auth::OptionalUser, team: Team, permissions: TeamGroupPermissions)]
pub async fn get_team_dao_handler(username: String) -> Result<TeamDao> {
    let conf = super::super::config::get();
    let cli = conf.common.dynamodb();

    let is_admin = permissions.contains(TeamGroupPermission::TeamAdmin);
    #[cfg(feature = "server")]
    let eligible_admins = if is_admin {
        list_eligible_admins(cli, &team.pk).await?
    } else {
        Vec::new()
    };
    #[cfg(not(feature = "server"))]
    let eligible_admins = Vec::new();

    Ok(TeamDao {
        team: TeamDaoTeamResponse {
            team_pk: team.pk.into(),
            username: team.username,
            nickname: team.display_name,
            dao_address: team.dao_address,
        },
        permissions: permissions.into(),
        eligible_admins,
    })
}

#[cfg(feature = "server")]
async fn list_eligible_admins(
    cli: &aws_sdk_dynamodb::Client,
    team_pk: &Partition,
) -> Result<Vec<EligibleAdminResponse>> {
    let team_owner = TeamOwner::get(cli, team_pk, Some(&EntityType::TeamOwner)).await?;
    let owner_pk = team_owner.as_ref().map(|owner| owner.user_pk.to_string());

    let mut all_user_team_groups = Vec::new();
    let mut bookmark: Option<String> = None;
    loop {
        let mut query_options = crate::features::auth::UserTeamGroupQueryOption::builder().limit(200);
        if let Some(b) = &bookmark {
            query_options = query_options.bookmark(b.clone());
        }
        let (items, next) =
            crate::features::auth::UserTeamGroup::find_by_team_pk(cli, team_pk.clone(), query_options).await?;
        all_user_team_groups.extend(items);
        if next.is_none() {
            break;
        }
        bookmark = next;
    }

    let mut user_has_group: HashSet<String> = HashSet::new();
    for utg in &all_user_team_groups {
        user_has_group.insert(utg.pk.to_string());
    }

    let mut user_keys: Vec<(Partition, EntityType)> = Vec::new();
    let mut seen = HashSet::new();
    for utg in &all_user_team_groups {
        let key = utg.pk.to_string();
        if seen.insert(key) {
            user_keys.push((utg.pk.clone(), EntityType::User));
        }
    }
    if let Some(owner) = &team_owner {
        let key = owner.user_pk.to_string();
        if seen.insert(key) {
            user_keys.push((owner.user_pk.clone(), EntityType::User));
        }
    }

    let users = if !user_keys.is_empty() {
        crate::features::auth::User::batch_get(cli, user_keys.clone()).await?
    } else {
        Vec::new()
    };

    let evm_keys: Vec<(Partition, EntityType)> = user_keys
        .iter()
        .map(|(pk, _)| (pk.clone(), EntityType::UserEvmAddress))
        .collect();
    let evm_items = if !evm_keys.is_empty() {
        crate::features::auth::UserEvmAddress::batch_get(cli, evm_keys).await?
    } else {
        Vec::new()
    };

    let evm_map: HashMap<String, String> = evm_items
        .into_iter()
        .map(|item| (item.pk.to_string(), item.evm_address))
        .collect();

    let mut eligible_admins: Vec<EligibleAdminResponse> = users
        .into_iter()
        .filter_map(|user| {
            let user_pk = user.pk.to_string();
            let evm_address = evm_map.get(&user_pk)?.clone();
            let is_owner = owner_pk.as_ref().map(|pk| pk == &user_pk).unwrap_or(false);
            let has_group = user_has_group.contains(&user_pk);
            if !(is_owner || has_group) {
                return None;
            }
            let profile_url = if user.profile_url.is_empty() {
                None
            } else {
                Some(user.profile_url)
            };
            Some(EligibleAdminResponse {
                user_id: user_pk,
                username: user.username,
                display_name: user.display_name,
                profile_url,
                is_owner,
                evm_address,
            })
        })
        .collect();

    if let Some(owner) = team_owner {
        let owner_pk_str = owner.user_pk.to_string();
        let exists = eligible_admins
            .iter()
            .any(|item| item.user_id == owner_pk_str);
        if !exists {
            if let Some(evm_address) = evm_map.get(&owner_pk_str).cloned() {
                eligible_admins.push(EligibleAdminResponse {
                    user_id: owner_pk_str,
                    username: owner.username,
                    display_name: owner.display_name,
                    profile_url: if owner.profile_url.is_empty() {
                        None
                    } else {
                        Some(owner.profile_url)
                    },
                    is_owner: true,
                    evm_address,
                });
            }
        }
    }

    eligible_admins.sort_by(|a, b| match (a.is_owner, b.is_owner) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.username.cmp(&b.username),
    });

    Ok(eligible_admins)
}
