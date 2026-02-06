use crate::types::*;
use bdk::prelude::*;
use std::collections::HashSet;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    DynamoEntity,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct SpaceIncentiveToken {
    pub pk: Partition,
    pub sk: EntityType,

    pub token_address: String,
    pub symbol: String,
    pub decimals: i64,
    pub balance: String,
    pub updated_at: i64,
}

impl SpaceIncentiveToken {
    pub fn new(
        incentive_address: impl std::fmt::Display,
        token_address: impl std::fmt::Display,
        symbol: String,
        decimals: i64,
        balance: String,
        updated_at: i64,
    ) -> Self {
        let token_address = token_address.to_string();
        Self {
            pk: Partition::SpaceIncentive(incentive_address.to_string().to_lowercase()),
            sk: EntityType::SpaceIncentiveToken(token_address.clone()),
            token_address,
            symbol,
            decimals,
            balance,
            updated_at,
        }
    }

    pub async fn find_by_incentive_address(
        cli: &aws_sdk_dynamodb::Client,
        incentive_address: &str,
        opt: SpaceIncentiveTokenQueryOption,
    ) -> crate::Result<(Vec<Self>, Option<String>)> {
        let opt = opt.sk(EntityType::SpaceIncentiveToken(String::new()).to_string());
        Self::query(
            cli,
            Partition::SpaceIncentive(incentive_address.to_string().to_lowercase()),
            opt,
        )
        .await
    }

    pub async fn list_token_addresses(
        cli: &aws_sdk_dynamodb::Client,
        incentive_address: impl std::fmt::Display,
    ) -> crate::Result<HashSet<String>> {
        let (items, _) =
            Self::find_by_incentive_address(cli, &incentive_address.to_string(), Self::opt_all())
                .await?;
        Ok(items.into_iter().map(|item| item.token_address).collect())
    }

    pub async fn upsert_balance(
        cli: &aws_sdk_dynamodb::Client,
        incentive_address: impl std::fmt::Display,
        token_address: impl std::fmt::Display,
        symbol: String,
        decimals: i64,
        balance: String,
        updated_at: i64,
    ) -> crate::Result<()> {
        let item = Self::new(
            incentive_address,
            token_address,
            symbol,
            decimals,
            balance,
            updated_at,
        );

        item.upsert(cli).await?;
        Ok(())
    }
}
