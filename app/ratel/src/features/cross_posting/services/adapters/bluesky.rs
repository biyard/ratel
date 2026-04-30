//! Bluesky AT Protocol adapter (Phase 1A).
//!
//! Endpoints used (all under `pds_host`):
//! - `com.atproto.server.refreshSession` — accessJwt rotation on 401
//! - `com.atproto.repo.uploadBlob` — image upload before embedding
//! - `com.atproto.repo.createRecord` (collection: `app.bsky.feed.post`) —
//!   the actual publish, with optional `app.bsky.embed.external` (rich-link
//!   card pointing at the Ratel backlink) or `app.bsky.embed.images`
//! - `app.bsky.feed.getPostThread` — engagement counts (Stage 4)
//! - `app.bsky.feed.getAuthorFeed` — recent-post scan for the
//!   `find_by_backlink` lock-recovery probe (Stage 2 step (b))
//!
//! Embed selection is mutually exclusive in this implementation:
//! - `images.is_empty()` → `app.bsky.embed.external` rich-link card
//! - else                → `app.bsky.embed.images`
//!
//! Combined `embed.recordWithMedia` is out of scope for 1A (would let us
//! show both a rich-link card AND images on the same post; minor UX win,
//! deferred).

use super::{
    CrossPostAdapter, DecryptedCredentials, EngagementCounts, ImageRef, LinkCard, PlatformError,
    PublishedRef,
};
use crate::features::cross_posting::types::SocialPlatform;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

const PUBLIC_PDS: &str = "https://bsky.social";
const POST_COLLECTION: &str = "app.bsky.feed.post";

/// Bluesky AT Protocol adapter. Stateless aside from a shared
/// `reqwest::Client` (Arc internally — cheap to clone).
#[derive(Debug, Clone)]
pub struct BlueskyAdapter {
    /// PDS host. Default: `https://bsky.social`. Overridable for self-hosted
    /// PDSes (rare in our user base; configurable for completeness).
    pub pds_host: String,
    client: reqwest::Client,
}

impl BlueskyAdapter {
    pub fn new() -> Self {
        Self::with_host(PUBLIC_PDS)
    }

    pub fn with_host(host: impl Into<String>) -> Self {
        Self { pds_host: host.into(), client: reqwest::Client::new() }
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
        creds: DecryptedCredentials,
        formatted_body: String,
        images: Vec<ImageRef>,
        link_card: LinkCard,
    ) -> Result<PublishedRef, PlatformError> {
        let (did, handle, jwt) = unwrap_bluesky_creds(creds)?;

        // Build the embed FIRST so any blob upload happens before the
        // createRecord call.
        let embed = if images.is_empty() {
            Some(self.build_external_embed(&link_card))
        } else {
            let blobs = self.upload_blobs(&jwt, images).await?;
            Some(build_images_embed(blobs))
        };

        let body = build_publish_body(&did, &formatted_body, &link_card.backlink_url, embed);

        let resp = self
            .post_authed(&format!("{}/xrpc/com.atproto.repo.createRecord", self.pds_host), &jwt, &body)
            .await?
            .json::<CreateRecordResponse>()
            .await
            .map_err(|e| PlatformError::Unknown(format!("createRecord parse: {e}")))?;

        Ok(PublishedRef {
            external_post_id: resp.uri.clone(),
            external_post_url: post_url_from_uri(&handle, &resp.uri),
        })
    }

    async fn fetch_engagement(
        &self,
        creds: DecryptedCredentials,
        external_post_id: &str,
    ) -> Result<EngagementCounts, PlatformError> {
        let (_did, _handle, jwt) = unwrap_bluesky_creds(creds)?;

        // getPostThread takes a query param `uri` — use depth=0 to avoid
        // pulling replies, since we only need the root post's counts.
        let url = format!(
            "{}/xrpc/app.bsky.feed.getPostThread?depth=0&uri={}",
            self.pds_host,
            urlencoding::encode(external_post_id)
        );
        let resp = self
            .get_authed(&url, &jwt)
            .await?
            .json::<GetPostThreadResponse>()
            .await
            .map_err(|e| PlatformError::Unknown(format!("getPostThread parse: {e}")))?;

        Ok(parse_engagement(&resp))
    }

