use std::collections::HashSet;

use crate::config;
use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::{SpaceDao, SpaceDaoToken, SpaceDaoTokenCursor};
use crate::types::{EntityType, Permissions, TeamGroupPermission};
use crate::utils::evm_token::{fetch_token_state, fetch_transfer_logs, format_addr, parse_address};
use crate::{AppState, Error};
use aide::NoApi;
use axum::Json;
use axum::extract::{Path, State};
use bdk::prelude::*;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{Address, U64, U256};

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct RefreshSpaceDaoTokensResponse {
    pub updated: i64,
    pub last_block: i64,
}

pub async fn refresh_space_dao_tokens_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> Result<Json<RefreshSpaceDaoTokensResponse>, Error> {
    permissions.permitted(TeamGroupPermission::SpaceRead)?;

    let dao = SpaceDao::get(&dynamo.client, &space_pk, Some(EntityType::SpaceDao)).await?;
    let Some(dao) = dao else {
        return Err(Error::BadRequest("space dao not found".to_string()));
    };

    let dao_addr = parse_address(&dao.contract_address)?;

    let conf = config::get();
    let provider = Provider::<Http>::try_from(conf.kaia.archive_endpoint).map_err(|err| {
        Error::InternalServerError(format!("archive provider init failed: {err:?}"))
    })?;

    let start_block = if dao.deploy_block > 0 {
        dao.deploy_block as u64
    } else {
        0
    };

    let mut last_block = match get_cursor(&dynamo.client, dao_addr).await? {
        Some(v) => v,
        None => {
            set_cursor(&dynamo.client, dao_addr, U64::from(start_block)).await?;
            U64::from(start_block)
        }
    };

    if last_block.is_zero() {
        return Ok(Json(RefreshSpaceDaoTokensResponse {
            updated: 0,
            last_block: 0,
        }));
    }

    let latest = provider
        .get_block_number()
        .await
        .map_err(|err| Error::InternalServerError(format!("archive get block failed: {err:?}")))?;

    let mut updated = 0;

    if last_block < latest {
        let logs = fetch_transfer_logs(&provider, dao_addr, last_block + 1, latest).await?;
        let mut token_set = load_existing_tokens(&dynamo.client, dao_addr).await?;
        for log in logs {
            token_set.insert(log.address);
        }

        for token in token_set {
            let (symbol, decimals, balance) = fetch_token_state(&provider, token, dao_addr).await;
            upsert_token_balance(&dynamo.client, dao_addr, token, &symbol, decimals, balance)
                .await?;
            updated += 1;
        }

        set_cursor(&dynamo.client, dao_addr, latest).await?;
        last_block = latest;
    }

    Ok(Json(RefreshSpaceDaoTokensResponse {
        updated,
        last_block: last_block.as_u64() as i64,
    }))
}

async fn load_existing_tokens(
    cli: &aws_sdk_dynamodb::Client,
    dao_addr: Address,
) -> Result<HashSet<Address>, Error> {
    let mut token_set = HashSet::new();
    let dao_key = format_addr(dao_addr);
    let (items, _) =
        SpaceDaoToken::find_by_dao_address(cli, &dao_key, SpaceDaoToken::opt_all()).await?;
    for item in items {
        if let Ok(parsed) = item.token_address.parse::<Address>() {
            token_set.insert(parsed);
        }
    }

    Ok(token_set)
}

async fn upsert_token_balance(
    cli: &aws_sdk_dynamodb::Client,
    dao_addr: Address,
    token: Address,
    symbol: &str,
    decimals: u8,
    balance: U256,
) -> Result<(), Error> {
    let now = chrono::Utc::now().timestamp_millis();
    let item = SpaceDaoToken::new(
        format_addr(dao_addr),
        format_addr(token),
        symbol.to_string(),
        decimals as i64,
        balance.to_string(),
        now,
    );

    item.upsert(cli).await?;

    Ok(())
}

async fn get_cursor(
    cli: &aws_sdk_dynamodb::Client,
    dao_addr: Address,
) -> Result<Option<U64>, Error> {
    let cursor = SpaceDaoTokenCursor::get_by_dao(cli, format_addr(dao_addr)).await?;
    Ok(cursor.map(|c| U64::from(c.last_block as u64)))
}

async fn set_cursor(
    cli: &aws_sdk_dynamodb::Client,
    dao_addr: Address,
    block: U64,
) -> Result<(), Error> {
    let cursor = SpaceDaoTokenCursor::new(format_addr(dao_addr), block.as_u64() as i64);
    cursor.upsert(cli).await?;
    Ok(())
}
