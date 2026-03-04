use crate::*;
use common::models::auth::AdminUser;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateGlobalRewardRequest {
    pub behavior: RewardUserBehavior,
    pub point: i64,
    pub period: RewardPeriod,
    pub condition: RewardCondition,
}

#[put("/api/admin/rewards", _user: AdminUser)]
pub async fn update_reward(req: UpdateGlobalRewardRequest) -> Result<Reward> {
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();

    if Reward::get(cli, Partition::Reward, Some(req.behavior.clone()))
        .await?
        .is_none()
    {
        return Err(Error::NotFound("Reward".into()));
    }

    let reward = Reward::updater(&Partition::Reward, &req.behavior)
        .with_point(req.point)
        .with_period(req.period)
        .with_condition(req.condition)
        .execute(cli)
        .await?;

    Ok(reward)
}
