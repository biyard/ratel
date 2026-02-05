use std::{fmt::Display, str::FromStr};

use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::*;

// RewardAction is available actions of a reward
// Like Poll, Quiz, etc

#[derive(
    Debug,
    Clone,
    DynamoEnum,
    SerializeDisplay,
    DeserializeFromStr,
    JsonSchema,
    OperationIo,
    Eq,
    PartialEq,
    Default,
)]
pub enum RewardAction {
    #[default]
    Poll,
    // Quiz,
}

impl TryFrom<&EntityType> for RewardAction {
    type Error = Error;
    fn try_from(entity_type: &EntityType) -> Result<Self> {
        match entity_type {
            EntityType::SpacePoll(_) => Ok(RewardAction::Poll),
            _ => Err(Error::InvalidEntityType),
        }
    }
}
