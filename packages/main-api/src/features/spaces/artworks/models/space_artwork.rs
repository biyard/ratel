use crate::{types::*, utils::time::get_now_timestamp_millis};
use bdk::prelude::*;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    DynamoEntity,
    Default,
    JsonSchema,
    aide::OperationIo,
)]
#[dynamo(table = "main")]
pub struct SpaceArtwork {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    pub contract_address: String,
    pub metadata_uri: String,
    pub metadata: String,
    pub owner_evm_address: String,
}

impl SpaceArtwork {
    pub fn new(
        space_pk: Partition,
        contract_address: String,
        metadata_uri: String,
        metadata: String,
        owner_evm_address: String,
    ) -> Self {
        let now = get_now_timestamp_millis();
        Self {
            pk: space_pk,
            sk: EntityType::SpaceArtwork,
            created_at: now,
            updated_at: now,
            contract_address,
            metadata_uri,
            metadata,
            owner_evm_address,
        }
    }
}
