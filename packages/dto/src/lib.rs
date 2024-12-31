pub mod common_query_response;
pub mod error;
pub mod topics;

pub use topics::*;

pub type Result<T> = std::result::Result<T, error::ServiceError>;
