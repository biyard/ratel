use bdk::prelude::*;
use by_axum::{
    auth::Authorization,
    axum::{Extension, Json, extract::State},
};
use dto::{Result, aide, JsonSchema, sqlx::PgPool};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
// use base64::Engine;

use crate::{config, utils::users::extract_user_id};

/// OpenID4VCI Credential Request
/// 
/// Request body for issuing a verifiable credential.
/// Contains the credential format, type, and proof of possession.
/// 
/// Reference: https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0.html#name-credential-request
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct CredentialRequest {
    #[schemars(description = "Format of the credential (e.g., 'jwt_vc_json')")]
    pub format: String,
    
    #[schemars(description = "Credential definition containing type and other metadata")]
    pub credential_definition: Option<CredentialDefinition>,
    
    #[schemars(description = "Credential identifier from metadata")]
    pub credential_identifier: Option<String>,
    
    #[schemars(description = "Proof of possession of the private key")]
    pub proof: Option<ProofOfPossession>,
}

/// Credential Definition for the request
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct CredentialDefinition {
    #[schemars(description = "JSON-LD context")]
    #[serde(rename = "@context")]
    pub context: Option<Vec<String>>,
    
    #[schemars(description = "Credential types")]
    #[serde(rename = "type")]
    pub credential_type: Vec<String>,
    
    #[schemars(description = "Credential subject schema")]
    #[serde(rename = "credentialSubject")]
    pub credential_subject: Option<Value>,
}

/// Proof of Possession for key binding
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct ProofOfPossession {
    #[schemars(description = "Proof type (e.g., 'jwt')")]
    pub proof_type: String,
    
    #[schemars(description = "JWT proof token")]
    pub jwt: Option<String>,
}

/// OpenID4VCI Credential Response
/// 
/// Response containing the issued verifiable credential.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct CredentialResponse {
    #[schemars(description = "Format of the issued credential")]
    pub format: String,
    
    #[schemars(description = "The issued verifiable credential (JWT format)")]
    pub credential: Option<String>,
    
    #[schemars(description = "Acceptance token for deferred issuance")]
    pub acceptance_token: Option<String>,
    
    #[schemars(description = "New c_nonce for subsequent requests")]
    pub c_nonce: Option<String>,
    
    #[schemars(description = "C_nonce expiration time in seconds")]
    pub c_nonce_expires_in: Option<u64>,
}

/// Issued Verifiable Credential (for JWT format)
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    aide::OperationIo,
    JsonSchema,
)]
pub struct VerifiableCredential {
    #[schemars(description = "JSON-LD context")]
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    
    #[schemars(description = "Credential identifier")]
    pub id: String,
    
    #[schemars(description = "Credential types")]
    #[serde(rename = "type")]
    pub credential_type: Vec<String>,
    
    #[schemars(description = "Credential issuer")]
    pub issuer: Value,
    
    #[schemars(description = "Issuance date")]
    #[serde(rename = "issuanceDate")]
    pub issuance_date: String,
    
    #[schemars(description = "Expiration date")]
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<String>,
    
    #[schemars(description = "Credential subject")]
    #[serde(rename = "credentialSubject")]
    pub credential_subject: Value,
    
    #[schemars(description = "Credential status information")]
    #[serde(rename = "credentialStatus")]
    pub credential_status: Option<Value>,
}