    async fn find_by_backlink(
        &self,
        creds: DecryptedCredentials,
        backlink_url: &str,
    ) -> Result<Option<PublishedRef>, PlatformError> {
        let (_did, handle, jwt) = unwrap_bluesky_creds(creds)?;

        let url = format!(
            "{}/xrpc/app.bsky.feed.getAuthorFeed?actor={}&limit=50",
            self.pds_host,
            urlencoding::encode(&handle)
        );
        let resp = self
            .get_authed(&url, &jwt)
            .await?
            .json::<GetAuthorFeedResponse>()
            .await
            .map_err(|e| PlatformError::Unknown(format!("getAuthorFeed parse: {e}")))?;

        Ok(scan_for_backlink(&resp, backlink_url, &handle))
    }
}

// ─────────────────────────────────────────────────────────────────────────
// Connect-time session creation (called from the connect controller, NOT
// the dispatcher trait — the trait is for ongoing publish/fetch only).
// ─────────────────────────────────────────────────────────────────────────

/// Newly-issued Bluesky session, returned to the connect controller for
/// AEAD-sealing into `SocialConnection.credential_ciphertext`.
#[derive(Clone)]
pub struct BlueskySession {
    pub did: String,
    pub handle: String,
    pub access_jwt: String,
    pub refresh_jwt: String,
}

#[derive(serde::Deserialize)]
struct CreateSessionResponse {
    #[serde(rename = "accessJwt")]
    access_jwt: String,
    #[serde(rename = "refreshJwt")]
    refresh_jwt: String,
    handle: String,
    did: String,
}

impl BlueskyAdapter {
    /// Validate `(identifier, app_password)` against Bluesky and return
    /// the issued session tokens. The plaintext app_password is never
    /// persisted; only the resulting JWTs (sealed via the credentials
    /// helper) are stored.
    pub async fn create_session(
        &self,
        identifier: &str,
        app_password: &str,
    ) -> Result<BlueskySession, PlatformError> {
        let url = format!("{}/xrpc/com.atproto.server.createSession", self.pds_host);
        let req_body =
            serde_json::json!({ "identifier": identifier, "password": app_password });
        let resp = self
            .client
            .post(&url)
            .json(&req_body)
            .send()
            .await
            .map_err(map_transport_error)?;
        let resp = check_status(resp).await?;
        let parsed: CreateSessionResponse = resp
            .json()
            .await
            .map_err(|e| PlatformError::Unknown(format!("createSession parse: {e}")))?;
        Ok(BlueskySession {
            did: parsed.did,
            handle: parsed.handle,
            access_jwt: parsed.access_jwt,
            refresh_jwt: parsed.refresh_jwt,
        })
    }

    /// Rotate an expired `accessJwt` using the long-lived `refreshJwt`
    /// (~90-day TTL on Bluesky). Endpoint is auth'd by the refresh token
    /// itself in the bearer slot, not by the access token. The response
    /// usually carries a fresh `refreshJwt` too — callers should always
    /// persist both fields back into `SocialConnection.credential_ciphertext`
    /// so a future refresh has the latest token.
    ///
    /// Failure modes:
    /// - `AuthExpired` — the refresh token itself is expired or revoked.
    ///   Caller surfaces "reconnect required" UX.
    /// - `NetworkError` / `Unknown` — transport / parsing failures; caller
    ///   may retry the publish path or fail the job depending on policy.
    pub async fn refresh_session(
        &self,
        refresh_jwt: &str,
    ) -> Result<BlueskySession, PlatformError> {
        let url = format!("{}/xrpc/com.atproto.server.refreshSession", self.pds_host);
        // refreshSession authenticates via the refresh JWT (Bearer), no body.
        let resp = self
            .client
            .post(&url)
            .bearer_auth(refresh_jwt)
            .send()
            .await
            .map_err(map_transport_error)?;
        let resp = check_status(resp).await?;
        let parsed: CreateSessionResponse = resp
            .json()
            .await
            .map_err(|e| PlatformError::Unknown(format!("refreshSession parse: {e}")))?;
        Ok(BlueskySession {
            did: parsed.did,
            handle: parsed.handle,
            access_jwt: parsed.access_jwt,
            refresh_jwt: parsed.refresh_jwt,
        })
    }
}

