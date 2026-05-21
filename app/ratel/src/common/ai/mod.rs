//! Generic AI provider abstraction. Lives in `common` (not under any
//! feature) so multiple features (post drafting, moderation, future
//! summarization, etc.) can share the same backend selection.
//!
//! Server-only — depends on AWS SDK + reqwest.

pub mod backends;
pub mod writer;

pub use writer::{writer_ai, WriterAi, WriterAiError, WriterAiRequest};
