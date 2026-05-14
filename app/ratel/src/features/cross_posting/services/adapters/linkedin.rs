//! LinkedIn UGC adapter (Phase 1B).
//!
//! Endpoints used (all under `https://api.linkedin.com`):
//! - `POST /v2/ugcPosts` — publish. Returns the new post URN in the
//!   `X-RestLi-Id` response header (canonical) and inside the JSON body.
//! - `POST /v2/assets?action=registerUpload` + the returned upload URL —
//!   two-step image upload before referencing the asset URN inside the
//!   ugcPost's `media` array.
//! - `GET  /v2/socialActions/{urn}` — engagement counts (likes / comments).
//!   LinkedIn does not surface a "reposts" counter on the UGC API; the
//!   `EngagementCounts.reposts` field is left at 0.
//! - `GET  /v2/ugcPosts?q=authors&authors=List({memberUrn})&count=50` —
//!   recent-post scan for the `find_by_backlink` lock-recovery probe.
//!
//! OAuth refresh:
//! - `POST https://www.linkedin.com/oauth/v2/accessToken` with
//!   `grant_type=refresh_token`. 365-day refresh TTL, no rotation of the
//!   refresh token itself in LinkedIn's flow — we re-write whatever the
//!   response carries (refresh_token may be omitted; fall back to the
//!   prior value).
//!
//! All requests include `LinkedIn-Version: 202404` and
//! `X-Restli-Protocol-Version: 2.0.0` per LinkedIn's versioned-API contract.

use super::{
    CrossPostAdapter, DecryptedCredentials, EngagementCounts, ImageRef, LinkCard, PlatformError,
    PublishedRef,
};
use crate::features::cross_posting::types::SocialPlatform;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

const API_HOST: &str = "https://api.linkedin.com";
const OAUTH_HOST: &str = "https://www.linkedin.com";
const LINKEDIN_VERSION: &str = "202404";

/// Public-feed permalink template. LinkedIn accepts the raw URN here.
const PUBLIC_FEED_PREFIX: &str = "https://www.linkedin.com/feed/update";

/// LinkedIn UGC adapter. Stateless aside from a shared `reqwest::Client`.
#[derive(Debug, Clone)]
pub struct LinkedInAdapter {
    pub api_host: String,
    pub oauth_host: String,
    client: reqwest::Client,
}

impl LinkedInAdapter {
    pub fn new() -> Self {
        Self::with_hosts(API_HOST, OAUTH_HOST)
    }

    pub fn with_hosts(api_host: impl Into<String>, oauth_host: impl Into<String>) -> Self {
        Self {
            api_host: api_host.into(),
            oauth_host: oauth_host.into(),
            client: reqwest::Client::new(),
        }
    }
}

impl Default for LinkedInAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CrossPostAdapter for LinkedInAdapter {
    fn platform(&self) -> SocialPlatform {
        SocialPlatform::LinkedIn
    }

    fn char_limit(&self) -> usize {
        SocialPlatform::LinkedIn.char_limit()
    }

    fn max_images(&self) -> usize {
        SocialPlatform::LinkedIn.max_images()
    }

    async fn publish(
        &self,
        creds: DecryptedCredentials,
        formatted_body: String,
        images: Vec<ImageRef>,
        link_card: LinkCard,
    ) -> Result<PublishedRef, PlatformError> {
        let (access_token, member_urn) = unwrap_linkedin_creds(creds)?;
        let author_urn = person_urn(&member_urn);

        // LinkedIn requires the media block to be either NONE+article OR
        // IMAGE+image asset URNs. Pick which based on attached images.
        let media_block = if images.is_empty() {
            article_media_block(&link_card)
        } else {
            let asset_urns = self
                .upload_images(&access_token, &author_urn, images)
                .await?;
            image_media_block(&asset_urns)
        };

        let body = build_ugc_post_body(&author_urn, &formatted_body, media_block);

        let resp = self
            .post_authed(
                &format!("{}/v2/ugcPosts", self.api_host),
                &access_token,
                &body,
            )
            .await?;

        // LinkedIn returns the new post URN in `X-RestLi-Id`. The body also
        // echoes it under `id`, but the header is the canonical contract.
        let urn = resp
            .headers()
            .get("X-RestLi-Id")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let urn = match urn {
            Some(u) if !u.is_empty() => u,
            _ => {
                // Header missing — try the body. If both are absent, surface
                // an error so the dispatcher can retry.
                let parsed = resp
                    .json::<UgcPostResponse>()
                    .await
                    .map_err(|e| PlatformError::Unknown(format!("ugcPosts parse: {e}")))?;
                parsed.id.ok_or_else(|| {
                    PlatformError::Unknown("ugcPosts response missing id/X-RestLi-Id".into())
                })?
            }
        };

        Ok(PublishedRef {
            external_post_url: post_url_from_urn(&urn),
            external_post_id: urn,
        })
    }

