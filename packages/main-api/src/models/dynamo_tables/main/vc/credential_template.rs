use bdk::prelude::*;
use crate::types::*;

/// Credential Template Entity
/// 
/// Stores templates and schemas for different credential types.
/// Enables dynamic credential format management and validation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct CredentialTemplate {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    #[dynamo(prefix = "ISSUER", name = "find_by_issuer", index = "gsi1", pk)]
    pub issuer_id: String,
    
    #[dynamo(prefix = "TYPE", name = "find_by_type", index = "gsi2", pk)]
    pub credential_type: String,
    #[dynamo(prefix = "VERSION", index = "gsi2", sk)]
    pub template_version: i64,

    /// Human-readable name for the template
    pub template_name: String,
    /// Description of what this credential represents
    pub description: String,
    /// JSON schema for credential subject validation
    pub subject_schema: String,
    /// Template for credential display properties
    pub display_template: Option<String>,
    /// Default expiration period in seconds
    pub default_expiry_seconds: Option<i64>,
    /// Required evidence for issuing this credential type
    pub required_evidence: Option<String>,
    /// Credential status configuration
    pub status_config: Option<String>,
    /// Additional metadata as JSON
    pub metadata: Option<String>,
    /// Whether this template is active
    pub is_active: bool,
    /// Whether this is the default template for the type
    pub is_default: bool,
}

impl CredentialTemplate {
    pub fn new(
        template_id: String,
        issuer_id: String,
        credential_type: String,
        template_name: String,
        description: String,
        subject_schema: String,
    ) -> Self {
        let pk = Partition::CredentialTemplate(template_id);
        let sk = EntityType::CredentialTemplate;
        let now = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            issuer_id,
            credential_type,
            template_version: 1,
            template_name,
            description,
            subject_schema,
            display_template: None,
            default_expiry_seconds: Some(365 * 24 * 60 * 60), // 1 year
            required_evidence: None,
            status_config: None,
            metadata: None,
            is_active: true,
            is_default: false,
        }
    }

    /// Update the template and increment version
    pub fn update_template(&mut self) {
        self.updated_at = chrono::Utc::now().timestamp_micros();
        self.template_version += 1;
    }

    /// Set as default template for this credential type
    pub fn set_as_default(&mut self) {
        self.is_default = true;
        self.update_template();
    }

    /// Unset as default template
    pub fn unset_as_default(&mut self) {
        self.is_default = false;
        self.update_template();
    }

    /// Deactivate the template
    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.update_template();
    }

    /// Get subject schema as JSON value
    pub fn get_subject_schema(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::from_str(&self.subject_schema)
    }

    /// Get display template as JSON value
    pub fn get_display_template(&self) -> Option<serde_json::Value> {
        self.display_template.as_ref()
            .and_then(|dt| serde_json::from_str(dt).ok())
    }

    /// Set display template
    pub fn set_display_template(&mut self, template: serde_json::Value) {
        self.display_template = Some(template.to_string());
        self.update_template();
    }

    /// Get required evidence as JSON value
    pub fn get_required_evidence(&self) -> Option<serde_json::Value> {
        self.required_evidence.as_ref()
            .and_then(|re| serde_json::from_str(re).ok())
    }

    /// Set required evidence
    pub fn set_required_evidence(&mut self, evidence: serde_json::Value) {
        self.required_evidence = Some(evidence.to_string());
        self.update_template();
    }
}
