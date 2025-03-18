mod assets;
mod comment;
mod error;
mod patron;
mod tables;
mod topic;
mod vote;

pub use assets::*;
pub use comment::*;
pub use error::*;
pub use patron::*;
pub use tables::*;
pub use topic::*;
pub use vote::*;

pub type Result<T> = std::result::Result<T, error::ServiceError>;
