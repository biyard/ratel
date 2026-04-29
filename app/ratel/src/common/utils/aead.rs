//! Authenticated symmetric encryption for at-rest secrets (Phase 1).
//!
//! Phase 1 of the cross-posting feature stores external-platform credentials
//! (Bluesky JWTs, LinkedIn / Threads OAuth tokens) sealed under AES-256-GCM
//! with a data key delivered via environment variable. AWS KMS is the
//! eventual target (per `roadmap/cross-posting.md` FR-1 #6); the migration
//! path is documented at
//! `docs/superpowers/specs/2026-04-28-cross-posting-design.md` →
//! "Credential encryption (Phase 1: envvar AEAD)".
//!
//! ## Sealed blob layout
//!
//! ```text
//! byte 0       : key version (matches the version of the envvar key that
//!                minted this blob — used for two-key rotation)
//! bytes 1..13  : 96-bit nonce (random per seal call)
//! bytes 13..   : AES-256-GCM ciphertext + 16-byte authentication tag
//! ```
//!
//! ## Envvar form
//!
//! `CROSS_POSTING_DATA_KEY = "v<int>:<base64-url-no-pad of 32 raw bytes>"`
//! (e.g. `"v1:n7Q...3xN"`). An optional `CROSS_POSTING_DATA_KEY_PREVIOUS`
//! with the same form is honored on `open()` paths to support rotation
//! transitions (new writes always go through CURRENT).
//!
//! Per the existing Ratel pattern (see `KAIA_FEEPAYER_KEY`,
//! `TELEGRAM_TOKEN`, `BBS_BLS_*`, etc.), envvar values are read via
//! `option_env!()` at **compile time** and baked into the binary —
//! Cargo automatically rebuilds when the value changes. AWS Lambda
//! runtime does not need a separate env var configured on the function.
//!
//! ## API layers
//!
//! - **Low-level**: [`seal_with_key`] / [`open_with_keys`] take explicit
//!   key material. Used by tests.
//! - **High-level**: [`seal`] / [`open`] read keys from compile-time
//!   `option_env!()` values via a [`OnceLock`] cache for parsed
//!   [`KeyMaterial`]. Used by feature code.

use aes_gcm::{
    AeadCore, Aes256Gcm, KeyInit,
    aead::{Aead, OsRng},
};
use base64::Engine as _;
use std::sync::OnceLock;
use thiserror::Error;

/// Size of the AES-256 key in bytes.
pub const KEY_LEN: usize = 32;
/// Size of the AES-GCM nonce in bytes.
pub const NONCE_LEN: usize = 12;
/// Number of leading bytes reserved for header (version + nonce).
const HEADER_LEN: usize = 1 + NONCE_LEN;

/// Envvar that holds the current sealing key.
pub const ENV_CURRENT: &str = "CROSS_POSTING_DATA_KEY";
/// Envvar that optionally holds the previous sealing key (during rotation).
pub const ENV_PREVIOUS: &str = "CROSS_POSTING_DATA_KEY_PREVIOUS";

#[derive(Debug, Error)]
pub enum AeadError {
    #[error("envvar {0} is not set")]
    EnvvarMissing(&'static str),

    #[error("envvar {envvar} value is malformed: {reason}")]
    EnvvarMalformed { envvar: &'static str, reason: String },

    #[error("sealed blob is too short ({0} bytes; minimum {min})", min = HEADER_LEN + 1)]
    BlobTooShort(usize),

    #[error("sealed blob version byte {0} does not match any known key")]
    UnknownKeyVersion(u8),

    #[error("AES-GCM seal failed")]
    SealFailed,

    #[error("AES-GCM open failed (wrong key, tampered ciphertext, or wrong nonce)")]
    OpenFailed,
}

/// Versioned 32-byte AES-256 key.
///
/// `version` matches the byte stored at offset 0 of every blob the key
/// produced — the open path uses it to look up the right key when more
/// than one is configured.
///
/// `Debug` is implemented manually to redact the raw key bytes — accidental
/// `tracing::debug!("{:?}", key)` must NEVER leak the secret. Only the
/// version byte is printed.
#[derive(Clone)]
pub struct KeyMaterial {
    pub version: u8,
    pub raw: [u8; KEY_LEN],
}

impl std::fmt::Debug for KeyMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyMaterial")
            .field("version", &self.version)
            .field("raw", &"<redacted>")
            .finish()
    }
}

