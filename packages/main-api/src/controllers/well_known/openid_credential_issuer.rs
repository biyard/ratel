use bdk::prelude::*;
use dto::{Result, by_axum::axum::Json};
use serde_json::Value;

use crate::config;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, aide::OperationIo, JsonSchema,
)]
pub struct OpenIdCredentialIssuerMetadata {
    /// The issuer identifier
    pub credential_issuer: String,
    /// The credential endpoint for issuing credentials
    pub credential_endpoint: String,
    /// List of supported credentials
    pub credentials_supported: Vec<Value>,
    /// Optional authorization server metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_server: Option<String>,
    /// Token endpoint for OAuth flows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint: Option<String>,
    /// Batch credential endpoint for bulk issuance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_credential_endpoint: Option<String>,
    /// Deferred credential endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deferred_credential_endpoint: Option<String>,
    /// Credential issuer display information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<Value>>,
    /// Supported credential signing algorithms
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_signing_alg_values_supported: Option<Vec<String>>,
    /// Supported credential encryption algorithms
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_encryption_alg_values_supported: Option<Vec<String>>,
    /// Supported credential encryption encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_encryption_enc_values_supported: Option<Vec<String>>,
    /// Whether proof is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_types_supported: Option<Value>,
}

/// OpenID4VCI Credential Issuer Metadata Endpoint
///
/// This endpoint provides metadata about the credential issuer's capabilities,
/// supported credentials, cryptographic methods, and service endpoints.
///
/// Reference: https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0.html#name-credential-issuer-metadata
pub async fn openid_credential_issuer_handler() -> Result<Json<OpenIdCredentialIssuerMetadata>> {
    let conf = config::get();
    let domain = &conf.domain;
    let base = format!("https://{}", domain);

    // Build comprehensive credentials_supported array
    let credentials_supported = build_supported_credentials(&base);

    // Build issuer display information
    let display = build_issuer_display(&base);

    // Build proof types supported
    let proof_types_supported = build_proof_types_supported();

    let metadata = OpenIdCredentialIssuerMetadata {
        credential_issuer: format!("https://{}", domain),
        credential_endpoint: format!("{}/oid4vci/credential", base),
        authorization_server: Some(format!("https://{}", domain)),
        token_endpoint: Some(format!("{}/oid4vci/token", base)),
        batch_credential_endpoint: Some(format!("{}/oid4vci/batch_credential", base)),
        deferred_credential_endpoint: Some(format!("{}/oid4vci/deferred", base)),
        credentials_supported,
        display: Some(display),
        credential_signing_alg_values_supported: Some(vec![
            "ES256".to_string(),
            "EdDSA".to_string(),
        ]),
        credential_encryption_alg_values_supported: None, // Not implemented yet
        credential_encryption_enc_values_supported: None, // Not implemented yet
        proof_types_supported: Some(proof_types_supported),
    };

    Ok(Json(metadata))
}

