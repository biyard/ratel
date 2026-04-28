//! Bluesky AT Protocol adapter.
//!
//! Phase 1A scope. App-password flow:
//! - `com.atproto.server.createSession` for credential validation (controller
//!   layer; not in this file)
//! - `com.atproto.server.refreshSession` for accessJwt rotation
//! - `com.atproto.repo.uploadBlob` to upload images
//! - `com.atproto.repo.createRecord` (collection: `app.bsky.feed.post`) to
//!   publish, with optional `app.bsky.embed.images` and / or
//!   `app.bsky.embed.external` (rich link card pointing at the Ratel
//!   backlink URL)
//! - `app.bsky.feed.getAuthorFeed` to scan recent posts during
//!   lock-recovery `find_by_backlink`
//! - `app.bsky.feed.getPostThread` (or post hydration in `getAuthorFeed`)
//!   for engagement counts
//!
//! Concrete HTTP integration lands in PR A2. This file currently provides
//! only the trait skeleton so the dispatcher and downstream consumers can
//! be wired against the real type.

use super::{
    CrossPostAdapter, DecryptedCredentials, EngagementCounts, ImageRef, PlatformError,
    PublishedRef,
};
use crate::features::cross_posting::types::SocialPlatform;
use async_trait::async_trait;

/// Bluesky AT Protocol adapter. Stateless aside from a shared `reqwest::Client`
/// pool — instances are cheap to construct (typically once per Lambda
/// initialization).
#[derive(Debug, Clone)]
pub struct BlueskyAdapter {
    /// PDS host. Default: `https://bsky.social`. Overridable for self-hosted
    /// PDSes (rare in our user base; configurable for completeness).
    pub pds_host: String,
}

impl BlueskyAdapter {
    pub fn new() -> Self {
        Self { pds_host: "https://bsky.social".to_string() }
    }

    pub fn with_host(host: impl Into<String>) -> Self {
        Self { pds_host: host.into() }
    }
}

impl Default for BlueskyAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CrossPostAdapter for BlueskyAdapter {
    fn platform(&self) -> SocialPlatform {
        SocialPlatform::Bluesky
    }

    fn char_limit(&self) -> usize {
        SocialPlatform::Bluesky.char_limit()
    }

    fn max_images(&self) -> usize {
        SocialPlatform::Bluesky.max_images()
    }

    async fn publish(
        &self,
        _creds: DecryptedCredentials,
        _formatted_body: String,
        _images: Vec<ImageRef>,
    ) -> Result<PublishedRef, PlatformError> {
        // PR A2: implement createRecord + uploadBlob + embed.external.
        Err(PlatformError::Unknown("bluesky adapter publish() not yet implemented".into()))
    }

    async fn fetch_engagement(
        &self,
        _creds: DecryptedCredentials,
        _external_post_id: &str,
    ) -> Result<EngagementCounts, PlatformError> {
        // PR A2: implement getPostThread / hydrated getAuthorFeed.
        Err(PlatformError::Unknown("bluesky adapter fetch_engagement() not yet implemented".into()))
    }

    async fn find_by_backlink(
        &self,
        _creds: DecryptedCredentials,
        _backlink_url: &str,
    ) -> Result<Option<PublishedRef>, PlatformError> {
        // PR A2: implement getAuthorFeed scan + body substring match.
        Err(PlatformError::Unknown(
            "bluesky adapter find_by_backlink() not yet implemented".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bluesky_adapter_reports_platform_facts() {
        let a = BlueskyAdapter::new();
        assert_eq!(a.platform(), SocialPlatform::Bluesky);
        assert_eq!(a.char_limit(), 300);
        assert_eq!(a.max_images(), 4);
    }

    #[test]
    fn bluesky_adapter_default_host_is_public_pds() {
        assert_eq!(BlueskyAdapter::new().pds_host, "https://bsky.social");
    }
}
