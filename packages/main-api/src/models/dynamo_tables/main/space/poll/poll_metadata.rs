use bdk::prelude::*;

use crate::models::{Poll, PollQuestion};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum PollMetadata {
    Poll(Poll),
    PollQuestion(PollQuestion),
}
