use crate::common::*;
use serde_with::{DeserializeFromStr, SerializeDisplay};

use super::Partition;

// `SubPartition` emits `#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]`
// on each generated wrapper struct; the schemars derive expansion uses
// unqualified `schemars::...` paths, so we alias the rmcp re-export here.
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
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
    UserNotification(String),      //notification id
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
    UserDeviceToken(String), // device_id — push (FCM/APNs) token registration

    // Migration framework
    LastBackfillVersion,

    // Launchpad partner integration (idempotency ledger for point deducts)
    LaunchpadDeduction(String), // launchpad idempotency_key

    // Character (account-level progression)
    CharacterXp,
    CharacterXpSource(String), // space_id (unprefixed; SubPartition wraps SpacePartition)
    CharacterSkill(String),    // skill_id ("money_tree", "ranker", ...)

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
    SpaceMeet(String), // SpaceMeet#{uuid}

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
    SpacePostSubscription(String),

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
    SpaceAnalyzeReport(String),       // SPACE_ANALYZE_REPORT#{ulid}
    SpaceAnalyzeReportResult(String), // SPACE_ANALYZE_REPORT_RESULT#{report_id} — poll/quiz/follow aggregations, 1:1 with report
    SpaceAnalyzeDiscussionResult(String, String), // SPACE_ANALYZE_DISCUSSION_RESULT#{report_id}#{discussion_id_and_request_uuid} — second field is "{discussion_id}#{request_uuid}" composite so begins_with by (report_id, discussion_id) groups history together
    SpaceDiscussion(String),
    SpaceDiscussionMember(String, String),
    SpaceDiscussionParticipant(String, String),

    SpaceRecommendation,
    SpaceReport(String), // SPACE_REPORT#{uuid} — one row per saved report on the space
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

    Notification(String),           // notification id
    SpaceStatusChangeEvent(String), // uuid_v7 (same id as pk)

    //
    SpaceAction,
    Reward,
    SpaceReward,

    /// Sort key for the singleton AnalyzeQuotaConfig row
    /// (`pk = AnalyzeQuotaConfig`).
    AnalyzeQuotaConfig,

    ContentReport,

    Category(String), // CATEGORY#${name}

    TestAccount(String), // TEST_ACCOUNT#${email}

    TimelineEntry(String), // TIMELINE_ENTRY#${timestamp}#${post_pk_inner}

    // AI Moderator
    AiModeratorConfig,
    AiModeratorMaterial(String), // AIMODMATERIAL#{material_id}

    // MCP
    McpClientSecret,

    // Activity
    SpaceActivity(String), // SPACE_ACTIVITY#action_id#timestamp
    SpaceScore,

    // Hot space ranking snapshot. PK: SPACE#{space_id}, SK: HotSpace.
    // Row carries denormalized counts + WindowedRankKey on gsi1 for global
    // ranking. Single shared stream — every viewer sees the same Hot tab.
    HotSpace,

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
    SubTeamLink(String),     // SUB_TEAM_LINK#{child_team_id}
    SubTeamDocument(String), // SUB_TEAM_DOCUMENT#{doc_id}
    /// Immutable snapshot of a `SubTeamDocument` at a specific version.
    /// Second segment is the version **zero-padded to 8 digits** so the
    /// lexicographic sk order matches numeric order (v1 < v2 < … < v10).
    SubTeamDocumentVersion(String, String), // SUB_TEAM_DOCUMENT_VERSION#{doc_id}#{version:08}
    SubTeamDocAgreement(String, String), // SUB_TEAM_DOC_AGREEMENT#{app_id}#{doc_id}
    SubTeamFormField(String), // SUB_TEAM_FORM_FIELD#{field_id}
    SubTeamApplication(String), // SUB_TEAM_APPLICATION#{application_id}
    /// In-progress (not yet submitted) sub-team application body —
    /// stored under the *applicant* team pk and keyed by the parent
    /// team id so a single applicant can hold one draft per parent
    /// they're considering.
    SubTeamApplicationDraft(String), // SUB_TEAM_APPLICATION_DRAFT#{parent_team_id}
    SubTeamAnnouncement(String), // SUB_TEAM_ANNOUNCEMENT#{announcement_id}
    /// Lightweight marker that exposes a parent's anchor announcement
    /// Post on a recognized child team's wall WITHOUT cloning the Post
    /// row. One row per (child team, announcement). Listed by
    /// `list_team_posts_handler` for the child team; each marker's
    /// `anchor_post_pk` resolves to the parent's single anchor `Post`,
    /// so likes / comments / shares accumulate on ONE row across every
    /// child, the URL is the same anchor URL everywhere, and the parent
    /// admin sees the full reception on their own anchor detail page.
    /// pk = TEAM#{child_team_id}, this sk encodes the announcement id.
    SubTeamAnnouncementFanout(String), // SUB_TEAM_ANNOUNCEMENT_FANOUT#{announcement_id}

    // Cross-posting feature (Phase 1: Bluesky / LinkedIn / Threads). All entities
    // share an existing Partition variant (User or Feed) — no new Partition.
    SocialConnection(String),       // pk=User(user_id), inner=platform.to_string()
    SyndicationDirective,           // pk=Feed(post_id), singleton per published post
    SyndicationJob(String),         // pk=Feed(post_id), inner=platform.to_string()
    EngagementSnapshot(String),     // pk=Feed(post_id), inner=platform.to_string()
    UserOnboardingFlags,            // pk=User(user_id), singleton per user

    // Ratel Arcade — *Fact or Fold*. v1 PR1 only registers subject + settings;
    // round/participant/bet/rationale/chat/settlement entries are added in PR3+.
    FactFoldSubject(String),        // pk=FactFoldSubjects, inner=subject_id
    FactFoldSettings,               // pk=FactFoldSettings (singleton)

    // PR3 — round + lobby. Per-participant rows (bets, rationales,
    // chat, settlements) come in PR4+; the Round itself carries the
    // participant_pks list for the join/leave + capacity check.
    FactFoldRound(String),          // pk=FactFold(round_id), inner=round_id
    FactFoldLobby,                  // pk=FactFoldLobbySingleton (singleton)

    // PR4 — per-participant round state. All keyed by user_id under
    // the same Partition::FactFold(round_id) as the Round itself, so
    // a single sk-prefix query lists everything for a round.
    FactFoldParticipant(String),    // pk=FactFold(round_id), inner=user_id
    FactFoldBet(String),            // pk=FactFold(round_id), inner=user_id
    FactFoldRationale(String),      // pk=FactFold(round_id), inner=user_id

    /// One row per chat message in a round. inner = uuid_v7 (time-
    /// sortable) so an sk-prefix query returns the chat log in
    /// chronological order. A DDB Stream filter on this sk prefix
    /// drives the SSE fan-out (PR4f).
    FactFoldChat(String),           // pk=FactFold(round_id), inner=msg_id

    // PR6 — settlement.
    /// One row per (round, user). Written by the settlement
    /// handler with the §FR-28~30 breakdown. `idempotency_key =
    /// round_id#user_id` ensures retries are no-ops.
    FactFoldSettlement(String),     // pk=FactFold(round_id), inner=user_id

    /// Per-user lifetime statistics. Lives under
    /// `Partition::User(user_pk)` so a single user query reads it
    /// alongside other user-scoped rows.
    FactFoldUserStats,              // pk=User(user_pk) (singleton sk)

    /// Marker row written when a user joins a round, used to enforce
    /// "one game per active subject window" — `pick_next_subject`
    /// checks for the row's absence before letting the user matchmake
    /// into a round bound to a given subject. Inner = subject_id.
    /// pk = User(user_pk).
    FactFoldSubjectPlay(String),

    /// Leaderboard entry row (PR7). inner = `{accuracy_bps:010}#{user_id}`
    /// — zero-padded basis-points accuracy followed by user id, so
    /// an sk-descending query at `Partition::FactFoldLeaderboard`
    /// returns top users first. Updated as a side effect of
    /// settlement.
    FactFoldLeaderboardEntry(String),

    // Ratel Arcade — chip wallet (PR4b).
    /// Singleton balance row under `Partition::ArcadeWallet(user_id)`.
    /// One row per user; carries `chip_balance` + `last_updated`.
    ArcadeWalletBalance,            // pk=ArcadeWallet(user_id) (singleton sk)
    /// Per-transaction ledger row under `Partition::ArcadeWallet(user_id)`.
    /// inner=ulid (time-sortable) so a single sk-prefix query lists
    /// transactions in chronological order.
    ArcadeWalletTxn(String),        // pk=ArcadeWallet(user_id), inner=txn_id

    /// Singleton arcade-wide settings (chip↔RP ratio, default buy-in,
    /// ...). Pairs with `Partition::ArcadeSettings`.
    ArcadeSettings,                 // pk=ArcadeSettings (singleton)
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
