use std::{fmt::Display, str::FromStr};

use chrono::Datelike;
use serde_with::{DeserializeFromStr, SerializeDisplay};

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
pub enum RewardType {
    #[default]
    PollRespond,
    BoardComment,
    BoardLike,
}
