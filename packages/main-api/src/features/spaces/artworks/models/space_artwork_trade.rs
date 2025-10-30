use crate::{types::*, utils::time::get_now_timestamp_millis};
use bdk::prelude::*;

#[derive(
    Debug, Default, Clone, serde::Serialize, serde::Deserialize, JsonSchema, aide::OperationIo,
)]
pub enum TradeType {
    #[default]
    Mint = 1,
    Transfer = 2,
}

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
pub struct SpaceArtworkTrade {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,

    pub nft_token_id: u64,
    pub from_address: String,
    pub to_address: String,
    pub transaction_hash: String,
    pub trade_type: TradeType,
}

impl SpaceArtworkTrade {
    pub fn new_mint(space_pk: Partition, to_address: String, transaction_hash: String) -> Self {
        let now = get_now_timestamp_millis();
        Self {
            pk: space_pk,
            sk: EntityType::SpaceArtworkTrade(transaction_hash.clone()),
            created_at: now,
            nft_token_id: 1,
            from_address: "0x0".to_string(),
            to_address,
            transaction_hash,
            trade_type: TradeType::Mint,
        }
    }

    pub fn new_transfer(
        space_pk: Partition,
        from_address: String,
        to_address: String,
        transaction_hash: String,
    ) -> Self {
        let now = get_now_timestamp_millis();
        Self {
            pk: space_pk,
            sk: EntityType::SpaceArtworkTrade(transaction_hash.clone()),
            created_at: now,
            nft_token_id: 1,
            from_address,
            to_address,
            transaction_hash,
            trade_type: TradeType::Transfer,
        }
    }
}
