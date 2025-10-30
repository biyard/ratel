use bdk::prelude::*;

use crate::features::spaces::artworks::{SpaceArtworkTrade, TradeType};
use crate::types::{EntityType, Partition};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, aide::OperationIo)]
pub struct SpaceArtworkTradeItem {
    pub pk: Partition,
    pub sk: EntityType,
    pub created_at: i64,
    pub nft_token_id: u64,
    pub from_address: String,
    pub to_address: String,
    pub transaction_hash: String,
    pub trade_type: TradeType,
}

impl From<SpaceArtworkTrade> for SpaceArtworkTradeItem {
    fn from(trade: SpaceArtworkTrade) -> Self {
        SpaceArtworkTradeItem {
            pk: trade.pk,
            sk: trade.sk,
            created_at: trade.created_at,
            nft_token_id: trade.nft_token_id,
            from_address: trade.from_address,
            to_address: trade.to_address,
            transaction_hash: trade.transaction_hash,
            trade_type: trade.trade_type,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, aide::OperationIo)]
pub struct ListSpaceArtworkTradeResponse {
    pub items: Vec<SpaceArtworkTradeItem>,
    pub bookmark: Option<String>,
}