// ─────────────────────────────────────────────────────────────────────────
// HTTP helpers (instance methods so we share self.client / self.pds_host)
// ─────────────────────────────────────────────────────────────────────────

impl BlueskyAdapter {
    async fn post_authed(
        &self,
        url: &str,
        jwt: &str,
        body: &serde_json::Value,
    ) -> Result<reqwest::Response, PlatformError> {
        let resp = self
            .client
            .post(url)
            .bearer_auth(jwt)
            .json(body)
            .send()
            .await
            .map_err(map_transport_error)?;
        check_status(resp).await
    }

    async fn get_authed(&self, url: &str, jwt: &str) -> Result<reqwest::Response, PlatformError> {
        let resp = self
            .client
            .get(url)
            .bearer_auth(jwt)
            .send()
            .await
            .map_err(map_transport_error)?;
        check_status(resp).await
    }

    async fn upload_blobs(
        &self,
        jwt: &str,
        images: Vec<ImageRef>,
    ) -> Result<Vec<BlobRef>, PlatformError> {
        let mut out = Vec::with_capacity(images.len());
        for img in images {
            let bytes = self
                .client
                .get(&img.url)
                .send()
                .await
                .map_err(|e| PlatformError::NetworkError(format!("fetch image: {e}")))?
                .error_for_status()
                .map_err(|e| PlatformError::NetworkError(format!("fetch image status: {e}")))?
                .bytes()
                .await
                .map_err(|e| PlatformError::NetworkError(format!("fetch image bytes: {e}")))?;

            let mime = guess_mime_from_url(&img.url);
            let upload_url = format!("{}/xrpc/com.atproto.repo.uploadBlob", self.pds_host);
            let resp = self
                .client
                .post(&upload_url)
                .bearer_auth(jwt)
                .header("Content-Type", mime)
                .body(bytes)
                .send()
                .await
                .map_err(map_transport_error)?;
            let resp = check_status(resp).await?;
            let upload: UploadBlobResponse = resp
                .json()
                .await
                .map_err(|e| PlatformError::Unknown(format!("uploadBlob parse: {e}")))?;
            out.push(upload.blob);
        }
        Ok(out)
    }

    /// Construct the Bluesky `app.bsky.embed.external` embed value from a
    /// LinkCard. Pulled out as a method so tests can construct it without
    /// the trait machinery.
    fn build_external_embed(&self, card: &LinkCard) -> serde_json::Value {
        build_external_embed_value(card)
    }
}

// ─────────────────────────────────────────────────────────────────────────
// Pure helpers (testable without HTTP)
// ─────────────────────────────────────────────────────────────────────────

/// Build the JSON body for `com.atproto.repo.createRecord`.
fn build_publish_body(
    did: &str,
    text: &str,
    backlink_url: &str,
    embed: Option<serde_json::Value>,
) -> serde_json::Value {
    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let facets = link_facets(text, backlink_url);

    let mut record = serde_json::json!({
        "$type": POST_COLLECTION,
        "text": text,
        "createdAt": now,
    });
    if !facets.is_empty() {
        record["facets"] = serde_json::Value::Array(facets);
    }
    if let Some(e) = embed {
        record["embed"] = e;
    }

    serde_json::json!({
        "repo": did,
        "collection": POST_COLLECTION,
        "record": record,
    })
}

/// Build the `app.bsky.embed.external` embed value. No `thumb` blob is
/// attached in this PR — OG-tag fetch + thumb upload is deferred to a
/// later polish PR. Bluesky renders the card without thumb just fine.
fn build_external_embed_value(card: &LinkCard) -> serde_json::Value {
    serde_json::json!({
        "$type": "app.bsky.embed.external",
        "external": {
            "uri": card.backlink_url,
            "title": card.fallback_title,
            "description": card.fallback_description,
        }
    })
}

/// Build the `app.bsky.embed.images` embed value.
fn build_images_embed(blobs: Vec<BlobRef>) -> serde_json::Value {
    let images: Vec<serde_json::Value> = blobs
        .into_iter()
        .map(|blob| {
            serde_json::json!({
                "image": blob.into_value(),
                "alt": "",
            })
        })
        .collect();
    serde_json::json!({
        "$type": "app.bsky.embed.images",
        "images": images,
    })
}

