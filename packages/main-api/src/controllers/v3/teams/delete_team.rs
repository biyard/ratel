use crate::models::{
    dynamo_tables::main::user::user_team_group::UserTeamGroup,
    team::{Team, TeamGroup, TeamMetadata, TeamOwner},
    user::{User, UserTeam},
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

#[derive(Debug, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct DeleteTeamResponse {
    #[schemars(description = "Success message")]
    pub message: String,
    #[schemars(description = "Number of entities deleted")]
    pub deleted_count: usize,
}

pub async fn delete_team_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(team_username): Path<String>,
) -> Result<Json<DeleteTeamResponse>, Error2> {
    tracing::debug!("Deleting team: {}", team_username);

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
            "Only the team owner can delete a team".into(),
        ));
    }

    let mut deleted_count = 0;

    // Delete all UserTeam entries (this is what makes the team appear in user's side menu)
    let user_team_sk = EntityType::UserTeam(team_pk.to_string());
    let (user_teams, _) = UserTeam::find_by_team(&dynamo.client, &user_team_sk, Default::default())
        .await
        .unwrap_or_else(|_| (Vec::new(), None));

    for user_team in user_teams {
        UserTeam::delete(&dynamo.client, user_team.pk, Some(user_team.sk)).await?;
        deleted_count += 1;
    }

    // Delete all team groups and their user relationships
    let metadata_results = TeamMetadata::query(&dynamo.client, &team_pk).await?;

    // Filter to get only TeamGroup entries
    let team_groups: Vec<TeamGroup> = metadata_results
        .into_iter()
        .filter_map(|m| match m {
            TeamMetadata::TeamGroup(group) => Some(group),
            _ => None,
        })
        .collect();

    for group in &team_groups {
        // Delete all UserTeamGroup relationships for this group
        let group_sk_string = group.sk.to_string();
        let (user_team_groups, _) =
            UserTeamGroup::find_by_team_pk(&dynamo.client, &team_pk, Default::default()).await?;

        for utg in user_team_groups {
            // Check if this UserTeamGroup is for the current group
            if let EntityType::UserTeamGroup(utg_group_sk) = &utg.sk {
                if *utg_group_sk == group_sk_string {
                    UserTeamGroup::delete(&dynamo.client, utg.pk, Some(utg.sk)).await?;
                    deleted_count += 1;
                }
            }
        }

        // Delete the group itself
        TeamGroup::delete(&dynamo.client, group.pk.clone(), Some(group.sk.clone())).await?;
        deleted_count += 1;
    }

    // Delete any remaining UserTeamGroup relationships (cleanup)
    let (remaining_user_team_groups, _) =
        UserTeamGroup::find_by_team_pk(&dynamo.client, &team_pk, Default::default()).await?;

    for utg in remaining_user_team_groups {
        UserTeamGroup::delete(&dynamo.client, utg.pk, Some(utg.sk)).await?;
        deleted_count += 1;
    }

    // Delete team owner
    TeamOwner::delete(
        &dynamo.client,
        team_owner.pk.clone(),
        Some(team_owner.sk.clone()),
    )
    .await?;
    deleted_count += 1;

    // Delete team itself
    Team::delete(&dynamo.client, team.pk.clone(), Some(team.sk.clone())).await?;
    deleted_count += 1;

    tracing::info!(
        "Successfully deleted team '{}' and {} related entities",
        team_username,
        deleted_count
    );

    Ok(Json(DeleteTeamResponse {
        message: format!("Team '{}' has been successfully deleted", team_username),
        deleted_count,
    }))
}
