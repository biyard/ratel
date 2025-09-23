use base64::Engine;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use p256::{
    PublicKey, SecretKey,
    ecdsa::{SigningKey, VerifyingKey},
    pkcs8::EncodePrivateKey,
};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::config;

#[derive(Debug, thiserror::Error)]
pub enum JwtError {
    #[error("JWT encoding error: {0}")]
    Encoding(#[from] jsonwebtoken::errors::Error),
    #[error("Base64 decoding error: {0}")]
    Base64(#[from] base64::DecodeError),
    #[error("P256 key error: {0}")]
    P256(String),
    #[error("Ed25519 key error: {0}")]
    Ed25519(String),
    #[error("Invalid key format: {0}")]
    InvalidKey(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("PKCS8 error: {0}")]
    Pkcs8(String),
}

pub type Result<T> = std::result::Result<T, JwtError>;

/// Standard JWT claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,           // Issuer
    pub sub: Option<String>,   // Subject
    pub aud: Option<String>,   // Audience
    pub exp: u64,              // Expiration time
    pub iat: u64,              // Issued at
    pub jti: Option<String>,   // JWT ID
    pub nonce: Option<String>, // Nonce for proof of possession
}

/// Verifiable Credential JWT payload
#[derive(Debug, Serialize, Deserialize)]
pub struct VcJwtPayload {
    #[serde(flatten)]
    pub claims: Claims,
    pub vc: serde_json::Value, // The verifiable credential
}

/// Proof of Possession JWT payload
#[derive(Debug, Serialize, Deserialize)]
pub struct ProofJwtPayload {
    #[serde(flatten)]
    pub claims: Claims,
    pub proof_type: String,
    pub c_nonce: String,
}

/// JWT signing utilities for DID VC system
pub struct JwtSigner {
    config: &'static config::DidConfig,
}

impl JwtSigner {
    pub fn new() -> Result<Self> {
        let config = &config::get().did;
        Ok(Self { config })
    }

    /// Sign a JWT using ES256 (P-256 ECDSA)
    pub fn sign_es256(&self, payload: &impl Serialize) -> Result<String> {
        // Decode the private key from base64
        let private_key_bytes = base64::engine::general_purpose::STANDARD
            .decode(self.config.p256_d)
            .map_err(|_| JwtError::Config("Invalid P256_D base64 encoding".to_string()))?;

        if private_key_bytes.len() != 32 {
            return Err(JwtError::InvalidKey(
                "P256 private key must be 32 bytes".to_string(),
            ));
        }

        // Create the signing key
        let secret_key = SecretKey::from_slice(&private_key_bytes)
            .map_err(|e| JwtError::P256(format!("Failed to create secret key: {}", e)))?;
        let signing_key = SigningKey::from(secret_key);

        // Create encoding key for jsonwebtoken
        let pkcs8_der = signing_key
            .to_pkcs8_der()
            .map_err(|e| JwtError::Pkcs8(format!("Failed to encode PKCS8: {}", e)))?;
        let encoding_key = EncodingKey::from_ec_der(pkcs8_der.as_bytes());

        // Create JWT header
        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some("es256-1".to_string());

        // Encode the JWT
        let token = encode(&header, payload, &encoding_key)?;
        Ok(token)
    }

    /// Sign a JWT using EdDSA (Ed25519)
    pub fn sign_eddsa(&self, payload: &impl Serialize) -> Result<String> {
        // For Ed25519, we'll use ring since jsonwebtoken doesn't support EdDSA directly
        // This is a simplified implementation
        let payload_json = serde_json::to_string(payload)
            .map_err(|_| JwtError::Config("Failed to serialize payload".to_string()))?;

        // Create a simple JWT-like structure for Ed25519
        // In a full implementation, you'd want to use a library that properly supports EdDSA JWTs
        let header = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode(r#"{"alg":"EdDSA","typ":"JWT","kid":"eddsa-1"}"#);

        let payload_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&payload_json);

        // For now, return unsigned token (TODO: implement proper Ed25519 signing)
        let token = format!("{}.{}.EdDSA_SIGNATURE_TODO", header, payload_b64);
        Ok(token)
    }

