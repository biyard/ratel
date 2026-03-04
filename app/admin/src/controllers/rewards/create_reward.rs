use crate::*;
use common::models::auth::User;
use common::models::reward::Reward;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateGlobalRewardRequest {
    pub behavior: RewardUserBehavior,
    pub point: i64,
    pub period: RewardPeriod,
    pub condition: RewardCondition,
}

#[post("/api/admin/rewards", _user: User)]
pub async fn createl_reward(req: CreateGlobalRewardRequest) -> Result<Reward> {
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();

    // Check if reward already exists
    if Reward::get(cli, Partition::Reward, Some(req.behavior.clone()))
        .await?
        .is_some()
    {
        return Err(Error::RewardAlreadyExists);
    }

    let reward = Reward::new(req.behavior, req.point, req.period, req.condition);
    reward.create(cli).await?;

    Ok(reward)
}
