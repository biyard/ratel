use crate::features::posts::*;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    Default,
    DynamoEnum,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema_repr))]
#[repr(u8)]
pub enum PostStatus {
    #[default]
    Draft = 1,
    Published = 2,
}
