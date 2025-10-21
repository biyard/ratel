use bdk::prelude::*;

#[derive(
    Debug,
    Clone,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    DynamoEnum,
    schemars::JsonSchema,
    PartialEq,
    aide::OperationIo,
)]
pub enum PollStatus {
    #[default]
    NotStarted = 1,
    InProgress = 2,
    Finish = 3,
}
