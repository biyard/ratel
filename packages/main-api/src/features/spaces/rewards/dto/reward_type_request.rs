use crate::{
    features::spaces::rewards::{PollReward, RewardKey, RewardPeriod, RewardType},
    *,
};

#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone)]
#[serde(untagged)]
pub enum RewardTypeRequest {
    PollRespond { poll_sk: SpacePollEntityType },
}

impl From<RewardTypeRequest> for RewardType {
    fn from(value: RewardTypeRequest) -> Self {
        match value {
            RewardTypeRequest::PollRespond { poll_sk: _ } => RewardType::PollRespond,
        }
    }
}

impl From<RewardTypeRequest> for RewardKey {
    fn from(value: RewardTypeRequest) -> Self {
        match value {
            RewardTypeRequest::PollRespond { poll_sk } => {
                RewardKey::Poll(poll_sk, PollReward::Respond)
            }
        }
    }
}
