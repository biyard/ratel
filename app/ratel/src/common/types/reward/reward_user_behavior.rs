use std::{fmt::Display, str::FromStr};

use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::common::types::reward::RewardAction;
use crate::common::*;

#[derive(
    Debug,
    Clone,
    DynamoEnum,
    SerializeDisplay,
    DeserializeFromStr,
    Eq,
    PartialEq,
    Default,
)]
pub enum RewardUserBehavior {
    #[default]
    RespondPoll,
    // BoardComment
    // ParticipateQuiz
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