    async fn fetch_engagement(
        &self,
        creds: DecryptedCredentials,
        external_post_id: &str,
    ) -> Result<EngagementCounts, PlatformError> {
        let (access_token, _member_urn) = unwrap_linkedin_creds(creds)?;

        let url = format!(
            "{}/v2/socialActions/{}",
            self.api_host,
            urlencoding::encode(external_post_id)
        );
        let resp = self
            .get_authed(&url, &access_token)
            .await?
            .json::<SocialActionsResponse>()
            .await
            .map_err(|e| PlatformError::Unknown(format!("socialActions parse: {e}")))?;

        Ok(parse_engagement(&resp))
    }

    async fn find_by_backlink(
        &self,
        creds: DecryptedCredentials,
        backlink_url: &str,
    ) -> Result<Option<PublishedRef>, PlatformError> {
        let (access_token, member_urn) = unwrap_linkedin_creds(creds)?;
        let author_urn = person_urn(&member_urn);

        // `q=authors` + `authors=List({urn})` requires Restli protocol v2;
        // header is set by `get_authed`. count=50 mirrors Bluesky's window.
        let url = format!(
            "{}/v2/ugcPosts?q=authors&authors=List({})&count=50",
            self.api_host,
            urlencoding::encode(&author_urn)
        );
        let resp = self
            .get_authed(&url, &access_token)
            .await?
            .json::<UgcPostsListResponse>()
            .await
            .map_err(|e| PlatformError::Unknown(format!("ugcPosts list parse: {e}")))?;

        Ok(scan_for_backlink(&resp, backlink_url))
    }

    /// LinkedIn implementation of FR-5 #35: rotate the access token via
    /// `/oauth/v2/accessToken` (`grant_type=refresh_token`). Refresh tokens
    /// have a 365-day TTL and do NOT auto-rotate — if the response omits
    /// `refresh_token`, we keep the prior value. This method is *pure* —
    /// caller (dispatcher) re-seals and persists onto `SocialConnection`.
    async fn try_refresh_credentials(
        &self,
        creds: DecryptedCredentials,
    ) -> Result<DecryptedCredentials, PlatformError> {
        let (refresh_token, member_urn) = match &creds {
            DecryptedCredentials::LinkedIn {
                refresh_token,
                member_urn,
                ..
            } => (refresh_token.clone(), member_urn.clone()),
            _ => {
                return Err(PlatformError::Unknown(
                    "LinkedIn adapter received non-LinkedIn credentials for refresh".into(),
                ));
            }
        };

        // No refresh token = app product tier doesn't grant one. Surface
        // AuthExpired so the dispatcher commits Failed and the user gets
        // the "reconnect" inbox CTA (FR-5 #35).
        let refresh_token = refresh_token.ok_or_else(|| {
            PlatformError::AuthExpired(
                "linkedin connection has no refresh token; user must reconnect".into(),
            )
        })?;

        let (client_id, client_secret) = oauth_client_credentials()?;

        let url = format!("{}/oauth/v2/accessToken", self.oauth_host);
        let form = [
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token.as_str()),
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
        ];

