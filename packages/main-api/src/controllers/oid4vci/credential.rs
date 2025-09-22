use aws_sdk_dynamodb::Client as DynamoClient;
use bdk::prelude::*;
use by_axum::{
    auth::Authorization,
    axum::{Extension, Json, extract::State},
};
use dto::{JsonSchema, Result, aide};
use serde::{Deserialize, Serialize};
use serde_dynamo::to_item;
use serde_json::Value;
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;
// use base64::Engine;

use crate::{
    config,
    models::dynamo_tables::main::vc::issued_credential::IssuedCredential,
    types::CredentialType,
    utils::{
        jwt::{Claims, JwtSigner, JwtVerifier, VcJwtPayload},
        users_dynamo::extract_user_id_dynamo,
    },
};

/// OpenID4VCI Credential Request
///
/// Request body for issuing a verifiable credential.
/// Contains the credential format, type, and proof of possession.
///
/// Reference: https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0.html#name-credential-request
#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Default, aide::OperationIo, JsonSchema,
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
    Debug, Clone, Serialize, Deserialize, PartialEq, Default, aide::OperationIo, JsonSchema,
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
    Debug, Clone, Serialize, Deserialize, PartialEq, Default, aide::OperationIo, JsonSchema,
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
    Debug, Clone, Serialize, Deserialize, PartialEq, Default, aide::OperationIo, JsonSchema,
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, aide::OperationIo, JsonSchema)]
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
    State(dynamo_client): State<Arc<DynamoClient>>,
    Json(request): Json<CredentialRequest>,
) -> Result<Json<CredentialResponse>> {
    tracing::debug!("OID4VCI credential request: {:?}", request);

    let conf = config::get();
    let table_name = &conf.dual_write.table_name;

    // Extract user ID from auth token
    let user_id = extract_user_id_dynamo(&dynamo_client, table_name, auth.clone()).await?;

    // Validate access token
    if let Some(auth) = auth {
        validate_access_token(&dynamo_client, table_name, &auth).await?;
    } else {
        return Err(dto::Error::Unknown(
            "Authorization header required".to_string(),
        ));
    }

    // Validate request format
    if request.format != "jwt_vc_json" {
        return Err(dto::Error::Unknown(
            "Unsupported credential format".to_string(),
        ));
    }

    // Determine credential type
    let credential_types = if let Some(def) = &request.credential_definition {
        def.credential_type.clone()
    } else if let Some(id) = &request.credential_identifier {
        let cred_enum = CredentialType::from_str_or_default(id);
        match cred_enum {
            CredentialType::Passport => vec![
                "VerifiableCredential".to_string(),
                "PassportCredential".to_string(),
            ],
            CredentialType::Medical => vec![
                "VerifiableCredential".to_string(),
                "MedicalCredential".to_string(),
            ],
        }
    } else {
        return Err(dto::Error::Unknown(
            "Missing credential definition or identifier".to_string(),
        ));
    };

    // Validate proof of possession if provided
    if let Some(proof) = &request.proof {
        validate_proof_of_possession(proof)?;
    }

    // Generate credential based on type
    let credential_type = parse_credential_type(&credential_types)?;
    let credential = generate_credential(&credential_type, &user_id).await?;

    // Sign the credential as JWT
    let signed_credential = sign_credential_as_jwt(&credential).await?;

    // Generate new c_nonce for future requests
    let c_nonce = generate_c_nonce();
    let c_nonce_expires_in = 600; // 10 minutes

    // Store credential issuance record in DynamoDB
    let credential_id = credential.id.clone();
    let status_list_index = extract_status_list_index(&credential);

    let issued_credential = IssuedCredential::new(
        credential_id,
        user_id.clone(),
        credential_type.to_string(),
        signed_credential.clone(),
        serde_json::to_string(&credential.credential_subject).unwrap_or_default(),
        Some(parse_expiration_date(&credential.expiration_date)),
        status_list_index as i64,
    );

    let credential_item = to_item(&issued_credential).map_err(|e| {
        dto::Error::Unknown(format!("Failed to serialize issued credential: {}", e))
    })?;

    dynamo_client
        .put_item()
        .table_name(*table_name)
        .set_item(Some(credential_item))
        .send()
        .await
        .map_err(|e| dto::Error::Unknown(format!("DynamoDB put_item failed: {}", e)))?;

    tracing::info!("Issued {} credential for user {}", credential_type, user_id);

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

    let jwt_token = proof
        .jwt
        .as_ref()
        .ok_or_else(|| dto::Error::Unknown("Missing JWT proof".to_string()))?;

    // Parse and validate the JWT
    let verifier = JwtVerifier::new();
    let claims = verifier
        .verify_jwt_proof_of_possession(jwt_token)
        .map_err(|e| dto::Error::Unknown(format!("JWT validation failed: {}", e)))?;

    // Validate required claims for proof of possession
    validate_proof_claims(&claims)?;

    tracing::debug!("Proof of possession validation passed");
    Ok(())
}