impl KeyMaterial {
    /// Parse a `"v<int>:<base64>"` envvar value into a [`KeyMaterial`].
    pub fn from_envvar_value(envvar: &'static str, value: &str) -> Result<Self, AeadError> {
        let (version_str, b64) = value.split_once(':').ok_or_else(|| {
            AeadError::EnvvarMalformed {
                envvar,
                reason: "expected 'v<int>:<base64>'".into(),
            }
        })?;
        let version = version_str
            .strip_prefix('v')
            .ok_or_else(|| AeadError::EnvvarMalformed {
                envvar,
                reason: "version must start with 'v'".into(),
            })?
            .parse::<u8>()
            .map_err(|_| AeadError::EnvvarMalformed {
                envvar,
                reason: "version must be u8".into(),
            })?;
        let raw_vec = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(b64)
            .map_err(|e| AeadError::EnvvarMalformed {
                envvar,
                reason: format!("base64 decode: {e}"),
            })?;
        if raw_vec.len() != KEY_LEN {
            return Err(AeadError::EnvvarMalformed {
                envvar,
                reason: format!("expected {KEY_LEN} key bytes, got {}", raw_vec.len()),
            });
        }
        let raw: [u8; KEY_LEN] = raw_vec.try_into().expect("len already checked");
        Ok(KeyMaterial { version, raw })
    }
}

// ─────────────────────────────────────────────────────────────────────────
// Low-level (key passed explicitly — used by tests)
// ─────────────────────────────────────────────────────────────────────────

/// Encrypt `plaintext` using `key`. The output is a `Vec<u8>` containing
/// `[key.version][12-byte nonce][ciphertext + tag]` and is safe to store
/// in DynamoDB.
pub fn seal_with_key(plaintext: &[u8], key: &KeyMaterial) -> Result<Vec<u8>, AeadError> {
    let cipher = Aes256Gcm::new(&key.raw.into());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|_| AeadError::SealFailed)?;

    let mut out = Vec::with_capacity(HEADER_LEN + ciphertext.len());
    out.push(key.version);
    out.extend_from_slice(nonce.as_slice());
    out.extend(ciphertext);
    Ok(out)
}

/// Decrypt a sealed blob using whichever of `current` / `previous` matches
/// the blob's version byte. `previous` is consulted only when present and
/// only if its version matches the blob.
pub fn open_with_keys(
    blob: &[u8],
    current: &KeyMaterial,
    previous: Option<&KeyMaterial>,
) -> Result<Vec<u8>, AeadError> {
    if blob.len() < HEADER_LEN + 1 {
        return Err(AeadError::BlobTooShort(blob.len()));
    }
    let version = blob[0];
    let nonce_bytes: [u8; NONCE_LEN] =
        blob[1..HEADER_LEN].try_into().expect("slice len matches NONCE_LEN");
    let ciphertext = &blob[HEADER_LEN..];

    let key = if version == current.version {
        current
    } else if let Some(prev) = previous.filter(|p| p.version == version) {
        prev
    } else {
        return Err(AeadError::UnknownKeyVersion(version));
    };

    let cipher = Aes256Gcm::new(&key.raw.into());
    cipher
        .decrypt(&nonce_bytes.into(), ciphertext)
        .map_err(|_| AeadError::OpenFailed)
}

// ─────────────────────────────────────────────────────────────────────────
// High-level (envvar-backed — used by feature code)
// ─────────────────────────────────────────────────────────────────────────

/// Process-wide cache of `(CURRENT, Option<PREVIOUS>)` parsed from
/// envvars. `OnceLock` so the first reader populates and subsequent
/// readers are lock-free.
static KEYS: OnceLock<Result<(KeyMaterial, Option<KeyMaterial>), AeadError>> = OnceLock::new();

fn keys() -> Result<&'static (KeyMaterial, Option<KeyMaterial>), &'static AeadError> {
    KEYS.get_or_init(load_keys_from_env).as_ref()
}

