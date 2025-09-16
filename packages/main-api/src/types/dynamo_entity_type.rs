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
    // USER_PK index is aligned by gsi1-index
    User,
    UserEvmAddress,
    UserReferralCode,
    UserPrincipal,
    UserPhoneNumber,
    UserTelegram,
    UserTeam,      // from Team
    UserTeamGroup, // from TeamGroup
    EmailVerification,

    // Feed entity types
    Post,
    PostAuthor, // from User
    PostSpace,

    // Team entity types
    // TEAM_PK index is aligned by gsi1-index
    // TEAM_GROUP_PK index is aligned by gsi1-index
    Team,
    TeamOwner, // from User
    TeamGroup,
    TeamMember,

    Space,
    SpaceMember,
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
