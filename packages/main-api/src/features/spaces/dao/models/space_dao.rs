use crate::types::*;
use crate::utils::time::get_now_timestamp_millis;
use aws_sdk_dynamodb::types::AttributeValue;
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
pub struct SpaceDao {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    pub contract_address: String,
    #[serde(default)]
    pub deploy_block: i64,
    #[serde(default)]
    pub remaining_count: i64,
    #[serde(default)]
    pub total_count: i64,
}

impl SpaceDao {
    pub fn new(space_pk: SpacePartition, contract_address: String, deploy_block: i64) -> Self {
        let now = get_now_timestamp_millis();
        Self {
            pk: space_pk.into(),
            sk: EntityType::SpaceDao,
            created_at: now,
            updated_at: now,
            contract_address,
            deploy_block,
            remaining_count: 0,
            total_count: 0,
        }
    }

    pub fn compose_type_sk(key: impl std::fmt::Display) -> String {
        key.to_string()
    }
}