/// Validate the claims in the proof of possession JWT
fn validate_proof_claims(claims: &serde_json::Value) -> Result<()> {
    // Check for required audience claim (should match issuer)
    let aud = claims
        .get("aud")
        .and_then(|v| v.as_str())
        .ok_or_else(|| dto::Error::Unknown("Missing or invalid 'aud' claim".to_string()))?;

    // Get issuer from config
    let expected_issuer = std::env::var("ISSUER_BASE_URL")
        .map_err(|_| dto::Error::Unknown("ISSUER_BASE_URL not configured".to_string()))?;

    if aud != expected_issuer {
        return Err(dto::Error::Unknown(format!(
            "Invalid audience: expected {}, got {}",
            expected_issuer, aud
        )));
    }

    // Check for nonce claim (c_nonce)
    let _nonce = claims
        .get("nonce")
        .or_else(|| claims.get("c_nonce"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| dto::Error::Unknown("Missing 'nonce' or 'c_nonce' claim".to_string()))?;

    // TODO: Validate the nonce against stored c_nonce
    // This would require looking up the nonce from the token exchange step

    // Check for issuer claim (should be the client/holder)
    let _iss = claims
        .get("iss")
        .and_then(|v| v.as_str())
        .ok_or_else(|| dto::Error::Unknown("Missing 'iss' claim".to_string()))?;

    // Check expiration time
    if let Some(exp) = claims.get("exp").and_then(|v| v.as_i64()) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        if exp < current_time {
            return Err(dto::Error::Unknown("Proof JWT has expired".to_string()));
        }
    } else {
        return Err(dto::Error::Unknown("Missing 'exp' claim".to_string()));
    }

    // Check issued at time (iat)
    if let Some(iat) = claims.get("iat").and_then(|v| v.as_i64()) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        // Allow some clock skew (5 minutes)
        if iat > current_time + 300 {
            return Err(dto::Error::Unknown(
                "Proof JWT issued in the future".to_string(),
            ));
        }
    }

    Ok(())
}

/// Parse credential type from type array
fn parse_credential_type(credential_types: &[String]) -> Result<CredentialType> {
    match credential_types.get(1).map(|s| s.as_str()) {
        Some("PassportCredential") => Ok(CredentialType::Passport),
        Some("MedicalCredential") => Ok(CredentialType::Medical),
        _ => Err(dto::Error::Unknown(
            "Unsupported credential type".to_string(),
        )),
    }
}

/// Generate credential based on type
async fn generate_credential(
    credential_type: &CredentialType,
    user_id: &str,
) -> Result<VerifiableCredential> {
    match credential_type {
        CredentialType::Passport => generate_passport_credential(user_id).await,
        CredentialType::Medical => generate_medical_credential(user_id).await,
    }
}

