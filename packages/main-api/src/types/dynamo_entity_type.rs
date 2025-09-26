use bdk::prelude::*;
use serde_with::{DeserializeFromStr, SerializeDisplay};

#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum, JsonSchema)]
pub enum EntityType {
    #[default]
    None,

    // User entity types
    // USER_PK index is aligned by gsi1-index
    User,
    UserMembership,
    UserEvmAddress,
    UserReferralCode,
    UserPrincipal,
    UserOAuth,
    UserPhoneNumber,
    UserTelegram,
    UserTeam(String),      // from Team
    UserTeamGroup(String), // from TeamGroup
    EmailVerification,

    // Feed entity types
    Post,
    PostAuthor, // from User
    PostSpace,
    PostComment(String), // PostComment should be sorted by timestamp
    PostArtwork,
    PostRepost,       //Unique
    PostLike(String), // PostLike#${User Pk}
    // Team entity types
    // TEAM_PK index is aligned by gsi1-index
    // TEAM_GROUP_PK index is aligned by gsi1-index
    Team,
    TeamOwner, // from User
    TeamGroup(String),
    TeamMember(String),

    // Space common entity types
    // SPACE_PK index is aligned by gsi2-index
    SpaceCommon,

    // Poll Space entity types
    PollSpace,

    // Survery space entity types
    SurveySpace,

    // Deliberation space entity types
    DeliberationSpace,

    // Sprint league space entity types
    SprintLeagueSpace,

    // Artwork space entity types
    ArtworkSpace,

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
