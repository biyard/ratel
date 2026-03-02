#[cfg(feature = "server")]
use std::collections::HashSet;

#[cfg(feature = "server")]
use crate::models::{SpaceIncentive, SpaceIncentiveToken};
use crate::*;
#[cfg(feature = "server")]
use common::utils::time::get_now_timestamp_millis;
#[cfg(feature = "server")]
use common::SpaceUserRole;

#[cfg(feature = "server")]
use crate::utils::{fetch_token_state, fetch_transfer_logs, format_addr, parse_address};
#[cfg(feature = "server")]
use ethers::providers::{Http, Middleware, Provider};
#[cfg(feature = "server")]
use ethers::types::{Address, U64};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RefreshSpaceIncentiveTokensResponse {
    pub updated: i64,
    pub last_block: i64,
}

#[post(
    "/v3/spaces/{space_pk}/incentives/tokens/refresh",
    role: SpaceUserRole
)]
pub async fn refresh_space_incentive_tokens(
    space_pk: SpacePartition,
) -> Result<RefreshSpaceIncentiveTokensResponse> {
    let _ = role;

    #[cfg(not(feature = "server"))]
    {
        let _ = space_pk;
        return Err(Error::NotSupported(
            "Token refresh is only available on server".to_string(),
        ));
    }

    #[cfg(feature = "server")]
    {
        let common_config = common::CommonConfig::default();
        let dynamo = common_config.dynamodb();

        let space_pk: Partition = space_pk.into();
        let incentive =
            SpaceIncentive::get(dynamo, &space_pk, Some(&EntityType::SpaceIncentive)).await?;
        let Some(incentive) = incentive else {
            return Err(Error::NotFound("Space incentive not found".to_string()));
        };

        let incentive_addr = parse_address(&incentive.contract_address)?;

        let archive_endpoint = std::env::var("KAIA_ARCHIVE_ENDPOINT")
            .ok()
            .filter(|v| !v.trim().is_empty())
            .unwrap_or_else(|| "https://archive-en-kairos.node.kaia.io".to_string());

        let provider = Provider::<Http>::try_from(archive_endpoint).map_err(|err| {
            error!("archive provider init failed: {err:?}");
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
            return Ok(RefreshSpaceIncentiveTokensResponse {
                updated: 0,
                last_block: 0,
            });
        }

        let latest = provider.get_block_number().await.map_err(|err| {
            error!("archive get block failed: {err:?}");
            Error::InternalServerError("archive get block failed".to_string())
        })?;

        let mut updated = 0;

        if last_block < latest {
            let logs =
                fetch_transfer_logs(&provider, incentive_addr, last_block + 1, latest).await?;
            let mut token_set = load_existing_tokens(dynamo, incentive_addr).await?;
            for log in logs {
                token_set.insert(log.address);
            }

            for token in token_set {
                let (symbol, decimals, balance) =
                    fetch_token_state(&provider, token, incentive_addr).await;

                SpaceIncentiveToken::upsert_balance(
                    dynamo,
                    format_addr(incentive_addr),
                    format_addr(token),
                    symbol.to_string(),
                    decimals as i64,
                    balance.to_string(),
                    get_now_timestamp_millis(),
                )
                .await?;

                updated += 1;
            }

            let mut updated_incentive = incentive.clone();
            updated_incentive.last_block = latest.as_u64() as i64;
            updated_incentive.updated_at = get_now_timestamp_millis();
            updated_incentive.upsert(dynamo).await?;
            last_block = latest;
        }

        Ok(RefreshSpaceIncentiveTokensResponse {
            updated,
            last_block: last_block.as_u64() as i64,
        })
    }
}

#[cfg(feature = "server")]
async fn load_existing_tokens(
    cli: &aws_sdk_dynamodb::Client,
    incentive_addr: Address,
) -> Result<HashSet<Address>> {
    let mut token_set = HashSet::new();
    let items = SpaceIncentiveToken::list_token_addresses(cli, format_addr(incentive_addr)).await?;

    for token_address in items {
        if let Ok(parsed) = token_address.parse::<Address>() {
            token_set.insert(parsed);
        }
    }

    Ok(token_set)
}
