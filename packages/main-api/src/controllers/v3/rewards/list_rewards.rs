use crate::Result;
use crate::features::spaces::rewards::{
    FeatureType, Reward, RewardAction, RewardCondition, RewardPeriod, RewardQueryOption,
};
use crate::*;
use bdk::prelude::*;
use by_axum::axum::Json;
use by_axum::axum::extract::Query;

#[derive(Debug, Clone, Default, serde::Deserialize, schemars::JsonSchema)]
pub struct ListRewardsQuery {
    /// Filter by feature type: "poll", "board"
    pub feature: Option<FeatureType>,
    pub bookmark: Option<String>,
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
    pub reward_action: RewardAction,
    pub point: i64,
    pub period: RewardPeriod,
    pub condition: RewardCondition,
}

impl From<Reward> for RewardResponse {
    fn from(value: Reward) -> Self {
        Self {
            reward_action: value.sk,
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
    Query(ListRewardsQuery { feature, bookmark }): Query<ListRewardsQuery>,
) -> Result<Json<ListItemsResponse<RewardResponse>>> {
    tracing::debug!("feature: {:?}, bookmark: {:?}", feature, bookmark);
    let (items, bookmark) = if let Some(feature) = feature {
        Reward::list_by_feature(&dynamo.client, &feature, bookmark).await?
    } else {
        Reward::query(
            &dynamo.client,
            Partition::Reward,
            RewardQueryOption::builder().limit(100),
        )
        .await?
    };
    let items = items
        .into_iter()
        .map(|item| RewardResponse::from(item))
        .collect();

    Ok(Json(ListItemsResponse { items, bookmark }))
}
