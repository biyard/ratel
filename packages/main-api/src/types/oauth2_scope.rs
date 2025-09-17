use bdk::prelude::*;
use serde_with::{DeserializeFromStr, SerializeDisplay};

/// OAuth 2.0 scopes for OpenID4VCI credential issuance
#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Default, JsonSchema)]
pub enum OAuth2Scope {
    #[default]
    /// Access to credential issuance endpoints
    CredentialIssuer,
    /// Access to passport credential issuance
    PassportCredential,
    /// Access to medical credential issuance
    MedicalCredential,
    /// Full access to all credential types
    All,
}

impl std::fmt::Display for OAuth2Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scope_str = match self {
            OAuth2Scope::CredentialIssuer => "credential_issuer",
            OAuth2Scope::PassportCredential => "passport_credential",
            OAuth2Scope::MedicalCredential => "medical_credential",
            OAuth2Scope::All => "credential_issuer passport_credential medical_credential",
        };
        write!(f, "{}", scope_str)
    }
}

impl std::str::FromStr for OAuth2Scope {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "credential_issuer" => Ok(OAuth2Scope::CredentialIssuer),
            "passport_credential" => Ok(OAuth2Scope::PassportCredential),
            "medical_credential" => Ok(OAuth2Scope::MedicalCredential),
            scope
                if scope.contains("credential_issuer")
                    && scope.contains("passport_credential")
                    && scope.contains("medical_credential") =>
            {
                Ok(OAuth2Scope::All)
            }
            _ => Err(format!("Unknown OAuth2 scope: {}", s)),
        }
    }
}

impl OAuth2Scope {
    /// Check if this scope allows access to a specific credential type
    pub fn allows_credential_type(&self, credential_type: &crate::types::CredentialType) -> bool {
        match (self, credential_type) {
            (OAuth2Scope::All, _) => true,
            (OAuth2Scope::CredentialIssuer, _) => true,
            (OAuth2Scope::PassportCredential, crate::types::CredentialType::Passport) => true,
            (OAuth2Scope::MedicalCredential, crate::types::CredentialType::Medical) => true,
            _ => false,
        }
    }

    /// Convert space-separated scope string to multiple scopes
    pub fn from_space_separated(s: &str) -> Vec<OAuth2Scope> {
        s.split_whitespace()
            .filter_map(|scope| scope.parse().ok())
            .collect()
    }

    /// Convert multiple scopes to space-separated string
    pub fn to_space_separated(scopes: &[OAuth2Scope]) -> String {
        scopes
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}
