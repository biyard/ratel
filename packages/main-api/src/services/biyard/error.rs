use crate::reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BiyardError {
    #[error("Biyard API request failed: {0}")]
    ApiError(String),

    #[error("Biyard API returned bad request: {0}")]
    BadRequest(String),

    #[error("Biyard API returned not found: {0}")]
    NotFound(String),

    #[error("Biyard API returned unauthorized: {0}")]
    Unauthorized(String),

    #[error("Biyard API response parse error: {0}")]
    ParseError(String),

    #[error("Biyard API internal server error: {0}")]
    InternalError(String),

    #[error("Biyard API returned empty response")]
    EmptyResponse,

    #[error("Biyard network error: {0}")]
    NetworkError(#[from] crate::reqwest::Error),
}

impl BiyardError {
    pub fn from_status(status: StatusCode, error_text: String) -> Self {
        tracing::error!(
            "Biyard API error - status: {}, error: {}",
            status,
            error_text
        );

        match status {
            StatusCode::BAD_REQUEST => Self::BadRequest(error_text),
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Self::Unauthorized(error_text),
            StatusCode::NOT_FOUND => Self::NotFound(error_text),
            StatusCode::INTERNAL_SERVER_ERROR
            | StatusCode::BAD_GATEWAY
            | StatusCode::SERVICE_UNAVAILABLE
            | StatusCode::GATEWAY_TIMEOUT => Self::InternalError(error_text),
            _ => Self::ApiError(format!("{}: {}", status, error_text)),
        }
    }

    pub fn parse_error(msg: impl std::fmt::Display) -> Self {
        Self::ParseError(msg.to_string())
    }
}
