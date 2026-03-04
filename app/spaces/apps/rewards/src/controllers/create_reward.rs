use crate::*;
use common::models::reward::Reward;
use space_common::models::{SpaceReward, SpaceRewardResponse};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateSpaceRewardRequest {
    pub action_key: EntityType,
    pub behavior: RewardUserBehavior,
    #[serde(default)]
    pub description: String,
    pub credits: i64,
}

#[post("/api/spaces/{space_id}/rewards", role: SpaceUserRole)]
pub async fn create_space_reward(
    space_id: SpacePartition,
    req: CreateSpaceRewardRequest,
) -> Result<SpaceRewardResponse> {
    let action = RewardAction::try_from(&req.action_key)?;
    if action != req.behavior.action() {
        return Err(Error::BadRequest("Behavior does not match action".into()));
    }

    SpaceReward::can_edit(&role);
    // Only space creators can create rewards
    match role {
        SpaceUserRole::Creator => {}
        _ => return Err(Error::NoPermission),
    }

    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();

    // Fetch global reward template for this behavior
    let reward = Reward::get(cli, Partition::Reward, Some(req.behavior.clone()))
        .await?
        .ok_or(Error::from(SpaceRewardError::RewardNotFound))?;

    let space_reward = SpaceReward::new(
        space_id,
        req.action_key,
        req.behavior,
        req.description,
        req.credits,
        reward.point,
        reward.period,
        reward.condition,
    );

    // TODO: Add UserMembership credit deduction when membership model is migrated
    space_reward.create(cli).await?;

    Ok(space_reward.into())
}
