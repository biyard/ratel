use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct UserRelationship {
    pub pk: Partition,
    #[dynamo(name = "find_by_relationship", index = "gsi1", pk)]
    pub sk: EntityType,

    #[dynamo(prefix = "REL", index = "gsi1", sk)]
    pub relationship: Relationship,
}

impl UserRelationship {
    pub fn new(pk: Partition, user: Partition, relationship: Relationship) -> Self {
        Self {
            pk,
            sk: EntityType::UserRelationship(user.to_string()),
            relationship,
        }
    }
}
