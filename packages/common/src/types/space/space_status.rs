use crate::*;

#[derive(
    Debug, Clone, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum, Eq, PartialEq,
)]
pub enum SpaceStatus {
    #[default]
    Waiting, // Draft
    InProgress, // Published
    Started,    // Started
    Finished,   // Finished
}