        let resp = self
            .client
            .post(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&form)
            .send()
            .await
            .map_err(map_transport_error)?;
        let resp = check_status(resp).await?;
        let parsed: TokenResponse = resp
            .json()
            .await
            .map_err(|e| PlatformError::Unknown(format!("oauth refresh parse: {e}")))?;

        // refresh_token may be omitted in LinkedIn's response — fall back to
        // the prior value so the next refresh attempt still has a token.
        let new_refresh = parsed.refresh_token.or(Some(refresh_token));

        Ok(DecryptedCredentials::LinkedIn {
            access_token: parsed.access_token,
            refresh_token: new_refresh,
            member_urn,
        })
    }
}

// ─────────────────────────────────────────────────────────────────────────
// Connect-time OAuth token-exchange (called from the connect controllers,
// NOT the dispatcher trait — same shape as Bluesky's `create_session`).
// ─────────────────────────────────────────────────────────────────────────

/// Newly-issued LinkedIn token pair, returned to the OAuth callback
/// controller for AEAD-sealing into `SocialConnection.credential_ciphertext`.
///
/// `refresh_token` is `Option` because LinkedIn only issues it for apps
/// with specific products attached. Standard OIDC-only apps get an
/// access token (60-day TTL) and no refresh token — caller falls back
/// to user-initiated reconnect when the access token expires.
#[derive(Clone)]
pub struct LinkedInSession {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub member_urn: String,
    /// Human-readable name from OIDC `/v2/userinfo` (`name` field).
    /// `Option` because OIDC spec lists `name` as optional — when the
    /// requested scopes don't include `profile` or the member opts out,
    /// the field can be absent. Callers should fall back to `member_urn`
    /// for the UI label in that case.
    pub display_name: Option<String>,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    /// LinkedIn does NOT always include `refresh_token` on the refresh
    /// path — only on the initial code-exchange. Optional field; callers
    /// fall back to the prior value when None.
    refresh_token: Option<String>,
    #[allow(dead_code)]
    expires_in: Option<i64>,
}

#[derive(Deserialize)]
struct UserinfoResponse {
    /// OIDC `sub` — the member's stable LinkedIn ID. Used to construct the
    /// `urn:li:person:{sub}` author URN required by `/v2/ugcPosts`.
    sub: String,
    /// OIDC `name` — the member's full display name. Optional in the OIDC
    /// spec; populated when the `profile` scope is granted (we always
    /// request it). Used as the connection's `external_handle` for the
    /// settings UI; falls back to `sub` (URN) when absent.
    name: Option<String>,
}

impl LinkedInAdapter {
    /// Exchange an OAuth `code` for an access/refresh token pair, then read
    /// `/v2/userinfo` to derive the member URN. Used by the connect-callback
    /// controller in 1B-C — kept here so OAuth state lives next to the
    /// publish path it feeds.
    pub async fn exchange_code(
        &self,
        code: &str,
        redirect_uri: &str,
    ) -> Result<LinkedInSession, PlatformError> {
        let (client_id, client_secret) = oauth_client_credentials()?;

        let token_url = format!("{}/oauth/v2/accessToken", self.oauth_host);
        let form = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", redirect_uri),
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
        ];
        let resp = self
            .client
            .post(&token_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&form)
            .send()
            .await
            .map_err(map_transport_error)?;
        let resp = check_status(resp).await?;
        let token: TokenResponse = resp
            .json()
            .await
            .map_err(|e| PlatformError::Unknown(format!("oauth exchange parse: {e}")))?;

        // refresh_token is optional. LinkedIn only includes it when the
        // app has a product attached that grants long-lived refresh
        // (e.g. "Share on LinkedIn"). OIDC-only apps return an
        // `access_token` (60-day TTL) and nothing else — the connection
        // still works for publishing until expiry, after which the user
        // reconnects via the inbox CTA (FR-5 #35 AuthExpired path).
        let refresh_token = token.refresh_token;

