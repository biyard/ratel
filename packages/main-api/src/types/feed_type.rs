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
pub enum FeedType {
    #[default]
    Post = 1,
    Repost = 2,
    Artwork = 3,
    // Belows are kinds of comments
    // Reply = 2,
    // Repost = 3,
    // DocReview = 4,
}
