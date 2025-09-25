use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default, JsonSchema)]
pub struct PostArtwork {
    pub pk: Partition,
    pub sk: EntityType,

    pub metadata: Vec<PostArtworkMetadata>,
}

impl PostArtwork {
    pub fn new(pk: Partition, metadata: Vec<PostArtworkMetadata>) -> Self {
        Self {
            pk,
            sk: EntityType::PostArtwork,
            metadata,
        }
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, JsonSchema)]
pub struct PostArtworkMetadata {
    pub trait_type: String,
    pub value: serde_json::Value,
    pub display_type: Option<String>,
}
