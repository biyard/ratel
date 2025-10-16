use crate::models::{
    dynamo_tables::main::user::user_team_group::UserTeamGroup,
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

#[derive(Debug, Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct DeleteGroupResponse {
    #[schemars(description = "Success message")]
    pub message: String,
    #[schemars(description = "Number of member relationships removed")]
    pub removed_members: usize,
}

pub async fn delete_group_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path((team_username, group_id)): Path<(String, String)>,
) -> Result<Json<DeleteGroupResponse>, Error2> {
    tracing::debug!("Deleting group {} from team: {}", group_id, team_username);

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

    // Check if user is the team owner
    let team_owner = TeamOwner::get(&dynamo.client, &team_pk, Some(&EntityType::TeamOwner))
        .await?
        .ok_or(Error2::NotFound("Team owner not found".into()))?;

    if team_owner.user_pk != auth_user.pk {
        return Err(Error2::Unauthorized(
            "Only the team owner can delete groups".into(),
        ));
    }

    // Find the specific group by ID using TeamMetadata::query
    let metadata_results = TeamMetadata::query(&dynamo.client, &team_pk).await?;

    // Filter to get only TeamGroup entries
    let team_groups: Vec<TeamGroup> = metadata_results
        .into_iter()
        .filter_map(|m| match m {
            TeamMetadata::TeamGroup(group) => Some(group),
            _ => None,
        })
        .collect();

    let target_group = team_groups
        .into_iter()
        .find(|group| {
            if let EntityType::TeamGroup(id) = &group.sk {
                *id == group_id
            } else {
                false
            }
        })
        .ok_or(Error2::NotFound("Group not found".into()))?;

    // Delete all UserTeamGroup relationships for this specific group
    // We need to query by team_pk and then filter for the specific group
    let group_sk_string = target_group.sk.to_string();

    let (all_user_team_groups, _) =
        UserTeamGroup::find_by_team_pk(&dynamo.client, &team_pk, Default::default()).await?;

    let mut removed_members = 0;
    for utg in all_user_team_groups {
        // Check if this UserTeamGroup is for the target group
        if let EntityType::UserTeamGroup(utg_group_sk) = &utg.sk {
            if *utg_group_sk == group_sk_string {
                UserTeamGroup::delete(&dynamo.client, utg.pk, Some(utg.sk)).await?;
                removed_members += 1;
            }
        }
    }

    // Delete the group itself
    TeamGroup::delete(&dynamo.client, target_group.pk, Some(target_group.sk)).await?;

    tracing::info!(
        "Successfully deleted group '{}' from team '{}', removed {} member relationships",
        group_id,
        team_username,
        removed_members
    );

    Ok(Json(DeleteGroupResponse {
        message: format!(
            "Group '{}' has been successfully deleted from team '{}'",
            group_id, team_username
        ),
        removed_members,
    }))
}
