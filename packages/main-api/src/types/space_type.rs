use bdk::prelude::*;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    Default,
    schemars::JsonSchema_repr,
)]
#[repr(u8)]
pub enum SpaceType {
    #[default]
    Legislation = 1,
    Poll = 2,
    Deliberation = 3,
    Nft = 4,
    Commitee = 5,
    SprintLeague = 6,
    Notice = 7,
    Dagit = 8,
}
