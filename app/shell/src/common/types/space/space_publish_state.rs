use crate::common::*;

#[derive(
    Debug, Clone, Copy, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum, Eq, PartialEq,
)]
pub enum SpacePublishState {
    #[default]
    Draft,
    Published,
}
