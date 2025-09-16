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
pub enum InvitationStatus {
    #[default]
    Pending = 1,
    Accepted = 2,
    Declined = 3,
}
