use crate::*;
use serde_with::{DeserializeFromStr, SerializeDisplay};

use super::Partition;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    SerializeDisplay,
    DeserializeFromStr,
    Default,
    DynamoEnum,
    SubPartition,
    JsonSchema,
)]
pub enum EntityType {
    #[default]
    None,

    Session,

    // Common
    Created(String), // CREATED#${timestamp}

    // User entity types
    // USER_PK index is aligned by gsi1-index
    User,
    UserEvmAddress,
    UserNotification(String), //notification id
    UserReferralCode,
    UserPrincipal,
    UserOAuth,
    UserPhoneNumber,
    UserTelegram,
    UserTeam(String),      // from Team
    UserTeamGroup(String), // from TeamGroup
    EmailVerification,
    PhoneVerification,
    UserRelationship(String),
    UserRefreshToken(String),

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
    SpaceParticipant,
    SpaceInvitation,
    SpaceTemplate(String),          // template name
    SpaceEmailVerification(String), //email
    SpaceRequirement(String),       // use SpaceRequirementType

    // Poll Feature entity types
    SpacePoll(String), // SpacePoll#{uuid or space_id}

    SpacePollQuestion,

    SpacePollResult,
    SpacePollUserAnswer(String, String), // space_pk, poll_pk

    // Survery space entity types
    SurveySpace,

    // Deliberation space entity types
    // DeliberationSpace,
    DeliberationSummary,
    DeliberationElearning,
    DeliberationRecommendation,
    DeliberationSurvey(String),
    DeliberationDiscussion(String),
    DeliberationDiscussionParticipant(String, String),
    DeliberationDiscussionMember(String, String),
    DeliberationQuestion(String),
    DeliberationResponse(String),

    // Sprint league space entity types
    SprintLeague,
    SprintLeaguePlayer(String), //Uuid
    SprintLeagueVote(String),   //#{SPACE_ID}#{UserPk_ID}

    // Artwork space entity types,
    SpaceArtwork,
    SpaceArtworkTrade(String), // Transaction hash

    Space,
    SpaceMember,
    Feed,
    Group,
    Metadata,
    Member,
    Follower,
    Following,
    Like,
    Bookmark,
    Comment,
    Badge,
    Industry,

    SpaceCategory(String),
    SpacePost(String),
    SpacePostComment(String),
    SpacePostCommentReply(String, String),
    SpacePostCommentLike(String, String),

    // Space - Topic feature
    Topic(String),                     // TOPIC#{topic_name}
    TopicArticle(String),              // TOPIC_ARTICLE#{topic_name}#{article_id}
    TopicArticleReply(String, String), // TOPIC_ARTICLE_REPLY#{topic_name}#{article_id}#{reply_id}
    TopicDiscussion(String),           // TOPIC_DISCUSSION#{discussion_id}

    //SPACE FEATURE
    SpaceFile,
    SpaceAnalyze,
    SpaceAnalyzeRequest(String),
    SpaceDiscussion(String),
    SpaceDiscussionMember(String, String),
    SpaceDiscussionParticipant(String, String),
    SpaceQuiz(String),
    SpaceRecommendation,
    SpaceReport,
    SpacePanels,
    SpacePanel(String),
    SpacePanelAttribute(String, String),
    SpacePanelParticipant(String), //user_pk

    SpaceInvitationMember(String),
    SpaceSurveyResponse(String), //Space pk

    // Membership
    Membership,
    UserMembership,  // PK: {USER_PK}, SK: UserMembership
    TeamMembership,  // PK: {TEAM_PK}, SK: TeamMembership

    // ServiceAdmin
    ServiceAdmin, // PK: SERVICE_ADMIN#{USER_PK}, SK: ServiceAdmin

    // DID
    DidDocument, // PK: DID#{did}, SK: DidDocument
    VerifiedAttributes,
    AttributeCode,

    //Telegram Feature
    TelegramChannel(String), // Telegram Chat ID

    // Payment features
    UserPayment,
    TeamPayment,
    Purchase,
    UserPurchase(String),
    TeamPurchase(String),

    Notification(String), // notification id

    //
    Reward(String), // Type

    ContentReport,
}

impl TryInto<Partition> for EntityType {
    type Error = Error;

    fn try_into(self) -> Result<Partition> {
        Ok(match self {
            EntityType::SpacePoll(v) => Partition::Poll(v),
            _ => Err(crate::Error::NotSupported(
                "It is not a type supported converting to Partition.".to_string(),
            ))?,
        })
    }
}
