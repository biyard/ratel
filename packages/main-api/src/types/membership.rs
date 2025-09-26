use bdk::prelude::*;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    JsonSchema,
    Default,
)]
#[repr(u8)]
pub enum Membership {
    #[default]
    Free = 1,
    Pro = 2,
    Max = 3,
    VIP = 4,
    Enterprise = 5,
    Admin = 99,
}
