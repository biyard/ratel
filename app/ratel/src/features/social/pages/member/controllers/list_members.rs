use super::super::dto::*;
use super::super::*;

use crate::features::posts::models::TeamOwner;
use std::collections::HashSet;

#[get("/api/teams/:team_pk/members?bookmark&limit", user: crate::features::auth::OptionalUser)]
pub async fn list_members_handler(
    team_pk: TeamPartition,
    bookmark: Option<String>,
    limit: Option<i32>,
) -> Result<ListItemsResponse<TeamMemberResponse>> {
    let conf = super::super::config::get();
    let cli = conf.common.dynamodb();
    let team_pk: Partition = team_pk.into();
    let _ = user;

    let is_first_page = bookmark.is_none();
    let page_limit = limit.unwrap_or(50).min(100);

    // Query all UserTeam records for this team via GSI.
    let mut opts = crate::features::auth::UserTeamQueryOption::builder().limit(page_limit);
    if let Some(bookmark) = bookmark {
        opts = opts.bookmark(bookmark);
    }
    let user_team_sk_prefix = EntityType::UserTeam(team_pk.to_string());
    let (user_teams, next_bookmark) =
        crate::features::auth::UserTeam::find_by_team(cli, &user_team_sk_prefix, opts).await?;

    // Batch-fetch user records for display fields.
    let user_keys: Vec<_> = {
        let mut seen = HashSet::new();
        user_teams
            .iter()
            .filter_map(|ut| {
                let key = ut.pk.to_string();
                if seen.insert(key) {
                    Some((ut.pk.clone(), EntityType::User))
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

    use std::collections::HashMap;
    let user_map: HashMap<String, crate::features::auth::User> =
        users.into_iter().map(|u| (u.pk.to_string(), u)).collect();

    let mut members: Vec<TeamMemberResponse> = Vec::new();
    for ut in user_teams {
        let user_pk_str = ut.pk.to_string();
        let Some(u) = user_map.get(&user_pk_str) else {
            continue;
        };
        members.push(TeamMemberResponse {
            user_id: user_pk_str,
            username: u.username.clone(),
            display_name: u.display_name.clone(),
            profile_url: u.profile_url.clone(),
            role: ut.role,
            is_owner: matches!(ut.role, TeamRole::Owner),
        });
    }

    // Ensure owner record is on the first page (UserTeam.role may already
    // be Owner but we also sync from TeamOwner as the authoritative source).
    if is_first_page {
        if let Some(team_owner) = TeamOwner::get(cli, &team_pk, Some(&EntityType::TeamOwner)).await? {
            let owner_pk_str = team_owner.user_pk.to_string();
            let existing_idx = members.iter().position(|m| m.user_id == owner_pk_str);
            let entry = TeamMemberResponse {
                user_id: owner_pk_str.clone(),
                username: team_owner.username.clone(),
                display_name: team_owner.display_name.clone(),
                profile_url: team_owner.profile_url.clone(),
                role: TeamRole::Owner,
                is_owner: true,
            };
            if let Some(idx) = existing_idx {
                members.remove(idx);
            }
            members.insert(0, entry);
        }
    }

    Ok(ListItemsResponse {
        items: members,
        bookmark: next_bookmark,
    })
}
