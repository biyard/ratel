use crate::models::{
    dynamo_tables::main::user::user_team_group::{UserTeamGroup, UserTeamGroupQueryOption},
    team::{Team, TeamGroup, TeamMetadata, TeamOwner},
    user::{User, UserTeam, UserTeamQueryOption},
};
use crate::types::EntityType;
use crate::{AppState, Error2};
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
    let mut transact_items: Vec<TransactWriteItem> = Vec::new();

    // Delete all UserTeam entries (this is what makes the team appear in user's side menu)
    let user_team_sk = EntityType::UserTeam(team_pk.to_string());
    let mut bookmark = None::<String>;

    loop {
        let query_opts = if let Some(ref b) = bookmark {
            UserTeamQueryOption::builder().bookmark(b.clone())
        } else {
            Default::default()
        };

        let (user_teams, new_bookmark) =
            UserTeam::find_by_team(&dynamo.client, &user_team_sk, query_opts).await?;

        if user_teams.is_empty() {
            break;
        }

        for user_team in user_teams {
            let delete_tx = UserTeam::delete_transact_write_item(user_team.pk, user_team.sk);
            transact_items.push(delete_tx);
            deleted_count += 1;
        }

        match new_bookmark {
            Some(b) => bookmark = Some(b),
            None => break,
        }
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
        let mut bookmark = None::<String>;

        loop {
            let query_opts = if let Some(ref b) = bookmark {
                UserTeamGroupQueryOption::builder().bookmark(b.clone())
            } else {
                Default::default()
            };

            let (user_team_groups, new_bookmark) =
                UserTeamGroup::find_by_team_pk(&dynamo.client, &team_pk, query_opts).await?;

            if user_team_groups.is_empty() {
                break;
            }

            for utg in user_team_groups {
                // Check if this UserTeamGroup is for the current group
                if let EntityType::UserTeamGroup(utg_group_sk) = &utg.sk {
                    if *utg_group_sk == group_sk_string {
                        let delete_tx = UserTeamGroup::delete_transact_write_item(utg.pk, utg.sk);
                        transact_items.push(delete_tx);
                        deleted_count += 1;
                    }
                }
            }

            match new_bookmark {
                Some(b) => bookmark = Some(b),
                None => break,
            }
        }

        // Delete the group itself
        let delete_tx = TeamGroup::delete_transact_write_item(group.pk.clone(), group.sk.clone());
        transact_items.push(delete_tx);
        deleted_count += 1;
    }

    // Delete any remaining UserTeamGroup relationships (cleanup)
    let mut bookmark = None::<String>;

    loop {
        let query_opts = if let Some(ref b) = bookmark {
            UserTeamGroupQueryOption::builder().bookmark(b.clone())
        } else {
            Default::default()
        };

        let (remaining_user_team_groups, new_bookmark) =
            UserTeamGroup::find_by_team_pk(&dynamo.client, &team_pk, query_opts).await?;

        if remaining_user_team_groups.is_empty() {
            break;
        }

        for utg in remaining_user_team_groups {
            let delete_tx = UserTeamGroup::delete_transact_write_item(utg.pk, utg.sk);
            transact_items.push(delete_tx);
            deleted_count += 1;
        }

        match new_bookmark {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    // Delete team owner
    let delete_tx =
        TeamOwner::delete_transact_write_item(team_owner.pk.clone(), team_owner.sk.clone());
    transact_items.push(delete_tx);
    deleted_count += 1;

    // Delete team itself
    let delete_tx = Team::delete_transact_write_item(team.pk.clone(), team.sk.clone());
    transact_items.push(delete_tx);
    deleted_count += 1;

    // Execute all deletes in a transaction
    // DynamoDB TransactWriteItems has a limit of 100 items per transaction
    // So we need to batch the deletes if there are more than 100
    for chunk in transact_items.chunks(100) {
        dynamo
            .client
            .transact_write_items()
            .set_transact_items(Some(chunk.to_vec()))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to delete team entities: {}", e);
                Error2::InternalServerError("Failed to delete team".into())
            })?;
    }

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
