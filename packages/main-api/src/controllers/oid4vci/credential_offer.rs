use bdk::prelude::*;
// use base64::Engine;
use by_axum::{
    auth::Authorization,
    axum::{Extension, Json, extract::{Query, State}},
};
use dto::{Result, aide, JsonSchema, sqlx::PgPool};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{config, utils::users::extract_user_id};

/// Credential Offer Query Parameters
/// 
/// Query parameters for requesting a credential offer.
/// These parameters determine what type of credential to offer and how.
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
pub struct CredentialOfferQuery {
    #[schemars(description = "Type of credential to offer (passport, medical, etc.)")]
    pub credential_type: Option<String>,
    
    #[schemars(description = "Pre-authorize the offer (skip user consent)")]
    pub pre_authorize: Option<bool>,
    
    #[schemars(description = "User PIN required for pre-authorized flow")]
    pub user_pin_required: Option<bool>,
    
    #[schemars(description = "Callback URL after credential issuance")]
    pub callback_url: Option<String>,
    
    #[schemars(description = "State parameter for callback")]
    pub state: Option<String>,
}

/// OpenID4VCI Credential Offer Response
/// 
/// Contains the credential offer object that can be encoded in QR codes
/// or deep links for wallet applications to process.
/// 
/// Reference: https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0.html#name-credential-offer
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
pub struct CredentialOfferResponse {
    #[schemars(description = "The credential offer object")]
    pub credential_offer: CredentialOffer,
    
    #[schemars(description = "QR code data URL (data:image/png;base64,...)")]
    pub qr_code: Option<String>,
    
    #[schemars(description = "Deep link URL for wallet apps")]
    pub deep_link: String,
    
    #[schemars(description = "Offer ID for tracking")]
    pub offer_id: String,
}

/// OpenID4VCI Credential Offer Object
/// 
/// The main credential offer object as defined in the OpenID4VCI specification.
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
pub struct CredentialOffer {
    #[schemars(description = "Credential issuer identifier")]
    pub credential_issuer: String,
    
    #[schemars(description = "Array of credential configurations")]
    pub credentials: Vec<CredentialConfiguration>,
    
    #[schemars(description = "Grants available for this offer")]
    pub grants: Option<GrantsObject>,
}

/// Credential Configuration
/// 
/// Describes the credentials being offered.
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
pub struct CredentialConfiguration {
    #[schemars(description = "Credential format (jwt_vc_json, ldp_vc, etc.)")]
    pub format: String,
    
    #[schemars(description = "Credential definition")]
    pub credential_definition: Option<CredentialDefinition>,
    
    #[schemars(description = "Reference to credential configuration in issuer metadata")]
    pub credential_configuration_id: Option<String>,
}

/// Credential Definition
/// 
/// Defines the structure and type of the credential.
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
    #[schemars(description = "JSON-LD context (for ldp_vc format)")]
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<Vec<String>>,
    
    #[schemars(description = "Credential types")]
    #[serde(rename = "type")]
    pub credential_type: Vec<String>,
    
    #[schemars(description = "Credential subject properties")]
    #[serde(rename = "credentialSubject", skip_serializing_if = "Option::is_none")]
    pub credential_subject: Option<HashMap<String, serde_json::Value>>,
}

/// Grants Object
/// 
/// Specifies the grants available for obtaining the credentials.
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
pub struct GrantsObject {
    #[schemars(description = "Authorization code grant configuration")]
    pub authorization_code: Option<AuthorizationCodeGrant>,
    
    #[schemars(description = "Pre-authorized code grant configuration")]
    #[serde(rename = "urn:ietf:params:oauth:grant-type:pre-authorized_code")]
    pub pre_authorized_code: Option<PreAuthorizedCodeGrant>,
}

/// Authorization Code Grant Configuration
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
pub struct AuthorizationCodeGrant {
    #[schemars(description = "Issuer state for the authorization")]
    pub issuer_state: Option<String>,
    
    #[schemars(description = "Authorization server URL")]
    pub authorization_server: Option<String>,
}

/// Pre-Authorized Code Grant Configuration
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
pub struct PreAuthorizedCodeGrant {
    #[schemars(description = "Pre-authorized code")]
    #[serde(rename = "pre-authorized_code")]
    pub pre_authorized_code: String,
    
    #[schemars(description = "Whether user PIN is required")]
    pub user_pin_required: Option<bool>,
}

