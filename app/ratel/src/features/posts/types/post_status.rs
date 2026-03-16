use crate::features::posts::*;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum PostStatus {
    #[default]
    Draft = 1,
    Published = 2,
}
