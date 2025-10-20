use bdk::prelude::{axum::extract::Path, *};

use crate::types::{EntityType, Partition};

pub(crate) type PollPath = Path<PollPathParam>;

#[derive(Debug, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct PollPathParam {
    pub space_pk: Partition,
    pub poll_sk: EntityType,
}
