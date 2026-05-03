//! Seal/open the per-platform credential blob stored on
//! `SocialConnection.credential_ciphertext`.
//!
//! The blob is JSON-serialized [`DecryptedCredentials`] sealed with the
//! AEAD utility (`crate::common::utils::aead`). JSON keeps the payload
//! self-describing across platforms (Bluesky / LinkedIn / Threads share
//! a single ciphertext column) and avoids per-platform serialization
//! code paths. The version byte at offset 0 of the AEAD blob handles
//! key-rotation transitions transparently.

use super::adapters::DecryptedCredentials;
use crate::common::utils::aead::{self, AeadError};

#[derive(Debug, thiserror::Error)]
pub enum CredentialError {
    #[error("AEAD seal failed: {0}")]
    Seal(AeadError),
    #[error("AEAD open failed: {0}")]
    Open(AeadError),
    #[error("credential JSON serialize failed: {0}")]
    Serialize(serde_json::Error),
    #[error("credential JSON deserialize failed: {0}")]
    Deserialize(serde_json::Error),
}

/// JSON-serialize and AEAD-seal credentials for at-rest storage on
/// `SocialConnection.credential_ciphertext`.
pub fn seal_credentials(creds: &DecryptedCredentials) -> Result<Vec<u8>, CredentialError> {
    let json = serde_json::to_vec(creds).map_err(CredentialError::Serialize)?;
    aead::seal(&json).map_err(CredentialError::Seal)
}

/// AEAD-open and JSON-deserialize a stored credential blob. Used by the
/// dispatcher just before calling the platform adapter.
pub fn open_credentials(blob: &[u8]) -> Result<DecryptedCredentials, CredentialError> {
    let json = aead::open(blob).map_err(CredentialError::Open)?;
    serde_json::from_slice(&json).map_err(CredentialError::Deserialize)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::utils::aead::{KeyMaterial, open_with_keys, seal_with_key};

    fn make_key() -> KeyMaterial {
        let mut raw = [0u8; 32];
        for (i, b) in raw.iter_mut().enumerate() {
            *b = i as u8;
        }
        KeyMaterial { version: 1, raw }
    }

    /// Round-trip via the low-level AEAD API (avoids envvar dependency in
    /// tests). The high-level seal_credentials/open_credentials path is
    /// the same modulo the envvar key lookup.
    #[test]
    fn json_roundtrip_via_low_level_aead() {
        let key = make_key();
        let creds = DecryptedCredentials::Bluesky {
            did: "did:plc:test".into(),
            handle: "user.bsky.social".into(),
            access_jwt: "access.jwt.token".into(),
            refresh_jwt: "refresh.jwt.token".into(),
        };

        let json = serde_json::to_vec(&creds).unwrap();
        let blob = seal_with_key(&json, &key).unwrap();
        let opened_json = open_with_keys(&blob, &key, None).unwrap();
        let opened: DecryptedCredentials = serde_json::from_slice(&opened_json).unwrap();

        match opened {
            DecryptedCredentials::Bluesky { did, handle, access_jwt, refresh_jwt } => {
                assert_eq!(did, "did:plc:test");
                assert_eq!(handle, "user.bsky.social");
                assert_eq!(access_jwt, "access.jwt.token");
                assert_eq!(refresh_jwt, "refresh.jwt.token");
            }
            _ => panic!("variant should round-trip as Bluesky"),
        }
    }
}
