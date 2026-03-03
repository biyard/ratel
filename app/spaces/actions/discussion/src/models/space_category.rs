use crate::*;

use crate::macros::DynamoEntity;

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpaceCategory {
    pub pk: Partition,
    pub sk: EntityType,

    pub name: String,
}

#[cfg(feature = "server")]
impl SpaceCategory {
    pub fn new(space_pk: SpacePartition, name: String) -> Self {
        Self {
            pk: space_pk.into(),
            sk: EntityType::SpaceCategory(name.clone()),
            name,
        }
    }
}
