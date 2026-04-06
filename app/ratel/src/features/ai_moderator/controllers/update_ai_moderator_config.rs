use crate::features::ai_moderator::*;

use super::get_ai_moderator_config::AiModeratorConfigResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct UpdateAiModeratorConfigRequest {
    pub enabled: bool,
    pub reply_interval: i64,
    #[serde(default)]
    pub guidelines: String,
}

#[mcp_tool(name = "update_ai_moderator", description = "Configure AI moderator for a discussion. Sets enabled state, reply interval, and guidelines. Requires creator role and premium membership.")]
#[post("/api/spaces/{space_id}/discussions/{discussion_id}/ai-moderator", role: SpaceUserRole, user: crate::features::auth::User)]
pub async fn update_ai_moderator_config(
    #[mcp(description = "Space partition key")]
    space_id: SpacePartition,
    #[mcp(description = "Discussion sort key (e.g. 'SpacePost#<uuid>')")]
    discussion_id: SpaceDiscussionEntityType,
    #[mcp(description = "AI moderator config: {\"enabled\": bool, \"reply_interval\": int, \"guidelines\": \"text\"}")]
    req: UpdateAiModeratorConfigRequest,
) -> Result<AiModeratorConfigResponse> {
    role.is_creator()?;

    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    super::require_premium(cli, &user).await?;

    if req.reply_interval < 1 {
        return Err(AiModeratorError::InvalidReplyInterval.into());
    }

    let pk = CompositePartition(space_id.clone(), discussion_id.to_string());
    let now = crate::common::utils::time::get_now_timestamp_millis();

    let existing =
        AiModeratorConfig::get(cli, &pk, Some(EntityType::AiModeratorConfig)).await?;

    match existing {
        Some(_) => {
            AiModeratorConfig::updater(&pk, &EntityType::AiModeratorConfig)
                .with_enabled(req.enabled)
                .with_reply_interval(req.reply_interval)
                .with_guidelines(req.guidelines.clone())
                .with_updated_at(now)
                .execute(cli)
                .await?;
        }
        None => {
            let mut config = AiModeratorConfig::new(space_id, discussion_id.to_string());
            config.enabled = req.enabled;
            config.reply_interval = req.reply_interval;
            config.guidelines = req.guidelines.clone();
            config.create(cli).await?;
        }
    }

    Ok(AiModeratorConfigResponse {
        enabled: req.enabled,
        reply_interval: req.reply_interval,
        guidelines: req.guidelines,
    })
}
