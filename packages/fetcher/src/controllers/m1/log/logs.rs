use bdk::prelude::*;
use by_axum::{
    auth::Authorization,
    axum::{Extension, Json, extract::State, native_routing::post},
};
use dto::*;
use ethers::contract::abigen;
use ethers::types::BlockNumber;
use ethers::types::{H160, U64};
use ethers_core::types::Filter;
use ethers_providers::{Http, Provider};
use std::sync::Arc;

use ethers::abi::RawLog;
use ethers::contract::EthLogDecode;
use ethers::types::H256;
use ethers::utils::keccak256;
use ethers_providers::Middleware;

abigen!(
    NftSpace,
    r#"[event TransferSingle(address indexed operator, address indexed from, address indexed to, uint256 id, uint256 value)]"#,
);

#[derive(Clone, Debug)]
pub struct LogController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl LogController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .native_route("/", post(Self::act_log))
            .with_state(self.clone()))
    }

    pub async fn act_log(
        State(ctrl): State<LogController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<EventLogAction>,
    ) -> Result<Json<EventLog>> {
        tracing::debug!("act_log {:?}", body);

        let res = match body {
            EventLogAction::FetchLogs(param) => ctrl.fetch_logs(param).await?,
        };

        Ok(Json(res))
    }
}

impl LogController {
    async fn fetch_logs(&self, param: EventLogFetchLogsRequest) -> Result<EventLog> {
        let mut tx = self.pool.begin().await.unwrap();
        tracing::info!("Fetching logs with parameters: {:?}", param);

        let contracts = SpaceContract::query_builder()
            .order_by_id_desc()
            .query()
            .map(SpaceContract::from)
            .fetch_all(&mut *tx)
            .await?;

        for contract in contracts {
            let v = EventLog::query_builder()
                .order_by_created_at_desc()
                .contract_address_equals(contract.contract_address.clone())
                .query()
                .map(EventLog::from)
                .fetch_optional(&mut *tx)
                .await?;

            let space_id = contract.space_id;
            let contract_address: H160 =
                contract.contract_address.parse().expect("invalid address");

            let rpc_url = crate::config::get().rpc_endpoint.to_string();

            let provider = Provider::<Http>::try_from(rpc_url)?
                .interval(std::time::Duration::from_millis(500));
            let client = Arc::new(provider);

            let latest_block: U64 = client.get_block_number().await?;

            let start_block_num: U64 = if v.is_some() {
                let v = v
                    .unwrap()
                    .block_number
                    .unwrap_or_default()
                    .to_string()
                    .parse::<u64>()
                    .unwrap_or_default();

                U64::from(v)
            } else {
                if latest_block > U64::from(30000u64) {
                    latest_block - U64::from(30000u64)
                } else {
                    U64::zero()
                }
            };

            let from_block = BlockNumber::Number(start_block_num);
            let to_block = BlockNumber::Number(latest_block);

            let transfer_single_sig: H256 = H256::from(keccak256(
                "TransferSingle(address,address,address,uint256,uint256)",
            ));

            let filter = Filter::new()
                .address(contract_address)
                .from_block(from_block)
                .to_block(to_block)
                .topic0(transfer_single_sig);

            let logs = client.get_logs(&filter).await?;
            tracing::info!("Raw logs count: {}", logs.len());

            for log in logs {
                let raw_log = RawLog {
                    topics: log.topics.clone(),
                    data: log.data.to_vec(),
                };

                match TransferSingleFilter::decode_log(&raw_log) {
                    Ok(parsed) => {
                        let operator: String = format!("{:?}", parsed.operator);
                        let from: String = format!("{:?}", parsed.from);
                        let to: String = format!("{:?}", parsed.to);
                        let id: i64 = parsed.id.to_string().parse::<i64>().unwrap_or_default();
                        let value: i64 =
                            parsed.value.to_string().parse::<i64>().unwrap_or_default();
                        let tx_hash: String = log
                            .transaction_hash
                            .map(|h| format!("{:?}", h))
                            .unwrap_or_default();
                        let block_number = log.block_number.map(|b| b.as_u64() as i64);
                        let contract_address = contract.contract_address.clone();

                        let event = EventLog::query_builder()
                            .tx_hash_equals(tx_hash.clone())
                            .query()
                            .map(EventLog::from)
                            .fetch_optional(&mut *tx)
                            .await?;

                        if event.is_none() {
                            let _ = match EventLog::get_repository(self.pool.clone())
                                .insert_with_tx(
                                    &mut *tx,
                                    tx_hash.clone(),
                                    contract_address.clone(),
                                    operator,
                                    from,
                                    to.clone(),
                                    id,
                                    value,
                                    block_number,
                                )
                                .await
                            {
                                Ok(_) => {
                                    tracing::info!("Inserted new event log: {:?}", tx_hash);
                                }
                                Err(e) => {
                                    tracing::error!(
                                        "Failed to insert event log with error: {:?}",
                                        e
                                    );
                                }
                            };

                            let _ = match SpaceHolder::get_repository(self.pool.clone())
                                .insert_with_tx(&mut *tx, space_id, contract_address, to.clone())
                                .await
                            {
                                Ok(_) => {
                                    tracing::info!(
                                        "Inserted new contract holders: {:?}",
                                        to.clone()
                                    );
                                }
                                Err(e) => {
                                    tracing::error!(
                                        "Failed to insert contract holder with error: {:?}",
                                        e
                                    );
                                }
                            };
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to decode log: {:?}", e);
                    }
                }
            }
        }

        tx.commit().await?;

        Ok(EventLog::default())
    }
}
