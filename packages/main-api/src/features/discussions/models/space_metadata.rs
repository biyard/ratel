use super::*;
use crate::{features::models::space_discussion::SpaceDiscussion, models::space::SpaceCommon};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum SpaceMetadata {
    SpaceDiscussion(SpaceDiscussion),
}
