//! Server-only services for the cross-posting pipeline.

pub mod adapters;
pub mod credentials;
pub mod dispatcher;
pub mod factory;
pub mod format;
pub mod shard;

pub use adapters::*;

// To be added in subsequent PRs:
//   - retry_sweeper.rs Stage 3 (1D)
//   - engagement.rs    Stage 4 adaptive sweeper (1D)
