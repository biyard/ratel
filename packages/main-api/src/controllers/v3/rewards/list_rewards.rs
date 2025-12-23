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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::v3_setup::*;
    use crate::*;

    #[tokio::test]
    async fn test_list_rewards() {
        let TestContextV3 { app, .. } = TestContextV3::setup().await;

        let (status, _headers, body) = get! {
            app: app,
            path: "/v3/rewards",
            response_type: ListRewardsResponse
        };

        assert_eq!(status, 200);
        assert_eq!(body.items.len(), 1, "Should have 1 reward type");

        // Check that all reward types exist
        let poll_respond = body
            .items
            .iter()
            .find(|r| r.reward_action == RewardAction::PollRespond);
        assert!(
            poll_respond.is_some(),
            "PollRespond reward type should exist"
        );
        assert_eq!(poll_respond.unwrap().point, 10_000);
    }

    #[tokio::test]
    async fn test_list_rewards_filtered_by_poll() {
        let TestContextV3 { app, .. } = TestContextV3::setup().await;

        let (status, _headers, body) = get! {
            app: app,
            path: "/v3/rewards?feature=poll",
            response_type: ListRewardsResponse
        };

        assert_eq!(status, 200);
        assert_eq!(body.items.len(), 1, "Should have 1 poll reward type");
        assert_eq!(body.items[0].reward_action, RewardAction::PollRespond);
    }
}
