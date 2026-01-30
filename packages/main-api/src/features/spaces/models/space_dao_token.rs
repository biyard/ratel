use crate::types::*;
use bdk::prelude::*;

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
    pub pk: String,
    pub sk: String,

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
            sk: format!("TOKEN#{}", token_address),
            token_address,
            symbol,
            decimals,
            balance,
            updated_at,
        }
    }

    pub fn compose_pk(dao_address: impl std::fmt::Display) -> String {
        format!("SPACE_DAO#{}", dao_address.to_string().to_lowercase())
    }

    pub fn compose_sk(key: impl std::fmt::Display) -> String {
        key.to_string()
    }

    pub async fn find_by_dao_address(
        cli: &aws_sdk_dynamodb::Client,
        dao_address: &str,
        opt: SpaceDaoTokenQueryOption,
    ) -> crate::Result<(Vec<Self>, Option<String>)> {
        let opt = opt.sk("TOKEN#".to_string());
        Self::query(cli, Self::compose_pk(dao_address), opt).await
    }
}