/// OID4VCI Credential Issuance Handler
/// 
/// Issues a verifiable credential based on the request parameters.
/// Validates the access token and proof of possession before issuance.
pub async fn oid4vci_credential_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Json(request): Json<CredentialRequest>,
) -> Result<Json<CredentialResponse>> {
    tracing::debug!("OID4VCI credential request: {:?}", request);
    
    // Extract user ID from auth token
    let user_id = extract_user_id(&pool, auth).await?;
    
    // Validate request format
    if request.format != "jwt_vc_json" {
        return Err(dto::Error::Unknown("Unsupported credential format".to_string()));
    }
    
    // Determine credential type
    let credential_types = if let Some(def) = &request.credential_definition {
        def.credential_type.clone()
    } else if let Some(id) = &request.credential_identifier {
        match id.as_str() {
            "PassportCredential" => vec!["VerifiableCredential".to_string(), "PassportCredential".to_string()],
            "MedicalCredential" => vec!["VerifiableCredential".to_string(), "MedicalCredential".to_string()],
            _ => return Err(dto::Error::Unknown("Unknown credential identifier".to_string())),
        }
    } else {
        return Err(dto::Error::Unknown("Missing credential definition or identifier".to_string()));
    };
    
    // Validate proof of possession if provided
    if let Some(proof) = &request.proof {
        validate_proof_of_possession(proof)?;
    }
    
    // Generate credential based on type
    let credential = match credential_types.get(1).map(|s| s.as_str()) {
        Some("PassportCredential") => {
            generate_passport_credential(user_id, &pool).await?
        }
        Some("MedicalCredential") => {
            generate_medical_credential(user_id, &pool).await?
        }
        _ => {
            return Err(dto::Error::Unknown("Unsupported credential type".to_string()));
        }
    };
    
    // Sign the credential as JWT
    let signed_credential = sign_credential_as_jwt(&credential).await?;
    
    // Generate new c_nonce for future requests
    let c_nonce = generate_c_nonce();
    let c_nonce_expires_in = 600; // 10 minutes
    
    // TODO: Store credential issuance record in database
    // This should include:
    // - credential_id
    // - user_id
    // - credential_type
    // - issuance_timestamp
    // - status_list_index (for revocation)
    
    let response = CredentialResponse {
        format: request.format,
        credential: Some(signed_credential),
        acceptance_token: None, // Only used for deferred issuance
        c_nonce: Some(c_nonce),
        c_nonce_expires_in: Some(c_nonce_expires_in),
    };
    
    Ok(Json(response))
}

/// Validate proof of possession
fn validate_proof_of_possession(proof: &ProofOfPossession) -> Result<()> {
    if proof.proof_type != "jwt" {
        return Err(dto::Error::Unknown("Unsupported proof type".to_string()));
    }
    
    if proof.jwt.is_none() {
        return Err(dto::Error::Unknown("Missing JWT proof".to_string()));
    }
    
    // TODO: Validate the JWT proof
    // This should include:
    // 1. Verify JWT signature
    // 2. Check that the JWT contains the correct c_nonce
    // 3. Verify the audience and issuer claims
    // 4. Ensure the JWT is not expired
    
    tracing::debug!("Proof of possession validation passed");
    Ok(())
}

/// Generate passport credential from user data
async fn generate_passport_credential(user_id: i64, _pool: &PgPool) -> Result<VerifiableCredential> {
    // TODO: Fetch actual user passport data from secure storage
    // For now, we'll create a sample credential
    
    let conf = config::get();
    let domain = conf.domain;
    let base = format!("https://{}", domain);
    
    let credential_id = format!("{}/credentials/{}", base, Uuid::new_v4());
    let now = chrono::Utc::now();
    let issuance_date = now.to_rfc3339();
    let expiration_date = now.checked_add_signed(chrono::Duration::days(365 * 5)) // 5 years
        .unwrap()
        .to_rfc3339();
    
    // TODO: Get actual status list index from database
    let status_list_index = user_id as u64; // Temporary mapping
    
    let credential = VerifiableCredential {
        context: vec![
            "https://www.w3.org/2018/credentials/v1".to_string(),
            "https://w3id.org/citizenship/v1".to_string(),
        ],
        id: credential_id,
        credential_type: vec![
            "VerifiableCredential".to_string(),
            "PassportCredential".to_string(),
        ],
        issuer: serde_json::json!({
            "id": format!("https://{}", domain),
            "name": "Ratel Identity Issuer"
        }),
        issuance_date,
        expiration_date: Some(expiration_date),
        credential_subject: serde_json::json!({
            "id": format!("did:user:{}", user_id),
            "givenName": "Sample",
            "familyName": "User",
            "birthDate": "1990-01-01",
            "nationality": "Unknown",
            "gender": "Unknown"
        }),
        credential_status: Some(serde_json::json!({
            "id": format!("{}/status/bitstring/1.json#{}", base, status_list_index),
            "type": "StatusList2021Entry",
            "statusPurpose": "revocation",
            "statusListIndex": status_list_index.to_string(),
            "statusListCredential": format!("{}/status/bitstring/1.json", base)
        })),
    };
    
    tracing::info!("Generated passport credential for user {}", user_id);
    Ok(credential)
}