/// Generate passport credential from user data
async fn generate_passport_credential(user_id: &str) -> Result<VerifiableCredential> {
    // TODO: Fetch actual user passport data from secure storage
    // For now, we'll create a sample credential

    let conf = config::get();
    let domain = conf.domain;
    let base = format!("https://{}", domain);

    let credential_id = format!("{}/credentials/{}", base, Uuid::new_v4());
    let now = chrono::Utc::now();
    let issuance_date = now.to_rfc3339();
    let expiration_date = now
        .checked_add_signed(chrono::Duration::days(365 * 5)) // 5 years
        .unwrap()
        .to_rfc3339();

    // TODO: Get actual status list index from database
    let status_list_index = user_id.len() as u64; // Temporary mapping based on user_id length

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
async fn generate_medical_credential(user_id: &str) -> Result<VerifiableCredential> {
    // TODO: Fetch actual user medical data from secure storage
    // For now, we'll create a sample credential

    let conf = config::get();
    let domain = conf.domain;
    let base = format!("https://{}", domain);

    let credential_id = format!("{}/credentials/{}", base, Uuid::new_v4());
    let now = chrono::Utc::now();
    let issuance_date = now.to_rfc3339();
    let expiration_date = now
        .checked_add_signed(chrono::Duration::days(365)) // 1 year
        .unwrap()
        .to_rfc3339();

    // TODO: Get actual status list index from database
    let status_list_index = (user_id.len() + 1000) as u64; // Offset for medical credentials

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
    // Create JWT signer with configured keys
    let signer = JwtSigner::new()
        .map_err(|e| dto::Error::Unknown(format!("Failed to initialize JWT signer: {}", e)))?;

    // Create the JWT payload for the verifiable credential
    let issuer_string = credential
        .issuer
        .as_str()
        .map(|s| s.to_string())
        .or_else(|| {
            // If issuer is an object, try to get the 'id' field
            credential
                .issuer
                .get("id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| credential.id.clone()); // Fallback to credential ID

    let claims = Claims {
        iss: issuer_string,
        sub: credential
            .credential_subject
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        aud: None, // Not needed for VC JWT
        iat: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        exp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 3600, // 1 hour expiry
        jti: Some(uuid::Uuid::new_v4().to_string()),
        nonce: None, // Not needed for VC JWT (only for proof of possession)
    };

    let payload = VcJwtPayload {
        claims,
        vc: serde_json::to_value(credential)
            .map_err(|e| dto::Error::Unknown(format!("Failed to serialize credential: {}", e)))?,
    };

    // Sign the JWT using ES256
    let jwt_token = signer
        .sign_es256(&payload)
        .map_err(|e| dto::Error::Unknown(format!("Failed to sign credential JWT: {}", e)))?;

    tracing::debug!("Successfully signed credential as JWT with ES256");
    Ok(jwt_token)
}

/// Generate a cryptographic nonce
fn generate_c_nonce() -> String {
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

async fn validate_access_token(
    _dynamo_client: &Arc<DynamoClient>,
    _table_name: &str,
    auth: &dto::by_axum::auth::Authorization,
) -> Result<()> {
    match auth {
        dto::by_axum::auth::Authorization::Bearer { claims: _ } => {
            // TODO: Validate bearer token against DynamoDB OAuthAccessToken table
            // For now, just accept any Bearer token
            Ok(())
        }
        _ => Err(dto::Error::Unauthorized),
    }
}

fn extract_status_list_index(credential: &VerifiableCredential) -> u64 {
    if let Some(status) = &credential.credential_status {
        if let Some(index_str) = status.get("statusListIndex").and_then(|v| v.as_str()) {
            if let Ok(index) = index_str.parse::<u64>() {
                return index;
            }
        }
    }

    // Fallback to a default index
    1000
}

fn parse_expiration_date(exp_date: &Option<String>) -> i64 {
    match exp_date {
        Some(date_str) => {
            // Try to parse RFC3339 format
            if let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(date_str) {
                datetime.timestamp()
            } else {
                // Fallback to 1 year from now
                (chrono::Utc::now() + chrono::Duration::days(365)).timestamp()
            }
        }
        None => {
            // Default to 1 year from now
            (chrono::Utc::now() + chrono::Duration::days(365)).timestamp()
        }
    }
}
