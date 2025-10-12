use crate::models::{
    dynamo_tables::main::user::user_team_group::{UserTeamGroup, UserTeamGroupQueryOption},
    team::{Team, TeamGroup, TeamOwner},
    user::User,
};
use crate::types::EntityType;
use crate::{AppState, Error2};
use bdk::prelude::*;
use dto::by_axum::{
    aide::NoApi,
    axum::{
        Json,
        extract::{Path, State},
    },
};
use dto::{JsonSchema, aide, schemars};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct MemberGroup {
    #[schemars(description = "Group ID")]
    pub group_id: String,
    #[schemars(description = "Group name")]
    pub group_name: String,
    #[schemars(description = "Group description")]
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
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
}

#[derive(Debug, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct ListMembersResponse {
    #[schemars(description = "List of team members")]
    pub members: Vec<TeamMember>,
    #[schemars(description = "Total member count")]
    pub total_count: usize,
}

pub async fn list_members_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(team_username): Path<String>,
) -> Result<Json<ListMembersResponse>, Error2> {
    tracing::debug!("Listing members for team: {}", team_username);

    // Check if user is authenticated
    let auth_user = user.ok_or(Error2::Unauthorized("Authentication required".into()))?;

    // Get team by username
    let team_results =
        Team::find_by_username_prefix(&dynamo.client, team_username.clone(), Default::default())
            .await?;

    let team = team_results
        .0
        .into_iter()
        .find(|t| t.username == team_username)
        .ok_or(Error2::NotFound("Team not found".into()))?;

    let team_pk = team.pk.clone();

    // Check if authenticated user is member or owner
    let team_owner = TeamOwner::get(&dynamo.client, &team_pk, Some(&EntityType::TeamOwner)).await?;
    let is_auth_user_owner = team_owner
        .as_ref()
        .map(|owner| owner.user_pk == auth_user.pk)
        .unwrap_or(false);

    // Check if authenticated user is a team member
    let auth_user_memberships = UserTeamGroup::find_by_team_pk(
        &dynamo.client,
        team_pk.clone(),
        UserTeamGroupQueryOption::builder()
            .sk(auth_user.pk.to_string())
            .limit(1),
    )
    .await?;

    let is_auth_user_member = !auth_user_memberships.0.is_empty();

    if !is_auth_user_owner && !is_auth_user_member {
        return Err(Error2::Unauthorized(
            "You must be a member of this team to view its members".into(),
        ));
    }

    // Get all team members
    let (all_user_team_groups, _) = UserTeamGroup::find_by_team_pk(
        &dynamo.client,
        team_pk.clone(),
        UserTeamGroupQueryOption::builder().limit(1000),
    )
    .await?;

    // Get all team groups for name mapping
    let (team_groups, _) = TeamGroup::query(&dynamo.client, team_pk.clone(), Default::default())
        .await
        .unwrap_or_else(|_| (Vec::new(), None)); // Handle case where no groups exist
    let group_map: HashMap<String, TeamGroup> = team_groups
        .into_iter()
        .map(|group| (group.sk.to_string(), group))
        .collect();

    // Group members by user
    let mut members_map: HashMap<String, TeamMember> = HashMap::new();

    for utg in all_user_team_groups {
        let user_pk = utg.pk.clone();
        let user_pk_str = user_pk.to_string();
        let group_sk = utg.sk.to_string();

        // Get user details
        let user_details = User::get(&dynamo.client, &user_pk, Some(&EntityType::User)).await?;

        if let Some(user) = user_details {
            let entry = members_map.entry(user_pk_str.clone()).or_insert_with(|| {
                TeamMember {
                    user_id: user_pk_str.clone(),
                    username: user.username.clone(),
                    display_name: user.display_name.clone(),
                    profile_url: user.profile_url.clone(),
                    groups: Vec::new(),
                    is_owner: false, // Will be set correctly below
                }
            });

            // Add group information if group exists
            if let Some(group) = group_map.get(&group_sk) {
                entry.groups.push(MemberGroup {
                    group_id: group_sk,
                    group_name: group.name.clone(),
                    description: group.description.clone(),
                });
            }
        }
    }

    // Add team owner if not already included and set owner flags
    if let Some(owner) = team_owner {
        let owner_pk = owner.user_pk.clone();
        let owner_pk_str = owner_pk.to_string();

        // Get or create owner entry
        let owner_entry = members_map
            .entry(owner_pk_str.clone())
            .or_insert_with(|| TeamMember {
                user_id: owner_pk_str.clone(),
                username: owner.username.clone(),
                display_name: owner.display_name.clone(),
                profile_url: owner.profile_url.clone(),
                groups: Vec::new(),
                is_owner: true,
            });

        owner_entry.is_owner = true;
    }

    let mut members: Vec<TeamMember> = members_map.into_values().collect();
    members.sort_by(|a, b| {
        // Sort by owner first, then by username
        match (a.is_owner, b.is_owner) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.username.cmp(&b.username),
        }
    });

    let total_count = members.len();

    tracing::debug!("Found {} members for team {}", total_count, team_username);

    Ok(Json(ListMembersResponse {
        members,
        total_count,
    }))
}
