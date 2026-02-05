use crate::features::spaces::rewards::{Reward, RewardAction, RewardCondition, RewardPeriod};
use crate::types::*;
use crate::*;
use axum::{extract::State, Json};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, OperationIo)]
pub struct UpsertRewardRequest {
    pub action: RewardAction,
    pub point: i64,
    pub period: RewardPeriod,
    pub condition: RewardCondition,
}

pub async fn upsert_reward_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Json(req): Json<UpsertRewardRequest>,
) -> Result<Json<Reward>> {
    let cli = &dynamo.client;

    // Upsert: create if not exists, update if exists
    let reward = Reward::updater(&Partition::Reward, &req.action)
        .with_point(req.point)
        .with_period(req.period)
        .with_condition(req.condition)
        .execute(cli)
        .await?;

    Ok(Json(reward))
}