/// Scan `text` for occurrences of `backlink_url` and emit a Bluesky link
/// facet for each. Byte offsets per AT Protocol spec
/// (https://docs.bsky.app/docs/advanced-guides/post-richtext) — these are
/// UTF-8 byte offsets, which Rust's `&str` indexing already gives us.
fn link_facets(text: &str, backlink_url: &str) -> Vec<serde_json::Value> {
    if backlink_url.is_empty() {
        return Vec::new();
    }
    let mut facets = Vec::new();
    let mut start = 0;
    while let Some(pos) = text[start..].find(backlink_url) {
        let abs = start + pos;
        let end = abs + backlink_url.len();
        facets.push(serde_json::json!({
            "index": { "byteStart": abs, "byteEnd": end },
            "features": [{ "$type": "app.bsky.richtext.facet#link", "uri": backlink_url }],
        }));
        start = end;
    }
    facets
}

/// `at://did:plc:.../app.bsky.feed.post/{rkey}` → `https://bsky.app/profile/{handle}/post/{rkey}`.
fn post_url_from_uri(handle: &str, at_uri: &str) -> String {
    let rkey = at_uri.rsplit('/').next().unwrap_or("");
    format!("https://bsky.app/profile/{handle}/post/{rkey}")
}

fn parse_engagement(resp: &GetPostThreadResponse) -> EngagementCounts {
    EngagementCounts {
        likes: resp.thread.post.like_count.unwrap_or(0) as i32,
        comments: resp.thread.post.reply_count.unwrap_or(0) as i32,
        reposts: resp.thread.post.repost_count.unwrap_or(0) as i32,
    }
}

fn scan_for_backlink(
    resp: &GetAuthorFeedResponse,
    backlink_url: &str,
    handle: &str,
) -> Option<PublishedRef> {
    for entry in &resp.feed {
        // Match if the body text contains the backlink URL OR the embed's
        // external.uri equals the backlink URL — covers both rendering paths.
        let text_has = entry.post.record.text.as_deref().is_some_and(|t| t.contains(backlink_url));
        let embed_has = entry
            .post
            .embed
            .as_ref()
            .and_then(|e| e.external.as_ref())
            .is_some_and(|x| x.uri == backlink_url);
        if text_has || embed_has {
            return Some(PublishedRef {
                external_post_id: entry.post.uri.clone(),
                external_post_url: post_url_from_uri(handle, &entry.post.uri),
            });
        }
    }
    None
}

fn unwrap_bluesky_creds(
    creds: DecryptedCredentials,
) -> Result<(String, String, String), PlatformError> {
    match creds {
        DecryptedCredentials::Bluesky { did, handle, access_jwt, .. } => Ok((did, handle, access_jwt)),
        _ => Err(PlatformError::Unknown(
            "non-Bluesky credentials passed to BlueskyAdapter".into(),
        )),
    }
}

fn map_transport_error(e: reqwest::Error) -> PlatformError {
    if e.is_timeout() {
        PlatformError::NetworkError(format!("timeout: {e}"))
    } else if e.is_connect() {
        PlatformError::NetworkError(format!("connect: {e}"))
    } else {
        PlatformError::NetworkError(format!("transport: {e}"))
    }
}

async fn check_status(resp: reqwest::Response) -> Result<reqwest::Response, PlatformError> {
    let status = resp.status();
    if status.is_success() {
        return Ok(resp);
    }
    let body = resp.text().await.unwrap_or_default();
    Err(classify_http_error(status, &body))
}

fn classify_http_error(status: reqwest::StatusCode, body: &str) -> PlatformError {
    let msg = format!("status={status} body={body}");

    // AT Protocol returns several auth-class errors with HTTP 400 instead of
    // 401/403 — `ExpiredToken`, `InvalidToken`, `AuthRequired`,
    // `AccountTakedown` all come back as `400 Bad Request` with the real
    // reason in the body's `"error"` field. Status alone would misclassify
    // these as `ContentRejected`, sending the user to the post-detail Retry
    // CTA when they actually need to reconnect under Settings → Connections.
    // Inspect the body envelope first; fall back to the status map.
    if let Ok(envelope) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(err_code) = envelope.get("error").and_then(|v| v.as_str()) {
            const AUTH_ERROR_CODES: &[&str] = &[
                "ExpiredToken",
                "InvalidToken",
                "AuthRequired",
                "AccountTakedown",
            ];
            if AUTH_ERROR_CODES.contains(&err_code) {
                return PlatformError::AuthExpired(msg);
            }
        }
    }

    match status.as_u16() {
        401 | 403 => PlatformError::AuthExpired(msg),
        429 => PlatformError::RateLimited(msg),
        400 | 422 => PlatformError::ContentRejected(msg),
        500..=599 => PlatformError::NetworkError(msg),
        _ => PlatformError::Unknown(msg),
    }
}