        // Pull the member's `sub` from /v2/userinfo (OIDC). With the
        // `openid profile` scope LinkedIn returns `sub` in the userinfo
        // response; we don't need the older /v2/me endpoint.
        let userinfo_url = format!("{}/v2/userinfo", self.api_host);
        let userinfo: UserinfoResponse = self
            .get_authed(&userinfo_url, &token.access_token)
            .await?
            .json()
            .await
            .map_err(|e| PlatformError::Unknown(format!("userinfo parse: {e}")))?;

        Ok(LinkedInSession {
            access_token: token.access_token,
            refresh_token,
            member_urn: userinfo.sub,
            display_name: userinfo.name,
        })
    }
}

// ─────────────────────────────────────────────────────────────────────────
// HTTP helpers
// ─────────────────────────────────────────────────────────────────────────

impl LinkedInAdapter {
    async fn post_authed(
        &self,
        url: &str,
        access_token: &str,
        body: &serde_json::Value,
    ) -> Result<reqwest::Response, PlatformError> {
        let resp = self
            .client
            .post(url)
            .bearer_auth(access_token)
            .header("X-Restli-Protocol-Version", "2.0.0")
            .header("LinkedIn-Version", LINKEDIN_VERSION)
            .json(body)
            .send()
            .await
            .map_err(map_transport_error)?;
        check_status(resp).await
    }

    async fn get_authed(
        &self,
        url: &str,
        access_token: &str,
    ) -> Result<reqwest::Response, PlatformError> {
        let resp = self
            .client
            .get(url)
            .bearer_auth(access_token)
            .header("X-Restli-Protocol-Version", "2.0.0")
            .header("LinkedIn-Version", LINKEDIN_VERSION)
            .send()
            .await
            .map_err(map_transport_error)?;
        check_status(resp).await
    }

    /// Two-step image upload: `registerUpload` to get a one-shot upload URL,
    /// then PUT the bytes there. Returns the asset URNs for embedding into
    /// the ugcPost's `media` array.
    async fn upload_images(
        &self,
        access_token: &str,
        author_urn: &str,
        images: Vec<ImageRef>,
    ) -> Result<Vec<String>, PlatformError> {
        let mut out = Vec::with_capacity(images.len());
        for img in images {
            // 1. registerUpload
            let register_body = build_register_upload_body(author_urn);
            let register_resp = self
                .post_authed(
                    &format!("{}/v2/assets?action=registerUpload", self.api_host),
                    access_token,
                    &register_body,
                )
                .await?
                .json::<RegisterUploadResponse>()
                .await
                .map_err(|e| PlatformError::Unknown(format!("registerUpload parse: {e}")))?;

            let upload_url = register_resp
                .value
                .upload_mechanism
                .media_upload_http_request
                .upload_url;
            let asset_urn = register_resp.value.asset;

            // 2. fetch image bytes from S3
            let bytes = self
                .client
                .get(&img.url)
                .send()
                .await
                .map_err(|e| PlatformError::NetworkError(format!("fetch image: {e}")))?
                .error_for_status()
                .map_err(|e| {
                    PlatformError::NetworkError(format!("fetch image status: {e}"))
                })?
                .bytes()
                .await
                .map_err(|e| {
                    PlatformError::NetworkError(format!("fetch image bytes: {e}"))
                })?;

            // 3. PUT to the upload URL. Note: this URL is one-shot and
            //    pre-authed — bearer token is NOT required (and using one
            //    actually returns 400). Send raw bytes only.
            let put_resp = self
                .client
                .put(&upload_url)
                .body(bytes)
                .send()
                .await
                .map_err(map_transport_error)?;
            check_status(put_resp).await?;

            out.push(asset_urn);
        }
        Ok(out)
    }
}

// ─────────────────────────────────────────────────────────────────────────
// Pure helpers (testable without HTTP)
// ─────────────────────────────────────────────────────────────────────────

