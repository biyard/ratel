use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, PartialEq, Eq, Deserialize)]
#[repr(u64)]
pub enum ServiceError {
    Unknown(String),

    NotFound,
    Unauthorized,
    UserAlreadyExists,

    VerifyException(String),
    SignException,
    DatabaseException(String),
    OpenApiResponseError(String),
    BadRequest,
    JsonDeserializeError(String),
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for ServiceError {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ServiceError::Unknown(s.to_string()))
    }
}

impl<E: Error + 'static> From<E> for ServiceError {
    fn from(e: E) -> Self {
        ServiceError::Unknown(e.to_string())
    }
}

impl ServiceError {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

unsafe impl Send for ServiceError {}
unsafe impl Sync for ServiceError {}

#[cfg(feature = "server")]
impl by_axum::axum::response::IntoResponse for ServiceError {
    fn into_response(self) -> by_axum::axum::response::Response {
        let default_status_code = by_axum::axum::http::StatusCode::BAD_REQUEST;

        (
            default_status_code,
            serde_json::to_value(self).unwrap().to_string(),
        )
            .into_response()
    }
}
