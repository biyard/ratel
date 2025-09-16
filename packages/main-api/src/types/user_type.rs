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
pub enum UserType {
    #[default]
    Individual = 1,
    Team = 2,
    Bot = 3,
    Anonymous = 99,
}
