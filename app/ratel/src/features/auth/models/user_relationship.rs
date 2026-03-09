use crate::features::auth::*;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    Default,
    EnumProp,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema_repr))]
#[repr(u8)]
pub enum Relationship {
    #[default]
    Following = 1,
    Follower = 2,
    Mutual = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
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
