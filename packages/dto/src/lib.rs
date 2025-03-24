mod error;
mod tables;

pub use error::*;
pub use tables::*;

pub type Result<T> = std::result::Result<T, error::ServiceError>;
