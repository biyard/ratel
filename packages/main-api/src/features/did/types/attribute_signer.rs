use crate::*;
use bdk::prelude::*;
use chrono::Utc;
use ssi::{bbs::BBSplusSecretKey, dids::DIDBuf};

use super::signed_attribute::SignedAttribute;

/// Attribute signer using BBS+ signatures for selective disclosure
pub struct AttributeSigner {
    issuer_did: DIDBuf,
    subject_did: DIDBuf,
    #[allow(dead_code)] // TODO: Will be used when implementing actual BBS+ signing
    bbs_key: &'static BBSplusSecretKey,
}

impl AttributeSigner {
    /// Create a new AttributeSigner with BBS+ key from config
    pub fn new(
        issuer_did: DIDBuf,
        subject_did: DIDBuf,
        bbs_key: &'static BBSplusSecretKey,
    ) -> Self {
        Self {
            issuer_did,
            subject_did,
            bbs_key,
        }
    }

    /// Sign attributes using BBS+ signatures
    /// Returns an AttributeIssuanceResponse containing signed attributes
    pub fn sign_attributes(
        &self,
        attributes: Vec<(&str, serde_json::Value)>,
        expires_in_days: Option<i64>,
    ) -> Result<AttributeIssuanceResponseV2> {
        let now = Utc::now();
        let signed_at = now.to_rfc3339();
        let expires_at =
            expires_in_days.map(|days| (now + chrono::Duration::days(days)).to_rfc3339());
        let verification_method = format!("{}#bbs-key-1", self.issuer_did);

        // Convert attributes to SignedAttribute format
        let signed_attributes: Vec<SignedAttribute> = attributes
            .into_iter()
            .map(|(key, value)| {
                // TODO: Actually sign with BBS+ key
                // For now, we're creating a placeholder signature structure
                // In production, this should use BBS+ signatures for selective disclosure
                SignedAttribute {
                    key: key.to_string(),
                    value,
                    signature: format!("bbs+placeholder:{}", key),
                    verification_method: verification_method.clone(),
                    signed_at: signed_at.clone(),
                    expires_at: expires_at.clone(),
                }
            })
            .collect();

        Ok(AttributeIssuanceResponseV2 {
            issuer_did: self.issuer_did.to_string(),
            subject_did: self.subject_did.to_string(),
            signed_attributes,
            issued_at: signed_at,
            expires_at,
            verification_method_url: verification_method,
        })
    }

    /// Verify signed attributes
    /// Returns the verified attributes if verification succeeds
    pub fn verify_attributes(
        &self,
        response: &AttributeIssuanceResponseV2,
    ) -> Result<Vec<(String, serde_json::Value)>> {
        // Check expiration
        if let Some(ref expires_at_str) = response.expires_at {
            let expires_at = chrono::DateTime::parse_from_rfc3339(expires_at_str)
                .map_err(|e| Error::BadRequest(format!("Invalid expiration date: {}", e)))?;
            let now = Utc::now();
            if now > expires_at {
                return Err(Error::BadRequest("Attributes have expired".into()));
            }
        }

        // TODO: Implement actual BBS+ signature verification
        // For now, we just return the attributes
        Ok(response
            .signed_attributes
            .iter()
            .map(|attr| (attr.key.clone(), attr.value.clone()))
            .collect())
    }
}

/// Response format for attribute issuance with BBS+ signatures
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct AttributeIssuanceResponseV2 {
    /// Issuer DID
    pub issuer_did: String,

    /// Subject DID
    pub subject_did: String,

    /// Signed attributes with BBS+ signatures
    pub signed_attributes: Vec<SignedAttribute>,

    /// Timestamp when the attributes were issued (RFC3339 format)
    pub issued_at: String,

    /// Optional expiration timestamp (RFC3339 format)
    pub expires_at: Option<String>,

    /// Verification method URL for signature verification
    pub verification_method_url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify_attributes() {
        use ssi::bbs::generate_secret_key;

        // Generate a test BBS+ key
        let mut rng = ssi::crypto::rand::rngs::OsRng {};
        let key = generate_secret_key(&mut rng);
        let key: &'static BBSplusSecretKey = Box::leak(Box::new(key));

        let issuer_did = DIDBuf::from_string("did:web:example.com".to_string()).unwrap();
        let subject_did = DIDBuf::from_string("did:web:example.com:alice".to_string()).unwrap();

        let signer = AttributeSigner::new(issuer_did, subject_did, key);

        // Sign attributes
        let attributes = vec![
            ("age", serde_json::json!(30)),
            ("gender", serde_json::json!("Female")),
        ];

        let response = signer.sign_attributes(attributes, Some(365)).unwrap();

        // Verify the response structure
        assert_eq!(response.issuer_did, "did:web:example.com");
        assert_eq!(response.subject_did, "did:web:example.com:alice");
        assert_eq!(response.signed_attributes.len(), 2);
        assert!(response.expires_at.is_some());

        // Verify attributes
        let verified = signer.verify_attributes(&response).unwrap();
        assert_eq!(verified.len(), 2);
        assert_eq!(verified[0].0, "age");
        assert_eq!(verified[0].1, serde_json::json!(30));
        assert_eq!(verified[1].0, "gender");
        assert_eq!(verified[1].1, serde_json::json!("Female"));
    }

    #[test]
    fn test_expired_attributes() {
        use ssi::bbs::generate_secret_key;

        let mut rng = ssi::crypto::rand::rngs::OsRng {};
        let key = generate_secret_key(&mut rng);
        let key: &'static BBSplusSecretKey = Box::leak(Box::new(key));

        let issuer_did = DIDBuf::from_string("did:web:example.com".to_string()).unwrap();
        let subject_did = DIDBuf::from_string("did:web:example.com:bob".to_string()).unwrap();

        let signer = AttributeSigner::new(issuer_did, subject_did, key);

        // Create expired attributes
        let mut response = signer
            .sign_attributes(vec![("age", serde_json::json!(25))], Some(0))
            .unwrap();

        // Set expiration to the past
        let past = Utc::now() - chrono::Duration::days(1);
        response.expires_at = Some(past.to_rfc3339());

        // Verification should fail
        let result = signer.verify_attributes(&response);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expired"));
    }
}
