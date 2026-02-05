use crate::features::spaces::rewards::{Reward, RewardAction, RewardCondition, RewardPeriod};
use crate::types::*;
use crate::*;
use axum::{Json, extract::State};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, OperationIo)]
pub struct CreateRewardRequest {
    pub action: RewardAction,
    pub point: i64,
    pub period: RewardPeriod,
    pub condition: RewardCondition,
}

pub async fn create_reward_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Json(req): Json<CreateRewardRequest>,
) -> Result<Json<Reward>> {
    let cli = &dynamo.client;

    // Check if reward already exists
    if Reward::get(cli, Partition::Reward, Some(req.action.clone()))
        .await?
        .is_some()
    {
        return Err(Error::Conflict(format!(
            "Reward for action {} already exists",
            req.action
        )));
    }

    // Create new reward
    let reward = Reward::new(req.action, req.point, req.period, req.condition);
    reward.create(cli).await?;

    Ok(Json(reward))
}
