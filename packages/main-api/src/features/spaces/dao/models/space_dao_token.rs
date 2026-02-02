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
pub struct SpaceDaoToken {
    pub pk: Partition,
    pub sk: EntityType,

    pub token_address: String,
    pub symbol: String,
    pub decimals: i64,
    pub balance: String,
    pub updated_at: i64,
}

impl SpaceDaoToken {
    pub fn new(
        dao_address: impl std::fmt::Display,
        token_address: impl std::fmt::Display,
        symbol: String,
        decimals: i64,
        balance: String,
        updated_at: i64,
    ) -> Self {
        let token_address = token_address.to_string();
        Self {
            pk: Self::compose_pk(dao_address),
            sk: EntityType::SpaceDaoToken(token_address.clone()),
            token_address,
            symbol,
            decimals,
            balance,
            updated_at,
        }
    }

    pub fn compose_pk(dao_address: impl std::fmt::Display) -> Partition {
        Partition::SpaceDao(dao_address.to_string().to_lowercase())
    }

    pub fn compose_sk(key: impl std::fmt::Display) -> String {
        key.to_string()
    }

    pub async fn find_by_dao_address(
        cli: &aws_sdk_dynamodb::Client,
        dao_address: &str,
        opt: SpaceDaoTokenQueryOption,
    ) -> crate::Result<(Vec<Self>, Option<String>)> {
        let opt = opt.sk(EntityType::SpaceDaoToken(String::new()).to_string());
        Self::query(cli, Self::compose_pk(dao_address), opt).await
    }

    pub async fn list_token_addresses(
        cli: &aws_sdk_dynamodb::Client,
        dao_address: impl std::fmt::Display,
    ) -> crate::Result<HashSet<String>> {
        let (items, _) =
            Self::find_by_dao_address(cli, &dao_address.to_string(), Self::opt_all()).await?;
        Ok(items.into_iter().map(|item| item.token_address).collect())
    }

    pub async fn upsert_balance(
        cli: &aws_sdk_dynamodb::Client,
        dao_address: impl std::fmt::Display,
        token_address: impl std::fmt::Display,
        symbol: String,
        decimals: i64,
        balance: String,
        updated_at: i64,
    ) -> crate::Result<()> {
        let item = Self::new(
            dao_address,
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
