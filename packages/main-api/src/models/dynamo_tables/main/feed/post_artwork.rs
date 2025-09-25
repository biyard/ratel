use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct PostArtworkMetadata {
    pub pk: Partition,
    pub sk: EntityType,

    pub trait_type: String,
    pub value: String,
    pub display_type: Option<String>,
}

impl PostArtworkMetadata {
    pub fn new(
        pk: Partition,
        trait_type: String,
        value: String,
        display_type: Option<String>,
    ) -> Self {
        Self {
            pk,
            sk: EntityType::PostArtwork(trait_type.clone()),
            trait_type,
            value,
            display_type,
        }
    }
}