/// Build the JSON body for `POST /v2/ugcPosts`. `media_block` is the
/// pre-built `shareMediaCategory` + `media` pair (article vs image).
fn build_ugc_post_body(
    author_urn: &str,
    text: &str,
    media_block: serde_json::Value,
) -> serde_json::Value {
    serde_json::json!({
        "author": author_urn,
        "lifecycleState": "PUBLISHED",
        "specificContent": {
            "com.linkedin.ugc.ShareContent": {
                "shareCommentary": { "text": text },
                "shareMediaCategory": media_block["shareMediaCategory"],
                "media": media_block["media"],
            }
        },
        "visibility": {
            // PUBLIC: visible to anyone on or off LinkedIn. Ratel's
            // syndicated copies are public-by-design (Stage 1 only fires
            // on Visibility::Public posts).
            "com.linkedin.ugc.MemberNetworkVisibility": "PUBLIC"
        }
    })
}

/// Build the `media` block for an article share — wraps the Ratel backlink
/// with the LinkCard's title/description as `originalUrl`/`title`/`description`.
fn article_media_block(card: &LinkCard) -> serde_json::Value {
    serde_json::json!({
        "shareMediaCategory": "ARTICLE",
        "media": [{
            "status": "READY",
            "originalUrl": card.backlink_url,
            "title": { "text": card.fallback_title },
            "description": { "text": card.fallback_description },
        }]
    })
}

/// Build the `media` block for an image share — references one or more
/// asset URNs returned by `registerUpload`.
fn image_media_block(asset_urns: &[String]) -> serde_json::Value {
    let media: Vec<serde_json::Value> = asset_urns
        .iter()
        .map(|urn| {
            serde_json::json!({
                "status": "READY",
                "media": urn,
            })
        })
        .collect();
    serde_json::json!({
        "shareMediaCategory": "IMAGE",
        "media": media,
    })
}

fn build_register_upload_body(author_urn: &str) -> serde_json::Value {
    serde_json::json!({
        "registerUploadRequest": {
            "owner": author_urn,
            "recipes": ["urn:li:digitalmediaRecipe:feedshare-image"],
            "serviceRelationships": [{
                "identifier": "urn:li:userGeneratedContent",
                "relationshipType": "OWNER",
            }]
        }
    })
}

fn parse_engagement(resp: &SocialActionsResponse) -> EngagementCounts {
    EngagementCounts {
        likes: resp
            .likes_summary
            .as_ref()
            .map(|s| s.aggregated_total_likes)
            .unwrap_or(0) as i32,
        comments: resp
            .comments_summary
            .as_ref()
            .map(|s| s.aggregated_total_comments)
            .unwrap_or(0) as i32,
        // LinkedIn UGC API doesn't expose a repost/share counter here.
        reposts: 0,
    }
}

fn scan_for_backlink(
    resp: &UgcPostsListResponse,
    backlink_url: &str,
) -> Option<PublishedRef> {
    for entry in &resp.elements {
        let text_has = entry.text(&entry.id).contains(backlink_url);
        let article_has = entry
            .specific_content
            .share_content
            .media
            .iter()
            .any(|m| m.original_url.as_deref() == Some(backlink_url));
        if text_has || article_has {
            let urn = entry.id.clone();
            return Some(PublishedRef {
                external_post_url: post_url_from_urn(&urn),
                external_post_id: urn,
            });
        }
    }
    None
}

fn person_urn(member_urn: &str) -> String {
    if member_urn.starts_with("urn:li:") {
        member_urn.to_string()
    } else {
        format!("urn:li:person:{member_urn}")
    }
}

fn post_url_from_urn(urn: &str) -> String {
    format!("{PUBLIC_FEED_PREFIX}/{urn}")
}

fn unwrap_linkedin_creds(
    creds: DecryptedCredentials,
) -> Result<(String, String), PlatformError> {
    match creds {
        DecryptedCredentials::LinkedIn {
            access_token,
            member_urn,
            ..
        } => Ok((access_token, member_urn)),
        _ => Err(PlatformError::Unknown(
            "non-LinkedIn credentials passed to LinkedInAdapter".into(),
        )),
    }
}

