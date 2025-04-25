mod error;
pub mod info;
mod joined_tables;
mod tables;

pub use error::*;
pub use info::*;
pub use joined_tables::*;
pub use tables::*;

pub type Result<T> = std::result::Result<T, error::ServiceError>;
