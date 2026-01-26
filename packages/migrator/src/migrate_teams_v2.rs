#![allow(warnings)]
use bdk::prelude::*;
use main_api::models::Team;

use crate::types::EntityType;

pub async fn migrate_teams_v2(cli: &aws_sdk_dynamodb::Client) {
    let mut total: usize = 0;
    let mut updated: usize = 0;

    let option = Team::opt_all();
    let (teams, next) = match Team::find_by_name_prefix(cli, EntityType::Team, option).await {
        Ok(result) => result,
        Err(err) => {
            tracing::error!("Failed to query teams: {:?}", err);
            return;
        }
    };

    tracing::info!("migrated teams: {:?}", teams);

    for mut team in teams {
        if team.sk != EntityType::Team {
            continue;
        }

        total += 1;

        match team.upsert(cli).await {
            Ok(_) => updated += 1,
            Err(err) => {
                tracing::error!("Failed to upsert team {}: {:?}", team.pk, err);
            }
        }
    }

    tracing::info!(
        "migrate_teams_v2 completed: total={} updated={}",
        total,
        updated
    );
}
