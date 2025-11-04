use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, aide::OperationIo)]
pub struct MintSpaceArtworkRequest {}
