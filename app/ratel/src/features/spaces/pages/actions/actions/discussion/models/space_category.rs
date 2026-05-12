use crate::features::spaces::pages::actions::actions::discussion::*;

use crate::features::spaces::pages::actions::actions::discussion::macros::DynamoEntity;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
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
