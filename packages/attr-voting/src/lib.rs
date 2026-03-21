pub mod authority;
pub mod error;
pub mod policy;
pub mod types;
pub mod vote;

pub use authority::VotingAuthority;
pub use error::AttrVotingError;
pub use types::{UserAttributes, VotePayload};
pub use vote::{decrypt_vote, encrypt_vote, EncryptedVote};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authority_can_decrypt_vote() {
        let authority = VotingAuthority::setup();
        let auth_attrs = UserAttributes::authority();
        let auth_sk = authority.generate_user_key(&auth_attrs).unwrap();

        let payload = VotePayload {
            choice: "yes".to_string(),
            metadata: None,
        };
        let encrypted = encrypt_vote(&authority.public_key, "alice", &payload).unwrap();
        let decrypted = decrypt_vote(&auth_sk, &encrypted).unwrap();

        assert_eq!(decrypted.choice, "yes");
        assert!(decrypted.metadata.is_none());
    }

    #[test]
    fn test_voter_can_decrypt_own_vote() {
        let authority = VotingAuthority::setup();
        let voter_attrs = UserAttributes::voter("alice");
        let voter_sk = authority.generate_user_key(&voter_attrs).unwrap();

        let payload = VotePayload {
            choice: "no".to_string(),
            metadata: None,
        };
        let encrypted = encrypt_vote(&authority.public_key, "alice", &payload).unwrap();
        let decrypted = decrypt_vote(&voter_sk, &encrypted).unwrap();

        assert_eq!(decrypted.choice, "no");
    }

    #[test]
    fn test_other_voter_cannot_decrypt() {
        let authority = VotingAuthority::setup();
        let other_attrs = UserAttributes::voter("bob");
        let other_sk = authority.generate_user_key(&other_attrs).unwrap();

        let payload = VotePayload {
            choice: "yes".to_string(),
            metadata: None,
        };
        let encrypted = encrypt_vote(&authority.public_key, "alice", &payload).unwrap();
        let result = decrypt_vote(&other_sk, &encrypted);

        assert!(result.is_err());
    }

    #[test]
    fn test_authority_decrypts_multiple_voters() {
        let authority = VotingAuthority::setup();
        let auth_attrs = UserAttributes::authority();
        let auth_sk = authority.generate_user_key(&auth_attrs).unwrap();

        for voter_id in &["alice", "bob", "charlie"] {
            let payload = VotePayload {
                choice: format!("vote-from-{voter_id}"),
                metadata: None,
            };
            let encrypted = encrypt_vote(&authority.public_key, voter_id, &payload).unwrap();
            let decrypted = decrypt_vote(&auth_sk, &encrypted).unwrap();
            assert_eq!(decrypted.choice, format!("vote-from-{voter_id}"));
        }
    }

    #[test]
    fn test_encrypted_vote_serialization_roundtrip() {
        let authority = VotingAuthority::setup();
        let auth_attrs = UserAttributes::authority();
        let auth_sk = authority.generate_user_key(&auth_attrs).unwrap();

        let payload = VotePayload {
            choice: "yes".to_string(),
            metadata: None,
        };
        let encrypted = encrypt_vote(&authority.public_key, "alice", &payload).unwrap();

        let json = serde_json::to_string(&encrypted).unwrap();
        let deserialized: EncryptedVote = serde_json::from_str(&json).unwrap();
        let decrypted = decrypt_vote(&auth_sk, &deserialized).unwrap();

        assert_eq!(decrypted.choice, "yes");
    }

    #[test]
    fn test_vote_with_metadata() {
        let authority = VotingAuthority::setup();
        let auth_attrs = UserAttributes::authority();
        let auth_sk = authority.generate_user_key(&auth_attrs).unwrap();

        let metadata = serde_json::json!({
            "timestamp": 1234567890,
            "proposal_id": "prop-42"
        });
        let payload = VotePayload {
            choice: "abstain".to_string(),
            metadata: Some(metadata.clone()),
        };
        let encrypted = encrypt_vote(&authority.public_key, "alice", &payload).unwrap();
        let decrypted = decrypt_vote(&auth_sk, &encrypted).unwrap();

        assert_eq!(decrypted.choice, "abstain");
        assert_eq!(decrypted.metadata.unwrap(), metadata);
    }

    #[test]
    fn test_authority_json_roundtrip() {
        let authority = VotingAuthority::setup();
        let json = authority.to_json().unwrap();
        let restored = VotingAuthority::from_json(&json).unwrap();

        let auth_sk = restored
            .generate_user_key(&UserAttributes::authority())
            .unwrap();
        let payload = VotePayload {
            choice: "yes".to_string(),
            metadata: None,
        };
        let encrypted = encrypt_vote(&restored.public_key, "alice", &payload).unwrap();
        let decrypted = decrypt_vote(&auth_sk, &encrypted).unwrap();
        assert_eq!(decrypted.choice, "yes");
    }

    #[test]
    fn test_user_key_serialization_roundtrip() {
        let authority = VotingAuthority::setup();
        let voter_attrs = UserAttributes::voter("blind-tag-abc123");
        let voter_sk = authority.generate_user_key(&voter_attrs).unwrap();

        let json = VotingAuthority::serialize_key(&voter_sk).unwrap();
        let restored_sk = VotingAuthority::deserialize_key(&json).unwrap();

        let payload = VotePayload {
            choice: "yes".to_string(),
            metadata: None,
        };
        let encrypted =
            encrypt_vote(&authority.public_key, "blind-tag-abc123", &payload).unwrap();
        let decrypted = decrypt_vote(&restored_sk, &encrypted).unwrap();
        assert_eq!(decrypted.choice, "yes");
    }

    #[test]
    fn test_blind_attr_prevents_raw_id_linkage() {
        let authority = VotingAuthority::setup();

        // Encrypt with a blinded attribute (HMAC-derived tag)
        let blind_tag = "a7f3b2c1e4d5f6";
        let payload = VotePayload {
            choice: "yes".to_string(),
            metadata: None,
        };
        let encrypted = encrypt_vote(&authority.public_key, blind_tag, &payload).unwrap();

        // Key with the matching blind attr can decrypt
        let voter_attrs = UserAttributes::voter(blind_tag);
        let voter_sk = authority.generate_user_key(&voter_attrs).unwrap();
        let decrypted = decrypt_vote(&voter_sk, &encrypted).unwrap();
        assert_eq!(decrypted.choice, "yes");

        // Key with the raw user ID cannot decrypt
        let raw_attrs = UserAttributes::voter("alice");
        let raw_sk = authority.generate_user_key(&raw_attrs).unwrap();
        assert!(decrypt_vote(&raw_sk, &encrypted).is_err());
    }

    #[test]
    fn test_encrypted_vote_from_json_backward_compat() {
        let authority = VotingAuthority::setup();
        let auth_sk = authority
            .generate_user_key(&UserAttributes::authority())
            .unwrap();

        let payload = VotePayload {
            choice: "compat".to_string(),
            metadata: None,
        };
        let encrypted = encrypt_vote(&authority.public_key, "compat-tag", &payload).unwrap();

        let wrapped_json = serde_json::to_string(&encrypted).unwrap();
        let parsed_wrapped = vote::EncryptedVote::from_json(&wrapped_json).unwrap();
        let wrapped_decrypted = decrypt_vote(&auth_sk, &parsed_wrapped).unwrap();
        assert_eq!(wrapped_decrypted.choice, "compat");

        let legacy_json = serde_json::to_string(&encrypted.ciphertext).unwrap();
        let parsed_legacy = vote::EncryptedVote::from_json(&legacy_json).unwrap();
        let legacy_decrypted = decrypt_vote(&auth_sk, &parsed_legacy).unwrap();
        assert_eq!(legacy_decrypted.choice, "compat");
    }
}
