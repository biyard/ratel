use crate::common::types::{EntityType, Error, Partition};
use dioxus::fullstack::Lazy;

pub static VOTE_CRYPTO_SERVICE: Lazy<Option<VoteCryptoService>> = Lazy::new(|| async move {
    let voter_tag_secret = match std::env::var("VOTER_TAG_SECRET") {
        Ok(v) if !v.is_empty() => v,
        _ => {
            tracing::warn!("VOTER_TAG_SECRET not configured — encrypted voting disabled");
            return dioxus::Ok(None);
        }
    };
    let authority_json = match std::env::var("ATTR_VOTING_AUTHORITY_JSON") {
        Ok(v) if !v.is_empty() => v,
        _ => {
            tracing::warn!(
                "ATTR_VOTING_AUTHORITY_JSON not configured — encrypted voting disabled"
            );
            return dioxus::Ok(None);
        }
    };

    match VoteCryptoService::new(&voter_tag_secret, &authority_json) {
        Ok(svc) => dioxus::Ok(Some(svc)),
        Err(e) => {
            tracing::error!("VoteCrypto init failed: {} — encrypted voting disabled", e);
            dioxus::Ok(None)
        }
    }
});
use attr_voting::{
    authority::VotingAuthority,
    types::{UserAttributes, VotePayload},
    vote::encrypt_vote,
};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub struct VoteCryptoService {
    voter_tag_secret: String,
    authority: VotingAuthority,
}

impl VoteCryptoService {
    pub fn new(voter_tag_secret: &str, authority_json: &str) -> Result<Self, Error> {
        let authority = VotingAuthority::from_json(authority_json)
            .map_err(|e| Error::InternalServerError(format!("Authority parse error: {}", e)))?;
        Ok(Self {
            voter_tag_secret: voter_tag_secret.to_string(),
            authority,
        })
    }

    pub fn build_voter_tag(
        &self,
        action_sk: &EntityType,
        user_pk: &Partition,
    ) -> Result<String, Error> {
        let user_id = user_inner_id(user_pk);
        let message = format!("{}:{}", action_sk, user_id);

        let mut mac = HmacSha256::new_from_slice(self.voter_tag_secret.as_bytes())
            .map_err(|e| Error::InternalServerError(format!("HMAC init error: {}", e)))?;
        mac.update(message.as_bytes());
        let result = mac.finalize().into_bytes();

        use base64::Engine;
        Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(result))
    }

    pub fn encrypt(
        &self,
        action_sk: &EntityType,
        user_pk: &Partition,
        choice: &impl serde::Serialize,
        metadata: Option<&impl serde::Serialize>,
    ) -> Result<EncryptedVoteEnvelope, Error> {
        let voter_tag = self.build_voter_tag(action_sk, user_pk)?;

        let choice_str = serde_json::to_string(choice)
            .map_err(|e| Error::InternalServerError(format!("Choice serialize error: {}", e)))?;

        let metadata_value = metadata
            .map(|m| serde_json::to_value(m))
            .transpose()
            .map_err(|e| Error::InternalServerError(format!("Metadata serialize error: {}", e)))?;

        let payload = VotePayload {
            choice: choice_str,
            metadata: metadata_value,
        };

        let encrypted = encrypt_vote(&self.authority.public_key, &voter_tag, &payload)
            .map_err(|e| Error::InternalServerError(format!("ABE encrypt error: {}", e)))?;

        let ciphertext_json = serde_json::to_string(&encrypted).map_err(|e| {
            Error::InternalServerError(format!("Ciphertext serialize error: {}", e))
        })?;

        use sha2::Digest;
        let hash = sha2::Sha256::digest(ciphertext_json.as_bytes());
        let ciphertext_hash = hex::encode(hash);

        Ok(EncryptedVoteEnvelope {
            ciphertext_json,
            ciphertext_hash,
            voter_tag,
        })
    }

    pub fn decrypt(
        &self,
        action_sk: &EntityType,
        user_pk: &Partition,
        ciphertext_blob: &[u8],
    ) -> Result<DecryptedVote, Error> {
        let voter_tag = self.build_voter_tag(action_sk, user_pk)?;
        let voter_sk_json = self.generate_voter_sk(&voter_tag)?;

        let ciphertext_json = String::from_utf8(ciphertext_blob.to_vec())
            .map_err(|e| Error::InternalServerError(format!("Invalid ciphertext blob: {}", e)))?;

        use sha2::Digest;
        let computed_hash = hex::encode(sha2::Sha256::digest(ciphertext_blob));

        let ciphertext = attr_voting::vote::EncryptedVote::from_json(&ciphertext_json)
            .map_err(|e| Error::InternalServerError(format!("Ciphertext deserialize error: {}", e)))?;

        let sk = VotingAuthority::deserialize_key(&voter_sk_json)
            .map_err(|e| Error::InternalServerError(format!("SK deserialize error: {}", e)))?;

        let payload = attr_voting::vote::decrypt_vote(&sk, &ciphertext)
            .map_err(|e| Error::InternalServerError(format!("Decrypt error: {}", e)))?;

        Ok(DecryptedVote {
            voter_tag,
            ciphertext_hash: computed_hash,
            choice: payload.choice,
            metadata: payload.metadata,
        })
    }

    pub fn generate_voter_sk(&self, voter_tag: &str) -> Result<String, Error> {
        let attrs = UserAttributes::voter(voter_tag);
        let sk = self
            .authority
            .generate_user_key(&attrs)
            .map_err(|e| Error::InternalServerError(format!("Keygen error: {}", e)))?;

        VotingAuthority::serialize_key(&sk)
            .map_err(|e| Error::InternalServerError(format!("SK serialize error: {}", e)))
    }
}

pub struct EncryptedVoteEnvelope {
    pub ciphertext_json: String,
    pub ciphertext_hash: String,
    pub voter_tag: String,
}

pub struct DecryptedVote {
    pub voter_tag: String,
    pub ciphertext_hash: String,
    pub choice: String,
    pub metadata: Option<serde_json::Value>,
}

pub fn user_inner_id(user_pk: &Partition) -> String {
    match user_pk {
        Partition::User(id) => id.clone(),
        other => other.to_string(),
    }
}
