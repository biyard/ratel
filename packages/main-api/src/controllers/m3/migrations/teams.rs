use crate::models::{Team, TeamQueryOption};
use crate::{AppState, Error, types::*};
use axum::{
    Json,
    extract::{Path, State},
};
use bdk::prelude::*;

#[derive(
    Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema, aide::OperationIo,
)]
pub struct TeamMigrationResponse {
    pub total_team_rows: u32,
    pub updated_team_rows: u32,
    pub failed_team_rows: Vec<String>,
}
pub async fn migrate_team_handler(
    State(AppState { dynamo, .. }): State<AppState>,
) -> Result<Json<TeamMigrationResponse>, Error> {
    let cli = &dynamo.client;

    let option = Team::opt_all();
    let (teams, _) = match Team::find_by_name_prefix(cli, EntityType::Team, option).await {
        Ok(result) => result,
        Err(err) => {
            tracing::error!("Failed to query teams: {:?}", err);
            return Err(err.into());
        }
    };
    let total = teams.len() as u32;
    let mut updated = 0;
    let mut failed = Vec::new();

    tracing::info!("Starting migration of {} teams", total);
    let team_rows: Vec<_> = teams.into_iter().filter(|team| team.sk == EntityType::Team).collect();

    for chunk in team_rows.chunks(20) {
        let futures: Vec<_> = chunk.iter().map(|team| {
            let cli = cli.clone();
            let team = team.clone();
            async move {
                team.upsert(&cli).await
            }
        }).collect();

        let results = futures::future::join_all(futures).await;

        for (idx, result) in results.into_iter().enumerate() {
            match result {
                Ok(_) => updated += 1,
                Err(err) => {
                    tracing::error!("Failed to upsert team: {:?}", err);
                    failed.push(chunk[idx].pk.to_string());
                }
            }
        }
    }

    Ok(Json(TeamMigrationResponse {
        total_team_rows: total,
        updated_team_rows: updated,
        failed_team_rows: failed,
    }))
}
