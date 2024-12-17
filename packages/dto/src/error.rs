use std::error::Error;

use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ServiceError {
    Unknown(String),

    /// Authentication: 100-150
    UserNotFound(String),
    UserPasswordNotMatched,
    UserIdentityGenerationException,
    UserAlreadyExists(String),

    /// DynamoDB: 500-550
    DynamoSerializeException(String),
    DynamoPutItemException(String),
    DynamoGetItemException(String),
    DynamoListItemsException(String),
    DynamoDeleteItemException(String),
    DynamoCreateItemException(String),

    // S3: 551-600
    S3PutObejctUriException(String),

    /// ICP Errors: 600-650
    IcpCanisterCallArgsEncodeException(String),
    IcpCanisterCallResponseDecodeException(String),
    IcpCanisterCallException(String),

    // Canister errors
    UnknownException(String),

    MintingCodeNotFound,
    MintingException(String),

    // Profile
    ProfileNotFound,

    // Credit
    AlreadyRewarded,
    CreditNotFound,

    AlreadyFollower,
    AlreadyReporter,

    AddMemberFailed,
    AddNftFailed,

    // Agit
    TransferAgitFailed(String),
    AddAdminFailed(String),

    // Search API
    CreateIndexFailed(String),

    // Auth
    ConfirmVerificationException,

    // Collection
    CollectionNotFound,

    // User
    NotOwner,

    // Payment
    NotPaidYet,
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
        use serde_json::json;

        let body = by_axum::axum::Json(json!({
            "error": {
                "message": self.to_string(),
            }
        }));

        (by_axum::axum::http::StatusCode::BAD_REQUEST, body).into_response()
    }
}
