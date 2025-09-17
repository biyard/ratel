use bdk::prelude::*;
use serde::{Deserialize, Serialize};

/// Supported verifiable credential types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub enum CredentialType {
    #[serde(rename = "PassportCredential")]
    Passport,
    #[serde(rename = "MedicalCredential")]
    Medical,
}

impl CredentialType {
    /// Get the credential type as a string for W3C VC types array
    pub fn as_vc_type(&self) -> &'static str {
        match self {
            CredentialType::Passport => "PassportCredential",
            CredentialType::Medical => "MedicalCredential",
        }
    }

    /// Get the credential configuration ID for OpenID4VCI metadata
    pub fn as_config_id(&self) -> &'static str {
        match self {
            CredentialType::Passport => "PassportCredential",
            CredentialType::Medical => "MedicalCredential",
        }
    }

    /// Get all supported credential types
    pub fn all() -> Vec<CredentialType> {
        vec![CredentialType::Passport, CredentialType::Medical]
    }

    /// Parse from string, returning default if unknown
    pub fn from_str_or_default(s: &str) -> CredentialType {
        match s {
            "PassportCredential" | "passport" => CredentialType::Passport,
            "MedicalCredential" | "medical" => CredentialType::Medical,
            _ => {
                tracing::warn!(
                    "Unknown credential type: {}, falling back to PassportCredential",
                    s
                );
                CredentialType::Passport
            }
        }
    }
}

impl Default for CredentialType {
    fn default() -> Self {
        CredentialType::Passport
    }
}

impl std::fmt::Display for CredentialType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_vc_type())
    }
}
