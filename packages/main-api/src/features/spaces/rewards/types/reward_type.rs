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

impl RewardType {
    pub fn point(&self) -> i64 {
        match self {
            RewardType::None => 0,
            RewardType::RespondPoll(_) => 10_000,
        }
    }
}