fn guess_mime_from_url(url: &str) -> &'static str {
    let lower = url.rsplit('?').last().unwrap_or(url).to_ascii_lowercase();
    if lower.ends_with(".png") {
        "image/png"
    } else if lower.ends_with(".gif") {
        "image/gif"
    } else if lower.ends_with(".webp") {
        "image/webp"
    } else {
        "image/jpeg" // safe default for unknown / .jpg / .jpeg
    }
}

// ─────────────────────────────────────────────────────────────────────────
// AT Protocol response shapes
// ─────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct CreateRecordResponse {
    uri: String,
    #[allow(dead_code)]
    cid: String,
}

#[derive(Debug, Deserialize)]
struct UploadBlobResponse {
    blob: BlobRef,
}

/// Opaque blob reference returned by uploadBlob. Re-serialized as-is
/// inside `app.bsky.embed.images` so we don't need to model `$link` etc.
#[derive(Debug, Deserialize, Serialize, Clone)]
struct BlobRef(serde_json::Value);

impl BlobRef {
    fn into_value(self) -> serde_json::Value {
        self.0
    }
}

#[derive(Debug, Deserialize)]
struct GetPostThreadResponse {
    thread: ThreadView,
}

#[derive(Debug, Deserialize)]
struct ThreadView {
    post: ThreadViewPost,
}

#[derive(Debug, Deserialize)]
struct ThreadViewPost {
    #[serde(rename = "likeCount")]
    like_count: Option<u32>,
    #[serde(rename = "replyCount")]
    reply_count: Option<u32>,
    #[serde(rename = "repostCount")]
    repost_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct GetAuthorFeedResponse {
    feed: Vec<FeedEntry>,
}

#[derive(Debug, Deserialize)]
struct FeedEntry {
    post: FeedViewPost,
}

#[derive(Debug, Deserialize)]
struct FeedViewPost {
    uri: String,
    record: FeedViewPostRecord,
    embed: Option<FeedViewPostEmbed>,
}

#[derive(Debug, Deserialize)]
struct FeedViewPostRecord {
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FeedViewPostEmbed {
    external: Option<FeedViewExternal>,
}

#[derive(Debug, Deserialize)]
struct FeedViewExternal {
    uri: String,
}

// ─────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_card() -> LinkCard {
        LinkCard {
            backlink_url: "https://ratel.foundation/p/abc?utm_source=bluesky".into(),
            fallback_title: "My Post".into(),
            fallback_description: "Short description.".into(),
            fallback_thumb_url: None,
        }
    }

    // ── adapter metadata ────────────────────────────────────────────────
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

    // ── build_publish_body ──────────────────────────────────────────────
    #[test]
    fn build_publish_body_sets_repo_collection_and_record_type() {
        let body = build_publish_body("did:plc:test", "hello", "https://r/p", None);
        assert_eq!(body["repo"], "did:plc:test");
        assert_eq!(body["collection"], POST_COLLECTION);
        assert_eq!(body["record"]["$type"], POST_COLLECTION);
        assert_eq!(body["record"]["text"], "hello");
        assert!(body["record"]["createdAt"].is_string());
    }

    #[test]
    fn build_publish_body_emits_link_facet_for_backlink() {
        let text = "Check this out: https://r/p?utm_source=bluesky";
        let backlink = "https://r/p?utm_source=bluesky";
        let body = build_publish_body("did:plc:test", text, backlink, None);
        let facets = body["record"]["facets"].as_array().unwrap();
        assert_eq!(facets.len(), 1);
        assert_eq!(
            facets[0]["features"][0]["$type"],
            "app.bsky.richtext.facet#link"
        );
        assert_eq!(facets[0]["features"][0]["uri"], backlink);
        // Byte offsets must point at the URL substring.
        let start = facets[0]["index"]["byteStart"].as_u64().unwrap() as usize;
        let end = facets[0]["index"]["byteEnd"].as_u64().unwrap() as usize;
        assert_eq!(&text[start..end], backlink);
    }

