#[cfg(feature = "server")]
pub use crate::common::by_macros::DynamoEntity;

#[cfg(not(feature = "server"))]
pub use by_macros::DummyDynamoEntity as DynamoEntity;
