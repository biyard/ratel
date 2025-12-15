use bdk::prelude::*;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    DynamoEnum,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub enum FileLinkTarget {
    #[default]
    Files,
    Overview,
    Board(String), // Board post ID
}
