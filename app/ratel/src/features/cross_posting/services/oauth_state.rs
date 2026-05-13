//! Stateless OAuth `state` parameter for the LinkedIn (and future Threads)
//! connect flow. Encodes `(user_pk, nonce, expiry)` into an HMAC-signed
//! base64url blob — no DB / cache lookup on the callback side.
//!
//! Format:
//! ```text
//! state = base64url(payload_json) "." base64url(hmac_sha256)
//! payload_json = {"user_pk":"USER#abc","nonce":"<uuid>","exp":<epoch_secs>}
//! hmac_sha256 = HMAC-SHA256(subkey, payload_bytes)
//! subkey = aead::derive_subkey(b"cross_posting/oauth_state/v1")
//! ```
//!
//! The subkey is derived from `CROSS_POSTING_DATA_KEY` so we don't need
//! a separate envvar; rotating that key invalidates every in-flight
//! state token (TTL ≤ 10 min, so users would only need to retry).
//!
//! The `nonce` UUID isn't load-bearing for security — the HMAC alone
//! prevents forgery. It exists so two parallel connect attempts from
//! the same user don't produce the same state string (avoids browser
//! history collisions and gives logs a discriminator).

use crate::common::Partition;
use crate::common::utils::aead;
use crate::common::utils::time;
use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD as B64;
use serde::{Deserialize, Serialize};

/// Subkey label. **Versioned** — bump the `v1` suffix if the payload
/// shape ever changes incompatibly, so old tokens stop verifying instead
/// of being misinterpreted under the new schema.
const SUBKEY_LABEL: &[u8] = b"cross_posting/oauth_state/v1";

/// State token TTL. 10 minutes is plenty for the LinkedIn consent screen
/// — the user clicks Allow within ~30s in the typical case, and tokens
/// shouldn't outlive the realistic decision window.
const STATE_TTL_SECS: i64 = 600;

#[derive(Debug, thiserror::Error)]
pub enum OauthStateError {
    #[error("oauth state malformed")]
    Malformed,
    #[error("oauth state signature mismatch")]
    SignatureMismatch,
    #[error("oauth state expired")]
    Expired,
    #[error("oauth state user mismatch")]
    UserMismatch,
    #[error("oauth state subkey derive failed: {0}")]
    Subkey(String),
}

/// Decoded state contents — what the caller learns after verification.
/// `user_pk` is what the caller compares against the authenticated
/// `User` on the callback to confirm the same browser that started the
/// flow is finishing it.
#[derive(Debug, Clone)]
pub struct DecodedState {
    pub user_pk: Partition,
    pub nonce: String,
    pub expires_at: i64,
}

/// JSON payload shape — kept private so callers can't depend on field
/// ordering / future additions.
#[derive(Serialize, Deserialize)]
struct Payload<'a> {
    user_pk: &'a str,
    nonce: &'a str,
    exp: i64,
}

/// Build a fresh state token tied to `user_pk` with a random `nonce` and
/// a `now + STATE_TTL_SECS` expiry. Only the init controller should call
/// this.
pub fn encode(user_pk: &Partition) -> Result<String, OauthStateError> {
    let nonce = uuid::Uuid::now_v7().to_string();
    let exp = (time::now() / 1000) + STATE_TTL_SECS;
    encode_with(user_pk, &nonce, exp)
}

/// Lower-level encode — exposed for tests so a fixed nonce / exp can be
/// pinned. Production code should use [`encode`].
pub fn encode_with(
    user_pk: &Partition,
    nonce: &str,
    exp: i64,
) -> Result<String, OauthStateError> {
    let user_pk_str = user_pk.to_string();
    let payload = Payload {
        user_pk: &user_pk_str,
        nonce,
        exp,
    };
    let payload_bytes = serde_json::to_vec(&payload).map_err(|e| {
        OauthStateError::Subkey(format!("payload serialize: {e}"))
    })?;
    let mac = compute_hmac(&payload_bytes)?;
    Ok(format!(
        "{}.{}",
        B64.encode(&payload_bytes),
        B64.encode(mac)
    ))
}

