use crate::models::{
    dynamo_tables::main::user::user_team_group::{UserTeamGroup, UserTeamGroupQueryOption},
    team::{Team, TeamGroup, TeamMetadata, TeamOwner},
    user::{User, UserTeam, UserTeamQueryOption},
};
use crate::types::EntityType;
use crate::{AppState, Error};
use aws_sdk_dynamodb::types::TransactWriteItem;
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
pub struct DeleteTeamResponse {
    #[schemars(description = "Success message")]
    pub message: String,
    #[schemars(description = "Number of entities deleted")]
    pub deleted_count: usize,
}

pub async fn delete_team_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(team_username): Path<String>,
) -> Result<Json<DeleteTeamResponse>, Error> {
    tracing::debug!("Deleting team: {}", team_username);

    // Get team by username
    let team_results =
        Team::find_by_username_prefix(&dynamo.client, team_username.clone(), Default::default())
            .await?;

    let team = team_results
        .0
        .into_iter()
        .find(|t| t.username == team_username)
        .ok_or(Error::NotFound("Team not found".into()))?;

    let team_pk = team.pk.clone();

    // Check if user is the team owner
    let team_owner = TeamOwner::get(&dynamo.client, &team_pk, Some(&EntityType::TeamOwner))
        .await?
        .ok_or(Error::NotFound("Team owner not found".into()))?;

    if team_owner.user_pk != user.pk {
        return Err(Error::Unauthorized(
            "Only the team owner can delete a team".into(),
        ));
    }

    let mut transact_items: Vec<TransactWriteItem> = Vec::new();

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
        // Delete the group itself
        let delete_tx = TeamGroup::delete_transact_write_item(group.pk.clone(), group.sk.clone());
        transact_items.push(delete_tx);
    }

    let user_teams: Vec<TransactWriteItem> = UserTeam::find_by_team(
        &dynamo.client,
        EntityType::UserTeam(team_pk.to_string()),
        UserTeam::opt_all(),
    )
    .await?
    .0
    .into_iter()
    .map(|utg| UserTeamGroup::delete_transact_write_item(utg.pk, utg.sk))
    .collect();

    transact_items.extend(user_teams);

    // Delete any remaining UserTeamGroup relationships (cleanup)
    let user_team_groups: Vec<TransactWriteItem> =
        UserTeamGroup::find_by_team_pk(&dynamo.client, &team_pk, UserTeamGroup::opt_all())
            .await?
            .0
            .into_iter()
            .map(|utg| UserTeamGroup::delete_transact_write_item(utg.pk, utg.sk))
            .collect();

    transact_items.extend(user_team_groups);

    // Delete team owner
    let delete_tx =
        TeamOwner::delete_transact_write_item(team_owner.pk.clone(), team_owner.sk.clone());
    transact_items.push(delete_tx);

    // Delete team itself
    let delete_tx = Team::delete_transact_write_item(team.pk.clone(), team.sk.clone());
    transact_items.push(delete_tx);

    // Execute all deletes in a transaction
    // DynamoDB TransactWriteItems has a limit of 100 items per transaction
    // So we need to batch the deletes if there are more than 100
    let deleted_count = transact_items.len();

    for chunk in transact_items.chunks(1) {
        dynamo
            .client
            .transact_write_items()
            .set_transact_items(Some(chunk.to_vec()))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to delete team entities: {}", e);
                Error::InternalServerError("Failed to delete team".into())
            })?;
    }

    tracing::debug!(
        "Successfully deleted team '{}' and {} related entities",
        team_username,
        deleted_count
    );

    Ok(Json(DeleteTeamResponse {
        message: format!("Team '{}' has been successfully deleted", team_username),
        deleted_count,
    }))
}
