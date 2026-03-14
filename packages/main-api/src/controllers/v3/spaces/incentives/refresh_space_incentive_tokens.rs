use std::collections::HashSet;

use crate::config;
use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::{SpaceIncentive, SpaceIncentiveToken};
use crate::types::{EntityType, Permissions, TeamGroupPermission};
use crate::utils::evm_token::{fetch_token_state, fetch_transfer_logs, format_addr, parse_address};
use crate::{AppState, Error};
use aide::NoApi;
use axum::Json;
use axum::extract::{Path, State};
use bdk::prelude::*;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{Address, U64};

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct RefreshSpaceIncentiveTokensResponse {
    pub updated: i64,
    pub last_block: i64,
}

pub async fn refresh_space_incentive_tokens_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> Result<Json<RefreshSpaceIncentiveTokensResponse>, Error> {
    permissions.permitted(TeamGroupPermission::SpaceRead)?;

    let incentive =
        SpaceIncentive::get(&dynamo.client, &space_pk, Some(EntityType::SpaceIncentive)).await?;
    let Some(incentive) = incentive else {
        return Err(Error::IncentiveNotFound);
    };

    let incentive_addr = parse_address(&incentive.contract_address)?;

    let conf = config::get();
    let provider = Provider::<Http>::try_from(conf.kaia.archive_endpoint).map_err(|err| {
        tracing::error!("archive provider init failed: {err:?}");
        Error::InternalServerError("archive provider init failed".to_string())
    })?;

    let mut last_block = if incentive.last_block > 0 {
        U64::from(incentive.last_block as u64)
    } else if incentive.deploy_block > 0 {
        U64::from(incentive.deploy_block as u64)
    } else {
        U64::from(0)
    };

    if last_block.is_zero() {
        return Ok(Json(RefreshSpaceIncentiveTokensResponse {
            updated: 0,
            last_block: 0,
        }));
    }

    let latest = provider.get_block_number().await.map_err(|err| {
        tracing::error!("archive get block failed: {err:?}");
        Error::InternalServerError("archive get block failed".to_string())
    })?;

    let mut updated = 0;

    if last_block < latest {
        let logs =
            fetch_transfer_logs(&provider, incentive_addr, last_block + 1, latest).await?;
        let mut token_set = load_existing_tokens(&dynamo.client, incentive_addr).await?;
        for log in logs {
            token_set.insert(log.address);
        }

        for token in token_set {
            let (symbol, decimals, balance) =
                fetch_token_state(&provider, token, incentive_addr).await;
            SpaceIncentiveToken::upsert_balance(
                &dynamo.client,
                format_addr(incentive_addr),
                format_addr(token),
                symbol.to_string(),
                decimals as i64,
                balance.to_string(),
                chrono::Utc::now().timestamp_millis(),
            )
            .await?;
            updated += 1;
        }

        let mut updated_incentive = incentive.clone();
        updated_incentive.last_block = latest.as_u64() as i64;
        updated_incentive.updated_at = chrono::Utc::now().timestamp_millis();
        updated_incentive.upsert(&dynamo.client).await?;
        last_block = latest;
    }

    Ok(Json(RefreshSpaceIncentiveTokensResponse {
        updated,
        last_block: last_block.as_u64() as i64,
    }))
}

async fn load_existing_tokens(
    cli: &aws_sdk_dynamodb::Client,
    incentive_addr: Address,
) -> Result<HashSet<Address>, Error> {
    let mut token_set = HashSet::new();
    let items =
        SpaceIncentiveToken::list_token_addresses(cli, format_addr(incentive_addr)).await?;
    for token_address in items {
        if let Ok(parsed) = token_address.parse::<Address>() {
            token_set.insert(parsed);
        }
    }

    Ok(token_set)
}
