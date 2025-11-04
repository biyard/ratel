use crate::aide::OperationIo;
use bdk::prelude::*;

/// Verification method types supported in DID documents
/// Based on W3C DID Core specification
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
    OperationIo,
)]
#[serde(rename_all = "PascalCase")]
pub enum VerificationMethodType {
    /// Ed25519 signature verification key (2018 spec)
    Ed25519VerificationKey2018,

    /// Ed25519 signature verification key (2020 spec)
    Ed25519VerificationKey2020,

    /// ECDSA secp256k1 verification key (2019 spec)
    EcdsaSecp256k1VerificationKey2019,

    /// JSON Web Key 2020
    JsonWebKey2020,

    /// Multikey format (supports multiple algorithms)
    Multikey,

    /// P-256 elliptic curve key
    P256Key2021,

    /// X25519 key agreement key (2020 spec)
    X25519KeyAgreementKey2020,

    /// RSA verification key (2018 spec)
    RsaVerificationKey2018,

    /// Custom or other verification method types
    #[serde(untagged)]
    Other(String),
}

impl VerificationMethodType {
    /// Check if this verification method type supports signature verification
    pub fn supports_signing(&self) -> bool {
        matches!(
            self,
            VerificationMethodType::Ed25519VerificationKey2018
                | VerificationMethodType::Ed25519VerificationKey2020
                | VerificationMethodType::EcdsaSecp256k1VerificationKey2019
                | VerificationMethodType::JsonWebKey2020
                | VerificationMethodType::Multikey
                | VerificationMethodType::P256Key2021
                | VerificationMethodType::RsaVerificationKey2018
        )
    }

    /// Check if this verification method type supports key agreement
    pub fn supports_key_agreement(&self) -> bool {
        matches!(
            self,
            VerificationMethodType::X25519KeyAgreementKey2020
                | VerificationMethodType::JsonWebKey2020
                | VerificationMethodType::Multikey
        )
    }
}

/// A verification method in a DID document
/// Used for cryptographic operations like signing and encryption
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct VerificationMethod {
    /// The verification method ID (usually a DID URL with fragment)
    pub id: String,

    /// The type of verification method
    #[serde(rename = "type")]
    pub method_type: VerificationMethodType,

    /// The DID of the controller of this verification method
    pub controller: String,

    /// Public key in Multibase format (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "publicKeyMultibase")]
    pub public_key_multibase: Option<String>,

    /// Public key in JWK format (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "publicKeyJwk")]
    pub public_key_jwk: Option<serde_json::Value>,

    /// Public key in base58 format (optional, deprecated)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "publicKeyBase58")]
    pub public_key_base58: Option<String>,

    /// Public key in hex format (optional, deprecated)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "publicKeyHex")]
    pub public_key_hex: Option<String>,
}

impl VerificationMethod {
    /// Check if this verification method has a valid public key
    pub fn has_public_key(&self) -> bool {
        self.public_key_multibase.is_some()
            || self.public_key_jwk.is_some()
            || self.public_key_base58.is_some()
            || self.public_key_hex.is_some()
    }

    /// Validate the verification method structure
    pub fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("Verification method ID cannot be empty".to_string());
        }

        if self.controller.is_empty() {
            return Err("Controller cannot be empty".to_string());
        }

        if !self.has_public_key() {
            return Err("Verification method must have a public key".to_string());
        }

        Ok(())
    }
}

/// Verification relationship - defines how a verification method can be used
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum VerificationRelationship {
    /// Reference to a verification method by ID
    Reference(String),
    /// Embedded verification method
    Embedded(VerificationMethod),
}

impl VerificationRelationship {
    /// Get the ID of the verification method (whether referenced or embedded)
    pub fn id(&self) -> &str {
        match self {
            VerificationRelationship::Reference(id) => id,
            VerificationRelationship::Embedded(vm) => &vm.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verification_method_type_supports_signing() {
        assert!(VerificationMethodType::Ed25519VerificationKey2020.supports_signing());
        assert!(VerificationMethodType::EcdsaSecp256k1VerificationKey2019.supports_signing());
        assert!(!VerificationMethodType::X25519KeyAgreementKey2020.supports_signing());
    }

    #[test]
    fn test_verification_method_type_supports_key_agreement() {
        assert!(VerificationMethodType::X25519KeyAgreementKey2020.supports_key_agreement());
        assert!(!VerificationMethodType::Ed25519VerificationKey2020.supports_key_agreement());
    }

    #[test]
    fn test_verification_method_validation() {
        let vm = VerificationMethod {
            id: "did:web:example.com#key-1".to_string(),
            method_type: VerificationMethodType::Ed25519VerificationKey2020,
            controller: "did:web:example.com".to_string(),
            public_key_multibase: Some("z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string()),
            public_key_jwk: None,
            public_key_base58: None,
            public_key_hex: None,
        };

        assert!(vm.validate().is_ok());
        assert!(vm.has_public_key());
    }
}
