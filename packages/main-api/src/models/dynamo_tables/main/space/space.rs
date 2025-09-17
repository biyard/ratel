use super::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum Space {
    DeliberationSpace(DeliberationSpace),
    PollSpace(PollSpace),
}
