use std::{fmt::Display, str::FromStr};

use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::common::types::*;
use crate::common::*;

/// Composite key for reward entities in DynamoDB.
///
/// Supports two forms:
/// - Global reward: `Reward##{behavior}` (when `space_pk` and `action_id` are `None`)
/// - Space-scoped reward: `SpaceReward##{space_pk}##{action_id}##{behavior}`
#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr, Default, PartialEq, Eq)]
pub struct RewardKey {
    pub space_pk: Option<SpacePartition>,
    pub action_id: Option<String>,
    pub behavior: RewardUserBehavior,
}

/// Serializes the key into its `##`-delimited string representation.
impl Display for RewardKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.space_pk, &self.action_id) {
            // Space-scoped reward: SPACE_REWARD##{space}##{action_id}##{behavior}
            (Some(space), Some(action_id)) => {
                write!(
                    f,
                    "{}##{}##{}##{}",
                    EntityType::SpaceReward,
                    space,
                    action_id,
                    self.behavior
                )
            }
            // Global reward: Reward##{behavior}
            (None, None) => {
                write!(f, "{}##{}", EntityType::Reward, self.behavior)
            }
            // Mixed Some/None is invalid
            _ => Err(std::fmt::Error),
        }
    }
}

/// Parses a `##`-delimited string back into a `RewardKey`.
/// Dispatches based on the leading entity type (`Reward` or `SpaceReward`).
impl FromStr for RewardKey {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("##").collect();

        if parts.len() < 2 {
            return Err(SpaceRewardError::InvalidEntityType.into());
        }

        let entity_type =
            EntityType::from_str(parts[0]).map_err(|_| SpaceRewardError::InvalidEntityType)?;

        match entity_type {
            EntityType::Reward => {
                if parts.len() == 2 {
                    let behavior = RewardUserBehavior::from_str(parts[1])
                        .map_err(|_| SpaceRewardError::InvalidEntityType)?;
                    Ok(RewardKey {
                        space_pk: None,
                        action_id: None,
                        behavior,
                    })
                } else {
                    Err(SpaceRewardError::InvalidEntityType.into())
                }
            }
            EntityType::SpaceReward => {
                if parts.len() != 4 {
                    return Err(SpaceRewardError::InvalidEntityType.into());
                }

                let space_pk = SpacePartition::from_str(parts[1])
                    .map_err(|_| SpaceRewardError::InvalidEntityType)?;
                let action_id = parts[2].to_string();
                let behavior = RewardUserBehavior::from_str(parts[3])
                    .map_err(|_| SpaceRewardError::InvalidEntityType)?;

                Ok(RewardKey {
                    space_pk: Some(space_pk),
                    action_id: Some(action_id),
                    behavior,
                })
            }
            _ => Err(SpaceRewardError::InvalidEntityType.into()),
        }
    }
}

/// Creates a space-scoped `RewardKey` from a `(SpacePartition, String, RewardUserBehavior)` tuple.
impl From<(SpacePartition, String, RewardUserBehavior)> for RewardKey {
    fn from((space_pk, action_id, behavior): (SpacePartition, String, RewardUserBehavior)) -> Self {
        Self {
            space_pk: Some(space_pk),
            action_id: Some(action_id),
            behavior,
        }
    }
}

/// Creates a global `RewardKey` from a `RewardUserBehavior` (no space scope).
impl From<RewardUserBehavior> for RewardKey {
    fn from(behavior: RewardUserBehavior) -> Self {
        Self {
            space_pk: None,
            action_id: None,
            behavior,
        }
    }
}

impl RewardKey {
    /// Builds a sort key prefix for DynamoDB `begins_with` queries on space rewards.
    ///
    /// - With `action_id`: `SpaceReward##{space_pk}##{action_id}` (filter by action)
    /// - Without: `SpaceReward##{space_pk}` (all rewards in the space)
    pub fn get_space_reward_sk_prefix(
        space_pk: SpacePartition,
        action_id: Option<String>,
    ) -> String {
        match action_id {
            Some(action_id) => {
                format!("{}##{}##{}", EntityType::SpaceReward, space_pk, action_id)
            }
            None => {
                format!("{}##{}", EntityType::SpaceReward, space_pk)
            }
        }
    }
}
