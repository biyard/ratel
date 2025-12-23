use crate::{
    features::spaces::rewards::{PollRewardKey, RewardAction, RewardKey},
    *,
};

#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone)]
#[serde(untagged)]
pub enum RewardTypeRequest {
    PollRespond { poll_sk: SpacePollEntityType },
}

impl From<RewardTypeRequest> for RewardAction {
    fn from(value: RewardTypeRequest) -> Self {
        match value {
            RewardTypeRequest::PollRespond { poll_sk: _ } => RewardAction::PollRespond,
        }
    }
}

impl From<RewardTypeRequest> for RewardKey {
    fn from(value: RewardTypeRequest) -> Self {
        match value {
            RewardTypeRequest::PollRespond { poll_sk } => {
                RewardKey::Poll(poll_sk, PollRewardKey::Respond)
            }
        }
    }
}
