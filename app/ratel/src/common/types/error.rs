use crate::{common::*, spaces::SpaceError};
use dioxus::fullstack::Loading;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate)]
pub enum Error {
    // NOTE: Built-in errors for Some macros
    #[error("Invalid partition key: {0}")]
    #[translate(en = "No data found", ko = "데이터를 찾을 수 없습니다.")]
    InvalidPartitionKey(String),

    #[error("Unauthorized access")]
    #[translate(en = "Unauthorized access", ko = "인증되지 않은 접근입니다.")]
    UnauthorizedAccess,
    #[error("Bookmark is invalid")]
    #[translate(en = "Please refresh the page", ko = "페이지를 새로고침 해주세요.")]
    InvalidBookmark,

    #[error("No session found")]
    #[translate(en = "Please sign in first", ko = "먼저 로그인 해주세요.")]
    NoSessionFound,

    // `#[serde(skip)]` here would make `serde_json::to_string(&Error::Aws(_))`
    // fail with "the enum variant Error::Aws cannot be serialized", which then
    // panics at `dioxus-fullstack-0.7.4/src/magic.rs:679` (unwrap on the
    // serializer Result) and kills the spawn_pinned worker thread. Instead we
    // emit the inner `to_string()` payload so serialization always succeeds.
    // The variant only exists on server (cfg-gated), and clients deserialize
    // the response body as `serde_json::Value` (see dioxus-fullstack
    // request.rs:64), so the unknown "Aws" tag is harmless on the wire.
    #[cfg(feature = "server")]
    #[serde(serialize_with = "serialize_aws_error", skip_deserializing)]
    #[error("AWS error: {0}")]
    #[translate(
        en = "Internal server error",
        ko = "서버 내부 오류가 발생하였습니다. 잠시 후 다시 시도해주세요."
    )]
    Aws(#[from] crate::common::utils::aws::error::AwsError),

    #[cfg(feature = "server")]
    #[serde(serialize_with = "serialize_session_error", skip_deserializing)]
    #[error("Session error: {0}")]
    #[translate(en = "Please sign in first", ko = "먼저 로그인 해주세요.")]
    Session(#[from] tower_sessions::session::Error),

    #[error("Username already exists")]
    #[translate(
        en = "Username already exists",
        ko = "이미 존재하는 사용자 이름입니다."
    )]
    UsernameAlreadyExists,

    #[error("Duplicate entry: {0}")]
    #[translate(en = "Duplicate entry", ko = "중복된 항목입니다.")]
    Duplicate(String),

    #[error("Not found: {0}")]
    #[translate(en = "Not found", ko = "찾을 수 없습니다.")]
    NotFound(String),

    #[error("Invitation not found")]
    #[translate(en = "Invitation not found", ko = "초대 항목을 찾을 수 없습니다.")]
    InvitationNotFound,

    #[error("User has no permission")]
    #[translate(en = "No permission", ko = "권한이 없습니다.")]
    NoPermission,

    #[error("Exceeded maximum attempt for email verification")]
    #[translate(
        en = "Exceeded maximum attempt for email verification",
        ko = "이메일 인증 최대 시도 횟수를 초과했습니다."
    )]
    ExceededAttemptEmailVerification,

    #[error("Exceeded maximum attempt for phone verification")]
    #[translate(
        en = "Exceeded maximum attempt for phone verification",
        ko = "전화 인증 최대 시도 횟수를 초과했습니다."
    )]
    ExceededAttemptPhoneVerification,

    #[error("Send SMS Failed: {0}")]
    #[translate(en = "Send SMS Failed", ko = "SMS 전송에 실패했습니다.")]
    SendSmsFailed(String),

    #[error("User participation is blocked for this space")]
    #[translate(en = "Participation is blocked", ko = "참여가 제한되어 있습니다.")]
    ParticipationBlocked,

    #[error("User lacks verified attributes required for participation")]
    #[translate(en = "Missing verified attributes", ko = "필수 인증 속성이 없습니다.")]
    LackOfVerifiedAttributes,

    #[error("Quota is full")]
    #[translate(en = "Quota is full", ko = "정원이 초과되었습니다.")]
    FullQuota,

    #[error("User is already participating in the space")]
    #[translate(en = "Already participating", ko = "이미 참여 중입니다.")]
    AlreadyParticipating,

    #[error("Participation is not open")]
    #[translate(
        en = "Participation is only available while the space is open.",
        ko = "참여는 스페이스가 열려 있는 동안만 가능합니다."
    )]
    ParticipationNotOpen,

    #[error("not found verification code")]
    #[translate(
        en = "Verification code not found",
        ko = "인증 코드를 찾을 수 없습니다."
    )]
    NotFoundVerificationCode,
    #[error("verification code is expired")]
    #[translate(
        en = "Verification code is expired",
        ko = "인증 코드가 만료되었습니다."
    )]
    ExpiredVerification,
    #[error("invalid verification code")]
    #[translate(
        en = "Invalid verification code",
        ko = "인증 코드가 유효하지 않습니다."
    )]
    InvalidVerificationCode,

    #[error("not found space")]
    #[translate(en = "Not found space", ko = "스페이스를 찾을 수 없습니다.")]
    SpaceNotFound,
    #[error("not found author")]
    #[translate(
        en = "Not found space author",
        ko = "스페이스 작성자를 찾을 수 없습니다."
    )]
    InvalidSpaceAuthor,

    #[error("discussion is not in progress")]
    #[translate(
        en = "Discussion is not in progress",
        ko = "토론이 진행중 상태가 아닙니다."
    )]
    DiscussionNotInProgress,

    #[error("space post ended")]
    #[translate(en = "Space post ended", ko = "스페이스 게시글이 종료되었습니다.")]
    SpacePostEnded,

    #[error("space post contents is too short")]
    #[translate(
        en = "Space post contents must be at least 10 characters long.",
        ko = "스페이스 게시글의 내용은 10자 이상이어야 합니다."
    )]
    ValidationTooShortContents,

    // DAO-related
    #[error("At least 3 admins must be selected")]
    #[translate(
        en = "At least 3 admins must be selected",
        ko = "최소 3명의 관리자를 선택해야 합니다."
    )]
    InsufficientAdmins,

    #[error("Transaction cancelled: You rejected the transaction")]
    #[translate(
        en = "Transaction cancelled: You rejected the transaction",
        ko = "트랜잭션 취소: 트랜잭션을 거부했습니다."
    )]
    TransactionRejected,

    #[error("MetaMask not installed")]
    #[translate(
        en = "MetaMask not installed. Please install MetaMask to continue",
        ko = "MetaMask가 설치되어 있지 않습니다. 계속하려면 MetaMask를 설치하세요."
    )]
    MetamaskNotInstalled,

    #[error("Wallet error: {0}")]
    #[translate(en = "Wallet error", ko = "지갑 오류가 발생했습니다.")]
    WalletError(String),

    // Membership-related
    #[error("Membership response missing")]
    #[translate(
        en = "Membership response missing",
        ko = "멤버십 응답이 누락되었습니다."
    )]
    MembershipResponseMissing,

    #[error("Failed to change membership")]
    #[translate(en = "Failed to change membership", ko = "멤버십 변경에 실패했습니다.")]
    MembershipChangeFailed,

    #[error("Membership already active")]
    #[translate(
        en = "Membership is already active",
        ko = "멤버십이 이미 활성화되어 있습니다."
    )]
    MembershipAlreadyActive,

    // Credential-related
    #[error("Invalid verification code input")]
    #[translate(
        en = "Invalid verification code",
        ko = "인증 코드가 유효하지 않습니다."
    )]
    InvalidCodeInput,

    #[error("Failed to response quiz")]
    #[translate(en = "Failed to submit response.", ko = "응답 제출에 실패했습니다.")]
    QuizResponseFailed,

    // PortOne related errors
    #[error("PortOne Inicis returned invalid identity")]
    #[translate(en = "Failed network request", ko = "요청에 실패했습니다.")]
    PortOneRequestFailure,

    #[error("PortOne Inicis returned invalid identity")]
    #[translate(en = "Failed to verify KYC", ko = "본인 인증에 실패했습니다.")]
    PortOneInicisInvalidIdentity,

    #[error("{0}")]
    #[translate(from)]
    Follow(#[from] crate::features::my_follower::types::FollowError),

    #[error("{0}")]
    #[translate(from)]
    Space(#[from] SpaceError),

    #[error("{0}")]
    #[translate(from)]
    SpaceReward(#[from] SpaceRewardError),

    #[error("{0}")]
    #[translate(from)]
    SpaceActionQuiz(
        #[from] crate::features::spaces::pages::actions::actions::quiz::SpaceActionQuizError,
    ),

    #[error("{0}")]
    #[translate(from)]
    SpaceActionDiscussion(
        #[from]
        crate::features::spaces::pages::actions::actions::discussion::SpaceActionDiscussionError,
    ),

    #[error("{0}")]
    #[translate(from)]
    ExchangePoints(
        #[from] crate::features::social::pages::user_reward::controllers::ExchangePointsError,
    ),

    #[error("{0}")]
    #[translate(from)]
    Member(#[from] crate::features::social::pages::member::types::MemberError),

    #[error("{0}")]
    #[translate(from)]
    SpaceStatusChange(
        #[from] crate::features::spaces::space_common::types::SpaceStatusChangeError,
    ),

    // Post related errors
    #[error("Invalid username")]
    #[translate(
        en = "Invalid username. Check URL.",
        ko = "유효하지 않은 사용자 이름입니다. URL을 확인해주세요."
    )]
    PostInvalidUsername,

    #[error("Web function")]
    #[translate(
        en = "This function is only available on web.",
        ko = "이 기능은 웹에서만 사용할 수 있습니다."
    )]
    OnlyWebFunction,
    #[error("Invalid email")]
    #[translate(en = "Invalid email", ko = "유효하지 않은 이메일입니다.")]
    InvalidEmail,

    #[error("invalid space action")]
    #[translate(
        en = "Please delete and re-create the action",
        ko = "액션을 삭제하고 다시 만들어주세요."
    )]
    SpaceActionNotFound,

    #[error("Space is not started")]
    #[translate(
        en = "Space is not started yet",
        ko = "스페이스가 아직 시작되지 않았습니다."
    )]
    SpaceNotStarted,

    #[error("Action has ended")]
    #[translate(en = "This action has ended", ko = "이 액션은 종료되었습니다.")]
    ActionEnded,

    #[error("Action is locked")]
    #[translate(
        en = "This action can no longer be modified after it has started.",
        ko = "액션이 시작된 이후에는 변경할 수 없습니다."
    )]
    ActionLocked,

    #[error("{0}")]
    #[translate(from)]
    McpServer(#[from] McpServerError),

    #[error("{0}")]
    #[translate(from)]
    AiModerator(#[from] crate::features::ai_moderator::types::AiModeratorError),

    #[error("{0}")]
    #[translate(from)]
    Activity(#[from] crate::features::activity::types::ActivityError),

    // Unit variants for common errors
    #[error("Internal error")]
    #[translate(
        en = "An unexpected error occurred",
        ko = "예기치 않은 오류가 발생했습니다."
    )]
    Internal,

    #[error("Unsupported operation")]
    #[translate(
        en = "This operation is not supported",
        ko = "지원되지 않는 작업입니다."
    )]
    UnsupportedOperation,

    #[error("Missing space ID")]
    #[translate(en = "Space ID is required", ko = "스페이스 ID가 필요합니다.")]
    MissingSpaceId,

    #[error("Invalid format")]
    #[translate(en = "Invalid data format", ko = "잘못된 데이터 형식입니다.")]
    InvalidFormat,

    #[error("Invalid team context")]
    #[translate(
        en = "Invalid team context",
        ko = "유효하지 않은 팀 컨텍스트입니다."
    )]
    InvalidTeamContext,

    #[error("User not found in context")]
    #[translate(en = "User not found", ko = "사용자를 찾을 수 없습니다.")]
    UserNotFoundInContext,

    #[error("Space role check failed")]
    #[translate(
        en = "Authorization check failed",
        ko = "권한 확인에 실패했습니다."
    )]
    SpaceUserRoleFailed,

    // Feature-specific error enums
    #[error("{0}")]
    #[translate(from)]
    Auth(#[from] crate::features::auth::types::AuthError),

    #[error("{0}")]
    #[translate(from)]
    Post(#[from] crate::features::posts::types::PostError),

    #[error("{0}")]
    #[translate(from)]
    Social(#[from] crate::features::social::types::SocialError),

    #[error("{0}")]
    #[translate(from)]
    MembershipPayment(
        #[from] crate::features::membership::types::MembershipPaymentError,
    ),

    #[error("{0}")]
    #[translate(from)]
    Timeline(#[from] crate::features::timeline::types::TimelineError),

    #[error("{0}")]
    #[translate(from)]
    Admin(#[from] crate::features::admin::types::AdminError),

    #[error("{0}")]
    #[translate(from)]
    Service(#[from] crate::common::services::ServiceError),

    #[error("{0}")]
    #[translate(from)]
    Infra(#[from] crate::common::utils::InfraError),

    #[error("{0}")]
    #[translate(from)]
    FileUpload(#[from] crate::common::components::file_uploader::FileUploadError),

    #[error("{0}")]
    #[translate(from)]
    SpaceAction(
        #[from] crate::features::spaces::pages::actions::types::SpaceActionError,
    ),

    #[error("{0}")]
    #[translate(from)]
    SpacePoll(
        #[from]
        crate::features::spaces::pages::actions::actions::poll::types::SpacePollError,
    ),

    #[error("{0}")]
    #[translate(from)]
    SpaceFollow(
        #[from]
        crate::features::spaces::pages::actions::actions::follow::types::SpaceFollowError,
    ),

    #[error("{0}")]
    #[translate(from)]
    SpaceApp(#[from] crate::features::spaces::pages::apps::types::SpaceAppError),

    #[error("{0}")]
    #[translate(from)]
    SpaceReport(
        #[from] crate::features::spaces::pages::report::types::SpaceReportError,
    ),
}

#[cfg(feature = "server")]
impl From<qdrant_client::QdrantError> for Error {
    fn from(e: qdrant_client::QdrantError) -> Self {
        tracing::error!("Qdrant error: {e}");
        Error::Infra(crate::common::utils::InfraError::QdrantFailed)
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(e: std::convert::Infallible) -> Self {
        match e {}
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        tracing::error!("Untyped string error: {s}");
        Error::Internal
    }
}

impl From<base64::DecodeError> for Error {
    fn from(e: base64::DecodeError) -> Self {
        tracing::error!("Base64 decode error: {e}");
        Error::Internal
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for Error {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::http::StatusCode;
        use bdk::prelude::axum::response::IntoResponse;

        let status = match &self {
            Error::UnauthorizedAccess
            | Error::NoSessionFound
            | Error::UserNotFoundInContext => StatusCode::UNAUTHORIZED,
            Error::InvalidPartitionKey(_)
            | Error::InvalidBookmark
            | Error::Duplicate(_)
            | Error::NoPermission
            | Error::ParticipationBlocked
            | Error::LackOfVerifiedAttributes
            | Error::FullQuota
            | Error::AlreadyParticipating
            | Error::ParticipationNotOpen
            | Error::UnsupportedOperation
            | Error::MissingSpaceId
            | Error::InvalidFormat
            | Error::InvalidTeamContext => StatusCode::BAD_REQUEST,
            Error::NotFound(_) | Error::InvitationNotFound => StatusCode::NOT_FOUND,
            Error::Follow(e) => e.status_code(),
            Error::SpaceReward(e) => e.status_code(),
            Error::SpaceActionQuiz(e) => e.status_code(),
            Error::SpaceActionDiscussion(e) => e.status_code(),
            Error::ExchangePoints(e) => e.status_code(),
            Error::McpServer(e) => e.status_code(),
            Error::Member(e) => e.status_code(),
            Error::SpaceStatusChange(e) => e.status_code(),
            Error::AiModerator(e) => e.status_code(),
            Error::Activity(e) => e.status_code(),
            Error::Auth(e) => e.status_code(),
            Error::Post(e) => e.status_code(),
            Error::Social(e) => e.status_code(),
            Error::MembershipPayment(e) => e.status_code(),
            Error::Timeline(e) => e.status_code(),
            Error::Admin(e) => e.status_code(),
            Error::Service(e) => e.status_code(),
            Error::Infra(e) => e.status_code(),
            Error::FileUpload(e) => e.status_code(),
            Error::SpaceAction(e) => e.status_code(),
            Error::SpacePoll(e) => e.status_code(),
            Error::SpaceFollow(e) => e.status_code(),
            Error::SpaceApp(e) => e.status_code(),
            Error::SpaceReport(e) => e.status_code(),
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl From<aws_sdk_dynamodb::Error> for Error {
    fn from(e: aws_sdk_dynamodb::Error) -> Self {
        Error::Aws(crate::common::utils::aws::error::AwsError::from(e))
    }
}

#[cfg(feature = "server")]
impl From<serde_dynamo::Error> for Error {
    fn from(e: serde_dynamo::Error) -> Self {
        Error::Aws(crate::common::utils::aws::error::AwsError::from(e))
    }
}

#[cfg(feature = "server")]
impl From<Error> for rmcp::ErrorData {
    fn from(e: Error) -> Self {
        match &e {
            Error::UnauthorizedAccess
            | Error::NoSessionFound
            | Error::UserNotFoundInContext => {
                rmcp::ErrorData::invalid_request(e.to_string(), None)
            }
            Error::NotFound(_) | Error::InvitationNotFound | Error::SpaceNotFound => {
                rmcp::ErrorData::invalid_params(e.to_string(), None)
            }
            Error::Duplicate(_)
            | Error::NoPermission
            | Error::InvalidPartitionKey(_)
            | Error::McpServer(_)
            | Error::UnsupportedOperation
            | Error::MissingSpaceId
            | Error::InvalidFormat
            | Error::InvalidTeamContext => {
                rmcp::ErrorData::invalid_params(e.to_string(), None)
            }
            _ => {
                tracing::error!("MCP internal error: {e}");
                rmcp::ErrorData::internal_error(
                    "An internal server error occurred.".to_string(),
                    None,
                )
            }
        }
    }
}

impl From<ServerFnError> for Error {
    fn from(e: ServerFnError) -> Self {
        tracing::error!("Server function error: {e}");
        Error::Internal
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for Error {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            Error::UnauthorizedAccess
            | Error::NoSessionFound
            | Error::UserNotFoundInContext => StatusCode::UNAUTHORIZED,
            Error::InvalidPartitionKey(_)
            | Error::InvalidBookmark
            | Error::Duplicate(_)
            | Error::NoPermission
            | Error::ParticipationBlocked
            | Error::LackOfVerifiedAttributes
            | Error::FullQuota
            | Error::AlreadyParticipating
            | Error::ParticipationNotOpen
            | Error::ExceededAttemptEmailVerification
            | Error::ExceededAttemptPhoneVerification
            | Error::SendSmsFailed(_)
            | Error::NotFoundVerificationCode
            | Error::ExpiredVerification
            | Error::InvalidVerificationCode
            | Error::UnsupportedOperation
            | Error::MissingSpaceId
            | Error::InvalidFormat
            | Error::InvalidTeamContext => StatusCode::BAD_REQUEST,
            Error::NotFound(_) | Error::InvitationNotFound => StatusCode::NOT_FOUND,
            Error::Follow(e) => e.status_code(),
            Error::SpaceReward(e) => e.status_code(),
            Error::SpaceActionQuiz(e) => e.status_code(),
            Error::SpaceActionDiscussion(e) => e.status_code(),
            Error::ExchangePoints(e) => e.status_code(),
            Error::McpServer(e) => e.status_code(),
            Error::Member(e) => e.status_code(),
            Error::AiModerator(e) => e.status_code(),
            Error::Activity(e) => e.status_code(),
            Error::Auth(e) => e.status_code(),
            Error::Post(e) => e.status_code(),
            Error::Social(e) => e.status_code(),
            Error::MembershipPayment(e) => e.status_code(),
            Error::Timeline(e) => e.status_code(),
            Error::Admin(e) => e.status_code(),
            Error::Service(e) => e.status_code(),
            Error::Infra(e) => e.status_code(),
            Error::FileUpload(e) => e.status_code(),
            Error::SpaceAction(e) => e.status_code(),
            Error::SpacePoll(e) => e.status_code(),
            Error::SpaceFollow(e) => e.status_code(),
            Error::SpaceApp(e) => e.status_code(),
            Error::SpaceReport(e) => e.status_code(),
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// Note: From<Error> for lambda_runtime::Error (= Box<dyn std::error::Error + Send + Sync>)
// is provided by the blanket impl in std, since Error implements std::error::Error + Send + Sync.
// This preserves the full error source chain for Lambda debugging, unlike the previous
// to_string()-based conversion.

#[cfg(feature = "server")]
fn serialize_aws_error<S>(
    err: &crate::common::utils::aws::error::AwsError,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&err.to_string())
}

#[cfg(feature = "server")]
fn serialize_session_error<S>(
    err: &tower_sessions::session::Error,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&err.to_string())
}
