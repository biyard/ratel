use crate::macros::DynamoEnum;

#[derive(
    Debug,
    Clone,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    DynamoEnum,
    PartialEq,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum PollStatus {
    #[default]
    NotStarted = 1,
    InProgress = 2,
    Finish = 3,
}
