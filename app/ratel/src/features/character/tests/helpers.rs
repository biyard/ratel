//! Shared fixtures for the character feature's tests.
//!
//! `TestContext` re-uses the project-wide setup at `crate::tests::TestContext`
//! (it spins up a fresh DynamoDB Local namespace and a router). The helpers
//! below add character-feature-specific factories on top.

#![allow(dead_code)]

pub use crate::common::types::*;
pub use crate::tests::TestContext;
use crate::features::activity::models::SpaceScore;
use crate::features::activity::types::AuthorPartition;

/// Build a SpaceScore row for `(user, space)` with `total_score = total`.
/// Matches the per-space score the existing aggregation pipeline produces.
pub fn make_score(user_pk: &Partition, space_id: &str, total: i64) -> SpaceScore {
    let space_part = SpacePartition(space_id.to_string());
    let author: AuthorPartition = AuthorPartition::from(user_pk.clone());
    let mut s = SpaceScore::new(space_part, author, "u".into(), "".into());
    s.total_score = total;
    s
}

/// Pump XP into the user's CharacterXp via the production code path
/// (not direct entity manipulation) so tests exercise `apply_character_xp_delta`.
pub async fn award_xp(ctx: &TestContext, user_pk: &Partition, total: i64) {
    crate::features::character::services::apply_character_xp_delta(
        &ctx.ddb,
        make_score(user_pk, "s", total),
    )
    .await
    .unwrap();
}

/// Run a future with an env var temporarily set, then restore the prior value.
/// Used by migration tests that toggle `MIGRATE`. Tests using this MUST run
/// serialized — when running in parallel, the env-var swap leaks across tests.
pub async fn run_with_env<F, Fut, T>(key: &str, val: &str, f: F) -> T
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = T>,
{
    let prev = std::env::var(key).ok();
    std::env::set_var(key, val);
    let r = f().await;
    if let Some(p) = prev {
        std::env::set_var(key, p);
    } else {
        std::env::remove_var(key);
    }
    r
}
