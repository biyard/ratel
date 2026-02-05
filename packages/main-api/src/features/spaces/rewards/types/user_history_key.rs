use std::{fmt::Display, str::FromStr};

use chrono::Datelike;
use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::{
    features::spaces::rewards::{RewardCondition, RewardPeriod, SpaceRewardSk},
    *,
};

#[derive(Debug, Default, Clone, JsonSchema, OperationIo, SerializeDisplay, DeserializeFromStr)]
pub struct UserRewardHistoryKey(pub SpaceRewardSk, pub String);

impl Display for UserRewardHistoryKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}###{}", self.0, self.1)
    }
}

impl FromStr for UserRewardHistoryKey {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, "###").collect();
        if parts.len() != 2 {
            return Err(Error::InvalidPartitionKey(
                "invalid composite partition format".to_string(),
            ));
        }
        let part1 = SpaceRewardSk::from_str(parts[0])
            .map_err(|_| Error::InvalidPartitionKey("Not SpaceRewardSk".to_string()))?;
        let part2 = parts[1].to_string();

        Ok(UserRewardHistoryKey(part1, part2))
    }
}
