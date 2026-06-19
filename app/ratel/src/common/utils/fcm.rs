//! Firebase Cloud Messaging (HTTP v1) sender for Android/iOS push.
//!
//! Auth: a Firebase **service account** key (the `GOOGLE_APPLICATION_CREDENTIALS`
//! JSON, default `.gcp/firebase-service-account.json`) is used to mint a
//! short-lived OAuth2 access token (RS256 self-signed JWT → token endpoint),
//! which is cached in-process until ~5 min before expiry. Each push is one
//! POST to `…/projects/{project_id}/messages:send`.
//!
//! Graceful degradation: if the key is missing/unreadable (e.g. the secret
//! isn't wired in this environment yet) we log once and treat the send as a
//! soft failure — the inbox row is already written, so the in-app notification
//! still works; only the push is skipped.
#![cfg(feature = "server")]

use std::sync::Mutex;
use std::sync::OnceLock;

use serde::Deserialize;
use serde_json::json;

const SCOPE: &str = "https://www.googleapis.com/auth/firebase.messaging";

#[derive(Debug, Clone, Deserialize)]
struct ServiceAccount {
    client_email: String,
    private_key: String,
    token_uri: String,
    project_id: String,
}

fn service_account() -> Option<&'static ServiceAccount> {
    static SA: OnceLock<Option<ServiceAccount>> = OnceLock::new();
    SA.get_or_init(|| {
        let path = std::env::var("GOOGLE_APPLICATION_CREDENTIALS")
            .unwrap_or_else(|_| ".gcp/firebase-service-account.json".to_string());
        match std::fs::read_to_string(&path) {
            Ok(s) => match serde_json::from_str::<ServiceAccount>(&s) {
                Ok(sa) => Some(sa),
                Err(e) => {
                    crate::error!("FCM: service account JSON parse failed ({path}): {e}");
                    None
                }
            },
            Err(e) => {
                crate::error!("FCM: service account not readable ({path}): {e} — push disabled");
                None
            }
        }
    })
    .as_ref()
}

#[derive(Clone)]
struct CachedToken {
    token: String,
    /// epoch seconds when the token should be considered expired
    expires_at: i64,
}

fn token_cache() -> &'static Mutex<Option<CachedToken>> {
    static C: OnceLock<Mutex<Option<CachedToken>>> = OnceLock::new();
    C.get_or_init(|| Mutex::new(None))
}

#[derive(serde::Serialize)]
struct JwtClaims<'a> {
    iss: &'a str,
    scope: &'a str,
    aud: &'a str,
    iat: i64,
    exp: i64,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: i64,
}

/// Mint (or reuse a cached) OAuth2 access token for FCM.
async fn access_token(sa: &ServiceAccount) -> Option<String> {
    let now = crate::common::utils::time::get_now_timestamp_millis() / 1000;

    if let Some(c) = token_cache().lock().ok().and_then(|g| g.clone()) {
        if c.expires_at > now + 300 {
            return Some(c.token);
        }
    }

    let claims = JwtClaims {
        iss: &sa.client_email,
        scope: SCOPE,
        aud: &sa.token_uri,
        iat: now,
        exp: now + 3600,
    };
    let key = jsonwebtoken::EncodingKey::from_rsa_pem(sa.private_key.as_bytes())
        .map_err(|e| crate::error!("FCM: bad private key: {e}"))
        .ok()?;
    let jwt = jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256),
        &claims,
        &key,
    )
    .map_err(|e| crate::error!("FCM: JWT sign failed: {e}"))
    .ok()?;

    let resp = reqwest::Client::new()
        .post(&sa.token_uri)
        .form(&[
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &jwt),
        ])
        .send()
        .await
        .map_err(|e| crate::error!("FCM: token request failed: {e}"))
        .ok()?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        crate::error!("FCM: token exchange {status}: {body}");
        return None;
    }

    let tr: TokenResponse = resp
        .json()
        .await
        .map_err(|e| crate::error!("FCM: token decode failed: {e}"))
        .ok()?;

    let cached = CachedToken {
        token: tr.access_token.clone(),
        expires_at: now + tr.expires_in,
    };
    if let Ok(mut g) = token_cache().lock() {
        *g = Some(cached);
    }
    Some(tr.access_token)
}

/// Title/body/CTA for one push.
pub struct PushMessage {
    pub title: String,
    pub body: String,
    /// App-relative route the tap should deep-link to (the inbox `url()`).
    pub url: String,
}

/// Outcome of a single-token send, so the caller can prune dead tokens.
#[derive(Debug, PartialEq)]
pub enum PushOutcome {
    Sent,
    /// FCM reports the token is no longer valid (UNREGISTERED / 404) → delete it.
    Stale,
    /// Transient/soft failure (also covers "push disabled" = no key).
    Failed,
}

/// Send one push to one device token via FCM HTTP v1.
///
/// The message carries BOTH `notification` (so Android shows it in the system
/// tray while the app is backgrounded) and `data.url` (read from the launch
/// intent on tap to deep-link). `android.priority = high` wakes the device.
pub async fn send_to_token(device_token: &str, msg: &PushMessage) -> PushOutcome {
    let Some(sa) = service_account() else {
        return PushOutcome::Failed;
    };
    let Some(access) = access_token(sa).await else {
        return PushOutcome::Failed;
    };

    let url = format!(
        "https://fcm.googleapis.com/v1/projects/{}/messages:send",
        sa.project_id
    );
    let payload = json!({
        "message": {
            "token": device_token,
            "notification": { "title": msg.title, "body": msg.body },
            "data": { "url": msg.url },
            // No `click_action`: the default launcher tap opens MainActivity
            // (singleTask) with the `data` payload as intent extras, which
            // MainActivity reads as the deep-link `url`. A custom click_action
            // would need a matching intent-filter we don't declare.
            "android": {
                "priority": "high",
                "notification": { "default_sound": true }
            },
            // iOS: FCM relays via APNs. The top-level `notification` maps to
            // `aps.alert`, and the `data` keys merge into the notification
            // userInfo — so `userInfo["url"]` reaches RatelPush's tap handler
            // (mirror of Android's intent-extra deep link). `sound: default`
            // plays the standard tone; `priority 10` = immediate delivery.
            "apns": {
                "headers": { "apns-priority": "10" },
                "payload": { "aps": { "sound": "default" } }
            }
        }
    });

    let resp = match reqwest::Client::new()
        .post(&url)
        .bearer_auth(&access)
        .json(&payload)
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            crate::error!("FCM: send request failed: {e}");
            return PushOutcome::Failed;
        }
    };

    let status = resp.status();
    if status.is_success() {
        return PushOutcome::Sent;
    }

    let body = resp.text().await.unwrap_or_default();
    // 404 / UNREGISTERED / INVALID_ARGUMENT on the token → prune it.
    if status.as_u16() == 404 || body.contains("UNREGISTERED") || body.contains("NOT_FOUND") {
        crate::error!("FCM: stale token (pruning): {status} {body}");
        return PushOutcome::Stale;
    }
    crate::error!("FCM: send failed: {status} {body}");
    PushOutcome::Failed
}
