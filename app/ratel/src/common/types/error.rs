use crate::common::*;
use dioxus::fullstack::Loading;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum Error {
    #[error("Unknown: {0}")]
    Unknown(String),

    // NOTE: Built-in errors for Some macros
    #[error("Invalid partition key: {0}")]
    InvalidPartitionKey(String),

    #[error("Not supported: {0}")]
    NotSupported(String),

    #[error("Unauthorized access")]
    UnauthorizedAccess,
    #[error("Unauthorized access: {0}")]
    Unauthorized(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Bookmark is invalid")]
    InvalidBookmark,

    #[error("No session found")]
    NoSessionFound,

    #[cfg(feature = "server")]
    #[serde(skip)]
    #[error("AWS error: {0}")]
    Aws(#[from] crate::common::utils::aws::error::AwsError),

    #[cfg(feature = "server")]
    #[serde(skip)]
    #[error("Session error: {0}")]
    Session(#[from] tower_sessions::session::Error),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Duplicate entry: {0}")]
    Duplicate(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("User has no permission")]
    NoPermission,

    #[error("Exceeded maximum attempt for email verification")]
    ExceededAttemptEmailVerification,

    #[error("Exceeded maximum attempt for phone verification")]
    ExceededAttemptPhoneVerification,

    #[error("Send SMS Failed: {0}")]
    SendSmsFailed(String),

    #[error("User participation is blocked for this space")]
    ParticipationBlocked,

    #[error("User lacks verified attributes required for participation")]
    LackOfVerifiedAttributes,

    #[error("Quota is full")]
    FullQuota,

    #[error("User is already participating in the space")]
    AlreadyParticipating,

    #[error("not found verification code")]
    NotFoundVerificationCode,
    #[error("verification code is expired")]
    ExpiredVerification,
    #[error("invalid verification code")]
    InvalidVerificationCode,

    #[error("not found space")]
    SpaceNotFound,
    #[error("not found author")]
    InvalidSpaceAuthor,

    #[error("discussion is not in progress")]
    DiscussionNotInProgress,

    #[error("space post ended")]
    SpacePostEnded,

    #[error("space post contents is too short")]
    ValidationTooShortContents,

    // DAO-related
    #[error("At least 3 admins must be selected")]
    InsufficientAdmins,

    #[error("Transaction cancelled: You rejected the transaction")]
    TransactionRejected,

    #[error("MetaMask not installed")]
    MetamaskNotInstalled,

    #[error("Wallet error: {0}")]
    WalletError(String),

    // Membership-related
    #[error("Membership response missing")]
    MembershipResponseMissing,

    #[error("Failed to change membership")]
    MembershipChangeFailed,

    // Credential-related
    #[error("Invalid verification code input")]
    InvalidCodeInput,

    #[error("Failed to response quiz")]
    QuizResponseFailed,

    // PortOne related errors
    #[error("PortOne Inicis returned invalid identity")]
    PortOneRequestFailure,

    #[error("PortOne Inicis returned invalid identity")]
    PortOneInicisInvalidIdentity,

    #[error("{0}")]
    SpaceReward(#[from] SpaceRewardError),

    #[error("{0}")]
    SpaceActionQuiz(#[from] crate::features::spaces::pages::actions::actions::quiz::SpaceActionQuizError),

    // Post related errors
    #[error("Invalid username")]
    PostInvalidUsername,

    #[error("Web function")]
    OnlyWebFunction,
    #[error("Invalid email")]
    InvalidEmail,

    #[error("invalid space action")]
    SpaceActionNotFound,

    #[error("Space is not started")]
    SpaceNotStarted,

    #[error("Action has ended")]
    ActionEnded,
}

// Manual `Translate` implementation: delegates to inner error's `translate()` for
// wrapper variants (`SpaceReward`, `SpaceActionQuiz`) so that `toast.error()` shows
// the specific per-variant translation rather than the generic outer message.
impl dioxus_translate::Translate for Error {
    fn translate(&self, lang: &dioxus_translate::Language) -> &'static str {
        match lang {
            dioxus_translate::Language::En => match self {
                Error::Unknown(..) => "Unknown error",
                Error::InvalidPartitionKey(..) => "No data found",
                Error::NotSupported(..) => "Not supported",
                Error::UnauthorizedAccess => "Unauthorized access",
                Error::Unauthorized(..) => "Unauthorized access",
                Error::InternalServerError(..) => "Internal server error",
                Error::InvalidBookmark => "Please refresh the page",
                Error::NoSessionFound => "Please sign in first",
                #[cfg(feature = "server")]
                Error::Aws(..) => "Internal server error",
                #[cfg(feature = "server")]
                Error::Session(..) => "Please sign in first",
                Error::BadRequest(..) => "Bad request",
                Error::Duplicate(..) => "Duplicate entry",
                Error::NotFound(..) => "Not found",
                Error::NoPermission => "No permission",
                Error::ExceededAttemptEmailVerification => "Exceeded maximum attempt for email verification",
                Error::ExceededAttemptPhoneVerification => "Exceeded maximum attempt for phone verification",
                Error::SendSmsFailed(..) => "Send SMS Failed",
                Error::ParticipationBlocked => "Participation is blocked",
                Error::LackOfVerifiedAttributes => "Missing verified attributes",
                Error::FullQuota => "Quota is full",
                Error::AlreadyParticipating => "Already participating",
                Error::NotFoundVerificationCode => "Verification code not found",
                Error::ExpiredVerification => "Verification code is expired",
                Error::InvalidVerificationCode => "Invalid verification code",
                Error::SpaceNotFound => "Not found space",
                Error::InvalidSpaceAuthor => "Not found space author",
                Error::DiscussionNotInProgress => "Discussion is not in progress",
                Error::SpacePostEnded => "Space post ended",
                Error::ValidationTooShortContents => "Space post contents must be at least 10 characters long.",
                Error::InsufficientAdmins => "At least 3 admins must be selected",
                Error::TransactionRejected => "Transaction cancelled: You rejected the transaction",
                Error::MetamaskNotInstalled => "MetaMask not installed. Please install MetaMask to continue",
                Error::WalletError(..) => "Wallet error",
                Error::MembershipResponseMissing => "Membership response missing",
                Error::MembershipChangeFailed => "Failed to change membership",
                Error::InvalidCodeInput => "Invalid verification code",
                Error::QuizResponseFailed => "Failed to submit response.",
                Error::PortOneRequestFailure => "Failed network request",
                Error::PortOneInicisInvalidIdentity => "Failed to verify KYC",
                Error::SpaceReward(e) => e.translate(lang),
                Error::SpaceActionQuiz(e) => e.translate(lang),
                Error::PostInvalidUsername => "Invalid username. Check URL.",
                Error::OnlyWebFunction => "This function is only available on web.",
                Error::InvalidEmail => "Invalid email",
                Error::SpaceActionNotFound => "Please delete and re-create the action",
                Error::SpaceNotStarted => "Space is not started yet",
                Error::ActionEnded => "This action has ended",
            },
            #[cfg(feature = "ko")]
            dioxus_translate::Language::Ko => match self {
                Error::Unknown(..) => "알수없는 에러가 발생하였습니다.",
                Error::InvalidPartitionKey(..) => "데이터를 찾을 수 없습니다.",
                Error::NotSupported(..) => "지원되지 않는 기능입니다.",
                Error::UnauthorizedAccess => "인증되지 않은 접근입니다.",
                Error::Unauthorized(..) => "인증되지 않은 접근입니다.",
                Error::InternalServerError(..) => "서버 내부 오류가 발생하였습니다. 잠시 후 다시 시도해주세요.",
                Error::InvalidBookmark => "페이지를 새로고침 해주세요.",
                Error::NoSessionFound => "먼저 로그인 해주세요.",
                #[cfg(feature = "server")]
                Error::Aws(..) => "서버 내부 오류가 발생하였습니다. 잠시 후 다시 시도해주세요.",
                #[cfg(feature = "server")]
                Error::Session(..) => "먼저 로그인 해주세요.",
                Error::BadRequest(..) => "잘못된 요청입니다.",
                Error::Duplicate(..) => "중복된 항목입니다.",
                Error::NotFound(..) => "찾을 수 없습니다.",
                Error::NoPermission => "권한이 없습니다.",
                Error::ExceededAttemptEmailVerification => "이메일 인증 최대 시도 횟수를 초과했습니다.",
                Error::ExceededAttemptPhoneVerification => "전화 인증 최대 시도 횟수를 초과했습니다.",
                Error::SendSmsFailed(..) => "SMS 전송에 실패했습니다.",
                Error::ParticipationBlocked => "참여가 제한되어 있습니다.",
                Error::LackOfVerifiedAttributes => "필수 인증 속성이 없습니다.",
                Error::FullQuota => "정원이 초과되었습니다.",
                Error::AlreadyParticipating => "이미 참여 중입니다.",
                Error::NotFoundVerificationCode => "인증 코드를 찾을 수 없습니다.",
                Error::ExpiredVerification => "인증 코드가 만료되었습니다.",
                Error::InvalidVerificationCode => "인증 코드가 유효하지 않습니다.",
                Error::SpaceNotFound => "스페이스를 찾을 수 없습니다.",
                Error::InvalidSpaceAuthor => "스페이스 작성자를 찾을 수 없습니다.",
                Error::DiscussionNotInProgress => "토론이 진행중 상태가 아닙니다.",
                Error::SpacePostEnded => "스페이스 게시글이 종료되었습니다.",
                Error::ValidationTooShortContents => "스페이스 게시글의 내용은 10자 이상이어야 합니다.",
                Error::InsufficientAdmins => "최소 3명의 관리자를 선택해야 합니다.",
                Error::TransactionRejected => "트랜잭션 취소: 트랜잭션을 거부했습니다.",
                Error::MetamaskNotInstalled => "MetaMask가 설치되어 있지 않습니다. 계속하려면 MetaMask를 설치하세요.",
                Error::WalletError(..) => "지갑 오류가 발생했습니다.",
                Error::MembershipResponseMissing => "멤버십 응답이 누락되었습니다.",
                Error::MembershipChangeFailed => "멤버십 변경에 실패했습니다.",
                Error::InvalidCodeInput => "인증 코드가 유효하지 않습니다.",
                Error::QuizResponseFailed => "응답 제출에 실패했습니다.",
                Error::PortOneRequestFailure => "요청에 실패했습니다.",
                Error::PortOneInicisInvalidIdentity => "본인 인증에 실패했습니다.",
                Error::SpaceReward(e) => e.translate(lang),
                Error::SpaceActionQuiz(e) => e.translate(lang),
                Error::PostInvalidUsername => "유효하지 않은 사용자 이름입니다. URL을 확인해주세요.",
                Error::OnlyWebFunction => "이 기능은 웹에서만 사용할 수 있습니다.",
                Error::InvalidEmail => "유효하지 않은 이메일입니다.",
                Error::SpaceActionNotFound => "액션을 삭제하고 다시 만들어주세요.",
                Error::SpaceNotStarted => "스페이스가 아직 시작되지 않았습니다.",
                Error::ActionEnded => "이 액션은 종료되었습니다.",
            },
        }
    }
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
