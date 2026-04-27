use rabe::schemes::bsw::{self, CpAbeCiphertext, CpAbePublicKey, CpAbeSecretKey};
use serde::{Deserialize, Serialize};

use crate::error::AttrVotingError;
use crate::policy::vote_policy;
use crate::types::VotePayload;

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedVote {
    pub ciphertext: CpAbeCiphertext,
}

pub fn encrypt_vote(
    pk: &CpAbePublicKey,
    voter_id: &str,
    payload: &VotePayload,
) -> Result<EncryptedVote, AttrVotingError> {
    let plaintext = serde_json::to_vec(payload)?;
    let (policy, language) = vote_policy(voter_id);
    let ciphertext = bsw::encrypt(pk, &policy, language, &plaintext)
        .map_err(|e| AttrVotingError::EncryptionFailed(e.to_string()))?;
    Ok(EncryptedVote { ciphertext })
}

impl EncryptedVote {
    /// Parse from JSON, accepting both `EncryptedVote` and raw `CpAbeCiphertext` formats.
    pub fn from_json(json: &str) -> Result<Self, AttrVotingError> {
        match serde_json::from_str::<EncryptedVote>(json) {
            Ok(v) => Ok(v),
            Err(_) => {
                let ciphertext: CpAbeCiphertext =
                    serde_json::from_str(json).map_err(AttrVotingError::SerializationError)?;
                Ok(EncryptedVote { ciphertext })
            }
        }
    }
}

pub fn decrypt_vote(
    sk: &CpAbeSecretKey,
    encrypted: &EncryptedVote,
) -> Result<VotePayload, AttrVotingError> {
    let plaintext =
        bsw::decrypt(sk, &encrypted.ciphertext).map_err(|_| AttrVotingError::DecryptionFailed)?;
    serde_json::from_slice(&plaintext).map_err(AttrVotingError::SerializationError)
}

pub fn decrypt_vote_json_with_key_json(
    secret_key_json: &str,
    encrypted_vote_json: &str,
) -> Result<VotePayload, AttrVotingError> {
    let sk: CpAbeSecretKey = serde_json::from_str(secret_key_json)?;
    let encrypted = EncryptedVote::from_json(encrypted_vote_json)?;
    decrypt_vote(&sk, &encrypted)
}

/// Encrypt a vote payload using a JSON-encoded ABE public key.
/// Returns the encrypted vote serialized as JSON, ready to ship to the server.
pub fn encrypt_vote_json_with_pk_json(
    public_key_json: &str,
    voter_id: &str,
    payload: &VotePayload,
) -> Result<String, AttrVotingError> {
    let pk: CpAbePublicKey = serde_json::from_str(public_key_json)?;
    let encrypted = encrypt_vote(&pk, voter_id, payload)?;
    serde_json::to_string(&encrypted).map_err(AttrVotingError::SerializationError)
}
