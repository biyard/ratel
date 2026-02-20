use crate::*;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    Default,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema_repr))]
#[repr(u8)]
pub enum BoosterType {
    #[default]
    NoBoost = 1,
    X2 = 2,
    X10 = 3,
    X100 = 4,
    Custom = 255,
}
