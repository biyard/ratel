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
    EnumProp,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema_repr))]
#[repr(u8)]
pub enum UserType {
    #[default]
    Individual = 1,
    Team = 2,
    Bot = 3,
    AnonymousSpaceUser = 4,

    Admin = 98,
    Anonymous = 99,
}
