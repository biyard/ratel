use crate::common::*;
use dioxus::fullstack::Loading;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate)]
pub enum Error {
    #[error("Unknown: {0}")]
    #[translate(en = "Unknown error", ko = "알수없는 에러가 발생하였습니다.")]
    Unknown(String),

    // NOTE: Built-in errors for Some macros
    #[error("Invalid partition key: {0}")]
    #[translate(en = "No data found", ko = "데이터를 찾을 수 없습니다.")]
    InvalidPartitionKey(String),

    #[error("Not supported: {0}")]
    #[translate(en = "Not supported", ko = "지원되지 않는 기능입니다.")]
    NotSupported(String),

    #[error("Unauthorized access")]
    #[translate(en = "Unauthorized access", ko = "인증되지 않은 접근입니다.")]
    UnauthorizedAccess,
    #[error("Unauthorized access: {0}")]
    #[translate(en = "Unauthorized access", ko = "인증되지 않은 접근입니다.")]
    Unauthorized(String),

    #[error("Internal server error: {0}")]
    #[translate(en = "Internal server error", ko = "서버 내부 오류가 발생하였습니다. 잠시 후 다시 시도해주세요.")]
    InternalServerError(String),

    #[error("Bookmark is invalid")]
    #[translate(en = "Please refresh the page", ko = "페이지를 새로고침 해주세요.")]
    InvalidBookmark,

    #[error("No session found")]
    #[translate(en = "Please sign in first", ko = "먼저 로그인 해주세요.")]
    NoSessionFound,

