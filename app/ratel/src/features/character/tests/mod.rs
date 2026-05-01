//! Feature-local tests. Declared from `features/character/mod.rs` under
//! `#[cfg(test)] mod tests;` so they compile only for `cargo test`.
//!
//! Layout:
//! - `helpers`           shared fixtures: TestContext, make_score, award_xp, run_with_env
//! - `leveling_tests`    pure-Rust unit tests for `leveling.rs`
//! - `character_xp_tests` apply_character_xp_delta + GET handlers
//! - `skill_tests`       level_up handler + Money Tree + Ranker effects
//! - `migration_tests`   LastBackfillVersion conditional advance + m001 + MIGRATE gate

mod character_xp_tests;
mod helpers;
mod leveling_tests;
mod migration_tests;
mod skill_tests;