/// Read the `LINKEDIN_CLIENT_ID` / `LINKEDIN_CLIENT_SECRET` envvars baked
/// into the binary at compile time. 1B-A wires these through the dev/prod
/// workflow + the env.sh Makefile target so they reach the docker container
/// that runs cargo build. Missing values map to AuthExpired so the
/// dispatcher commits Failed and surfaces "reconnect" — same UX as a
/// revoked token, since we genuinely cannot refresh without the secret.
fn oauth_client_credentials() -> Result<(String, String), PlatformError> {
    let client_id = option_env!("LINKEDIN_CLIENT_ID")
        .filter(|v| !v.is_empty())
        .ok_or_else(|| {
            PlatformError::AuthExpired("LINKEDIN_CLIENT_ID not configured".into())
        })?;
    let client_secret = option_env!("LINKEDIN_CLIENT_SECRET")
        .filter(|v| !v.is_empty())
        .ok_or_else(|| {
            PlatformError::AuthExpired("LINKEDIN_CLIENT_SECRET not configured".into())
        })?;
    Ok((client_id.to_string(), client_secret.to_string()))
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
    match status.as_u16() {
        401 | 403 => PlatformError::AuthExpired(msg),
        429 => PlatformError::RateLimited(msg),
        400 | 422 => PlatformError::ContentRejected(msg),
        500..=599 => PlatformError::NetworkError(msg),
        _ => PlatformError::Unknown(msg),
    }
}

// ─────────────────────────────────────────────────────────────────────────
// LinkedIn API response shapes
// ─────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct UgcPostResponse {
    id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RegisterUploadResponse {
    value: RegisterUploadValue,
}

#[derive(Debug, Deserialize)]
struct RegisterUploadValue {
    asset: String,
    #[serde(rename = "uploadMechanism")]
    upload_mechanism: UploadMechanism,
}

#[derive(Debug, Deserialize)]
struct UploadMechanism {
    #[serde(
        rename = "com.linkedin.digitalmedia.uploading.MediaUploadHttpRequest"
    )]
    media_upload_http_request: MediaUploadHttpRequest,
}

#[derive(Debug, Deserialize)]
struct MediaUploadHttpRequest {
    #[serde(rename = "uploadUrl")]
    upload_url: String,
}

#[derive(Debug, Deserialize)]
struct SocialActionsResponse {
    #[serde(rename = "likesSummary")]
    likes_summary: Option<LikesSummary>,
    #[serde(rename = "commentsSummary")]
    comments_summary: Option<CommentsSummary>,
}

#[derive(Debug, Deserialize)]
struct LikesSummary {
    #[serde(rename = "aggregatedTotalLikes")]
    aggregated_total_likes: u64,
}

#[derive(Debug, Deserialize)]
struct CommentsSummary {
    #[serde(rename = "aggregatedTotalComments")]
    aggregated_total_comments: u64,
}

#[derive(Debug, Deserialize)]
struct UgcPostsListResponse {
    elements: Vec<UgcPostElement>,
}

#[derive(Debug, Deserialize, Serialize)]
struct UgcPostElement {
    id: String,
    #[serde(rename = "specificContent")]
    specific_content: SpecificContent,
}