/// Build the credentials_supported array with comprehensive credential definitions
fn build_supported_credentials(base: &str) -> Vec<Value> {
    vec![
        // Identity Credential
        serde_json::json!({
            "format": "jwt_vc_json",
            "id": "IdentityCredential",
            "credential_definition": {
                "type": [
                    "VerifiableCredential",
                    "IdentityCredential"
                ],
                "credentialSubject": {
                    "id": {
                        "mandatory": true,
                        "display": [
                            {
                                "name": "Subject ID",
                                "locale": "en-US"
                            }
                        ]
                    },
                    "name": {
                        "mandatory": true,
                        "display": [
                            {
                                "name": "Full Name",
                                "locale": "en-US"
                            }
                        ]
                    },
                    "email": {
                        "mandatory": false,
                        "display": [
                            {
                                "name": "Email Address",
                                "locale": "en-US"
                            }
                        ]
                    }
                }
            },
            "cryptographic_binding_methods_supported": [
                "did:key",
                "jwk"
            ],
            "credential_signing_alg_values_supported": [
                "ES256",
                "EdDSA"
            ],
            "proof_types_supported": {
                "jwt": {
                    "proof_signing_alg_values_supported": [
                        "ES256",
                        "EdDSA"
                    ]
                }
            },
            "display": [
                {
                    "name": "Digital Identity Credential",
                    "description": "A verifiable digital identity credential for the Ratel platform",
                    "locale": "en-US",
                    "logo": {
                        "uri": format!("{}/images/identity-credential-logo.png", base),
                        "alt_text": "Identity Credential Logo"
                    },
                    "background_color": "#1F2937",
                    "text_color": "#FFFFFF"
                }
            ]
        }),
        // Verification Credential  
        serde_json::json!({
            "format": "jwt_vc_json",
            "id": "VerificationCredential",
            "credential_definition": {
                "type": [
                    "VerifiableCredential",
                    "VerificationCredential"
                ],
                "credentialSubject": {
                    "verificationType": {
                        "mandatory": true,
                        "display": [
                            {
                                "name": "Verification Type",
                                "locale": "en-US"
                            }
                        ]
                    },
                    "verificationLevel": {
                        "mandatory": true,
                        "display": [
                            {
                                "name": "Verification Level",
                                "locale": "en-US"
                            }
                        ]
                    },
                    "verifiedAt": {
                        "mandatory": true,
                        "display": [
                            {
                                "name": "Verified At",
                                "locale": "en-US"
                            }
                        ]
                    }
                }
            },
            "cryptographic_binding_methods_supported": [
                "did:key",
                "jwk"
            ],
            "credential_signing_alg_values_supported": [
                "ES256",
                "EdDSA"
            ],
            "proof_types_supported": {
                "jwt": {
                    "proof_signing_alg_values_supported": [
                        "ES256",
                        "EdDSA"
                    ]
                }
            },
            "display": [
                {
                    "name": "Verification Credential",
                    "description": "A credential proving completion of verification process",
                    "locale": "en-US",
                    "logo": {
                        "uri": format!("{}/images/verification-credential-logo.png", base),
                        "alt_text": "Verification Credential Logo"
                    },
                    "background_color": "#059669",
                    "text_color": "#FFFFFF"
                }
            ]
        }),
        // Membership Credential
        serde_json::json!({
            "format": "jwt_vc_json",
            "id": "MembershipCredential", 
            "credential_definition": {
                "type": [
                    "VerifiableCredential",
                    "MembershipCredential"
                ],
                "credentialSubject": {
                    "organizationName": {
                        "mandatory": true,
                        "display": [
                            {
                                "name": "Organization",
                                "locale": "en-US"
                            }
                        ]
                    },
                    "membershipType": {
                        "mandatory": true,
                        "display": [
                            {
                                "name": "Membership Type",
                                "locale": "en-US"
                            }
                        ]
                    },
                    "memberSince": {
                        "mandatory": true,
                        "display": [
                            {
                                "name": "Member Since",
                                "locale": "en-US"
                            }
                        ]
                    }
                }
            },
            "cryptographic_binding_methods_supported": [
                "did:key",
                "jwk"
            ],
            "credential_signing_alg_values_supported": [
                "ES256",
                "EdDSA"
            ],
            "proof_types_supported": {
                "jwt": {
                    "proof_signing_alg_values_supported": [
                        "ES256",
                        "EdDSA"
                    ]
                }
            },
            "display": [
                {
                    "name": "Membership Credential",
                    "description": "A credential proving membership in an organization",
                    "locale": "en-US",
                    "logo": {
                        "uri": format!("{}/images/membership-credential-logo.png", base),
                        "alt_text": "Membership Credential Logo"
                    },
                    "background_color": "#7C3AED",
                    "text_color": "#FFFFFF"
                }
            ]
        })
    ]
}

/// Build issuer display information
fn build_issuer_display(base: &str) -> Vec<Value> {
    vec![serde_json::json!({
        "name": "Ratel Credential Issuer",
        "description": "Decentralized identity and verifiable credential issuer for the Ratel ecosystem",
        "locale": "en-US",
        "logo": {
            "uri": format!("{}/images/logo.png", base),
            "alt_text": "Ratel Logo"
        },
        "background_color": "#000000",
        "text_color": "#FFFFFF",
        "background_image": {
            "uri": format!("{}/images/issuer-background.png", base),
            "alt_text": "Ratel Background"
        }
    })]
}

/// Build proof types supported configuration
fn build_proof_types_supported() -> Value {
    serde_json::json!({
        "jwt": {
            "proof_signing_alg_values_supported": [
                "ES256",
                "EdDSA"
            ]
        }
    })
}
