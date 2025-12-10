use crate::{
    features::spaces::rewards::{PollReward, RewardPeriod, RewardType},
    *,
};

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub enum SpaceRewardType {
    PollRespond(String), // poll_sk
}

impl From<SpaceRewardType> for RewardType {
    fn from(value: SpaceRewardType) -> Self {
        match value {
            SpaceRewardType::PollRespond(poll_sk) => {
                RewardType::Poll(SpacePollEntityType(poll_sk), PollReward::Respond)
            }
        }
    }
}
