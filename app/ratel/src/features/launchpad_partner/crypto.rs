//! Crypto for the Launchpad handoff, byte-compatible with Launchpad's
//! `demo_preview/server.rs` and `demo/brand-demo/server.js`.
//!
//! - `encrypt_user_token`: AES-256-GCM(key = SHA256(secret)), output
//!   `base64url(nonce[12] ‖ ciphertext ‖ tag)`. Launchpad decrypts it.
//! - `verify_signature`: HMAC-SHA256(secret, "{timestamp}.{raw_body}")
//!   compared against the hex `x-launchpad-signature` header.

#![cfg(feature = "server")]

use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

/// Encrypt a company user key (ratel user uuid) into the `lp_user` token.
pub fn encrypt_user_token(secret: &str, user_id: &str) -> Result<String, String> {
    let key = Sha256::digest(secret.as_bytes());
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| e.to_string())?;
    // 12-byte nonce. Production would use a CSPRNG; for the demo a uuid v4
    // tail is sufficient entropy and avoids a getrandom feature gate.
    let uuid = uuid::Uuid::new_v4();
    let nonce_bytes = &uuid.as_bytes()[..12];
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(nonce_bytes), user_id.as_bytes())
        .map_err(|e| e.to_string())?;
    let mut blob = Vec::with_capacity(12 + ciphertext.len());
    blob.extend_from_slice(nonce_bytes);
    blob.extend_from_slice(&ciphertext);
    Ok(URL_SAFE_NO_PAD.encode(blob))
}

/// Verify a Launchpad callback signature over `"{timestamp}.{raw_body}"`.
pub fn verify_signature(secret: &str, timestamp: &str, signature_hex: &str, raw_body: &str) -> bool {
    if timestamp.is_empty() || signature_hex.is_empty() {
        return false;
    }
    // `new_from_slice` exists on both `aes_gcm::KeyInit` and `hmac::Mac`
    // (both in scope), so disambiguate explicitly.
    let Ok(mut mac) = <Hmac<Sha256> as Mac>::new_from_slice(secret.as_bytes()) else {
        return false;
    };
    mac.update(timestamp.as_bytes());
    mac.update(b".");
    mac.update(raw_body.as_bytes());
    let Ok(sig) = hex::decode(signature_hex) else {
        return false;
    };
    mac.verify_slice(&sig).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sign(secret: &str, timestamp: &str, body: &str) -> String {
        let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(timestamp.as_bytes());
        mac.update(b".");
        mac.update(body.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }

    // Replicates Launchpad's decrypt to prove the token round-trips.
    fn decrypt(secret: &str, token: &str) -> String {
        let blob = URL_SAFE_NO_PAD.decode(token).unwrap();
        let (nonce, ct) = blob.split_at(12);
        let key = Sha256::digest(secret.as_bytes());
        let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
        let pt = cipher.decrypt(Nonce::from_slice(nonce), ct).unwrap();
        String::from_utf8(pt).unwrap()
    }

    #[test]
    fn token_round_trips_for_launchpad() {
        let secret = "lps_test_secret";
        let token = encrypt_user_token(secret, "user-abc-123").unwrap();
        assert_eq!(decrypt(secret, &token), "user-abc-123");
    }

    #[test]
    fn valid_signature_verifies() {
        let secret = "lps_test_secret";
        let ts = "1717459200000";
        let body = r#"{"project_id":"p","company_user_key":"u"}"#;
        let sig = sign(secret, ts, body);
        assert!(verify_signature(secret, ts, &sig, body));
    }

    #[test]
    fn tampered_body_fails() {
        let secret = "lps_test_secret";
        let ts = "1717459200000";
        let sig = sign(secret, ts, "original");
        assert!(!verify_signature(secret, ts, &sig, "tampered"));
    }

    #[test]
    fn missing_parts_fail() {
        assert!(!verify_signature("s", "", "ab", "body"));
        assert!(!verify_signature("s", "ts", "", "body"));
        assert!(!verify_signature("s", "ts", "nothex!!", "body"));
    }
}