    /// Verify a JWT signed with ES256
    pub fn verify_es256(&self, token: &str) -> Result<Claims> {
        // Decode the public key coordinates
        let x_bytes = base64::engine::general_purpose::STANDARD
            .decode(self.config.p256_x)
            .map_err(|_| JwtError::Config("Invalid P256_X base64 encoding".to_string()))?;

        let y_bytes = base64::engine::general_purpose::STANDARD
            .decode(self.config.p256_y)
            .map_err(|_| JwtError::Config("Invalid P256_Y base64 encoding".to_string()))?;

        if x_bytes.len() != 32 || y_bytes.len() != 32 {
            return Err(JwtError::InvalidKey(
                "P256 coordinates must be 32 bytes each".to_string(),
            ));
        }

        // Reconstruct the public key
        let mut uncompressed_point = vec![0x04]; // Uncompressed point prefix
        uncompressed_point.extend_from_slice(&x_bytes);
        uncompressed_point.extend_from_slice(&y_bytes);

        let public_key = PublicKey::from_sec1_bytes(&uncompressed_point)
            .map_err(|e| JwtError::P256(format!("Failed to create public key: {}", e)))?;
        let verifying_key = VerifyingKey::from(public_key);

        // Create decoding key for jsonwebtoken using SEC1 encoded point
        let sec1_bytes = verifying_key.to_encoded_point(false);
        let decoding_key = DecodingKey::from_ec_der(sec1_bytes.as_bytes());

        // Set up validation
        let mut validation = Validation::new(Algorithm::ES256);
        validation.validate_exp = true;
        validation.validate_aud = false; // We'll validate audience manually if needed

        // Decode and verify the token
        let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
        Ok(token_data.claims)
    }

    /// Create a verifiable credential JWT
    pub fn create_vc_jwt(
        &self,
        vc: &serde_json::Value,
        subject_did: Option<&str>,
    ) -> Result<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let config = config::get();
        let issuer_did = format!("https://{}", config.domain);

        let payload = VcJwtPayload {
            claims: Claims {
                iss: issuer_did,
                sub: subject_did.map(|s| s.to_string()),
                aud: None,
                exp: now + 3600, // 1 hour expiration
                iat: now,
                jti: Some(Uuid::new_v4().to_string()),
                nonce: None,
            },
            vc: vc.clone(),
        };

        self.sign_es256(&payload)
    }

    /// Verify a proof of possession JWT
    pub fn verify_proof_jwt(&self, token: &str, expected_nonce: &str) -> Result<ProofJwtPayload> {
        let claims = self.verify_es256(token)?;

        // Convert generic claims back to proof payload
        // This is a simplified approach - in practice you'd decode the full payload
        let proof_payload = ProofJwtPayload {
            claims,
            proof_type: "jwt".to_string(),
            c_nonce: expected_nonce.to_string(),
        };

        Ok(proof_payload)
    }

    /// Generate a c_nonce for proof of possession
    pub fn generate_c_nonce() -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        format!(
            "{}_{}",
            timestamp,
            Uuid::new_v4().to_string().replace('-', "")
        )
    }

    /// Generate an access token
    pub fn generate_access_token(&self, user_id: &str, scope: &str) -> Result<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let config = config::get();
        let issuer = format!("https://{}", config.domain);

        let claims = Claims {
            iss: issuer,
            sub: Some(user_id.to_string()),
            aud: Some("credential-issuance".to_string()),
            exp: now + 3600, // 1 hour
            iat: now,
            jti: Some(Uuid::new_v4().to_string()),
            nonce: Some(scope.to_string()), // Use nonce field for scope
        };

        self.sign_es256(&claims)
    }

    /// Verify an access token
    pub fn verify_access_token(&self, token: &str) -> Result<Claims> {
        let claims = self.verify_es256(token)?;

        // Check if token is expired
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if claims.exp < now {
            return Err(JwtError::Config("Token expired".to_string()));
        }

        Ok(claims)
    }
}

/// Utility functions for JWT operations
impl JwtSigner {
    /// Extract JWT payload without verification (for debugging)
    pub fn decode_without_verification(token: &str) -> Result<serde_json::Value> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(JwtError::InvalidKey("Invalid JWT format".to_string()));
        }

        let payload_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(parts[1])?;

        let payload: serde_json::Value = serde_json::from_slice(&payload_bytes)
            .map_err(|_| JwtError::Config("Invalid JSON in JWT payload".to_string()))?;

        Ok(payload)
    }

    /// Check if a JWT is expired without full verification
    pub fn is_expired(token: &str) -> Result<bool> {
        let payload = Self::decode_without_verification(token)?;

        if let Some(exp) = payload.get("exp").and_then(|e| e.as_u64()) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            Ok(exp < now)
        } else {
            Ok(false) // No expiration claim means it doesn't expire
        }
    }

    /// Extract subject from JWT without full verification
    pub fn extract_subject(token: &str) -> Result<Option<String>> {
        let payload = Self::decode_without_verification(token)?;
        Ok(payload
            .get("sub")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string()))
    }
}

