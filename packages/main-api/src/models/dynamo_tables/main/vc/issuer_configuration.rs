use bdk::prelude::*;
use crate::types::*;

/// Issuer Configuration Entity
/// 
/// Stores dynamic configuration for the credential issuer.
/// Allows runtime configuration changes without code deployment.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct IssuerConfiguration {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    #[dynamo(prefix = "ISSUER", name = "find_by_issuer", index = "gsi1", pk)]
    pub issuer_id: String,
    
    #[dynamo(prefix = "STATUS", name = "find_by_status", index = "gsi2", pk)]
    pub status: String,
    #[dynamo(prefix = "VERSION", index = "gsi2", sk)]
    pub config_version: i64,

    /// Display name for the issuer
    pub display_name: String,
    /// Description of the issuer
    pub description: Option<String>,
    /// Issuer logo URL
    pub logo_url: Option<String>,
    /// Background color for branding
    pub background_color: Option<String>,
    /// Text color for branding
    pub text_color: Option<String>,
    /// Supported credential types as JSON array
    pub supported_credential_types: String,
    /// Default credential expiration in seconds
    pub default_credential_expiry: i64,
    /// Maximum batch size for status updates
    pub max_batch_size: i64,
    /// Rate limiting configuration as JSON
    pub rate_limits: Option<String>,
    /// Additional issuer metadata as JSON
    pub metadata: Option<String>,
    /// Whether the issuer is active
    pub is_active: bool,
}

impl IssuerConfiguration {
    pub fn new(
        issuer_id: String,
        display_name: String,
        supported_credential_types: Vec<String>,
    ) -> Self {
        let pk = Partition::IssuerConfig(issuer_id.clone());
        let sk = EntityType::IssuerConfiguration;
        let now = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            issuer_id,
            status: "active".to_string(),
            config_version: 1,
            display_name,
            description: None,
            logo_url: None,
            background_color: None,
            text_color: None,
            supported_credential_types: serde_json::to_string(&supported_credential_types).unwrap_or_default(),
            default_credential_expiry: 365 * 24 * 60 * 60, // 1 year in seconds
            max_batch_size: 1000,
            rate_limits: None,
            metadata: None,
            is_active: true,
        }
    }

    /// Update the configuration and increment version
    pub fn update_config(&mut self) {
        self.updated_at = chrono::Utc::now().timestamp_micros();
        self.config_version += 1;
    }

    /// Get supported credential types
    pub fn get_supported_credential_types(&self) -> Vec<String> {
        serde_json::from_str(&self.supported_credential_types).unwrap_or_default()
    }

    /// Set supported credential types
    pub fn set_supported_credential_types(&mut self, types: Vec<String>) {
        self.supported_credential_types = serde_json::to_string(&types).unwrap_or_default();
        self.update_config();
    }

    /// Deactivate the issuer
    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.status = "inactive".to_string();
        self.update_config();
    }

    /// Get rate limits configuration
    pub fn get_rate_limits(&self) -> Option<serde_json::Value> {
        self.rate_limits.as_ref()
            .and_then(|rl| serde_json::from_str(rl).ok())
    }

    /// Set rate limits configuration
    pub fn set_rate_limits(&mut self, limits: serde_json::Value) {
        self.rate_limits = Some(limits.to_string());
        self.update_config();
    }
}
