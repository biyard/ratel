#![allow(warnings)]
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use bdk::prelude::*;
use ethers::contract::abigen;
use ethers::providers::{Middleware, Provider, Ws};
use ethers::types::{Address, BlockNumber, Filter, H256, Log, U64, U256};
use futures::StreamExt;
use futures::stream::select;
use main_api::features::spaces::models::{SpaceDao, SpaceDaoToken, SpaceDaoTokenCursor};

abigen!(
    ERC20Minimal,
    r#"[
        function symbol() view returns (string)
        function decimals() view returns (uint8)
        function balanceOf(address) view returns (uint256)
    ]"#,
);

pub struct DaoTokenIndexConfig {
    pub poll_interval: Duration,
}

pub async fn run_space_dao_token_indexer(
    cli: aws_sdk_dynamodb::Client,
    rpc_url: &str,
    cfg: DaoTokenIndexConfig,
) -> main_api::Result<()> {
    loop {
        let provider = Provider::<Ws>::connect(rpc_url).await.map_err(|err| {
            main_api::Error::InternalServerError(format!("RPC provider init failed: {err:?}"))
        })?;

        if let Err(err) = index_once(&cli, &provider).await {
            tracing::error!("space dao token indexer failed: {:?}", err);
        }

        tokio::time::sleep(cfg.poll_interval).await;
    }
}

async fn index_once(
    cli: &aws_sdk_dynamodb::Client,
    provider: &Provider<Ws>,
) -> main_api::Result<()> {
    let daos = list_space_daos(cli).await?;
    if daos.is_empty() {
        tracing::info!("space dao token indexer: no dao items");
        return Ok(());
    }

    let mut set = tokio::task::JoinSet::new();
    for dao in daos {
        let cli = cli.clone();
        let provider = provider.clone();
        set.spawn(async move { index_dao_stream(&cli, &provider, dao).await });
    }

    while let Some(res) = set.join_next().await {
        match res {
            Ok(Ok(())) => {}
            Ok(Err(err)) => return Err(err),
            Err(err) => {
                return Err(main_api::Error::InternalServerError(format!(
                    "dao index task join failed: {err:?}"
                )));
            }
        }
    }

    Ok(())
}

async fn index_dao_stream(
    cli: &aws_sdk_dynamodb::Client,
    provider: &Provider<Ws>,
    dao: SpaceDao,
) -> main_api::Result<()> {
    let dao_addr = match dao.contract_address.parse::<Address>() {
        Ok(v) => v,
        Err(err) => {
            tracing::warn!("invalid dao address {}: {:?}", dao.contract_address, err);
            return Ok(());
        }
    };

    let start_block = if dao.deploy_block > 0 {
        dao.deploy_block as u64
    } else {
        0
    };

    let mut last_block = match get_cursor(cli, dao_addr).await? {
        Some(v) => v,
        None => {
            set_cursor(cli, dao_addr, U64::from(start_block)).await?;
            U64::from(start_block)
        }
    };

    if last_block.is_zero() {
        tracing::info!(
            "space dao token indexer: skip dao {} (start block=0)",
            dao.contract_address
        );
        return Ok(());
    }

    let latest = provider.get_block_number().await.map_err(|err| {
        main_api::Error::InternalServerError(format!("RPC get block failed: {err:?}"))
    })?;

    if last_block < latest {
        let logs = fetch_transfer_logs(provider, dao_addr, last_block + 1, latest).await?;
        let mut token_set = load_existing_tokens(cli, dao_addr).await?;
        for log in logs {
            token_set.insert(log.address);
        }
        for token in token_set {
            let (symbol, decimals, balance) = fetch_token_state(provider, token, dao_addr).await;
            upsert_token_balance(cli, dao_addr, token, &symbol, decimals, balance).await?;
        }
        set_cursor(cli, dao_addr, latest).await?;
        last_block = latest;
    }

    let transfer_sig =
        H256::from_slice(ethers::utils::keccak256("Transfer(address,address,uint256)").as_slice());
    let topic_addr = H256::from(dao_addr);

    let filter_from = Filter::new().topic0(transfer_sig).topic1(topic_addr);
    let filter_to = Filter::new().topic0(transfer_sig).topic2(topic_addr);

    let s1 = provider.subscribe_logs(&filter_from).await.map_err(|err| {
        main_api::Error::InternalServerError(format!("RPC subscribe_logs failed: {err:?}"))
    })?;
    let s2 = provider.subscribe_logs(&filter_to).await.map_err(|err| {
        main_api::Error::InternalServerError(format!("RPC subscribe_logs failed: {err:?}"))
    })?;

    let mut stream = select(s1, s2);

    while let Some(log) = stream.next().await {
        let token = log.address;
        let (symbol, decimals, balance) = fetch_token_state(provider, token, dao_addr).await;
        upsert_token_balance(cli, dao_addr, token, &symbol, decimals, balance).await?;

        if let Some(bn) = log.block_number {
            let bn_u64 = U64::from(bn.as_u64());
            if bn_u64 > last_block {
                set_cursor(cli, dao_addr, bn_u64).await?;
                last_block = bn_u64;
            }
        }
    }

    Ok(())
}

