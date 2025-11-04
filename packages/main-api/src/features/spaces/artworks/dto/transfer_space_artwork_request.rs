use bdk::prelude::*;

use crate::types::Partition;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, aide::OperationIo)]
pub struct TransferSpaceArtworkRequest {
    pub to_user_pk: Partition,
}
