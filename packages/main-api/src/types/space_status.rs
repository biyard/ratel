use bdk::prelude::*;

#[derive(
    Debug,
    Clone,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    DynamoEnum,
    JsonSchema,
    Eq,
    PartialEq,
)]
pub enum SpaceStatus {
    #[default]
    Waiting,
    InProgress,
    Started,
    Finished,
}