impl UgcPostElement {
    /// LinkedIn's UGC list response wraps the body text under
    /// `specificContent.com.linkedin.ugc.ShareContent.shareCommentary.text`.
    /// Returns an empty string when missing — callers fall through to the
    /// article-URL match path.
    fn text(&self, _id_hint: &str) -> &str {
        self.specific_content
            .share_content
            .share_commentary
            .as_ref()
            .map(|c| c.text.as_str())
            .unwrap_or("")
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct SpecificContent {
    #[serde(rename = "com.linkedin.ugc.ShareContent")]
    share_content: ShareContent,
}

#[derive(Debug, Deserialize, Serialize)]
struct ShareContent {
    #[serde(rename = "shareCommentary", default)]
    share_commentary: Option<ShareCommentary>,
    #[serde(default)]
    media: Vec<ShareMedia>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ShareCommentary {
    text: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ShareMedia {
    #[serde(rename = "originalUrl")]
    original_url: Option<String>,
}

// ─────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_card() -> LinkCard {
        LinkCard {
            backlink_url: "https://ratel.foundation/p/abc?utm_source=linkedin".into(),
            fallback_title: "My Post".into(),
            fallback_description: "Short description.".into(),
            fallback_thumb_url: None,
        }
    }

    // ── adapter metadata ────────────────────────────────────────────────
    #[test]
    fn linkedin_adapter_reports_platform_facts() {
        let a = LinkedInAdapter::new();
        assert_eq!(a.platform(), SocialPlatform::LinkedIn);
        assert_eq!(a.char_limit(), 3_000);
        assert_eq!(a.max_images(), 1);
    }

    #[test]
    fn linkedin_adapter_default_hosts_are_public_apis() {
        let a = LinkedInAdapter::new();
        assert_eq!(a.api_host, "https://api.linkedin.com");
        assert_eq!(a.oauth_host, "https://www.linkedin.com");
    }

    // ── person_urn ──────────────────────────────────────────────────────
    #[test]
    fn person_urn_wraps_bare_id() {
        assert_eq!(person_urn("abc123"), "urn:li:person:abc123");
    }

    #[test]
    fn person_urn_passes_through_full_urn() {
        assert_eq!(person_urn("urn:li:person:abc123"), "urn:li:person:abc123");
    }

    // ── post_url_from_urn ───────────────────────────────────────────────
    #[test]
    fn post_url_from_urn_uses_feed_update_path() {
        let url = post_url_from_urn("urn:li:share:7100000000000000000");
        assert_eq!(
            url,
            "https://www.linkedin.com/feed/update/urn:li:share:7100000000000000000"
        );
    }

    // ── build_ugc_post_body ─────────────────────────────────────────────
    #[test]
    fn build_ugc_post_body_sets_author_lifecycle_and_visibility() {
        let media = article_media_block(&sample_card());
        let body = build_ugc_post_body("urn:li:person:abc", "hello world", media);
        assert_eq!(body["author"], "urn:li:person:abc");
        assert_eq!(body["lifecycleState"], "PUBLISHED");
        assert_eq!(
            body["visibility"]["com.linkedin.ugc.MemberNetworkVisibility"],
            "PUBLIC"
        );
        assert_eq!(
            body["specificContent"]["com.linkedin.ugc.ShareContent"]["shareCommentary"]["text"],
            "hello world"
        );
    }

    #[test]
    fn build_ugc_post_body_with_article_media_carries_backlink() {
        let card = sample_card();
        let media = article_media_block(&card);
        let body = build_ugc_post_body("urn:li:person:abc", "txt", media);
        let arr = body["specificContent"]["com.linkedin.ugc.ShareContent"]["media"]
            .as_array()
            .unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0]["originalUrl"], card.backlink_url);
        assert_eq!(arr[0]["title"]["text"], card.fallback_title);
        assert_eq!(arr[0]["description"]["text"], card.fallback_description);
        assert_eq!(
            body["specificContent"]["com.linkedin.ugc.ShareContent"]["shareMediaCategory"],
            "ARTICLE"
        );
    }

