use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct SpaceMember {
    pub pk: Partition,
    #[dynamo(prefix = "TS", name = "find_by_info_prefix", index = "gsi1", pk)]
    pub sk: EntityType,

    #[dynamo(prefix = "INFO", index = "gsi1", sk)]
    pub info: String,
}

impl SpaceMember {
    pub fn new(pk: Partition, info: String) -> Self {
        Self {
            pk,
            sk: EntityType::SpaceMember,
            info,
        }
    }
}
