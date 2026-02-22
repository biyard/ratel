use crate::types::*;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct PostArtwork {
    pub pk: Partition,
    pub sk: EntityType,

    pub metadata: Vec<PostArtworkMetadata>,
}

#[cfg(feature = "server")]
impl PostArtwork {
    pub fn new(pk: Partition, metadata: Vec<PostArtworkMetadata>) -> Self {
        Self {
            pk,
            sk: EntityType::PostArtwork,
            metadata,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct PostArtworkMetadata {
    pub trait_type: String,
    pub value: String,
    pub display_type: Option<String>,
}