fn load_keys_from_env() -> Result<(KeyMaterial, Option<KeyMaterial>), AeadError> {
    // option_env! is a compile-time macro: the values seen here are baked
    // into the binary at `cargo build` time from the build process's env.
    // The github-actions workflow exports CROSS_POSTING_DATA_KEY{,_PREVIOUS}
    // for the build step; local dev sources `app/ratel/env.sh` (gitignored
    // per-developer file). Cargo auto-rebuilds when these values change.
    let current_raw = option_env!("CROSS_POSTING_DATA_KEY")
        .filter(|v| !v.is_empty())
        .ok_or(AeadError::EnvvarMissing(ENV_CURRENT))?;
    let current = KeyMaterial::from_envvar_value(ENV_CURRENT, current_raw)?;

    // Empty values are treated as unset so PREVIOUS stays genuinely optional —
    // GitHub Actions evaluates `${{ secrets.X }}` to "" when X is not
    // registered, and the option_env! result for an unset env is None
    // (which we Option::filter back to None for the empty case too).
    let previous = match option_env!("CROSS_POSTING_DATA_KEY_PREVIOUS").filter(|v| !v.is_empty()) {
        Some(v) => Some(KeyMaterial::from_envvar_value(ENV_PREVIOUS, v)?),
        None => None,
    };

    if let Some(p) = previous.as_ref() {
        if p.version == current.version {
            return Err(AeadError::EnvvarMalformed {
                envvar: ENV_PREVIOUS,
                reason: "must differ from CURRENT version".into(),
            });
        }
    }

    Ok((current, previous))
}

/// Seal `plaintext` under the CURRENT envvar key. Result is the same
/// versioned blob produced by [`seal_with_key`].
pub fn seal(plaintext: &[u8]) -> Result<Vec<u8>, AeadError> {
    // Clone the small AeadError if init failed so callers see the same shape
    // every time (clone-on-error path; success path returns &'static).
    let (current, _) = keys().map_err(clone_err)?;
    seal_with_key(plaintext, current)
}

/// Open a sealed blob using the CURRENT envvar key, falling back to
/// PREVIOUS (when set and matching the blob's version byte).
pub fn open(blob: &[u8]) -> Result<Vec<u8>, AeadError> {
    let (current, previous) = keys().map_err(clone_err)?;
    open_with_keys(blob, current, previous.as_ref())
}

fn clone_err(e: &AeadError) -> AeadError {
    // AeadError is small and not Clone (thiserror-derived). Reconstruct
    // an equivalent error for the caller.
    match e {
        AeadError::EnvvarMissing(s) => AeadError::EnvvarMissing(s),
        AeadError::EnvvarMalformed { envvar, reason } => {
            AeadError::EnvvarMalformed { envvar, reason: reason.clone() }
        }
        AeadError::BlobTooShort(n) => AeadError::BlobTooShort(*n),
        AeadError::UnknownKeyVersion(v) => AeadError::UnknownKeyVersion(*v),
        AeadError::SealFailed => AeadError::SealFailed,
        AeadError::OpenFailed => AeadError::OpenFailed,
    }
}

