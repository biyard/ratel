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
#[repr(u8)]
pub enum SpaceType {
    #[default]
    None = 0,

    Poll = 1,
    Notice = 2,
    Deliberation = 3,
    SprintLeague = 4,
    Artwork = 5,
}
