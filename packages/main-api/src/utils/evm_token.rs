use std::collections::HashSet;
use std::sync::Arc;

use crate::Error;
use ethers::contract::abigen;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{Address, BlockNumber, Filter, H256, Log, U64, U256};

abigen!(
    ERC20Minimal,
    r#"[
        function symbol() view returns (string)
        function decimals() view returns (uint8)
        function balanceOf(address) view returns (uint256)
    ]"#,
);

pub fn parse_address(value: &str) -> Result<Address, Error> {
    value.parse::<Address>().map_err(|_| Error::InvalidResource)
}

pub fn format_addr(addr: Address) -> String {
    format!("{:#x}", addr)
}

pub async fn fetch_transfer_logs(
    provider: &Provider<Http>,
    contract_addr: Address,
    from: U64,
    to: U64,
) -> Result<Vec<Log>, Error> {
    let transfer_sig =
        H256::from_slice(ethers::utils::keccak256("Transfer(address,address,uint256)").as_slice());

    let from_block = BlockNumber::Number(from);
    let to_block = BlockNumber::Number(to);
    let topic_addr = H256::from(contract_addr);

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
        tracing::error!("archive get_logs failed: {err:?}");
        Error::InternalServerError("archive get_logs failed".to_string())
    })?;
    let mut logs_to = provider.get_logs(&filter_to).await.map_err(|err| {
        tracing::error!("archive get_logs failed: {err:?}");
        Error::InternalServerError("archive get_logs failed".to_string())
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

pub async fn fetch_token_state(
    provider: &Provider<Http>,
    token: Address,
    contract_addr: Address,
) -> (String, u8, U256) {
    let contract = ERC20Minimal::new(token, Arc::new(provider.clone()));
    let symbol = contract
        .symbol()
        .call()
        .await
        .unwrap_or_else(|_| format_addr(token));
    let decimals = contract.decimals().call().await.unwrap_or(18);
    let balance = contract
        .balance_of(contract_addr)
        .call()
        .await
        .unwrap_or_default();
    (symbol, decimals, balance)
}
