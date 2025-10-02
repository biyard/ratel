use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default, JsonSchema)]
pub struct DeliberationSpace {
    pub pk: Partition,
    pub sk: EntityType,
}

impl DeliberationSpace {
    pub fn new() -> Self {
        let uid = uuid::Uuid::new_v4().to_string();

        Self {
            pk: Partition::DeliberationSpace(uid),
            sk: EntityType::Space,
        }
    }
}
