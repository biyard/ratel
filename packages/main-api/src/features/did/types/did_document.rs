use super::verification_method::{VerificationMethod, VerificationRelationship};
use bdk::prelude::*;

/// A service in a DID document
/// Services enable discovery of service endpoints for interacting with the DID subject
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Service {
    /// The service ID (usually a DID URL with fragment)
    pub id: String,

    /// The service type (e.g., "LinkedDomains", "CredentialRegistry")
    #[serde(rename = "type")]
    pub service_type: ServiceType,

    /// The service endpoint (URL or object)
    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: ServiceEndpoint,
}

/// Service type definitions
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ServiceType {
    /// Single service type
    Single(String),
    /// Multiple service types
    Multiple(Vec<String>),
}

impl ServiceType {
    /// Check if the service type matches a given type name
    pub fn matches(&self, type_name: &str) -> bool {
        match self {
            ServiceType::Single(t) => t == type_name,
            ServiceType::Multiple(types) => types.iter().any(|t| t == type_name),
        }
    }
}

/// Service endpoint - can be a URL string or complex object
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ServiceEndpoint {
    /// Simple string URL
    String(String),
    /// Array of URLs
    Array(Vec<String>),
    /// Complex endpoint object
    Object(serde_json::Value),
}

/// Complete DID Document structure
/// Based on W3C DID Core specification: https://www.w3.org/TR/did-core/
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct DidDocument {
    /// JSON-LD context
    #[serde(rename = "@context")]
    pub context: DidContext,

    /// The DID that this document describes
    pub id: String,

    /// Optional list of DIDs that can also be used to refer to this subject
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "alsoKnownAs")]
    pub also_known_as: Option<Vec<String>>,

    /// The DID(s) that control this document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<DidController>,

    /// Verification methods defined in this document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "verificationMethod")]
    pub verification_method: Option<Vec<VerificationMethod>>,

    /// Verification methods for authentication purposes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<Vec<VerificationRelationship>>,

    /// Verification methods for assertion purposes (e.g., issuing credentials)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assertionMethod")]
    pub assertion_method: Option<Vec<VerificationRelationship>>,

    /// Verification methods for key agreement (e.g., encryption)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "keyAgreement")]
    pub key_agreement: Option<Vec<VerificationRelationship>>,

    /// Verification methods for capability invocation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "capabilityInvocation")]
    pub capability_invocation: Option<Vec<VerificationRelationship>>,

    /// Verification methods for capability delegation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "capabilityDelegation")]
    pub capability_delegation: Option<Vec<VerificationRelationship>>,

    /// Services for interacting with the DID subject
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Vec<Service>>,
}

/// JSON-LD context for DID documents
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum DidContext {
    /// Single context URL
    Single(String),
    /// Array of context URLs
    Multiple(Vec<String>),
}

impl Default for DidContext {
    fn default() -> Self {
        DidContext::Single("https://www.w3.org/ns/did/v1".to_string())
    }
}

/// Controller field can be a single DID or array of DIDs
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum DidController {
    /// Single controller DID
    Single(String),
    /// Multiple controller DIDs
    Multiple(Vec<String>),
}

impl DidDocument {
    /// Validate the DID document structure
    pub fn validate(&self) -> Result<(), String> {
        // Validate ID
        if self.id.is_empty() {
            return Err("DID document ID cannot be empty".to_string());
        }

        if !self.id.starts_with("did:") {
            return Err("DID document ID must start with 'did:'".to_string());
        }

        // Validate verification methods if present
        if let Some(vms) = &self.verification_method {
            for vm in vms {
                vm.validate()?;
            }
        }

        Ok(())
    }

    /// Get all verification method IDs from the document
    pub fn verification_method_ids(&self) -> Vec<String> {
        self.verification_method
            .as_ref()
            .map(|vms| vms.iter().map(|vm| vm.id.clone()).collect())
            .unwrap_or_default()
    }

    /// Find a verification method by ID
    pub fn find_verification_method(&self, id: &str) -> Option<&VerificationMethod> {
        self.verification_method
            .as_ref()
            .and_then(|vms| vms.iter().find(|vm| vm.id == id))
    }

    /// Get authentication verification methods
    pub fn authentication_methods(&self) -> Vec<&VerificationMethod> {
        self.resolve_verification_relationships(self.authentication.as_ref())
    }

    /// Get assertion verification methods
    pub fn assertion_methods(&self) -> Vec<&VerificationMethod> {
        self.resolve_verification_relationships(self.assertion_method.as_ref())
    }

    /// Get key agreement verification methods
    pub fn key_agreement_methods(&self) -> Vec<&VerificationMethod> {
        self.resolve_verification_relationships(self.key_agreement.as_ref())
    }

    /// Helper to resolve verification relationships (references or embedded)
    fn resolve_verification_relationships<'a>(
        &'a self,
        relationships: Option<&'a Vec<VerificationRelationship>>,
    ) -> Vec<&'a VerificationMethod> {
        relationships
            .map(|rels| {
                rels.iter()
                    .filter_map(|rel| match rel {
                        VerificationRelationship::Reference(id) => {
                            self.find_verification_method(id)
                        }
                        VerificationRelationship::Embedded(vm) => Some(vm),
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Find a service by ID
    pub fn find_service(&self, id: &str) -> Option<&Service> {
        self.service
            .as_ref()
            .and_then(|services| services.iter().find(|s| s.id == id))
    }

    /// Find services by type
    pub fn find_services_by_type(&self, service_type: &str) -> Vec<&Service> {
        self.service
            .as_ref()
            .map(|services| {
                services
                    .iter()
                    .filter(|s| s.service_type.matches(service_type))
                    .collect()
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::did::types::verification_method::VerificationMethodType;

    #[test]
    fn test_did_document_validation() {
        let doc = DidDocument {
            context: DidContext::default(),
            id: "did:web:example.com".to_string(),
            also_known_as: None,
            controller: None,
            verification_method: None,
            authentication: None,
            assertion_method: None,
            key_agreement: None,
            capability_invocation: None,
            capability_delegation: None,
            service: None,
        };

        assert!(doc.validate().is_ok());
    }

    #[test]
    fn test_did_document_invalid_id() {
        let doc = DidDocument {
            context: DidContext::default(),
            id: "not-a-did".to_string(),
            also_known_as: None,
            controller: None,
            verification_method: None,
            authentication: None,
            assertion_method: None,
            key_agreement: None,
            capability_invocation: None,
            capability_delegation: None,
            service: None,
        };

        assert!(doc.validate().is_err());
    }

    #[test]
    fn test_service_type_matches() {
        let single = ServiceType::Single("LinkedDomains".to_string());
        assert!(single.matches("LinkedDomains"));
        assert!(!single.matches("Other"));

        let multiple = ServiceType::Multiple(vec![
            "LinkedDomains".to_string(),
            "CredentialRegistry".to_string(),
        ]);
        assert!(multiple.matches("LinkedDomains"));
        assert!(multiple.matches("CredentialRegistry"));
        assert!(!multiple.matches("Other"));
    }
}
