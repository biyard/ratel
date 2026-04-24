use crate::common::*;
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
)]
#[cfg_attr(feature = "server", derive(JsonSchema))]
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
    UserInboxNotification(String), // uuid_v7, time-ordered
    InboxDedupMarker(String),      // "{kind}#{source_hash}"
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
    SpaceIncentive,
    SpaceIncentiveUser(String),
    SpaceIncentiveScore(String),
    SpaceIncentiveToken(String),
    SpaceDao,
    SpaceDaoSample(String),
    SpaceParticipant,
    SpaceAdmin(String), // SPACE_ADMIN#{user_pk}
    SpaceInvitation,
    SpaceTemplate(String),          // template name
    SpaceEmailVerification(String), //email
    SpaceRequirement(String),       // use SpaceRequirementType

    // Poll Feature entity types
    SpacePoll(String), // SpacePoll#{uuid or space_id}
    SpaceActionFollow(String),
    SpaceSubscription, // SpaceSubscription#{uuid or space_id}
    SpaceSubscriptionUser(String),

    SpacePollQuestion,

    SpacePollResult,
    SpacePollUserAnswer(String, String), // space_pk, poll_pk

    SpaceQuiz(String),        // SpaceQuiz#{uuid}
    SpaceQuizAnswer(String),  // SpaceQuizAnswer#{quiz_id}
    SpaceQuizAttempt(String), // SpaceQuizAttempt#{quiz_id}#{attempt_id}

    // Meet action entity types
    SpaceMeet(String),        // SpaceMeet#{uuid}

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
    Follower(String),
    Following(String),
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
    FileLink(String), // FileLink#{file_id}
    SpaceAnalyze,
    SpaceAnalyzeRequest(String),
    SpaceDiscussion(String),
    SpaceDiscussionMember(String, String),
    SpaceDiscussionParticipant(String, String),

    SpaceRecommendation,
    SpaceReport,
    SpacePanels,
    SpacePanel(String),
    SpacePanelAttribute(String, String),
    SpacePanelParticipant(String), //user_pk
    SpaceDashboardExtension(String),

    SpaceInvitationMember(String),
    SpaceSurveyResponse(String), //Space pk
    SpaceApp(String),

    // Membership
    Membership,
    UserMembership, // PK: {USER_PK}, SK: UserMembership
    TeamMembership, // PK: {TEAM_PK}, SK: TeamMembership

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
    SpaceStatusChangeEvent(String), // uuid_v7 (same id as pk)

    //
    SpaceAction,
    Reward,
    SpaceReward,

    ContentReport,

    Category(String), // CATEGORY#${name}

    TimelineEntry(String), // TIMELINE_ENTRY#${timestamp}#${post_pk_inner}

    // AI Moderator
    AiModeratorConfig,
    AiModeratorMaterial(String), // AIMODMATERIAL#{material_id}

    // MCP
    McpClientSecret,

    // Activity
    SpaceActivity(String), // SPACE_ACTIVITY#action_id#timestamp
    SpaceScore,

    // Essence — user's knowledge graph entries. Each row is a reference to
    // something the user authored (Post, Poll, Quiz, PostComment,
    // DiscussionComment) or imported (Notion). pk = USER#{user_id}.
    Essence(String), // ESSENCE#{uuid}
    /// Singleton counter row per user; atomic ADDs from `Essence::put` /
    /// delete keep the aggregates consistent.
    UserEssenceStats,

    // Sub-team governance — parent team owns the records in its own pk space.
    // pk = TEAM#{parent_team_id} throughout (SubTeamDocAgreement's composite
    // sk encodes application_id + doc_id so one parent pk can hold
    // agreements for many applications).
    SubTeamLink(String),                  // SUB_TEAM_LINK#{child_team_id}
    SubTeamDocument(String),              // SUB_TEAM_DOCUMENT#{doc_id}
    SubTeamDocAgreement(String, String),  // SUB_TEAM_DOC_AGREEMENT#{app_id}#{doc_id}
    SubTeamFormField(String),             // SUB_TEAM_FORM_FIELD#{field_id}
    SubTeamApplication(String),           // SUB_TEAM_APPLICATION#{application_id}
    SubTeamAnnouncement(String),          // SUB_TEAM_ANNOUNCEMENT#{announcement_id}
}

impl TryInto<Partition> for EntityType {
    type Error = Error;

    fn try_into(self) -> Result<Partition> {
        Ok(match self {
            EntityType::SpacePoll(v) => Partition::Poll(v),
            _ => Err(crate::common::Error::UnsupportedOperation)?,
        })
    }
}
