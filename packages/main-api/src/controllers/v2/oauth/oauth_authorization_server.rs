use bdk::prelude::*;
use dto::{Result, by_axum::axum::Json};

use crate::{
    config,
    models::oauth::{
        code_challenge::CodeChallengeMethod, response_type::ResponseType, scope::Scope,
    },
};

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct AuthorizationServerMetadata {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub registration_endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwks_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_types_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_types_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint_auth_methods_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_documentation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_challenge_methods_supported: Option<Vec<String>>,
}

pub async fn oauth_authorization_server_handler() -> Result<Json<AuthorizationServerMetadata>> {
    let conf = config::get();

    let domain = conf.domain.to_string();
    let metadata = AuthorizationServerMetadata {
        registration_endpoint: format!("https://{}/v2/oauth/register", domain),
        authorization_endpoint: format!("https://{}/v2/oauth/authorize", domain),
        token_endpoint: format!("https://{}/v2/oauth/token", domain),
        issuer: domain,
        scopes_supported: Some(Scope::variants()),
        jwks_uri: None,
        response_types_supported: Some(ResponseType::variants()),
        code_challenge_methods_supported: Some(CodeChallengeMethod::variants()),
        grant_types_supported: None,
        token_endpoint_auth_methods_supported: None,

        service_documentation: None,
    };
    tracing::debug!("OAuth metadata: {:?}", metadata);
    Ok(Json(metadata))
}
