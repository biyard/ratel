use bdk::prelude::*;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    schemars::JsonSchema_repr,
    Default,
    EnumProp,
)]
#[repr(u8)]
pub enum Relationship {
    #[default]
    Following = 1,
    Follower = 2,
    Mutual = 3,
}
