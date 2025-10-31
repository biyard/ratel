use crate::*;
use types::*;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default, JsonSchema, OperationIo)]
pub struct Topic {
    pub pk: Partition,
    #[dynamo(prefix = "TS", name = "find_by_info_prefix", index = "gsi1", pk)]
    pub sk: EntityType,

    #[dynamo(prefix = "INFO", index = "gsi1", sk)]
    pub name: String,
}

impl Topic {
    pub fn new(pk: Partition, name: String) -> Self {
        match pk {
            Partition::Space(_) => {}
            _ => panic!("Partition must be of type Topic"),
        }

        Self {
            pk,
            sk: EntityType::Topic(name.clone()),
            name,
        }
    }
}
