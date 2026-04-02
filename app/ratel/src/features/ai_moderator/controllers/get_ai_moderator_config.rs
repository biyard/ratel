use crate::features::ai_moderator::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct AiModeratorConfigResponse {
    pub enabled: bool,
    pub reply_interval: i64,
    pub guidelines: String,
}

#[get("/api/spaces/{space_id}/discussions/{discussion_id}/ai-moderator", _role: SpaceUserRole)]
pub async fn get_ai_moderator_config(
    space_id: SpacePartition,
    discussion_id: SpaceDiscussionEntityType,
) -> Result<AiModeratorConfigResponse> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let pk = CompositePartition(space_id, discussion_id.to_string());

    match AiModeratorConfig::get(cli, &pk, Some(EntityType::AiModeratorConfig)).await? {
        Some(config) => Ok(AiModeratorConfigResponse {
            enabled: config.enabled,
            reply_interval: config.reply_interval,
            guidelines: config.guidelines,
        }),
        None => Ok(AiModeratorConfigResponse::default()),
    }
}
