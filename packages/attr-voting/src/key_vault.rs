use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use base64::Engine;
use pbkdf2::pbkdf2_hmac;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::error::AttrVotingError;

const VERSION: u8 = 1;
const KDF_NAME: &str = "PBKDF2-SHA256";
const CIPHER_NAME: &str = "AES-256-GCM";
const ITERATIONS: u32 = 310_000;
const KEY_LEN: usize = 32;
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptedSecretKeyBundle {
    pub version: u8,
    pub kdf: String,
    pub iterations: u32,
    pub cipher: String,
    pub salt: String,
    pub nonce: String,
    pub ciphertext: String,
}

pub fn wrap_secret_key(
    user_secret: &str,
    secret_key_json: &str,
) -> Result<String, AttrVotingError> {
    let mut salt = [0u8; SALT_LEN];
    let mut nonce = [0u8; NONCE_LEN];
    getrandom::getrandom(&mut salt)
        .map_err(|e| AttrVotingError::EncryptionFailed(e.to_string()))?;
    getrandom::getrandom(&mut nonce)
        .map_err(|e| AttrVotingError::EncryptionFailed(e.to_string()))?;
    wrap_secret_key_with_params(user_secret, secret_key_json, &salt, &nonce)
}

pub fn wrap_secret_key_with_params(
    user_secret: &str,
    secret_key_json: &str,
    salt: &[u8],
    nonce: &[u8],
) -> Result<String, AttrVotingError> {
    if salt.len() != SALT_LEN || nonce.len() != NONCE_LEN {
        return Err(AttrVotingError::EncryptionFailed(
            "invalid salt or nonce length".to_string(),
        ));
    }

    let key = derive_key(user_secret, salt);
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| AttrVotingError::EncryptionFailed(e.to_string()))?;
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(nonce), secret_key_json.as_bytes())
        .map_err(|e| AttrVotingError::EncryptionFailed(e.to_string()))?;

    let bundle = EncryptedSecretKeyBundle {
        version: VERSION,
        kdf: KDF_NAME.to_string(),
        iterations: ITERATIONS,
        cipher: CIPHER_NAME.to_string(),
        salt: b64(salt),
        nonce: b64(nonce),
        ciphertext: b64(&ciphertext),
    };

    serde_json::to_string(&bundle).map_err(AttrVotingError::SerializationError)
}

pub fn unwrap_secret_key(user_secret: &str, bundle_json: &str) -> Result<String, AttrVotingError> {
    let bundle: EncryptedSecretKeyBundle = serde_json::from_str(bundle_json)?;
    if bundle.version != VERSION
        || bundle.kdf != KDF_NAME
        || bundle.iterations != ITERATIONS
        || bundle.cipher != CIPHER_NAME
    {
        return Err(AttrVotingError::DecryptionFailed);
    }

    let salt = b64_decode(&bundle.salt)?;
    let nonce = b64_decode(&bundle.nonce)?;
    let ciphertext = b64_decode(&bundle.ciphertext)?;
    if salt.len() != SALT_LEN || nonce.len() != NONCE_LEN {
        return Err(AttrVotingError::DecryptionFailed);
    }

    let key = derive_key(user_secret, &salt);
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|_| AttrVotingError::DecryptionFailed)?;
    let plaintext = cipher
        .decrypt(Nonce::from_slice(&nonce), ciphertext.as_ref())
        .map_err(|_| AttrVotingError::DecryptionFailed)?;

    String::from_utf8(plaintext).map_err(|_| AttrVotingError::DecryptionFailed)
}

fn derive_key(user_secret: &str, salt: &[u8]) -> [u8; KEY_LEN] {
    let mut key = [0u8; KEY_LEN];
    pbkdf2_hmac::<Sha256>(user_secret.as_bytes(), salt, ITERATIONS, &mut key);
    key
}

fn b64(bytes: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

fn b64_decode(value: &str) -> Result<Vec<u8>, AttrVotingError> {
    base64::engine::general_purpose::STANDARD
        .decode(value)
        .map_err(|_| AttrVotingError::DecryptionFailed)
}