/// Credential Offer Handler
/// 
/// Creates credential offers that can be shared via QR codes or deep links.
/// Supports both authorization code and pre-authorized code flows.
pub async fn credential_offer_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Query(query): Query<CredentialOfferQuery>,
) -> Result<Json<CredentialOfferResponse>> {
    tracing::debug!("Credential offer request: {:?}", query);
    
    let conf = config::get();
    let domain = conf.domain;
    let _base = format!("https://{}", domain);
    
    // Extract user ID if authenticated
    let user_id = if auth.is_some() {
        Some(extract_user_id(&pool, auth).await?)
    } else {
        None
    };
    
    // Generate unique offer ID
    let offer_id = format!("offer_{}", Uuid::new_v4().to_string().replace('-', ""));
    
    // Determine credential type
    let credential_type = query.credential_type
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "PassportCredential".to_string());
    
    // Create credential configuration based on type
    let credential_config = create_credential_configuration(&credential_type)?;
    
    // Create grants object
    let grants = create_grants_object(&query, &offer_id)?;
    
    // Build the credential offer
    let credential_offer = CredentialOffer {
        credential_issuer: format!("https://{}", domain),
        credentials: vec![credential_config],
        grants: Some(grants),
    };
    
    // Create deep link URL
    let offer_json = serde_json::to_string(&credential_offer)
        .map_err(|e| dto::Error::Unknown(format!("Failed to serialize credential offer: {}", e)))?;
    
    let encoded_offer = base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE, offer_json.as_bytes());
    let deep_link = format!("openid-credential-offer://?credential_offer={}", encoded_offer);
    
    // TODO: Generate QR code
    // For now, we'll leave it as None. In a real implementation, you would:
    // 1. Use a QR code library like `qrcode` to generate the QR code
    // 2. Encode it as a base64 PNG data URL
    let qr_code = None;
    
    // TODO: Store the offer in database for tracking and validation
    // This should include:
    // - offer_id
    // - user_id (if authenticated)
    // - credential_type
    // - expiration time
    // - pre_authorized_code (if applicable)
    
    tracing::info!("Created credential offer {} for user {:?} with type {}", 
                   offer_id, user_id, credential_type);
    
    let response = CredentialOfferResponse {
        credential_offer,
        qr_code,
        deep_link,
        offer_id,
    };
    
    Ok(Json(response))
}

/// Create credential configuration based on credential type
fn create_credential_configuration(credential_type: &str) -> Result<CredentialConfiguration> {
    match credential_type {
        "PassportCredential" => {
            Ok(CredentialConfiguration {
                format: "jwt_vc_json".to_string(),
                credential_configuration_id: Some("PassportCredential".to_string()),
                credential_definition: Some(CredentialDefinition {
                    context: None, // Not needed for JWT format
                    credential_type: vec![
                        "VerifiableCredential".to_string(),
                        "PassportCredential".to_string(),
                    ],
                    credential_subject: None, // Will be populated during issuance
                }),
            })
        },
        "MedicalCredential" => {
            Ok(CredentialConfiguration {
                format: "jwt_vc_json".to_string(),
                credential_configuration_id: Some("MedicalCredential".to_string()),
                credential_definition: Some(CredentialDefinition {
                    context: None, // Not needed for JWT format
                    credential_type: vec![
                        "VerifiableCredential".to_string(),
                        "MedicalCredential".to_string(),
                    ],
                    credential_subject: None, // Will be populated during issuance
                }),
            })
        },
        _ => {
            tracing::warn!("Unknown credential type: {}, falling back to PassportCredential", credential_type);
            create_credential_configuration("PassportCredential")
        }
    }
}

/// Create grants object based on query parameters
fn create_grants_object(query: &CredentialOfferQuery, offer_id: &str) -> Result<GrantsObject> {
    let pre_authorize = query.pre_authorize.unwrap_or(true); // Default to pre-authorized flow
    
    if pre_authorize {
        // Pre-authorized code flow
        let pre_authorized_code = format!("pre_auth_{}_{}", 
                                        offer_id, 
                                        Uuid::new_v4().to_string().replace('-', ""));
        
        Ok(GrantsObject {
            authorization_code: None,
            pre_authorized_code: Some(PreAuthorizedCodeGrant {
                pre_authorized_code,
                user_pin_required: query.user_pin_required,
            }),
        })
    } else {
        // Authorization code flow
        let issuer_state = format!("state_{}_{}", 
                                 offer_id, 
                                 Uuid::new_v4().to_string().replace('-', ""));
        
        Ok(GrantsObject {
            authorization_code: Some(AuthorizationCodeGrant {
                issuer_state: Some(issuer_state),
                authorization_server: None, // Will use default from metadata
            }),
            pre_authorized_code: None,
        })
    }
}
