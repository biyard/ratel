use crate::features::ai_moderator::*;

use super::get_ai_moderator_config::AiModeratorConfigResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAiModeratorConfigRequest {
    pub enabled: bool,
    pub reply_interval: i64,
    #[serde(default)]
    pub guidelines: String,
}

#[post("/api/spaces/{space_id}/discussions/{discussion_id}/ai-moderator", role: SpaceUserRole, user: crate::features::auth::User)]
pub async fn update_ai_moderator_config(
    space_id: SpacePartition,
    discussion_id: SpaceDiscussionEntityType,
    req: UpdateAiModeratorConfigRequest,
) -> Result<AiModeratorConfigResponse> {
    role.is_creator()?;

    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    // Server-side premium enforcement
    let membership = crate::features::membership::models::UserMembership::get(
        cli,
        user.pk.clone(),
        Some(EntityType::UserMembership),
    )
    .await?;
    let is_paid = membership
        .as_ref()
        .map_or(false, |m| !m.membership_pk.0.contains("Free"));
    if !is_paid {
        return Err(AiModeratorError::PremiumRequired.into());
    }

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
