use std::{fmt::Display, str::FromStr};

use chrono::Datelike;
use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::{
    features::spaces::rewards::{BoardReward, PollReward, RewardCondition, RewardPeriod},
    *,
};

#[derive(
    Debug, Clone, DynamoEnum, Default, SerializeDisplay, DeserializeFromStr, JsonSchema, OperationIo,
)]
pub enum RewardType {
    #[default]
    Default,
    Poll(SpacePollEntityType, PollReward),
    Board(SpacePostEntityType, BoardReward),
}

impl RewardType {
    pub fn detail(&self) -> RewardDetail {
        match self {
            RewardType::Default => RewardDetail {
                point: 0,
                period: RewardPeriod::Once,
                condition: RewardCondition::None,
            },
            RewardType::Poll(_, reward) => reward.detail(),
            RewardType::Board(_, reward) => reward.detail(),
        }
    }
}
pub trait FeatureRewardTrait {
    fn is_empty(&self) -> bool;
    fn detail(&self) -> RewardDetail;
}
pub struct RewardDetail {
    pub point: i64,
    pub period: RewardPeriod,
    pub condition: RewardCondition,
}
#[derive(Debug, Clone, JsonSchema, OperationIo, SerializeDisplay, DeserializeFromStr)]
pub struct RewardHistoryType(pub RewardType, pub String);

impl Display for RewardHistoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}##{}", self.0, self.1)
    }
}

impl FromStr for RewardHistoryType {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, "##").collect();
        if parts.len() != 2 {
            return Err(Error::InvalidPartitionKey(
                "invalid composite partition format".to_string(),
            ));
        }
        let part1 = RewardType::from_str(parts[0])
            .map_err(|_| Error::InvalidPartitionKey("Not RewardType".to_string()))?;
        let part2 = parts[1].to_string();

        Ok(RewardHistoryType(part1, part2))
    }
}
impl Default for RewardHistoryType {
    fn default() -> Self {
        Self(RewardType::Default, "".to_string())
    }
}
