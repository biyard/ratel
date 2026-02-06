use crate::Result;
use crate::features::spaces::rewards::{
    Reward, RewardAction, RewardCondition, RewardPeriod, RewardQueryOption, RewardUserBehavior,
};
use crate::*;
use bdk::prelude::*;
use by_axum::axum::Json;
use by_axum::axum::extract::Query;

#[derive(Debug, Clone, Default, serde::Deserialize, schemars::JsonSchema)]
pub struct ListRewardsQuery {
    pub action: Option<RewardAction>,
}

#[derive(
    Debug,
    Clone,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct RewardResponse {
    pub reward_behavior: RewardUserBehavior,
    pub point: i64,
    pub period: RewardPeriod,
    pub condition: RewardCondition,
}

impl From<Reward> for RewardResponse {
    fn from(value: Reward) -> Self {
        Self {
            reward_behavior: value.sk,
            point: value.point,
            period: value.period,
            condition: value.condition,
        }
    }
}
#[derive(
    Debug,
    Clone,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct ListRewardsResponse {
    pub items: Vec<RewardResponse>,
}

pub async fn list_rewards_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Query(ListRewardsQuery { action }): Query<ListRewardsQuery>,
) -> Result<Json<ListItemsResponse<RewardResponse>>> {
    let opt = Reward::opt_all();

    let (items, _) = if let Some(action) = action {
        let pk = Reward::compose_gsi1_pk(action);
        Reward::find_by_action(&dynamo.client, &pk, opt).await?
    } else {
        Reward::query(&dynamo.client, Partition::Reward, opt).await?
    };
    let items = items
        .into_iter()
        .map(|item| RewardResponse::from(item))
        .collect();

    Ok(Json(ListItemsResponse {
        items,
        bookmark: None,
    }))
}
