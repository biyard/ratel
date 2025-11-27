use crate::*;

#[derive(
    Debug,
    Clone,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    DynamoEnum,
    JsonSchema,
    OperationIo,
)]
pub enum RewardType {
    #[default]
    None,
    RespondPoll(String), // Poll SK
}
