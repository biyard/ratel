use std::fmt::Display;

use chrono::Datelike;

use crate::{
    features::spaces::rewards::{FeatureRewardTrait, RewardCondition, RewardDetail, RewardPeriod},
    *,
};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    DynamoEnum,
    JsonSchema,
    Default,
    serde::Serialize,
    serde::Deserialize,
    OperationIo,
)]
pub enum BoardReward {
    #[default]
    Default,
    Comment(String),

    Like(String),
}

impl FeatureRewardTrait for BoardReward {
    fn is_empty(&self) -> bool {
        false
    }
    fn detail(&self) -> RewardDetail {
        match self {
            BoardReward::Comment(_) => RewardDetail {
                point: 100,
                period: RewardPeriod::Daily,
                condition: RewardCondition::MaxClaims(5),
            },
            BoardReward::Like(_) => RewardDetail {
                point: 500,
                period: RewardPeriod::Daily,
                condition: RewardCondition::MaxUserClaims(3),
            },
            _ => RewardDetail {
                point: 0,
                period: RewardPeriod::Once,
                condition: RewardCondition::MaxClaims(0),
            },
        }
    }
}
