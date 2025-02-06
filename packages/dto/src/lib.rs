mod assembly_members;
mod assets;
mod comment;
mod error;
mod patrons;
mod topic;
mod users;
mod vote;

pub use assembly_members::*;
pub use assets::*;
pub use comment::*;
pub use error::*;
pub use patrons::*;
pub use topic::*;
pub use users::*;
pub use vote::*;

pub type Result<T> = std::result::Result<T, error::ServiceError>;
