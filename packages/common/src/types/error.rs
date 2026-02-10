use crate::*;
pub use thiserror::Error;

#[derive(Debug, Error)]
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
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Unknown(s)
    }
}
