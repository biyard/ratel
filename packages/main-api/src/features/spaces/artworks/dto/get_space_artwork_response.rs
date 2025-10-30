use bdk::prelude::*;

use crate::features::spaces::artworks::SpaceArtwork;
use crate::types::{EntityType, Partition};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, aide::OperationIo)]
pub struct GetSpaceArtworkResponse {
    pub pk: Partition,
    pub sk: EntityType,
    pub created_at: i64,
    pub updated_at: i64,
    pub contract_address: String,
    pub metadata_uri: String,
    pub metadata: String,
}

impl From<SpaceArtwork> for GetSpaceArtworkResponse {
    fn from(artwork: SpaceArtwork) -> Self {
        GetSpaceArtworkResponse {
            pk: artwork.pk,
            sk: artwork.sk,
            created_at: artwork.created_at,
            updated_at: artwork.updated_at,
            contract_address: artwork.contract_address,
            metadata_uri: artwork.metadata_uri,
            metadata: artwork.metadata,
        }
    }
}
