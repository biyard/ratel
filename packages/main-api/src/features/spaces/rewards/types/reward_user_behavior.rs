use std::{fmt::Display, str::FromStr};

use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::features::spaces::rewards::{RewardAction, RewardCondition, RewardPeriod};
use crate::*;

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
pub enum RewardUserBehavior {
    #[default]
    RespondPoll,
    // BoardComment
    // ParticipateQuiz
    // ...
}

impl RewardUserBehavior {
    pub fn action(&self) -> RewardAction {
        match self {
            Self::RespondPoll => RewardAction::Poll,
        }
    }

    pub fn list_behaviors(action: RewardAction) -> Vec<Self> {
        match action {
            RewardAction::Poll => vec![Self::RespondPoll],
        }
    }
}
