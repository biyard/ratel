use crate::features::spaces::rewards::{
    Reward, RewardCondition, RewardPeriod, RewardUserBehavior,
};
use crate::types::*;
use crate::*;
use axum::{extract::State, Json};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, OperationIo)]
pub struct UpdateRewardRequest {
    pub behavior: RewardUserBehavior,
    pub point: i64,
    pub period: RewardPeriod,
    pub condition: RewardCondition,
}

pub async fn update_reward_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Json(req): Json<UpdateRewardRequest>,
) -> Result<Json<Reward>> {
    let cli = &dynamo.client;

    // Verify reward exists
    if Reward::get(cli, Partition::Reward, Some(req.behavior.clone()))
        .await?
        .is_none()
    {
        return Err(Error::RewardNotFound);
    }

    // Update existing reward using updater pattern
    let reward = Reward::updater(&Partition::Reward, &req.behavior)
        .with_point(req.point)
        .with_period(req.period)
        .with_condition(req.condition)
        .execute(cli)
        .await?;

    Ok(Json(reward))
}
