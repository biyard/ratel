use crate::Result;
use crate::features::spaces::rewards::{RewardCondition, RewardConfig, RewardPeriod, RewardType};

use bdk::prelude::*;
use by_axum::axum::Json;

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
    pub reward_type: RewardType,
    pub point: i64,
    pub period: RewardPeriod,
    pub condition: RewardCondition,
}

impl From<RewardConfig> for RewardResponse {
    fn from(config: RewardConfig) -> Self {
        Self {
            reward_type: config.reward_type,
            point: config.point,
            period: config.period,
            condition: config.condition,
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

/// List all available reward types
///
/// Returns all reward definitions that can be configured for spaces.
/// This endpoint is public and doesn't require authentication.
pub async fn list_rewards_handler() -> Result<Json<ListRewardsResponse>> {
    let items = RewardType::all()
        .into_iter()
        .map(RewardResponse::from)
        .collect();

    Ok(Json(ListRewardsResponse { items }))
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
            .find(|r| r.reward_type == RewardType::PollRespond);
        assert!(
            poll_respond.is_some(),
            "PollRespond reward type should exist"
        );
        assert_eq!(poll_respond.unwrap().point, 10_000);
    }
}
