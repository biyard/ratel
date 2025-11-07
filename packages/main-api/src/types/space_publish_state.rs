use bdk::prelude::*;

#[derive(
    Debug,
    Clone,
    Copy,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    DynamoEnum,
    JsonSchema,
    Eq,
    PartialEq,
)]
pub enum SpacePublishState {
    #[default]
    Draft,
    Published,
}
