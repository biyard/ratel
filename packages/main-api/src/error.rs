use bdk::prelude::*;
use ssi::dids::InvalidDID;
use thiserror::Error;

#[derive(Debug, Error, RestError, aide::OperationIo)]
pub enum Error {
    #[error("Unknown")]
    #[rest_error(code = 1)]
    Unknown(String),

    #[error("DynamoDB error: {0}")]
    #[rest_error(code = 100)]
    DynamoDbError(#[from] aws_sdk_dynamodb::Error),
    #[error("AWS Ses error: {0}")]
    #[rest_error(status = 500)]
    SesServiceError(#[from] crate::utils::aws::ses::SesServiceError),
    #[error("SerdeDynamo error: {0}")]
    #[rest_error(status = 500)]
    SerdeDynamo(#[from] serde_dynamo::Error),
    #[error("SerdeJson error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Validation error: {0}")]
    ValidationError(#[from] validator::ValidationError),
    #[error("Session error")]
    SessionError(#[from] tower_sessions::session::Error),
    #[error("Invalid partition key: {0}")]
    InvalidPartitionKey(String),
    #[error("Item not found: {0}")]
    #[rest_error(status = 404)]
    NotFound(String),
    #[error("Item already exists: {0}")]
    AlreadyExists(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Unauthorized: {0}")]
    #[rest_error(status = 401)]
    Unauthorized(String),
    #[error("Internal server error: {0}")]
    #[rest_error(status = 500)]
    InternalServerError(String),
    #[error("Duplicate entry: {0}")]
    Duplicate(String),
    #[error("Aws chime error: {0}")]
    AwsChimeError(String),
    #[error("Other error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Validation errors: {0}")]
    ValidationErrors(#[from] validator::ValidationErrors),
    #[error("Decoding error: {0}")]
    Utf8Decoding(#[from] std::str::Utf8Error),
    #[error("Misconfiguration: {0}")]
    Misconfiguration(String),
    #[error("Operation not supported: {0}")]
    NotSupported(String),
    #[error("The item has dependencies and cannot be deleted: {0:?}")]
    HasDependencies(Vec<String>),
    #[error("Bookmark is invalid")]
    InvalidBookmark,
    #[error("Base64 decode error: {0}")]
    Base64Error(#[from] base64::DecodeError),
    #[error("Klaytn error: {0}")]
    Klaytn(String),
    #[error("AWS S3 error: {0}")]
    AwsS3Error(String),
    #[error("AWS MediaConvert error: {0}")]
    AwsMediaPipelinesError(String),
    #[error("Kaia Wallet error: {0}")]
    AssetError(String),
    #[error("Textract error: {0}")]
    AwsTextractError(String),
    #[error("Server error: {0}")]
    ServerError(String),
    #[error("AWS Rekognition error: {0}")]
    AwsRekognitionError(String),
    #[error("AWS Bedrock error: {0}")]
    AwsBedrockError(String),
    #[error("HMac initialization error: {0}")]
    HMacInitError(String),
    #[error("Telegram wallet error: {0}")]
    TelegramError(#[from] teloxide::RequestError),
    #[error("Chrono parse error: {0}")]
    TimeParseError(#[from] chrono::ParseError),

    // Authorization errors 400 ~
    #[error("No session found")]
    #[rest_error(status = 401, code = 400)]
    NoSessionFound,
    #[error("No user found in session")]
    #[rest_error(status = 401, code = 401)]
    NoUserFound,
    #[error("No permission to access this resource")]
    #[rest_error(status = 401, code = 403)]
    NoPermission,
    #[error("Wallet error: {0}")]
    WalletError(String),
    #[error("User is not an admin")]
    #[rest_error(status = 403, code = 404)]
    UserNotAdmin,
    #[error("User is already an admin")]
    #[rest_error(status = 400, code = 405)]
    UserAlreadyAdmin,
    #[error("Invalid resource")]
    InvalidResource,

    // /v3/auth endpoints 1000 ~
    #[error("Exceeded maximum attempt for email verification")]
    #[rest_error(code = 1000)]
    ExceededAttemptEmailVerification,
    #[error("Exceeded maximum attempt for phone verification")]
    #[rest_error(code = 1001)]
    ExceededAttemptPhoneVerification,
    #[error("Failed to send email via AWS SES: {0}")]
    AwsSesSendEmailException(String),
    #[error("Verification code not found or expired")]
    NotFoundVerificationCode,
    #[error("Verification code has expired")]
    ExpiredVerification,
    #[error("Invalid verification code")]
    InvalidVerificationCode,
    #[error("Send SMS Failed: {0}")]
    SendSmsFailed(String),

    // /v3/posts endpoints 2000 ~
    #[error("Post visibility is incorrectly configured: {0}")]
    #[rest_error(code = 2000)]
    PostIncorrectConfiguredVisibility(String),
    #[error("Post not found")]
    #[rest_error(status = 404)]
    PostNotFound,
    #[error("Failed to like/unlike the post")]
    PostLikeError,
    #[error("Failed to comment on the post")]
    PostCommentError,
    #[error("Failed to reply to the comment")]
    PostReplyError,
    #[error("Failed to report post")]
    PostReportError,
    #[error("Failed to report space")]
    SpaceReportError,
    #[error("Failed to report space post")]
    SpacePostReportError,
    #[error("Failed to report space post comment")]
    SpacePostCommentReportError,
    #[error("Failed to report post comment")]
    PostCommentReportError,

    // /v3/spaces endpoints 3000 ~
    #[error("Space not found")]
    #[rest_error(code = 3000)]
    SpaceNotFound,
    #[error("InvalidTimeRange")]
    InvalidTimeRange,
    #[error("Space cannot be edited in its current status")]
    SpaceNotEditable,
    #[error("PK must be a Partition::Space")]
    InvalidSpacePartitionKey,
    #[error("Space requirements are invalid")]
    SpaceInvalidRequirements,
    #[error("User participation is blocked for this space")]
    ParticipationBlocked,
    #[error("User lacks verified attributes required for participation")]
    LackOfVerifiedAttributes,
    #[error("Quota is full")]
    FullQuota,

    // members feature 3050 ~
    #[rest_error(code = 3050)]
    #[error("Member not found")]
    NoInvitationFound,
    #[error("User is already participating in the space")]
    AlreadyParticipating,

    // /v3/spaces/deliberations endpoints 3100 ~
    #[rest_error(code = 3100)]
    #[error("Deliberation space not found")]
    NotFoundDeliberationSpace,

    // poll features errors 3200 ~
    #[rest_error(code = 3200)]
    #[error("Poll Not found")]
    NotFoundPoll,
    #[error("Poll is not in progress")]
    PollNotInProgress,
    #[error("questions are invalid")]
    PollInvalidQuestions,
    #[error("Answers do not match with questions")]
    PollAnswersMismatchQuestions,
    #[error("Poll cannot be updated in its current status")]
    ImmutablePollState,
    #[error("User cannot update answer")]
    ImmutablePollUserAnswer,
    #[error("Poll Result not found")]
    NotFoundPollResult,
    #[error("User is not a participant in the space")]
    UserNotParticipant,

    #[rest_error(code = 3300)]
    #[error("Sprint League not found")]
    NotFoundSprintLeague,
    #[error("Sprint League players are invalid")]
    InvalidSprintLeaguePlayer,
    #[error("Failed to vote in sprint league: {0}")]
    SprintLeagueVoteError(String),

    // teams 4000 ~
    #[error("Team not found")]
    #[rest_error(status = 404, code = 4000)]
    TeamNotFound,

    // /v3/spaces endpoints 5000 ~
    #[rest_error(code = 5000)]
    #[error("space not found")]
    NotFoundSpace,
    #[error("already published space")]
    AlreadyPublishedSpace,
    #[error("not published space")]
    NotPublishedSpace,
    #[error("finished space")]
    FinishedSpace,

    // /v3/discussions endpoints 6000 ~
    #[rest_error(code = 6000)]
    #[error("discussion not found")]
    NotFoundDiscussion,

    // membership endpoints 7000 ~
    #[error("Insufficient credits")]
    #[rest_error(status = 400, code = 7000)]
    InsufficientCredits,
    #[error("Membership may be expired")]
    ExpiredMembership,
    #[error("User membership not found")]
    NoUserMembershipFound,
    #[error("Membership not found")]
    NoMembershipFound,
    #[error("Membership already active")]
    MembershipAlreadyActive,
    #[error("Invalid membership tier")]
    #[rest_error(status = 400, code = 7001)]
    InvalidMembershipTier,
    #[error("Invalid Membership")]
    InvalidMembership,

    // /v3/panels endpoints 8000 ~
    #[rest_error(code = 8000)]
    #[error("panel not found")]
    NotFoundPanel,
    #[error("already participate user")]
    AlreadyParticipateUser,
    #[error("already full panel")]
    AlreadyFullPanel,
    #[error("invalid panel")]
    InvalidPanel,
    #[error("invalid panel quota")]
    InvalidPanelQuota,

    // NFT Artwork space errors
    #[rest_error(code = 9000)]
    #[error("Artwork not found")]
    NotFoundArtwork,
    #[rest_error(code = 9001)]
    #[error("Artwork not found")]
    ArtworkNotFound,
    #[rest_error(code = 9002)]
    #[error("Artwork already minted")]
    ArtworkAlreadyMinted,
    #[rest_error(code = 9003)]
    #[error("Artwork owner mismatch")]
    ArtworkOwnerMismatch,
    #[rest_error(code = 9004)]
    #[error("Contract address not configured")]
    ContractAddressNotConfigured,
    #[rest_error(code = 9005)]
    #[error("JSON serialization error: {0}")]
    JsonError(String),
    #[error("Metadata for the artwork is missing or invalid")]
    ArtworkMetadataMissingOrInvalid,
    #[error("Invalid user evm address")]
    InvalidUserEvmAddress,

    // Payment errors 10,000 ~
    #[error("Invalid identification for payment")]
    #[rest_error(code = 10000)]
    InvalidIdentification,
    #[error("Card information is required for payment")]
    CardInfoRequired,
    #[error("No user purchase found for payment")]
    NoUserPurchaseFound,
    #[error("PortOne billing key error")]
    PortOneBillingKeyError,
    #[error("PortOne payment list error: {0}")]
    #[rest_error(status = 500)]
    PortOnePaymentListError(String),
    #[error("PortOne payment not found: {0}")]
    #[rest_error(status = 500)]
    PortOnePaymentNotFound(String),
    #[error("PortOne cancel payment error: {0}")]
    #[rest_error(status = 500)]
    PortOneCancelPaymentError(String),

    // Biyard API errors 10,050 ~
    #[error("Biyard error: {0}")]
    #[rest_error(code = 10050)]
    Biyard(#[from] crate::services::biyard::BiyardError),

    // Reward errors 10,100 ~
    #[error("Reward already claimed in this period")]
    #[rest_error(status = 400, code = 10100)]
    RewardAlreadyClaimedInPeriod,
    #[error("Reward not found")]
    RewardNotFound,
    #[error("Reward max claims reached")]
    RewardMaxClaimsReached,
    #[error("Reward max points reached")]
    RewardMaxPointsReached,
    #[error("User reward max claims reached")]
    RewardMaxUserClaimsReached,
    #[error("User reward max points reached")]
    RewardMaxUserPointsReached,
    #[error("Reward disabled")]
    #[rest_error(status = 500)]
    RewardDisabled,

    // DID feature errors 11,000 ~
    #[error("Invalid DID format")]
    #[rest_error(code = 11000)]
    InvalidDID(#[from] InvalidDID<String>),
    #[error("VC Signature error: {0}")]
    Signature(String),
    #[error("invalide gender")]
    InvalidGender,
    #[error("attribute code not found")]
    AttributeCodeNotFound,

    #[error("analyze not found")]
    AnalyzeNotFound,

    #[error("No permission to access this resource")]
    #[rest_error(status = 401, code = 11001)]
    Forbidden,

    // web 1,000,000 ~
    #[error("Web error: {0}")]
    #[rest_error(code = 1_000_000)]
    WebError(#[from] askama::Error),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Unknown(s)
    }
}