    #[test]
    fn build_publish_body_includes_embed_when_provided() {
        let card = sample_card();
        let embed = build_external_embed_value(&card);
        let body = build_publish_body("did:plc:test", "txt", &card.backlink_url, Some(embed));
        assert_eq!(body["record"]["embed"]["$type"], "app.bsky.embed.external");
        assert_eq!(body["record"]["embed"]["external"]["uri"], card.backlink_url);
        assert_eq!(body["record"]["embed"]["external"]["title"], "My Post");
    }

    #[test]
    fn build_publish_body_omits_facets_when_no_backlink_in_text() {
        let body = build_publish_body("did:plc:test", "no urls here", "https://other/p", None);
        assert!(body["record"].get("facets").is_none());
    }

    // ── link_facets ─────────────────────────────────────────────────────
    #[test]
    fn link_facets_handles_unicode_byte_offsets() {
        let url = "https://r/p";
        let text = format!("안녕 {url} 끝");
        let facets = link_facets(&text, url);
        assert_eq!(facets.len(), 1);
        let start = facets[0]["index"]["byteStart"].as_u64().unwrap() as usize;
        let end = facets[0]["index"]["byteEnd"].as_u64().unwrap() as usize;
        assert_eq!(&text[start..end], url);
    }

    #[test]
    fn link_facets_returns_empty_when_url_absent() {
        let facets = link_facets("no url", "https://r/p");
        assert!(facets.is_empty());
    }

    #[test]
    fn link_facets_returns_empty_when_backlink_url_empty() {
        let facets = link_facets("some text", "");
        assert!(facets.is_empty());
    }

    // ── post_url_from_uri ───────────────────────────────────────────────
    #[test]
    fn post_url_from_uri_builds_public_url() {
        let url = post_url_from_uri(
            "user.bsky.social",
            "at://did:plc:abc/app.bsky.feed.post/3kxyz",
        );
        assert_eq!(url, "https://bsky.app/profile/user.bsky.social/post/3kxyz");
    }

    // ── build_external_embed_value ──────────────────────────────────────
    #[test]
    fn build_external_embed_value_includes_uri_title_description() {
        let card = sample_card();
        let v = build_external_embed_value(&card);
        assert_eq!(v["$type"], "app.bsky.embed.external");
        assert_eq!(v["external"]["uri"], card.backlink_url);
        assert_eq!(v["external"]["title"], card.fallback_title);
        assert_eq!(v["external"]["description"], card.fallback_description);
    }

    // ── parse_engagement ────────────────────────────────────────────────
    #[test]
    fn parse_engagement_extracts_counts_from_thread_response() {
        let resp = GetPostThreadResponse {
            thread: ThreadView {
                post: ThreadViewPost {
                    like_count: Some(12),
                    reply_count: Some(3),
                    repost_count: Some(7),
                },
            },
        };
        let counts = parse_engagement(&resp);
        assert_eq!(counts, EngagementCounts { likes: 12, comments: 3, reposts: 7 });
    }

    #[test]
    fn parse_engagement_treats_missing_counts_as_zero() {
        let resp = GetPostThreadResponse {
            thread: ThreadView {
                post: ThreadViewPost {
                    like_count: None,
                    reply_count: None,
                    repost_count: None,
                },
            },
        };
        let counts = parse_engagement(&resp);
        assert_eq!(counts, EngagementCounts::default());
    }

    // ── scan_for_backlink ───────────────────────────────────────────────
    fn make_feed_entry(uri: &str, text: Option<&str>, embed_uri: Option<&str>) -> FeedEntry {
        FeedEntry {
            post: FeedViewPost {
                uri: uri.into(),
                record: FeedViewPostRecord { text: text.map(|s| s.into()) },
                embed: embed_uri.map(|u| FeedViewPostEmbed {
                    external: Some(FeedViewExternal { uri: u.into() }),
                }),
            },
        }
    }