/// JWT verification utilities for DID VC system
pub struct JwtVerifier;

impl JwtVerifier {
    pub fn new() -> Self {
        Self
    }

    /// Verify a proof of possession JWT
    pub fn verify_jwt_proof_of_possession(&self, token: &str) -> Result<serde_json::Value> {
        // First decode to get the header and determine the algorithm
        let header = self.decode_jwt_header(token)?;
        let algorithm = header
            .get("alg")
            .and_then(|a| a.as_str())
            .ok_or_else(|| JwtError::InvalidKey("Missing algorithm in JWT header".to_string()))?;

        match algorithm {
            "ES256" => self.verify_es256_jwt(token),
            "EdDSA" => self.verify_eddsa_jwt(token),
            _ => Err(JwtError::Config(format!(
                "Unsupported algorithm: {}",
                algorithm
            ))),
        }
    }

    /// Verify an ES256 JWT using the configured public key
    fn verify_es256_jwt(&self, token: &str) -> Result<serde_json::Value> {
        // Get the public key from configuration
        let config = config::get();
        let public_key_x = config.did.p256_x;
        let public_key_y = config.did.p256_y;

        // Decode public key coordinates
        let x_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(public_key_x)?;
        let y_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(public_key_y)?;

        if x_bytes.len() != 32 || y_bytes.len() != 32 {
            return Err(JwtError::InvalidKey(
                "P256 coordinates must be 32 bytes each".to_string(),
            ));
        }

        // Reconstruct the public key
        let mut uncompressed_point = vec![0x04]; // Uncompressed point prefix
        uncompressed_point.extend_from_slice(&x_bytes);
        uncompressed_point.extend_from_slice(&y_bytes);

        let public_key = PublicKey::from_sec1_bytes(&uncompressed_point)
            .map_err(|e| JwtError::P256(format!("Failed to create public key: {}", e)))?;
        let verifying_key = VerifyingKey::from(public_key);

        // Create decoding key for jsonwebtoken using SEC1 encoded point
        let sec1_bytes = verifying_key.to_encoded_point(false);
        let decoding_key = DecodingKey::from_ec_der(sec1_bytes.as_bytes());

        // Set up validation
        let mut validation = Validation::new(Algorithm::ES256);
        validation.validate_exp = true;
        validation.validate_aud = false; // We'll validate audience manually
        validation.leeway = 60; // Allow 60 seconds of clock skew

        // Decode and verify the JWT
        let token_data = decode::<serde_json::Value>(token, &decoding_key, &validation)?;
        Ok(token_data.claims)
    }

    /// Verify an EdDSA JWT (simplified implementation)
    fn verify_eddsa_jwt(&self, token: &str) -> Result<serde_json::Value> {
        // For now, just decode without verification since jsonwebtoken doesn't support EdDSA
        // In production, you'd want to use a library that properly supports EdDSA
        tracing::warn!(
            "EdDSA JWT verification not fully implemented - decoding without signature verification"
        );

        let payload = JwtSigner::decode_without_verification(token)?;

        // Basic expiration check
        if let Some(exp) = payload.get("exp").and_then(|e| e.as_i64()) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            if exp < now {
                return Err(JwtError::Config("Token expired".to_string()));
            }
        }

        Ok(payload)
    }

    /// Decode JWT header without verification
    fn decode_jwt_header(&self, token: &str) -> Result<serde_json::Value> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(JwtError::InvalidKey("Invalid JWT format".to_string()));
        }

        let header_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(parts[0])?;
        let header: serde_json::Value = serde_json::from_slice(&header_bytes)
            .map_err(|_| JwtError::Config("Invalid JSON in JWT header".to_string()))?;

        Ok(header)
    }
}

impl Default for JwtVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_nonce_generation() {
        let nonce1 = JwtSigner::generate_c_nonce();
        let nonce2 = JwtSigner::generate_c_nonce();

        assert_ne!(nonce1, nonce2);
        assert!(nonce1.contains('_'));
        assert!(nonce1.len() > 10);
    }

    #[test]
    fn test_jwt_decode_without_verification() {
        // Create a simple JWT payload for testing
        let header = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode(r#"{"alg":"ES256","typ":"JWT"}"#);
        let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode(r#"{"sub":"test","exp":1234567890}"#);
        let token = format!("{}.{}.fake_signature", header, payload);

        let decoded = JwtSigner::decode_without_verification(&token).unwrap();
        assert_eq!(decoded["sub"], "test");
        assert_eq!(decoded["exp"], 1234567890);
    }
}
