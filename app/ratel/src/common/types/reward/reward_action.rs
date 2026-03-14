use std::{fmt::Display, str::FromStr};

use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::common::*;
#[derive(
    Debug, Clone, DynamoEnum, SerializeDisplay, DeserializeFromStr, Eq, PartialEq, Default,
)]
pub enum RewardAction {
    #[default]
    SpacePoll,
    SpaceStudyAndQuiz,
    SpaceDiscussion,
    SpaceFollow,
}
