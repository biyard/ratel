use std::{fmt::Display, str::FromStr};

use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::{features::spaces::rewards::RewardKey, *};
pub type TimeKey = String;
#[derive(Debug, Default, Clone, JsonSchema, OperationIo, SerializeDisplay, DeserializeFromStr)]
pub struct UserRewardHistoryKey(pub RewardKey, pub TimeKey);

impl Display for UserRewardHistoryKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}###{}", self.0, self.1)
    }
}

impl FromStr for UserRewardHistoryKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.splitn(2, "###").collect();
        if parts.len() != 2 {
            return Err(Error::InvalidPartitionKey("invalid format".to_string()));
        }
        let reward_key = RewardKey::from_str(parts[0])?;
        let time_key = parts[1].to_string();
        Ok(UserRewardHistoryKey(reward_key, time_key))
    }
}
