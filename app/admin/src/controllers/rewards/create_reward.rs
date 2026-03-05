use crate::*;
use common::models::auth::AdminUser;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateGlobalRewardRequest {
    pub behavior: RewardUserBehavior,
    pub point: i64,
    pub period: RewardPeriod,
    pub condition: RewardCondition,
}

#[post("/api/admin/rewards", _user: AdminUser)]
pub async fn create_reward(req: CreateGlobalRewardRequest) -> Result<Reward> {
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();

    if Reward::get(cli, Partition::Reward, Some(req.behavior.clone()))
        .await?
        .is_some()
    {
        return Err(SpaceRewardError::RewardAlreadyExists.into());
    }

    let reward = Reward::new(req.behavior, req.point, req.period, req.condition);
    reward.create(cli).await?;

    Ok(reward)
}
