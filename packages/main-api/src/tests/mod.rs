//! Integration tests for main-api
//!
//! This module contains various integration tests including:
//! - DynamoDB operations and models
//! - SQS message handling
//! - Dual-write functionality
//! - API endpoint testing

pub mod macros;
pub mod test_utils;
pub mod v3_setup;

// Re-export commonly used items for backward compatibility
// pub use test_utils::*;

pub mod dynamo_test;
pub use dynamo_test::*;
