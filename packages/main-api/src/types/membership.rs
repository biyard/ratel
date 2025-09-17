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
pub enum Membership {
    #[default]
    Free = 1,
    Paid1 = 2,
    Paid2 = 3,
    Paid3 = 4,
    Admin = 99,
}
