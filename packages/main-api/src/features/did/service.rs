use crate::features::did::{
    dto::{
        DidDocumentMetadata, ResolveDidResponse, ValidateDidDocumentRequest,
        ValidateDidDocumentResponse,
    },
    resolver::{DidResolver, ResolutionOptions},
    types::{DidDocument, DidIdentifier},
};
use std::sync::Arc;

/// Service for DID operations
/// Provides high-level methods for resolving, validating, and working with DIDs
pub struct DidService {
    resolver: Arc<DidResolver>,
}

impl DidService {
    /// Create a new DID service with default resolver options
    pub fn new() -> Result<Self, String> {
        let resolver = DidResolver::new()?;
        Ok(Self {
            resolver: Arc::new(resolver),
        })
    }

    /// Create a new DID service with custom resolver options
    pub fn with_options(options: ResolutionOptions) -> Result<Self, String> {
        let resolver = DidResolver::with_options(options)?;
        Ok(Self {
            resolver: Arc::new(resolver),
        })
    }

    /// Resolve a DID to a DID document
    pub async fn resolve_did(&self, did_str: &str) -> Result<ResolveDidResponse, String> {
        tracing::info!("Resolving DID: {}", did_str);

        // Resolve using the resolver
        let result = self.resolver.resolve(did_str).await?;

        // Build response
        Ok(ResolveDidResponse {
            did_document: result.document,
            did_resolution_metadata: result.metadata,
            did_document_metadata: Some(DidDocumentMetadata {
                created: None,
                updated: None,
                deactivated: None,
                version_id: None,
                next_version_id: None,
                equivalent_id: None,
                canonical_id: None,
            }),
        })
    }