async fn list_space_daos(cli: &aws_sdk_dynamodb::Client) -> main_api::Result<Vec<SpaceDao>> {
    SpaceDao::list_all(cli).await
}

async fn fetch_transfer_logs(
    provider: &Provider<Ws>,
    dao_addr: Address,
    from: U64,
    to: U64,
) -> main_api::Result<Vec<Log>> {
    let transfer_sig =
        H256::from_slice(ethers::utils::keccak256("Transfer(address,address,uint256)").as_slice());

    let from_block = BlockNumber::Number(from);
    let to_block = BlockNumber::Number(to);
    let topic_addr = H256::from(dao_addr);

    let filter_from = Filter::new()
        .topic0(transfer_sig)
        .topic1(topic_addr)
        .from_block(from_block)
        .to_block(to_block);
    let filter_to = Filter::new()
        .topic0(transfer_sig)
        .topic2(topic_addr)
        .from_block(from_block)
        .to_block(to_block);

    let mut logs = provider.get_logs(&filter_from).await.map_err(|err| {
        main_api::Error::InternalServerError(format!("RPC get_logs failed: {err:?}"))
    })?;
    let mut logs_to = provider.get_logs(&filter_to).await.map_err(|err| {
        main_api::Error::InternalServerError(format!("RPC get_logs failed: {err:?}"))
    })?;
    logs.append(&mut logs_to);

    let mut seen = HashSet::new();
    let mut unique = Vec::with_capacity(logs.len());
    for log in logs {
        let key = (log.transaction_hash, log.log_index);
        if seen.insert(key) {
            unique.push(log);
        }
    }

    Ok(unique)
}

async fn load_existing_tokens(
    cli: &aws_sdk_dynamodb::Client,
    dao_addr: Address,
) -> main_api::Result<HashSet<Address>> {
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

async fn fetch_token_state(
    provider: &Provider<Ws>,
    token: Address,
    dao_addr: Address,
) -> (String, u8, U256) {
    let contract = ERC20Minimal::new(token, Arc::new(provider.clone()));
    let symbol = contract
        .symbol()
        .call()
        .await
        .unwrap_or_else(|_| format_addr(token));
    let decimals = contract.decimals().call().await.unwrap_or(18);
    let balance = contract
        .balance_of(dao_addr)
        .call()
        .await
        .unwrap_or_default();
    (symbol, decimals, balance)
}

async fn upsert_token_balance(
    cli: &aws_sdk_dynamodb::Client,
    dao_addr: Address,
    token: Address,
    symbol: &str,
    decimals: u8,
    balance: U256,
) -> main_api::Result<()> {
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
) -> main_api::Result<Option<U64>> {
    let cursor = SpaceDaoTokenCursor::get_by_dao(cli, format_addr(dao_addr)).await?;
    Ok(cursor.map(|c| U64::from(c.last_block as u64)))
}

async fn set_cursor(
    cli: &aws_sdk_dynamodb::Client,
    dao_addr: Address,
    block: U64,
) -> main_api::Result<()> {
    let cursor = SpaceDaoTokenCursor::new(format_addr(dao_addr), block.as_u64() as i64);
    cursor.upsert(cli).await?;
    Ok(())
}

fn format_addr(addr: Address) -> String {
    format!("{:#x}", addr)
}
