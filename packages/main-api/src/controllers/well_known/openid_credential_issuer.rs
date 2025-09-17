use bdk::prelude::*;
use dto::{Result, by_axum::axum::Json};
use serde_json::Value;

use crate::config;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, aide::OperationIo, JsonSchema,
)]
pub struct OpenIdCredentialIssuerMetadata {
    pub credential_issuer: String,
    pub authorization_servers: Vec<String>,
    pub credential_endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_credential_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deferred_credential_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_endpoint: Option<String>,
    pub credentials_supported: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_signing_alg_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_encryption_alg_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_encryption_enc_values_supported: Option<Vec<String>>,
}

/// OpenID4VCI Credential Issuer Metadata Endpoint
///
/// This endpoint provides metadata about the credential issuer's capabilities,
/// supported credentials, cryptographic methods, and service endpoints.
///
/// Reference: https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0.html#name-credential-issuer-metadata
pub async fn openid_credential_issuer_handler() -> Result<Json<OpenIdCredentialIssuerMetadata>> {
    let conf = config::get();
    let domain = conf.domain;
    let base = format!("https://{}", domain);

    let credentials_supported = vec![
        serde_json::json!({
            "format": "jwt_vc_json",
            "id": "PassportCredential",
            "credential_definition": {
                "type": [
                    "VerifiableCredential",
                    "PassportCredential"
                ],
                "credentialSubject": {
                    "givenName": {
                        "mandatory": true,
                        "display": [
                            {
                                "name": "Given Name",
                                "locale": "en-US"
                            }
                        ]
                    },
                    "familyName": {
                        "mandatory": true,
                        "display": [
                            {
                                "name": "Family Name",
                                "locale": "en-US"
                            }
                        ]
                    },
                    "birthDate": {
                        "mandatory": true,
                        "display": [
                            {
                                "name": "Date of Birth",
                                "locale": "en-US"
                            }
                        ]
                    },
                    "nationality": {
                        "mandatory": true,
                        "display": [
                            {
                                "name": "Nationality",
                                "locale": "en-US"
                            }
                        ]
                    },
                    "gender": {
                        "mandatory": false,
                        "display": [
                            {
                                "name": "Gender",
                                "locale": "en-US"
                            }
                        ]
                    }
                }
            },
            "proof_types_supported": {
                "jwt": {
                    "proof_signing_alg_values_supported": [
                        "ES256",    // P-256 ECDSA
                        "EdDSA"     // Ed25519
                    ]
                }
            },
            "display": [
                {
                    "name": "Passport Credential",
                    "description": "A verifiable credential containing passport information",
                    "locale": "en-US",
                    "logo": {
                        "uri": format!("{}/images/passport-credential-logo.png", base),
                        "alt_text": "Passport Credential Logo"
                    },
                    "background_color": "#1F2937",
                    "text_color": "#FFFFFF"
                }
            ]
        }),
        serde_json::json!({
            "format": "jwt_vc_json",
            "id": "MedicalCredential",
            "credential_definition": {
                "type": [
                    "VerifiableCredential",
                    "MedicalCredential"
                ],
                "credentialSubject": {
                    "height": {
                        "mandatory": false,
                        "display": [
                            {
                                "name": "Height (cm)",
                                "locale": "en-US"
                            }
                        ]
                    },
                    "weight": {
                        "mandatory": false,
                        "display": [
                            {
                                "name": "Weight (kg)",
                                "locale": "en-US"
                            }
                        ]
                    },
                    "bmi": {
                        "mandatory": false,
                        "display": [
                            {
                                "name": "BMI",
                                "locale": "en-US"
                            }
                        ]
                    },
                    "bloodPressureSystolic": {
                        "mandatory": false,
                        "display": [
                            {
                                "name": "Blood Pressure Systolic",
                                "locale": "en-US"
                            }
                        ]
                    },
                    "bloodPressureDiastolic": {
                        "mandatory": false,
                        "display": [
                            {
                                "name": "Blood Pressure Diastolic",
                                "locale": "en-US"
                            }
                        ]
                    }
                }
            },
            "proof_types_supported": {
                "jwt": {
                    "proof_signing_alg_values_supported": [
                        "ES256",    // P-256 ECDSA
                        "EdDSA"     // Ed25519
                    ]
                }
            },
            "display": [
                {
                    "name": "Medical Credential",
                    "description": "A verifiable credential containing medical check-up information",
                    "locale": "en-US",
                    "logo": {
                        "uri": format!("{}/images/medical-credential-logo.png", base),
                        "alt_text": "Medical Credential Logo"
                    },
                    "background_color": "#059669",
                    "text_color": "#FFFFFF"
                }
            ]
        }),
    ];

    let display = vec![serde_json::json!({
        "name": "Ratel Identity Issuer",
        "description": "Decentralized identity credential issuer for the Ratel platform",
        "locale": "en-US",
        "logo": {
            // "uri": format!("{}/images/logo.png", base),
            "alt_text": "Ratel Logo"
        },
        "background_color": "#000000",
        "text_color": "#FFFFFF"
    })];

    let metadata = OpenIdCredentialIssuerMetadata {
        credential_issuer: format!("https://{}", domain),
        authorization_servers: vec![format!("https://{}", domain)],
        credential_endpoint: format!("{}/oid4vci/credential", base),
        batch_credential_endpoint: Some(format!("{}/oid4vci/batch_credential", base)),
        deferred_credential_endpoint: Some(format!("{}/oid4vci/deferred_credential", base)),
        notification_endpoint: Some(format!("{}/oid4vci/notification", base)),
        credentials_supported,
        display: Some(display),
        credential_signing_alg_values_supported: Some(vec![
            "ES256".to_string(),
            "EdDSA".to_string(),
        ]),
        // Set to none until actually implemented
        credential_encryption_alg_values_supported: None,
        credential_encryption_enc_values_supported: None,
    };

    Ok(Json(metadata))
}
