//! Integration tests for main-api
//!
//! This module contains various integration tests including:
//! - DynamoDB operations and models
//! - SQS message handling
//! - Dual-write functionality
//! - API endpoint testing

pub mod test_utils;

// Re-export commonly used items for backward compatibility
pub use test_utils::*;
