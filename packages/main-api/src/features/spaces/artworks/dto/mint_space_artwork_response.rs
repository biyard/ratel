use bdk::prelude::*;

use crate::features::spaces::artworks::{SpaceArtwork, SpaceArtworkTrade};
use crate::types::Partition;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, aide::OperationIo)]
pub struct MintSpaceArtworkResponse {
    pub space_pk: Partition,
    pub nft_token_id: u64,
    pub contract_address: String,
    pub recipient_address: String,
    pub metadata_uri: String,
    pub transaction_hash: String,
}

impl From<(SpaceArtwork, SpaceArtworkTrade)> for MintSpaceArtworkResponse {
    fn from((artwork, trade): (SpaceArtwork, SpaceArtworkTrade)) -> Self {
        MintSpaceArtworkResponse {
            space_pk: artwork.pk,
            nft_token_id: trade.nft_token_id,
            contract_address: artwork.contract_address,
            recipient_address: trade.to_address,
            metadata_uri: artwork.metadata_uri,
            transaction_hash: trade.transaction_hash,
        }
    }
}
