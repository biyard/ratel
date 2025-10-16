use crate::models::{
    dynamo_tables::main::user::user_team_group::{UserTeamGroup, UserTeamGroupQueryOption},
    team::{Team, TeamGroup, TeamMetadata, TeamOwner},
    user::User,
};
use crate::types::EntityType;
use crate::{AppState, Error2};
use bdk::prelude::*;
use by_axum::{
    aide::NoApi,
    axum::{
        Json,
        extract::{Path, State},
    },
};
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

#[derive(Debug, Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
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

    // Get all team groups for name mapping using TeamMetadata
    let metadata_results = TeamMetadata::query(&dynamo.client, team_pk.clone())
        .await
        .unwrap_or_else(|_| Vec::new());

    // Extract only TeamGroup entries from metadata
    let team_groups: Vec<TeamGroup> = metadata_results
        .into_iter()
        .filter_map(|m| match m {
            TeamMetadata::TeamGroup(group) => Some(group),
            _ => None,
        })
        .collect();

    // Create map using the TeamGroup SK directly (EntityType enum)
    // This will match against the inner string from UserTeamGroup
    let group_map: HashMap<String, TeamGroup> = team_groups
        .into_iter()
        .map(|group| {
            // Extract the inner UUID from EntityType::TeamGroup(uuid) and format as TEAM_GROUP#uuid
            let key = match &group.sk {
                EntityType::TeamGroup(uuid) => format!("TEAM_GROUP#{}", uuid),
                _ => group.sk.to_string(),
            };
            (key, group)
        })
        .collect();

    // Group members by user
    let mut members_map: HashMap<String, TeamMember> = HashMap::new();

    for utg in all_user_team_groups {
        let user_pk = utg.pk.clone();
        let user_pk_str = user_pk.to_string();

        // Extract the actual group SK from UserTeamGroup SK
        // UserTeamGroup.sk is EntityType::UserTeamGroup("TEAM_GROUP#{uuid}")
        // We need to extract "TEAM_GROUP#{uuid}" and find the matching TeamGroup
        let group_sk_string = if let EntityType::UserTeamGroup(inner) = &utg.sk {
            inner.clone()
        } else {
            continue; // Skip if not the expected format
        };

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
            // Look up by the full TEAM_GROUP#{uuid} string
            if let Some(group) = group_map.get(&group_sk_string) {
                // Extract just the UUID for the group_id
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

    Ok(Json(ListMembersResponse {
        members,
        total_count,
    }))
}
