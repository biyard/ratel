pub mod macros;
pub mod setup;

// Re-export axum so test files can use `axum::Router`, `axum::http::...`
// after `use super::*;` without each declaring its own `use crate::axum;`.
pub use crate::axum;

mod cors_tests;
mod cross_posting_tests;
mod discussion_tests;
mod fact_or_fold_tests;
mod home_tests;
mod inbox_helper_tests;
mod mcp_tests;
mod meet_action_tests;
mod notifications_tests;
mod post_tests;
mod space_action_notification_tests;
mod space_member_tests;
mod space_status_change_tests;
mod sub_team_tests;
mod xp_dedup_tests;

pub use setup::*;
