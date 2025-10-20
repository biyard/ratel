use bdk::prelude::*;

use crate::types::{EntityType, Partition};

#[derive(Debug, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct SprintLeaguePlayerPathParam {
    pub space_pk: Partition,
    pub player_sk: EntityType,
}
