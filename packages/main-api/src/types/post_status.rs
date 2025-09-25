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
pub enum PostStatus {
    #[default]
    Draft,
    Published,
}
