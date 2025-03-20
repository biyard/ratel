use std::error::Error;

use serde::{Deserialize, Serialize};

use bdk::prelude::*;

#[derive(Debug, Serialize)]
pub struct ServiceException {
    pub inner: ServiceError,
}

impl std::fmt::Display for ServiceException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl Error for ServiceException {}

#[derive(Debug, Serialize, PartialEq, Eq, Deserialize, Translate)]
#[repr(u64)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
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
    WalletNotFound,
    WalletError(String),
    HtmlParseError(String),
    UniqueViolation(String),
}

impl<E: Error + 'static> From<E> for ServiceError {
    fn from(e: E) -> Self {
        ServiceError::Unknown(e.to_string())
    }
}

impl Into<ServiceException> for ServiceError {
    fn into(self) -> ServiceException {
        ServiceException { inner: self }
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
        (
            by_axum::axum::http::StatusCode::BAD_REQUEST,
            by_axum::axum::Json(self),
        )
            .into_response()
    }
}
