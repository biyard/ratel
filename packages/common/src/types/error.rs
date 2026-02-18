use crate::*;
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
    #[translate(
        en = "Internal server error",
        ko = "서버 내부 오류가 발생하였습니다. 잠시 후 다시 시도해주세요."
    )]
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
    #[translate(
        en = "Internal server error",
        ko = "서버 내부 오류가 발생하였습니다. 잠시 후 다시 시도해주세요."
    )]
    Aws(#[from] crate::utils::aws::error::AwsError),

    #[cfg(feature = "server")]
    #[serde(skip)]
    #[error("Session error: {0}")]
    #[translate(en = "Please sign in first", ko = "먼저 로그인 해주세요.")]
    Session(#[from] tower_sessions::session::Error),

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
            Error::InvalidPartitionKey(_) | Error::NotSupported(_) | Error::InvalidBookmark => {
                StatusCode::BAD_REQUEST
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl From<aws_sdk_dynamodb::Error> for Error {
    fn from(e: aws_sdk_dynamodb::Error) -> Self {
        Error::Aws(crate::utils::aws::error::AwsError::from(e))
    }
}

#[cfg(feature = "server")]
impl From<serde_dynamo::Error> for Error {
    fn from(e: serde_dynamo::Error) -> Self {
        Error::Aws(crate::utils::aws::error::AwsError::from(e))
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
            | Error::NotFoundVerificationCode
            | Error::ExpiredVerification
            | Error::InvalidVerificationCode => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
