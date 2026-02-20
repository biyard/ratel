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
pub enum PostType {
    #[default]
    Post = 1,
    Repost = 2,
    Artwork = 3,
}
