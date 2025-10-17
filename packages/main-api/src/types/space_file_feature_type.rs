use bdk::prelude::*;

#[derive(
    Debug,
    Clone,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    DynamoEnum,
    PartialEq,
    schemars::JsonSchema,
)]
pub enum SpaceFileFeatureType {
    #[default]
    Overview,
    Recommendation,
}
