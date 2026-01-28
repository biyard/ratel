use crate::types::*;
use crate::utils::time::get_now_timestamp_millis;
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
    pub sampling_count: i64,
    pub reward_amount: i64,
}

impl SpaceDao {
    pub fn new(
        space_pk: SpacePartition,
        contract_address: String,
        sampling_count: i64,
        reward_amount: i64,
    ) -> Self {
        let now = get_now_timestamp_millis();
        Self {
            pk: space_pk.into(),
            sk: EntityType::SpaceDao,
            created_at: now,
            updated_at: now,
            contract_address,
            sampling_count,
            reward_amount,
        }
    }
}
