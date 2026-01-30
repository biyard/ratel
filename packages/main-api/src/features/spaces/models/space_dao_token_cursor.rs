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
pub struct SpaceDaoTokenCursor {
    pub pk: String,
    pub sk: String,
    pub last_block: i64,
    pub updated_at: i64,
}

impl SpaceDaoTokenCursor {
    pub fn compose_pk(dao_address: impl std::fmt::Display) -> String {
        format!("SPACE_DAO#{}", dao_address)
    }

    pub fn new(dao_address: impl std::fmt::Display, last_block: i64) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        Self {
            pk: Self::compose_pk(dao_address),
            sk: "CURSOR".to_string(),
            last_block,
            updated_at: now,
        }
    }

    pub async fn get_by_dao(
        cli: &aws_sdk_dynamodb::Client,
        dao_address: impl std::fmt::Display,
    ) -> crate::Result<Option<Self>> {
        Self::get(cli, Self::compose_pk(dao_address), Some("CURSOR")).await
    }
}
