use bdk::prelude::*;
use dto::schemars::JsonSchema_repr;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    Default,
    JsonSchema_repr,
)]
#[repr(u8)]
pub enum Theme {
    Light = 1,
    Dark = 2,
    #[default]
    SystemDefault = 3,
}
