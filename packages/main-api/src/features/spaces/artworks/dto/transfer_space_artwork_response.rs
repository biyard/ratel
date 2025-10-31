use bdk::prelude::*;

use crate::features::spaces::artworks::{SpaceArtwork, SpaceArtworkTrade};
use crate::types::Partition;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, aide::OperationIo)]
pub struct TransferSpaceArtworkResponse {
    pub space_pk: Partition,
    pub nft_token_id: u64,
    pub from_address: String,
    pub to_address: String,
    pub transaction_hash: String,
}

impl From<(SpaceArtwork, SpaceArtworkTrade)> for TransferSpaceArtworkResponse {
    fn from((artwork, trade): (SpaceArtwork, SpaceArtworkTrade)) -> Self {
        TransferSpaceArtworkResponse {
            space_pk: artwork.pk,
            nft_token_id: trade.nft_token_id,
            from_address: trade.from_address,
            to_address: trade.to_address,
            transaction_hash: trade.transaction_hash,
        }
    }
}
