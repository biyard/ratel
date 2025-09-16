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
pub enum UrlType {
    #[default]
    None = 0,
    Image = 1,
}
