use std::{fmt::Display, str::FromStr};

use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::*;
#[derive(
    Debug, Clone, DynamoEnum, SerializeDisplay, DeserializeFromStr, Eq, PartialEq, Default,
)]
pub enum RewardAction {
    #[default]
    Poll,
    // Quiz,
    // Discussion
}

impl TryFrom<&EntityType> for RewardAction {
    type Error = Error;
    fn try_from(entity_type: &EntityType) -> crate::Result<Self> {
        match entity_type {
            EntityType::SpacePoll(_) => Ok(RewardAction::Poll),
            _ => Err(SpaceRewardError::InvalidEntityType.into()),
        }
    }
}
