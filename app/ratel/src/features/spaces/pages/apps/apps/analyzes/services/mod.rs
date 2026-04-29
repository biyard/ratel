//! Server-side services for the analyzes feature. Houses logic that is
//! shared between request-time controllers (preview, on-demand
//! triggers) and the DDB-stream Lambda handlers (auto poll/quiz/follow
//! aggregation, on-demand discussion text analysis).
//!
//! All modules here are gated behind `feature = "server"` because they
//! depend on the DynamoDB client and other server-only models.

#[cfg(feature = "server")]
pub mod auto_analysis;
#[cfg(feature = "server")]
pub mod discussion_analysis;
#[cfg(feature = "server")]
pub mod intersection;
#[cfg(feature = "server")]
pub mod record_hydrate;
#[cfg(feature = "server")]
pub mod text_pipeline;
