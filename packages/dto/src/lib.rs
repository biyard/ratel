mod error;
pub mod info;
mod joined_tables;
mod resource;
mod tables;

pub use error::*;
pub use info::*;
pub use joined_tables::*;
pub use resource::*;
pub use tables::*;

pub type Result<T> = std::result::Result<T, error::Error>;

// Re-export DynamoDB types for convenience
pub use aws_sdk_dynamodb::types::AttributeValue;

// Helper functions
pub fn string_attr(value: &str) -> AttributeValue {
    AttributeValue::S(value.to_string())
}

pub fn number_attr(value: i64) -> AttributeValue {
    AttributeValue::N(value.to_string())
}

pub fn bool_attr(value: bool) -> AttributeValue {
    AttributeValue::Bool(value)
}