// ─────────────────────────────────────────────────────────────────────────
// Tests — exclusively through the low-level API to avoid envvar mutation
// (which is process-global and breaks `cargo test` parallelism).
// ─────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_key(version: u8, seed: u8) -> KeyMaterial {
        // Deterministic 32-byte fill so tests don't depend on RNG.
        let mut raw = [0u8; KEY_LEN];
        for (i, b) in raw.iter_mut().enumerate() {
            *b = seed.wrapping_add(i as u8);
        }
        KeyMaterial { version, raw }
    }

    fn b64_no_pad(bytes: &[u8]) -> String {
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
    }

    // ── round-trip ──────────────────────────────────────────────────────

    #[test]
    fn seal_open_round_trip() {
        let key = make_key(1, 0x42);
        let plaintext = b"bluesky-app-password-xxxx-xxxx";
        let blob = seal_with_key(plaintext, &key).unwrap();
        let recovered = open_with_keys(&blob, &key, None).unwrap();
        assert_eq!(recovered, plaintext);
    }

    #[test]
    fn seal_produces_different_ciphertext_each_call_due_to_nonce() {
        let key = make_key(1, 0x42);
        let plaintext = b"same input";
        let a = seal_with_key(plaintext, &key).unwrap();
        let b = seal_with_key(plaintext, &key).unwrap();
        // Header + ciphertext differ entirely (different random nonce).
        assert_ne!(a, b);
        // Both still decrypt to the same plaintext.
        assert_eq!(open_with_keys(&a, &key, None).unwrap(), plaintext);
        assert_eq!(open_with_keys(&b, &key, None).unwrap(), plaintext);
    }

    #[test]
    fn blob_layout_is_version_then_nonce_then_ciphertext() {
        let key = make_key(7, 0x11);
        let blob = seal_with_key(b"x", &key).unwrap();
        assert_eq!(blob[0], 7, "version byte");
        assert_eq!(blob.len(), HEADER_LEN + 1 /* "x" */ + 16 /* GCM tag */);
    }

    // ── two-key fallback (rotation) ─────────────────────────────────────

    #[test]
    fn open_falls_back_to_previous_key_when_version_matches() {
        let prev = make_key(1, 0x11);
        let curr = make_key(2, 0x22);
        // Blob created under the now-PREVIOUS key.
        let blob = seal_with_key(b"old data", &prev).unwrap();
        // Open with current as primary, previous as fallback — should succeed.
        let recovered = open_with_keys(&blob, &curr, Some(&prev)).unwrap();
        assert_eq!(recovered, b"old data");
    }

    #[test]
    fn open_uses_current_when_version_matches_current() {
        let prev = make_key(1, 0x11);
        let curr = make_key(2, 0x22);
        let blob = seal_with_key(b"new data", &curr).unwrap();
        let recovered = open_with_keys(&blob, &curr, Some(&prev)).unwrap();
        assert_eq!(recovered, b"new data");
    }

    #[test]
    fn open_rejects_unknown_version() {
        let key = make_key(1, 0x11);
        let blob = seal_with_key(b"x", &make_key(99, 0x22)).unwrap();
        let err = open_with_keys(&blob, &key, None).unwrap_err();
        assert!(matches!(err, AeadError::UnknownKeyVersion(99)));
    }

    // ── tamper detection ────────────────────────────────────────────────

    #[test]
    fn open_rejects_tampered_ciphertext_byte() {
        let key = make_key(1, 0x42);
        let mut blob = seal_with_key(b"hello world", &key).unwrap();
        // Flip a bit in the ciphertext region.
        let last = blob.len() - 1;
        blob[last] ^= 0x01;
        let err = open_with_keys(&blob, &key, None).unwrap_err();
        assert!(matches!(err, AeadError::OpenFailed));
    }

    #[test]
    fn open_rejects_tampered_nonce() {
        let key = make_key(1, 0x42);
        let mut blob = seal_with_key(b"hello", &key).unwrap();
        blob[1] ^= 0x80;
        let err = open_with_keys(&blob, &key, None).unwrap_err();
        assert!(matches!(err, AeadError::OpenFailed));
    }

    #[test]
    fn open_rejects_blob_too_short() {
        let key = make_key(1, 0x42);
        let err = open_with_keys(&[1, 2, 3], &key, None).unwrap_err();
        assert!(matches!(err, AeadError::BlobTooShort(3)));
    }

    #[test]
    fn open_rejects_wrong_key_same_version() {
        // Same version byte but different key bytes — should fail with
        // OpenFailed (GCM tag mismatch), not UnknownKeyVersion.
        let key_a = make_key(1, 0x11);
        let key_b = make_key(1, 0xAA);
        let blob = seal_with_key(b"secret", &key_a).unwrap();
        let err = open_with_keys(&blob, &key_b, None).unwrap_err();
        assert!(matches!(err, AeadError::OpenFailed));
    }

    // ── envvar parsing ──────────────────────────────────────────────────

    #[test]
    fn envvar_parse_accepts_valid_form() {
        let value = format!("v3:{}", b64_no_pad(&[0u8; KEY_LEN]));
        let key = KeyMaterial::from_envvar_value(ENV_CURRENT, &value).unwrap();
        assert_eq!(key.version, 3);
        assert_eq!(key.raw, [0u8; KEY_LEN]);
    }

    #[test]
    fn envvar_parse_rejects_missing_colon() {
        let err = KeyMaterial::from_envvar_value(ENV_CURRENT, "v1nope").unwrap_err();
        assert!(matches!(err, AeadError::EnvvarMalformed { .. }));
    }

    #[test]
    fn envvar_parse_rejects_missing_v_prefix() {
        let value = format!("1:{}", b64_no_pad(&[0u8; KEY_LEN]));
        let err = KeyMaterial::from_envvar_value(ENV_CURRENT, &value).unwrap_err();
        assert!(matches!(err, AeadError::EnvvarMalformed { .. }));
    }

    #[test]
    fn envvar_parse_rejects_wrong_key_length() {
        let value = format!("v1:{}", b64_no_pad(&[0u8; 16])); // 16 ≠ 32
        let err = KeyMaterial::from_envvar_value(ENV_CURRENT, &value).unwrap_err();
        assert!(matches!(err, AeadError::EnvvarMalformed { .. }));
    }

    #[test]
    fn envvar_parse_rejects_non_base64_payload() {
        let err = KeyMaterial::from_envvar_value(ENV_CURRENT, "v1:!!!not-b64!!!").unwrap_err();
        assert!(matches!(err, AeadError::EnvvarMalformed { .. }));
    }
}
