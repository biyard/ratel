//! Mock data types for the Phase-1 analyzes arena UI.
//!
//! Phase 1 ships the foundation + LIST page using hard-coded mock
//! data. Real controllers/server-functions/DynamoDB will replace
//! `mock_reports()` in a later phase. None of these types are wired
//! to network IO yet — they are deliberately plain structs.

mod report;

pub use report::*;
