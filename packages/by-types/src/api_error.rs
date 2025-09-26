use std::fmt::{Debug, Display};

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(tag = "status_code", rename_all = "snake_case")]
pub enum ApiError<T> {
    BadRequest(T),
    Unauthorized(T),
    Forbidden(T),
    NotFound(T),
    InternalServerError(T),
}

impl<T> Display for ApiError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::BadRequest(_) => write!(f, "Bad Request"),
            ApiError::Unauthorized(_) => write!(f, "Unauthorized"),
            ApiError::Forbidden(_) => write!(f, "Forbidden"),
            ApiError::NotFound(_) => write!(f, "Not Found"),
            ApiError::InternalServerError(_) => write!(f, "Internal Server Error"),
        }
    }
}

impl<T> std::error::Error for ApiError<T> where T: Debug {}

impl<T> ApiError<T> {
    pub fn into_inner(self) -> T {
        match self {
            ApiError::BadRequest(body) => body,
            ApiError::Unauthorized(body) => body,
            ApiError::Forbidden(body) => body,
            ApiError::NotFound(body) => body,
            ApiError::InternalServerError(body) => body,
        }
    }
}

impl<T> axum::response::IntoResponse for ApiError<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let code = match self {
            ApiError::BadRequest(_) => axum::http::StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => axum::http::StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => axum::http::StatusCode::FORBIDDEN,
            ApiError::NotFound(_) => axum::http::StatusCode::NOT_FOUND,
            ApiError::InternalServerError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        (code, axum::Json(self)).into_response()
    }
}

impl From<std::io::Error> for ApiError<Box<dyn std::error::Error>> {
    fn from(error: std::io::Error) -> Self {
        ApiError::InternalServerError(Box::new(error))
    }
}

impl<T> From<reqwest::Error> for ApiError<T>
where
    T: From<String>,
{
    fn from(error: reqwest::Error) -> Self {
        ApiError::InternalServerError(error.to_string().into())
    }
}

impl<T> From<sqlx::Error> for ApiError<T>
where
    T: From<String>,
{
    fn from(error: sqlx::Error) -> Self {
        ApiError::InternalServerError(error.to_string().into())
    }
}

impl<T> From<validator::ValidationError> for ApiError<T>
where
    T: From<String>,
{
    fn from(error: validator::ValidationError) -> Self {
        ApiError::BadRequest(error.to_string().into())
    }
}
