use serde::{Deserialize, Serialize};

#[derive(
    Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Default, strum_macros::Display,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum EntityType {
    #[default]
    None,

    // User entity types
    User,
    UserEvmAddress,
    UserReferralCode,
    UserPrincipal,
    UserPhoneNumber,
    UserTelegram,
    EmailVerification,

    // Feed entity types
    Post,
    PostAuthor,
    PostSpace,

    Space,
    Feed,
    Group,
    Discussion,
    Metadata,
    Member,
    Follower,
    Following,
    Like,
    Bookmark,
    Comment,
    Badge,
    Industry,
}