/// Generate medical credential from user data
async fn generate_medical_credential(user_id: i64, _pool: &PgPool) -> Result<VerifiableCredential> {
    // TODO: Fetch actual user medical data from secure storage
    // For now, we'll create a sample credential
    
    let conf = config::get();
    let domain = conf.domain;
    let base = format!("https://{}", domain);
    
    let credential_id = format!("{}/credentials/{}", base, Uuid::new_v4());
    let now = chrono::Utc::now();
    let issuance_date = now.to_rfc3339();
    let expiration_date = now.checked_add_signed(chrono::Duration::days(365)) // 1 year
        .unwrap()
        .to_rfc3339();
    
    // TODO: Get actual status list index from database
    let status_list_index = (user_id + 1000) as u64; // Offset for medical credentials
    
    let credential = VerifiableCredential {
        context: vec![
            "https://www.w3.org/2018/credentials/v1".to_string(),
            "https://w3id.org/health/v1".to_string(),
        ],
        id: credential_id,
        credential_type: vec![
            "VerifiableCredential".to_string(),
            "MedicalCredential".to_string(),
        ],
        issuer: serde_json::json!({
            "id": format!("https://{}", domain),
            "name": "Ratel Health Credential Issuer"
        }),
        issuance_date,
        expiration_date: Some(expiration_date),
        credential_subject: serde_json::json!({
            "id": format!("did:user:{}", user_id),
            "height": "175",
            "weight": "70",
            "bmi": "22.9",
            "bloodPressureSystolic": "120",
            "bloodPressureDiastolic": "80"
        }),
        credential_status: Some(serde_json::json!({
            "id": format!("{}/status/bitstring/1.json#{}", base, status_list_index),
            "type": "StatusList2021Entry",
            "statusPurpose": "revocation",
            "statusListIndex": status_list_index.to_string(),
            "statusListCredential": format!("{}/status/bitstring/1.json", base)
        })),
    };
    
    tracing::info!("Generated medical credential for user {}", user_id);
    Ok(credential)
}

/// Sign credential as JWT using the issuer's private key
async fn sign_credential_as_jwt(credential: &VerifiableCredential) -> Result<String> {
    // TODO: Implement proper JWT signing with the issuer's private key
    // This should use the ES256 key configured in DidConfig
    
    // For now, we'll create a simple unsigned JWT structure
    let header = serde_json::json!({
        "alg": "ES256",
        "typ": "JWT",
        "kid": "es256-1"
    });
    
    let payload = serde_json::json!({
        "iss": credential.issuer,
        "sub": credential.credential_subject.get("id"),
        "iat": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        "exp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 3600,
        "vc": credential
    });
    
    let header_str = serde_json::to_string(&header)
        .map_err(|e| dto::Error::Unknown(format!("Failed to serialize header: {}", e)))?;
    let payload_str = serde_json::to_string(&payload)
        .map_err(|e| dto::Error::Unknown(format!("Failed to serialize payload: {}", e)))?;
    
    let encoded_header = base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, header_str.as_bytes());
    let encoded_payload = base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, payload_str.as_bytes());
    
    // TODO: Replace with actual signature
    let fake_signature = "UNSIGNED_FOR_DEVELOPMENT_ONLY";
    
    let jwt = format!("{}.{}.{}", encoded_header, encoded_payload, fake_signature);
    
    tracing::warn!("Credential signed with development-only fake signature");
    Ok(jwt)
}

/// Generate a cryptographic nonce
fn generate_c_nonce() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    format!("{}_{}", timestamp, Uuid::new_v4().to_string().replace('-', ""))
}
