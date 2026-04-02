use crate::features::ai_moderator::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct AiModeratorConfig {
    pub pk: Partition,
    pub sk: EntityType,
    pub enabled: bool,
    pub reply_interval: i64,
    pub guidelines: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[cfg(feature = "server")]
impl AiModeratorConfig {
    pub fn new(space_pk: SpacePartition) -> Self {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        Self {
            pk: space_pk.into(),
            sk: EntityType::AiModeratorConfig,
            enabled: false,
            reply_interval: 5,
            guidelines: String::new(),
            created_at: now,
            updated_at: now,
        }
    }
}
