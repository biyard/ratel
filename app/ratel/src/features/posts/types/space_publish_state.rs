use crate::features::posts::*;

#[derive(
    Debug,
    Clone,
    Copy,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    DynamoEnum,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum SpacePublishState {
    #[default]
    Draft,
    Published,
}
