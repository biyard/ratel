use crate::features::ai_moderator::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct AiModeratorMaterial {
    pub pk: CompositePartition<SpacePartition, String>,
    pub sk: EntityType,
    pub file_name: String,
    pub file_url: String,
    pub created_at: i64,
}

#[cfg(feature = "server")]
impl AiModeratorMaterial {
    pub fn new(
        space_id: SpacePartition,
        discussion_sk: String,
        file_name: String,
        file_url: String,
    ) -> Self {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        let material_id = uuid::Uuid::now_v7().to_string();
        Self {
            pk: CompositePartition(space_id, discussion_sk),
            sk: EntityType::AiModeratorMaterial(material_id),
            file_name,
            file_url,
            created_at: now,
        }
    }
}