    /// Validate a DID document
    pub fn validate_did_document(
        &self,
        request: ValidateDidDocumentRequest,
    ) -> ValidateDidDocumentResponse {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        match request.did_document.validate() {
            Ok(_) => {}
            Err(e) => errors.push(e),
        }

        // Additional validation checks
        self.validate_context(&request.did_document, &mut errors, &mut warnings);
        self.validate_verification_methods(&request.did_document, &mut errors, &mut warnings);
        self.validate_services(&request.did_document, &mut errors, &mut warnings);

        ValidateDidDocumentResponse {
            valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    /// Validate the @context field
    fn validate_context(
        &self,
        document: &DidDocument,
        errors: &mut Vec<String>,
        _warnings: &mut Vec<String>,
    ) {
        use crate::features::did::types::DidContext;

        let contexts = match &document.context {
            DidContext::Single(ctx) => vec![ctx.as_str()],
            DidContext::Multiple(ctxs) => ctxs.iter().map(|s| s.as_str()).collect(),
        };

        // Must include the base DID context
        if !contexts.contains(&"https://www.w3.org/ns/did/v1") {
            errors.push(
                "DID document must include 'https://www.w3.org/ns/did/v1' in @context".to_string(),
            );
        }
    }

    /// Validate verification methods
    fn validate_verification_methods(
        &self,
        document: &DidDocument,
        errors: &mut Vec<String>,
        warnings: &mut Vec<String>,
    ) {
        if let Some(vms) = &document.verification_method {
            for (idx, vm) in vms.iter().enumerate() {
                // Validate each verification method
                if let Err(e) = vm.validate() {
                    errors.push(format!("Verification method {}: {}", idx, e));
                }

                // Check if ID is a proper DID URL
                if !vm.id.starts_with(&document.id) && !vm.id.starts_with("did:") {
                    warnings.push(format!(
                        "Verification method {} ID does not reference the document's DID",
                        idx
                    ));
                }

                // Check if controller matches the document ID
                if vm.controller != document.id {
                    warnings.push(format!(
                        "Verification method {} controller ('{}') differs from document ID",
                        idx, vm.controller
                    ));
                }
            }
        }

        // Check verification relationships reference valid methods
        if let Some(auth) = &document.authentication {
            self.validate_verification_relationships(
                document,
                auth,
                "authentication",
                errors,
                warnings,
            );
        }

        if let Some(assertion) = &document.assertion_method {
            self.validate_verification_relationships(
                document,
                assertion,
                "assertionMethod",
                errors,
                warnings,
            );
        }

        if let Some(key_agreement) = &document.key_agreement {
            self.validate_verification_relationships(
                document,
                key_agreement,
                "keyAgreement",
                errors,
                warnings,
            );
        }
    }

    /// Validate verification relationship references
    fn validate_verification_relationships(
        &self,
        document: &DidDocument,
        relationships: &[crate::features::did::types::VerificationRelationship],
        relationship_type: &str,
        errors: &mut Vec<String>,
        _warnings: &mut Vec<String>,
    ) {
        use crate::features::did::types::VerificationRelationship;

        for (idx, rel) in relationships.iter().enumerate() {
            if let VerificationRelationship::Reference(id) = rel {
                // Check if the referenced verification method exists
                if document.find_verification_method(id).is_none() {
                    errors.push(format!(
                        "{} relationship {} references non-existent verification method: {}",
                        relationship_type, idx, id
                    ));
                }
            }
        }
    }

    /// Validate services
    fn validate_services(
        &self,
        document: &DidDocument,
        errors: &mut Vec<String>,
        warnings: &mut Vec<String>,
    ) {
        if let Some(services) = &document.service {
            for (idx, service) in services.iter().enumerate() {
                // Validate service ID
                if service.id.is_empty() {
                    errors.push(format!("Service {} has empty ID", idx));
                }

                // Check if service ID starts with # (fragment) or is a full DID URL
                if !service.id.starts_with('#') && !service.id.starts_with("did:") {
                    warnings.push(format!(
                        "Service {} ID should start with '#' or be a full DID URL",
                        idx
                    ));
                }
            }
        }
    }

    /// Parse and validate a DID string
    pub fn parse_did(&self, did_str: &str) -> Result<DidIdentifier, String> {
        let did = DidIdentifier::parse(did_str)?;
        did.validate()?;
        Ok(did)
    }

    /// Check if a DID method is supported
    pub fn is_method_supported(&self, method: &str) -> bool {
        // Currently only did:web is fully supported
        matches!(method, "web")
    }

    /// Get information about supported DID methods
    pub fn supported_methods(&self) -> Vec<String> {
        vec!["web".to_string()]
    }
}

impl Default for DidService {
    fn default() -> Self {
        Self::new().expect("Failed to create default DID service")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::did::types::{
        DidContext, DidDocument, VerificationMethod, VerificationMethodType,
    };

    #[test]
    fn test_service_creation() {
        let service = DidService::new();
        assert!(service.is_ok());
    }

    #[test]
    fn test_parse_did() {
        let service = DidService::new().unwrap();
        let result = service.parse_did("did:web:example.com");
        assert!(result.is_ok());

        let did = result.unwrap();
        assert_eq!(did.method.method_name(), "web");
        assert_eq!(did.method_specific_id, "example.com");
    }

    #[test]
    fn test_is_method_supported() {
        let service = DidService::new().unwrap();
        assert!(service.is_method_supported("web"));
        assert!(!service.is_method_supported("key"));
        assert!(!service.is_method_supported("plc"));
    }

    #[test]
    fn test_validate_empty_document() {
        let service = DidService::new().unwrap();

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

        let response =
            service.validate_did_document(ValidateDidDocumentRequest { did_document: doc });

        assert!(response.valid);
    }

    #[test]
    fn test_validate_document_with_verification_method() {
        let service = DidService::new().unwrap();

        let vm = VerificationMethod {
            id: "did:web:example.com#key-1".to_string(),
            method_type: VerificationMethodType::Ed25519VerificationKey2020,
            controller: "did:web:example.com".to_string(),
            public_key_multibase: Some(
                "z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string(),
            ),
            public_key_jwk: None,
            public_key_base58: None,
            public_key_hex: None,
        };

        let doc = DidDocument {
            context: DidContext::default(),
            id: "did:web:example.com".to_string(),
            also_known_as: None,
            controller: None,
            verification_method: Some(vec![vm]),
            authentication: None,
            assertion_method: None,
            key_agreement: None,
            capability_invocation: None,
            capability_delegation: None,
            service: None,
        };

        let response =
            service.validate_did_document(ValidateDidDocumentRequest { did_document: doc });

        assert!(response.valid);
    }
}