    #[test]
    fn build_ugc_post_body_with_image_media_uses_image_category() {
        let media = image_media_block(&[
            "urn:li:digitalmediaAsset:img1".to_string(),
            "urn:li:digitalmediaAsset:img2".to_string(),
        ]);
        let body = build_ugc_post_body("urn:li:person:abc", "txt", media);
        assert_eq!(
            body["specificContent"]["com.linkedin.ugc.ShareContent"]["shareMediaCategory"],
            "IMAGE"
        );
        let arr = body["specificContent"]["com.linkedin.ugc.ShareContent"]["media"]
            .as_array()
            .unwrap();
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0]["media"], "urn:li:digitalmediaAsset:img1");
        assert_eq!(arr[0]["status"], "READY");
    }

    // ── build_register_upload_body ──────────────────────────────────────
    #[test]
    fn register_upload_body_sets_owner_and_recipe() {
        let body = build_register_upload_body("urn:li:person:abc");
        assert_eq!(body["registerUploadRequest"]["owner"], "urn:li:person:abc");
        assert_eq!(
            body["registerUploadRequest"]["recipes"][0],
            "urn:li:digitalmediaRecipe:feedshare-image"
        );
        assert_eq!(
            body["registerUploadRequest"]["serviceRelationships"][0]["relationshipType"],
            "OWNER"
        );
    }

    // ── parse_engagement ────────────────────────────────────────────────
    #[test]
    fn parse_engagement_extracts_likes_and_comments() {
        let resp = SocialActionsResponse {
            likes_summary: Some(LikesSummary {
                aggregated_total_likes: 42,
            }),
            comments_summary: Some(CommentsSummary {
                aggregated_total_comments: 7,
            }),
        };
        let counts = parse_engagement(&resp);
        assert_eq!(
            counts,
            EngagementCounts {
                likes: 42,
                comments: 7,
                reposts: 0,
            }
        );
    }

    #[test]
    fn parse_engagement_treats_missing_summaries_as_zero() {
        let resp = SocialActionsResponse {
            likes_summary: None,
            comments_summary: None,
        };
        assert_eq!(parse_engagement(&resp), EngagementCounts::default());
    }

    // ── scan_for_backlink ───────────────────────────────────────────────
    fn make_element(
        id: &str,
        text: Option<&str>,
        article_url: Option<&str>,
    ) -> UgcPostElement {
        UgcPostElement {
            id: id.into(),
            specific_content: SpecificContent {
                share_content: ShareContent {
                    share_commentary: text.map(|t| ShareCommentary { text: t.into() }),
                    media: article_url
                        .map(|u| {
                            vec![ShareMedia {
                                original_url: Some(u.into()),
                            }]
                        })
                        .unwrap_or_default(),
                },
            },
        }
    }

    #[test]
    fn scan_for_backlink_matches_text_containing_url() {
        let resp = UgcPostsListResponse {
            elements: vec![
                make_element("urn:li:share:1", Some("unrelated"), None),
                make_element(
                    "urn:li:share:2",
                    Some("see https://r/p?utm_source=linkedin yo"),
                    None,
                ),
            ],
        };
        let hit = scan_for_backlink(&resp, "https://r/p?utm_source=linkedin").unwrap();
        assert_eq!(hit.external_post_id, "urn:li:share:2");
        assert_eq!(
            hit.external_post_url,
            "https://www.linkedin.com/feed/update/urn:li:share:2"
        );
    }

    #[test]
    fn scan_for_backlink_matches_article_original_url() {
        let resp = UgcPostsListResponse {
            elements: vec![make_element(
                "urn:li:share:77",
                Some("body without url"),
                Some("https://r/p?utm_source=linkedin"),
            )],
        };
        let hit = scan_for_backlink(&resp, "https://r/p?utm_source=linkedin").unwrap();
        assert_eq!(hit.external_post_id, "urn:li:share:77");
    }

    #[test]
    fn scan_for_backlink_returns_none_when_no_match() {
        let resp = UgcPostsListResponse {
            elements: vec![make_element("urn:li:share:1", Some("nothing here"), None)],
        };
        assert!(scan_for_backlink(&resp, "https://r/p?utm_source=linkedin").is_none());
    }

    // ── classify_http_error ─────────────────────────────────────────────
    #[test]
    fn classify_401_as_auth_expired() {
        let err = classify_http_error(reqwest::StatusCode::UNAUTHORIZED, "expired");
        assert!(matches!(err, PlatformError::AuthExpired(_)));
    }

    #[test]
    fn classify_429_as_rate_limited() {
        let err = classify_http_error(reqwest::StatusCode::TOO_MANY_REQUESTS, "slow");
        assert!(matches!(err, PlatformError::RateLimited(_)));
    }

    #[test]
    fn classify_400_as_content_rejected() {
        let err = classify_http_error(reqwest::StatusCode::BAD_REQUEST, "bad");
        assert!(matches!(err, PlatformError::ContentRejected(_)));
    }

    #[test]
    fn classify_500_as_network_error() {
        let err = classify_http_error(reqwest::StatusCode::INTERNAL_SERVER_ERROR, "down");
        assert!(matches!(err, PlatformError::NetworkError(_)));
    }
}