    #[test]
    fn scan_for_backlink_matches_text_containing_url() {
        let resp = GetAuthorFeedResponse {
            feed: vec![
                make_feed_entry("at://x/post/1", Some("unrelated"), None),
                make_feed_entry("at://x/post/2", Some("see https://r/p?utm_source=bluesky now"), None),
            ],
        };
        let hit =
            scan_for_backlink(&resp, "https://r/p?utm_source=bluesky", "u.bsky.social").unwrap();
        assert_eq!(hit.external_post_id, "at://x/post/2");
        assert_eq!(hit.external_post_url, "https://bsky.app/profile/u.bsky.social/post/2");
    }

    #[test]
    fn scan_for_backlink_matches_embed_external_uri() {
        let resp = GetAuthorFeedResponse {
            feed: vec![make_feed_entry(
                "at://x/post/77",
                Some("body without url"),
                Some("https://r/p?utm_source=bluesky"),
            )],
        };
        let hit =
            scan_for_backlink(&resp, "https://r/p?utm_source=bluesky", "u.bsky.social").unwrap();
        assert_eq!(hit.external_post_id, "at://x/post/77");
    }

    #[test]
    fn scan_for_backlink_returns_none_when_no_match() {
        let resp = GetAuthorFeedResponse {
            feed: vec![make_feed_entry("at://x/post/1", Some("nothing here"), None)],
        };
        assert!(scan_for_backlink(&resp, "https://r/p?utm_source=bluesky", "u.bsky.social").is_none());
    }

    // ── classify_http_error ─────────────────────────────────────────────
    #[test]
    fn classify_401_as_auth_expired() {
        let err = classify_http_error(reqwest::StatusCode::UNAUTHORIZED, "expired");
        assert!(matches!(err, PlatformError::AuthExpired(_)));
    }

    #[test]
    fn classify_429_as_rate_limited() {
        let err = classify_http_error(reqwest::StatusCode::TOO_MANY_REQUESTS, "slow down");
        assert!(matches!(err, PlatformError::RateLimited(_)));
    }

    #[test]
    fn classify_400_as_content_rejected() {
        let err = classify_http_error(reqwest::StatusCode::BAD_REQUEST, "bad input");
        assert!(matches!(err, PlatformError::ContentRejected(_)));
    }

    #[test]
    fn classify_400_with_expired_token_as_auth_expired() {
        // AT Protocol returns auth-class errors with HTTP 400 — body's
        // `error` field is what actually carries the bucket.
        let body = r#"{"error":"ExpiredToken","message":"Token has expired"}"#;
        let err = classify_http_error(reqwest::StatusCode::BAD_REQUEST, body);
        assert!(
            matches!(err, PlatformError::AuthExpired(_)),
            "expected AuthExpired, got {err:?}"
        );
    }

    #[test]
    fn classify_400_with_invalid_token_as_auth_expired() {
        let body = r#"{"error":"InvalidToken","message":"bad sig"}"#;
        let err = classify_http_error(reqwest::StatusCode::BAD_REQUEST, body);
        assert!(matches!(err, PlatformError::AuthExpired(_)));
    }

    #[test]
    fn classify_500_as_network_error() {
        let err = classify_http_error(reqwest::StatusCode::INTERNAL_SERVER_ERROR, "down");
        assert!(matches!(err, PlatformError::NetworkError(_)));
    }

    // ── guess_mime_from_url ─────────────────────────────────────────────
    #[test]
    fn guess_mime_picks_correct_type_per_extension() {
        assert_eq!(guess_mime_from_url("https://x.com/img.png"), "image/png");
        assert_eq!(guess_mime_from_url("https://x.com/img.gif"), "image/gif");
        assert_eq!(guess_mime_from_url("https://x.com/img.webp"), "image/webp");
        assert_eq!(guess_mime_from_url("https://x.com/img.jpg"), "image/jpeg");
        assert_eq!(guess_mime_from_url("https://x.com/img.jpeg"), "image/jpeg");
        // Unknown extensions default to jpeg as a safe guess.
        assert_eq!(guess_mime_from_url("https://x.com/img.xyz"), "image/jpeg");
    }

    #[test]
    fn guess_mime_strips_query_string() {
        assert_eq!(
            guess_mime_from_url("https://x.com/img.png?signed=1&v=2"),
            "image/png"
        );
    }
}