/// Verify the HMAC, parse, and check expiry. The caller is then
/// responsible for matching `decoded.user_pk` against the currently
/// authenticated user — if those differ it's a cross-user replay attempt
/// (legitimate flow always finishes in the same browser session).
pub fn decode_and_verify(state: &str) -> Result<DecodedState, OauthStateError> {
    let (payload_b64, mac_b64) = state.split_once('.').ok_or(OauthStateError::Malformed)?;
    let payload_bytes = B64
        .decode(payload_b64)
        .map_err(|_| OauthStateError::Malformed)?;
    let received_mac = B64
        .decode(mac_b64)
        .map_err(|_| OauthStateError::Malformed)?;

    let expected_mac = compute_hmac(&payload_bytes)?;
    if !constant_time_eq(&expected_mac, &received_mac) {
        return Err(OauthStateError::SignatureMismatch);
    }

    let parsed: Payload<'_> = serde_json::from_slice(&payload_bytes)
        .map_err(|_| OauthStateError::Malformed)?;
    let now_secs = time::now() / 1000;
    if parsed.exp < now_secs {
        return Err(OauthStateError::Expired);
    }

    let user_pk: Partition = parsed
        .user_pk
        .parse()
        .map_err(|_| OauthStateError::Malformed)?;

    Ok(DecodedState {
        user_pk,
        nonce: parsed.nonce.to_string(),
        expires_at: parsed.exp,
    })
}

fn compute_hmac(payload: &[u8]) -> Result<Vec<u8>, OauthStateError> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    let subkey = aead::derive_subkey(SUBKEY_LABEL)
        .map_err(|e| OauthStateError::Subkey(format!("{e}")))?;
    let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(&subkey)
        .expect("HMAC accepts any key length");
    mac.update(payload);
    Ok(mac.finalize().into_bytes().to_vec())
}

/// Constant-time byte comparison so a timing-side-channel can't
/// distinguish "first byte mismatched" from "all bytes mismatched".
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pk() -> Partition {
        Partition::User("test-user-id".into())
    }

    // The unit tests below only run when CROSS_POSTING_DATA_KEY is
    // baked into the test binary (the `make test` env). Without the key,
    // `derive_subkey` returns AeadError and these tests would noisily
    // fail with an unrelated error — gate on the env so they're skipped
    // cleanly in environments that lack it.
    fn key_available() -> bool {
        option_env!("CROSS_POSTING_DATA_KEY")
            .map(|v| !v.is_empty())
            .unwrap_or(false)
    }

    #[test]
    fn encode_then_decode_roundtrips_user_pk_and_nonce() {
        if !key_available() {
            return;
        }
        let now_secs = time::now() / 1000;
        let token = encode_with(&pk(), "abc-nonce", now_secs + 60).unwrap();
        let decoded = decode_and_verify(&token).unwrap();
        assert_eq!(decoded.user_pk, pk());
        assert_eq!(decoded.nonce, "abc-nonce");
        assert!(decoded.expires_at >= now_secs);
    }

    #[test]
    fn malformed_state_returns_malformed() {
        if !key_available() {
            return;
        }
        let err = decode_and_verify("not-a-state").unwrap_err();
        assert!(matches!(err, OauthStateError::Malformed));
    }

    #[test]
    fn tampered_payload_fails_signature_check() {
        if !key_available() {
            return;
        }
        let now_secs = time::now() / 1000;
        let token = encode_with(&pk(), "abc", now_secs + 60).unwrap();
        // Swap the payload b64 segment for a different (valid b64) string.
        let (_, sig) = token.split_once('.').unwrap();
        let bad_payload = B64.encode(b"{\"user_pk\":\"USER#evil\",\"nonce\":\"x\",\"exp\":999999999999}");
        let tampered = format!("{bad_payload}.{sig}");
        let err = decode_and_verify(&tampered).unwrap_err();
        assert!(matches!(err, OauthStateError::SignatureMismatch));
    }

    #[test]
    fn expired_state_returns_expired() {
        if !key_available() {
            return;
        }
        let now_secs = time::now() / 1000;
        let token = encode_with(&pk(), "abc", now_secs - 1).unwrap();
        let err = decode_and_verify(&token).unwrap_err();
        assert!(matches!(err, OauthStateError::Expired));
    }

    #[test]
    fn constant_time_eq_returns_true_on_equal_slices() {
        assert!(constant_time_eq(b"abc", b"abc"));
        assert!(!constant_time_eq(b"abc", b"abd"));
        assert!(!constant_time_eq(b"abc", b"abcd"));
    }
}
