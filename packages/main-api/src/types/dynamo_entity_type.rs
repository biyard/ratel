use bdk::prelude::*;
use serde_with::{DeserializeFromStr, SerializeDisplay};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    SerializeDisplay,
    DeserializeFromStr,
    Default,
    DynamoEnum,
    JsonSchema,
)]
pub enum EntityType {
    #[default]
    None,

    Session,

    // User entity types
    // USER_PK index is aligned by gsi1-index
    User,
    UserEvmAddress,
    UserReferralCode,
    UserPrincipal,
    UserOAuth,
    UserPhoneNumber,
    UserTelegram,
    UserTeam(String),      // from Team
    UserTeamGroup(String), // from TeamGroup
    EmailVerification,
    UserRelationship(String),

    // Feed entity types
    Post,
    PostAuthor, // from User
    PostSpace,
    PostComment(String),              // PostComment should be sorted by timestamp
    PostCommentReply(String, String), // PostCommentReply#${PostComment ID}#${uuid}
    PostArtwork,
    PostRepost, //Unique
    // TODO: suffix based strategy
    PostLike(String),                // PostLike#${User Pk}
    PostCommentLike(String, String), // PostCommentLike#${User Pk}#${PostComment Sk}
    // Team entity types
    // TEAM_PK index is aligned by gsi1-index
    // TEAM_GROUP_PK index is aligned by gsi1-index
    Team,
    TeamOwner, // from User
    TeamGroup(String),
    TeamMember(String, String), // TeamMember#${TeamGroup Pk inner}#${User Pk inner}

    // Space common entity types
    // SPACE_PK index is aligned by gsi2-index
    SpaceCommon,

    // Poll Space entity types
    // PollSpace,
    PollSpaceSurvey,
    PollSpaceSurveyResponse(String), //space_pk

    // Survery space entity types
    SurveySpace,

    // Deliberation space entity types
    // DeliberationSpace,
    DeliberationSpaceSummary,
    DeliberationSpaceElearning,
    DeliberationSpaceRecommendation,
    DeliberationSpaceSurvey(String),
    DeliberationSpaceDiscussion(String),
    DeliberationSpaceParticipant(String),
    DeliberationSpaceMember(String),
    DeliberationSpaceQuestion(String),
    DeliberationSpaceResponse(String),

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
