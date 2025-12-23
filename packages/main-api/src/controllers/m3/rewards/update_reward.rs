use crate::features::membership::dto::*;
use crate::features::spaces::rewards::{Reward, RewardAction, RewardCondition, RewardPeriod};
use crate::*;
use crate::{AppState, Error, features::membership::*, types::*};
use axum::{
    Json,
    extract::{Path, State},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, aide::OperationIo)]
pub struct UpdateRewardRequest {
    pub action: RewardAction,
    pub point: i64,
    pub period: RewardPeriod,
    pub condition: RewardCondition,
}

pub async fn update_reward_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Json(req): Json<UpdateRewardRequest>,
) -> Result<Json<Reward>> {
    let cli = &dynamo.client;

    let reward = Reward::updater(&Partition::Reward, &req.action)
        .with_point(req.point)
        .with_period(req.period)
        .with_condition(req.condition)
        .execute(cli)
        .await?;

    Ok(Json(reward))
}
