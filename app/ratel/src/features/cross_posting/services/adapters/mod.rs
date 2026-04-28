//! Per-platform adapter trait and supporting types.
//!
//! Stage 2 dispatcher selects an adapter via `match job.platform` and calls
//! `adapter.publish(creds, body, images)`. Adding a new platform = new
//! adapter struct that implements `CrossPostAdapter`; the dispatcher itself
//! is unchanged.

mod bluesky;

pub use bluesky::*;

use crate::features::cross_posting::types::SocialPlatform;
use async_trait::async_trait;

/// Decrypted credentials for a single platform connection. Produced by
/// the dispatcher's KMS-decrypt step from `SocialConnection.credential_ciphertext`
/// — the plaintext lives only inside the Lambda's stack frame for the
/// duration of one publish / fetch call.
#[derive(Debug, Clone)]
pub enum DecryptedCredentials {
    Bluesky {
        handle: String,
        access_jwt: String,
        refresh_jwt: String,
    },
    LinkedIn {
        access_token: String,
        refresh_token: String,
        member_urn: String,
    },
    Threads {
        access_token: String,
        refresh_token: String,
        ig_user_id: String,
    },
}

/// Reference to an image attached to a post. Holds the canonical S3 URL —
/// each adapter is responsible for fetching the bytes and re-uploading to
/// the platform's blob store as needed.
#[derive(Debug, Clone)]
pub struct ImageRef {
    pub url: String,
}

impl ImageRef {
    pub fn from_s3(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}

/// Result of a successful publish — used to populate
/// `SyndicationJob.external_post_id` / `external_post_url`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedRef {
    pub external_post_id: String,
    pub external_post_url: String,
}

/// Engagement counts pulled from the platform during Stage 4 refresh.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EngagementCounts {
    pub likes: i32,
    pub comments: i32,
    pub reposts: i32,
}

/// Platform-side error returned from any adapter call. Maps directly into
/// `SyndicationJob.last_error_category` for retry policy classification.
#[derive(Debug, thiserror::Error)]
pub enum PlatformError {
    /// Token expired / invalid; non-retryable. Surface "Reconnect" CTA.
    #[error("auth expired: {0}")]
    AuthExpired(String),
    /// Platform 429; retryable with backoff.
    #[error("rate limited: {0}")]
    RateLimited(String),
    /// Platform rejected content (length, policy, image format, etc.);
    /// non-retryable.
    #[error("content rejected: {0}")]
    ContentRejected(String),
    /// HTTP-layer failure (timeout, DNS, connection reset); retryable.
    #[error("network error: {0}")]
    NetworkError(String),
    /// Anything else; retryable as a precaution.
    #[error("unknown error: {0}")]
    Unknown(String),
}

/// One adapter per external platform. All methods take credentials by value
/// because the dispatcher decrypts once per attempt.
#[async_trait]
pub trait CrossPostAdapter: Send + Sync {
    fn platform(&self) -> SocialPlatform;
    fn char_limit(&self) -> usize;
    fn max_images(&self) -> usize;

    /// Publish a single post to the platform. The body is already formatted
    /// (truncated, backlink appended) by the time it arrives here — the
    /// adapter just sends it.
    async fn publish(
        &self,
        creds: DecryptedCredentials,
        formatted_body: String,
        images: Vec<ImageRef>,
    ) -> Result<PublishedRef, PlatformError>;

    /// Fetch likes / comments / reposts for an existing platform post.
    /// Called by Stage 4 (1D) on its adaptive cadence.
    async fn fetch_engagement(
        &self,
        creds: DecryptedCredentials,
        external_post_id: &str,
    ) -> Result<EngagementCounts, PlatformError>;

    /// Reconcile path used when Stage 2 steals a lock from a dead attempt.
    /// Searches the user's recent posts on the platform for a copy whose
    /// body contains `backlink_url` (the URL is unique per Ratel post
    /// thanks to the `?utm_source=` query param). Returns `Some` if the
    /// previous attempt actually published before dying, `None` otherwise.
    async fn find_by_backlink(
        &self,
        creds: DecryptedCredentials,
        backlink_url: &str,
    ) -> Result<Option<PublishedRef>, PlatformError>;
}
