use crate::*;

#[derive(
    Debug,
    Clone,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    DynamoEnum,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum SpaceStatus {
    #[default]
    Waiting, // Draft
    InProgress, // Published
    Started,    // Started
    Finished,   // Finished
}
