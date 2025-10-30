use crate::*;
use types::*;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default, JsonSchema, OperationIo)]
pub struct TopicDiscussion {
    pub pk: Partition,
    #[dynamo(prefix = "TS", name = "find_by_info_prefix", index = "gsi1", pk)]
    pub sk: EntityType,

    #[dynamo(prefix = "INFO", index = "gsi1", sk)]
    pub info: String,
}

impl TopicDiscussion {
    pub fn new(pk: Partition, info: String) -> Self {
        Self {
            pk,
            sk: EntityType::TopicDiscussion,
            info,
        }
    }
}
