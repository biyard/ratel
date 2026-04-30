//! Server-only services for the cross-posting pipeline.

pub mod adapters;
pub mod credentials;
pub mod dispatcher;
pub mod factory;
pub mod format;
pub mod shard;
pub mod sweeper;
pub mod sweeper_poller;

pub use adapters::*;

// To be added in subsequent PRs:
//   - engagement.rs    Stage 4 adaptive sweeper (1D)
