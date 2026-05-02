//! Versioned migration framework. Run on server bootstrap only when
//! `MIGRATE=true` is set in the environment. Atomic version advance via
//! `LastBackfillVersion::advance_to` (conditional UpdateItem) so multiple
//! replicas in the same release can't double-run a migration.
//!
//! Add a new migration by creating `mNNN_description.rs` with a `pub async
//! fn run(cli)` and wiring it into `runner::run_migrations()` under the
//! appropriate `if stored < N` gate.

#[cfg(feature = "server")]
mod m001_backfill_character_xp;
#[cfg(feature = "server")]
mod runner;

#[cfg(feature = "server")]
pub use runner::run_migrations;