    #[cfg(feature = "server")]
    #[serde(skip)]
    #[error("AWS error: {0}")]
    #[translate(en = "Internal server error", ko = "서버 내부 오류가 발생하였습니다. 잠시 후 다시 시도해주세요.")]
    Aws(#[from] crate::common::utils::aws::error::AwsError),

    #[cfg(feature = "server")]
    #[serde(skip)]
    #[error("Session error: {0}")]
    #[translate(en = "Please sign in first", ko = "먼저 로그인 해주세요.")]
    Session(#[from] tower_sessions::session::Error),

    #[error("Bad request: {0}")]
    #[translate(en = "Bad request", ko = "잘못된 요청입니다.")]
    BadRequest(String),

    #[error("Duplicate entry: {0}")]
    #[translate(en = "Duplicate entry", ko = "중복된 항목입니다.")]
    Duplicate(String),

    #[error("Not found: {0}")]
    #[translate(en = "Not found", ko = "찾을 수 없습니다.")]
    NotFound(String),

    #[error("User has no permission")]
    #[translate(en = "No permission", ko = "권한이 없습니다.")]
    NoPermission,

    #[error("Exceeded maximum attempt for email verification")]
    #[translate(en = "Exceeded maximum attempt for email verification", ko = "이메일 인증 최대 시도 횟수를 초과했습니다.")]
    ExceededAttemptEmailVerification,

    #[error("Exceeded maximum attempt for phone verification")]
    #[translate(en = "Exceeded maximum attempt for phone verification", ko = "전화 인증 최대 시도 횟수를 초과했습니다.")]
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

    #[error("not found verification code")]
    #[translate(en = "Verification code not found", ko = "인증 코드를 찾을 수 없습니다.")]
    NotFoundVerificationCode,
    #[error("verification code is expired")]
    #[translate(en = "Verification code is expired", ko = "인증 코드가 만료되었습니다.")]
    ExpiredVerification,
    #[error("invalid verification code")]
    #[translate(en = "Invalid verification code", ko = "인증 코드가 유효하지 않습니다.")]
    InvalidVerificationCode,

    #[error("not found space")]
    #[translate(en = "Not found space", ko = "스페이스를 찾을 수 없습니다.")]
    SpaceNotFound,
    #[error("not found author")]
    #[translate(en = "Not found space author", ko = "스페이스 작성자를 찾을 수 없습니다.")]
    InvalidSpaceAuthor,

    #[error("discussion is not in progress")]
    #[translate(en = "Discussion is not in progress", ko = "토론이 진행중 상태가 아닙니다.")]
    DiscussionNotInProgress,

    #[error("space post ended")]
    #[translate(en = "Space post ended", ko = "스페이스 게시글이 종료되었습니다.")]
    SpacePostEnded,

    #[error("space post contents is too short")]
    #[translate(en = "Space post contents must be at least 10 characters long.", ko = "스페이스 게시글의 내용은 10자 이상이어야 합니다.")]
    ValidationTooShortContents,

    // DAO-related
    #[error("At least 3 admins must be selected")]
    #[translate(en = "At least 3 admins must be selected", ko = "최소 3명의 관리자를 선택해야 합니다.")]
    InsufficientAdmins,

    #[error("Transaction cancelled: You rejected the transaction")]
    #[translate(en = "Transaction cancelled: You rejected the transaction", ko = "트랜잭션 취소: 트랜잭션을 거부했습니다.")]
    TransactionRejected,

    #[error("MetaMask not installed")]
    #[translate(en = "MetaMask not installed. Please install MetaMask to continue", ko = "MetaMask가 설치되어 있지 않습니다. 계속하려면 MetaMask를 설치하세요.")]
    MetamaskNotInstalled,

    #[error("Wallet error: {0}")]
    #[translate(en = "Wallet error", ko = "지갑 오류가 발생했습니다.")]
    WalletError(String),

    // Membership-related
    #[error("Membership response missing")]
    #[translate(en = "Membership response missing", ko = "멤버십 응답이 누락되었습니다.")]
    MembershipResponseMissing,

    #[error("Failed to change membership")]
    #[translate(en = "Failed to change membership", ko = "멤버십 변경에 실패했습니다.")]
    MembershipChangeFailed,

    // Credential-related
    #[error("Invalid verification code input")]
    #[translate(en = "Invalid verification code", ko = "인증 코드가 유효하지 않습니다.")]
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
    SpaceReward(#[from] SpaceRewardError),

    #[error("{0}")]
    #[translate(from)]
    SpaceActionQuiz(#[from] crate::features::spaces::pages::actions::actions::quiz::SpaceActionQuizError),

    // Post related errors
    #[error("Invalid username")]
    #[translate(en = "Invalid username. Check URL.", ko = "유효하지 않은 사용자 이름입니다. URL을 확인해주세요.")]
    PostInvalidUsername,

    #[error("Web function")]
    #[translate(en = "This function is only available on web.", ko = "이 기능은 웹에서만 사용할 수 있습니다.")]
    OnlyWebFunction,
    #[error("Invalid email")]
    #[translate(en = "Invalid email", ko = "유효하지 않은 이메일입니다.")]
    InvalidEmail,

    #[error("invalid space action")]
    #[translate(en = "Please delete and re-create the action", ko = "액션을 삭제하고 다시 만들어주세요.")]
    SpaceActionNotFound,

    #[error("Space is not started")]
    #[translate(en = "Space is not started yet", ko = "스페이스가 아직 시작되지 않았습니다.")]
    SpaceNotStarted,

    #[error("Action has ended")]
    #[translate(en = "This action has ended", ko = "이 액션은 종료되었습니다.")]
    ActionEnded,
}

impl From<std::convert::Infallible> for Error {
    fn from(e: std::convert::Infallible) -> Self {
        match e {}
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Unknown(s)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(e: base64::DecodeError) -> Self {
        Error::Unknown(e.to_string())
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for Error {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::http::StatusCode;
        use bdk::prelude::axum::response::IntoResponse;

        let status = match &self {
            Error::UnauthorizedAccess | Error::NoSessionFound => StatusCode::UNAUTHORIZED,
            Error::InvalidPartitionKey(_)
            | Error::NotSupported(_)
            | Error::InvalidBookmark
            | Error::BadRequest(_)
            | Error::Duplicate(_)
            | Error::NoPermission
            | Error::ParticipationBlocked
            | Error::LackOfVerifiedAttributes
            | Error::FullQuota
            | Error::AlreadyParticipating => StatusCode::BAD_REQUEST,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::SpaceReward(e) => e.status_code(),
            Error::SpaceActionQuiz(e) => e.status_code(),
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

impl From<ServerFnError> for Error {
    fn from(e: ServerFnError) -> Self {
        Error::Unknown(format!("Server function error: {}", e))
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for Error {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            Error::UnauthorizedAccess | Error::NoSessionFound | Error::Unauthorized(_) => {
                StatusCode::UNAUTHORIZED
            }
            Error::InvalidPartitionKey(_)
            | Error::NotSupported(_)
            | Error::InvalidBookmark
            | Error::BadRequest(_)
            | Error::Duplicate(_)
            | Error::NoPermission
            | Error::ParticipationBlocked
            | Error::LackOfVerifiedAttributes
            | Error::FullQuota
            | Error::AlreadyParticipating
            | Error::ExceededAttemptEmailVerification
            | Error::ExceededAttemptPhoneVerification
            | Error::SendSmsFailed(_)
            | Error::NotFoundVerificationCode
            | Error::ExpiredVerification
            | Error::InvalidVerificationCode => StatusCode::BAD_REQUEST,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::SpaceReward(e) => e.status_code(),
            Error::SpaceActionQuiz(e) => e.status_code(),
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
