use thiserror::Error;

#[derive(Debug, Error)]
pub enum AttrVotingError {
    #[error("encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("decryption failed")]
    DecryptionFailed,
    #[error("keygen failed: {0}")]
    KeygenFailed(String),
    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}
