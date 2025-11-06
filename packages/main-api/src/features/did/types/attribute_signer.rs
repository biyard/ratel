use super::*;
use base64::Engine;
use chrono::Utc;
use ssi::crypto::p256::ecdsa::{Signature, SigningKey, signature::Signer};

pub struct AttributeSigner {
    issuer_did: String,
    issuer_domain: String,
    signing_key: SigningKey,
    verification_method_id: String,
}

impl AttributeSigner {
    /// Create a new AttributeSigner with a deterministic key based on secret
    /// In production, this should use a securely stored key from KMS or similar
    /// TODO: Replace with proper key management (e.g., AWS KMS, HSM)
    pub fn new(issuer_domain: String, username: String) -> Self {
        let issuer_did = format!("did:web:{}:{}", issuer_domain, username);

        // Generate a deterministic seed from configuration
        // In production, this should be replaced with a proper key from secure storage
        let seed = [0u8; 32]; // TODO: Replace with actual secret from config/environment
        let signing_key = SigningKey::from_bytes(&seed.into()).expect("Failed to create signing key");

        let verification_method_id = format!("{}#key-1", issuer_did);

        Self {
            issuer_did,
            issuer_domain,
            signing_key,
            verification_method_id,
        }
    }

    /// Create AttributeSigner from an existing signing key
    pub fn from_key(
        issuer_domain: String,
        username: String,
        signing_key: SigningKey,
    ) -> Self {
        let issuer_did = format!("did:web:{}:{}", issuer_domain, username);
        let verification_method_id = format!("{}#key-1", issuer_did);

        Self {
            issuer_did,
            issuer_domain,
            signing_key,
            verification_method_id,
        }
    }

    /// Create a signing key from bytes
    pub fn signing_key_from_bytes(bytes: &[u8; 32]) -> Result<SigningKey, Box<dyn std::error::Error>> {
        Ok(SigningKey::from_bytes(bytes.into())?)
    }

    pub fn sign_attribute(
        &self,
        key: &str,
        value: serde_json::Value,
        subject_did: &str,
        expires_in_days: Option<i64>,
    ) -> Result<SignedAttribute, Box<dyn std::error::Error>> {
        let now = Utc::now();

        let claim_data = serde_json::json!({
            "issuer": self.issuer_did,
            "subject": subject_did,
            "key": key,
            "value": value,
            "issuedAt": now.to_rfc3339(),
        });

        let claim_bytes = serde_json::to_vec(&claim_data)?;

        let signature: Signature = self.signing_key.sign(&claim_bytes);
        let signature_b64 = base64::engine::general_purpose::STANDARD.encode(signature.to_bytes());

        Ok(SignedAttribute {
            key: key.to_string(),
            value,
            signature: signature_b64,
            verification_method: self.verification_method_id.clone(),
            signed_at: now.to_rfc3339(),
            expires_at: expires_in_days
                .map(|days| (now + chrono::Duration::days(days)).to_rfc3339()),
        })
    }

    pub fn sign_attributes(
        &self,
        attributes: Vec<(&str, serde_json::Value)>,
        subject_did: &str,
        expires_in_days: Option<i64>,
    ) -> Result<AttributeIssuanceResponse, Box<dyn std::error::Error>> {
        let mut signed_attributes = Vec::new();

        for (key, value) in attributes {
            let signed = self.sign_attribute(key, value, subject_did, expires_in_days)?;
            signed_attributes.push(signed);
        }

        Ok(AttributeIssuanceResponse {
            signed_attributes,
            issuer_did: self.issuer_did.clone(),
            issuer_did_document_url: format!("https://{}/.well-known/did.json", self.issuer_domain),
            credential_schema: None,
        })
    }
}
