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
)]
pub enum PollStatus {
    #[default]
    Ready = 1,
    InProgress = 2,
    Finish = 3,
}
