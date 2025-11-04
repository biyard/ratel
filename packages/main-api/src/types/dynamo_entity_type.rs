use bdk::prelude::*;
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

    SpaceEmailVerification(String), //email

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

    // Artwork space entity types
    ArtworkSpace,

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

    // Space - Topic feature
    Topic(String),                     // TOPIC#{topic_name}
    TopicArticle(String),              // TOPIC_ARTICLE#{topic_name}#{article_id}
    TopicArticleReply(String, String), // TOPIC_ARTICLE_REPLY#{topic_name}#{article_id}#{reply_id}
    TopicDiscussion(String),           // TOPIC_DISCUSSION#{discussion_id}

    //SPACE FEATURE
    SpaceFile,
    SpaceDiscussion(String),
    SpaceDiscussionMember(String, String),
    SpaceDiscussionParticipant(String, String),
    SpaceQuiz(String),
    SpaceRecommendation,
    SpacePanel(String),
    SpacePanelParticipant(String, String),

    SpaceInvitationMember(String),
    SpaceSurveyResponse(String), //Space pk

    // Membership
    Membership,
    UserMembership, // PK: {USER_PK}, SK: UserMembership

    // ServiceAdmin
    ServiceAdmin, // PK: SERVICE_ADMIN#{USER_PK}, SK: ServiceAdmin

    // DID
    DidDocument, // PK: DID#{did}, SK: DidDocument

    //Telegram Feature
    TelegramChannel(String), // Telegram Chat ID

    // Payment features
    UserPayment,
    Purchase,
    UserPurchase(String),
}

use crate::Error;

impl TryInto<Partition> for EntityType {
    type Error = Error;

    fn try_into(self) -> Result<Partition, Self::Error> {
        Ok(match self {
            EntityType::SpacePoll(v) => Partition::Poll(v),
            _ => Err(crate::Error::NotSupported(
                "It is not a type supported converting to Partition.".to_string(),
            ))?,
        })
    }
}
