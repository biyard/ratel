use std::{fmt::Display, str::FromStr};

use chrono::Datelike;
use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::{
    features::spaces::rewards::{PollRewardKey, RewardCondition, RewardKey, RewardPeriod},
    *,
};

#[derive(Debug, Clone, JsonSchema, OperationIo, SerializeDisplay, DeserializeFromStr)]
pub struct UserRewardHistoryKey(pub RewardKey, pub String);

impl Display for UserRewardHistoryKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}##{}", self.0, self.1)
    }
}

impl FromStr for UserRewardHistoryKey {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, "##").collect();
        if parts.len() != 2 {
            return Err(Error::InvalidPartitionKey(
                "invalid composite partition format".to_string(),
            ));
        }
        let part1 = RewardKey::from_str(parts[0])
            .map_err(|_| Error::InvalidPartitionKey("Not RewardKey".to_string()))?;
        let part2 = parts[1].to_string();

        Ok(UserRewardHistoryKey(part1, part2))
    }
}
impl Default for UserRewardHistoryKey {
    fn default() -> Self {
        Self(RewardKey::Default, "".to_string())
    }
}
