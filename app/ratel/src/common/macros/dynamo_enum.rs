#[cfg(feature = "server")]
pub use crate::common::by_macros::DynamoEnum;

#[cfg(not(feature = "server"))]
pub use by_macros::DummyDynamoEntity as DynamoEnum;
