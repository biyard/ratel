//! Server-only services for the cross-posting pipeline.

pub mod adapters;
pub mod connection;
pub mod credentials;
pub mod dispatcher;
pub mod factory;
pub mod format;
pub mod oauth_state;
pub mod shard;
pub use adapters::*;

// Phase 1A+1D scope: failed jobs notify the author and surface a manual
// Retry CTA on the post-detail panel. We do NOT auto-retry — see the
// design doc § "Stage 3 (manual retry + notification)" for the spec
// rewrite. Stage 4 (adaptive engagement scheduler) is still pending.
