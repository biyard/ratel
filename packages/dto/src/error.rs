use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[repr(u64)]
pub enum ServiceError {
    Unknown(String) = 1000,
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
