use base64::Engine;
use bdk::prelude::*;
use by_axum::axum::Json;
use dto::{JsonSchema, Result, aide};
use serde::{Deserialize, Serialize};

use crate::config;

/// W3C Verifiable Credentials Status List v2021
///
/// This endpoint provides a bitstring-based status list for tracking
/// the revocation/suspension status of issued verifiable credentials.
///
/// Reference: https://www.w3.org/TR/vc-status-list/
#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Default, aide::OperationIo, JsonSchema,
)]
pub struct StatusListCredential {
    #[schemars(description = "JSON-LD context")]
    #[serde(rename = "@context")]
    pub context: Vec<String>,

    #[schemars(description = "Credential identifier")]
    pub id: String,

    #[schemars(description = "Credential type")]
    #[serde(rename = "type")]
    pub credential_type: Vec<String>,

    #[schemars(description = "Credential issuer")]
    pub issuer: String,

    #[schemars(description = "Issuance date")]
    #[serde(rename = "issuanceDate")]
    pub issuance_date: String,

    #[schemars(description = "Credential subject containing the status list")]
    #[serde(rename = "credentialSubject")]
    pub credential_subject: StatusListSubject,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Default, aide::OperationIo, JsonSchema,
)]
pub struct StatusListSubject {
    #[schemars(description = "Subject identifier")]
    pub id: String,

    #[schemars(description = "Subject type")]
    #[serde(rename = "type")]
    pub subject_type: String,

    #[schemars(description = "Purpose of the status list (revocation or suspension)")]
    #[serde(rename = "statusPurpose")]
    pub status_purpose: String,

    #[schemars(description = "Base64-encoded compressed bitstring")]
    #[serde(rename = "encodedList")]
    pub encoded_list: String,
}

/// Get Credential Status List
///
/// Returns a W3C StatusList2021Credential containing a bitstring
/// that represents the revocation/suspension status of issued credentials.
///
/// Each bit in the list corresponds to a credential's status:
/// - 0: Valid/Active
/// - 1: Revoked/Suspended
pub async fn get_status_list_handler() -> Result<Json<StatusListCredential>> {
    let conf = config::get();
    let domain = conf.domain;
    let base = format!("https://{}", domain);

    // Get current timestamp in ISO 8601 format
    let now = chrono::Utc::now().to_rfc3339();

    // Generate a compressed bitstring for the status list
    // For now, we'll create an empty list (all credentials valid)
    // In a real implementation, this would come from a database
    let bitstring = generate_empty_bitstring(1000); // Support 1000 credentials initially
    let encoded_list = base64::engine::general_purpose::STANDARD.encode(&bitstring);

    let status_list = StatusListCredential {
        context: vec![
            "https://www.w3.org/2018/credentials/v1".to_string(),
            "https://w3id.org/vc/status-list/2021/v1".to_string(),
        ],
        id: format!("{}/status/bitstring/1.json", base),
        credential_type: vec![
            "VerifiableCredential".to_string(),
            "StatusList2021Credential".to_string(),
        ],
        issuer: format!("https://{}", domain),
        issuance_date: now,
        credential_subject: StatusListSubject {
            id: format!("{}/status/bitstring/1.json#list", base),
            subject_type: "StatusList2021".to_string(),
            status_purpose: "revocation".to_string(),
            encoded_list,
        },
    };

    Ok(Json(status_list))
}

/// Generate an empty bitstring (all zeros) compressed with gzip
///
/// This creates a bitstring where all bits are 0, meaning all credentials
/// are valid/active. In a production system, this would be stored in a
/// database and updated when credentials are revoked.
fn generate_empty_bitstring(size: usize) -> Vec<u8> {
    use std::io::Write;

    // Create a byte array with all zeros (each byte represents 8 credentials)
    let byte_size = (size + 7) / 8; // Round up to nearest byte
    let bitstring = vec![0u8; byte_size];

    // Compress with gzip as required by the spec
    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(&bitstring).unwrap();
    encoder.finish().unwrap()
}
