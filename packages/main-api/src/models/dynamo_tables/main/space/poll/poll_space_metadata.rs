use super::*;
use crate::models::space::SpaceCommon;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum PollSpaceMetadata {
    SpaceCommon(SpaceCommon),
    PollSpace(PollSpace),
    PollSpaceSurvey(PollSpaceSurvey),
}
