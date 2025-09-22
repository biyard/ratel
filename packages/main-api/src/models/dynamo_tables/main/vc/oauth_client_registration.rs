use bdk::prelude::*;
use crate::types::*;

/// OAuth Client Registration Entity
/// 
/// Manages registered OAuth clients for OpenID4VCI flows.
/// Stores client metadata, credentials, and configuration.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct OAuthClientRegistration {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    #[dynamo(prefix = "OWNER", name = "find_by_owner", index = "gsi1", pk)]
    pub owner_id: String,
    
    #[dynamo(prefix = "STATUS", name = "find_by_status", index = "gsi2", pk)]
    pub status: String,
    #[dynamo(prefix = "TYPE", index = "gsi2", sk)]
    pub client_type: String,

    /// Client name for display purposes
    pub client_name: String,
    /// Optional client secret for confidential clients
    pub client_secret: Option<String>,
    /// Allowed redirect URIs
    pub redirect_uris: String, // JSON array as string
    /// Allowed grant types
    pub grant_types: String, // JSON array as string
    /// Allowed response types
    pub response_types: String, // JSON array as string
    /// Allowed scopes
    pub scope: String,
    /// Token endpoint authentication method
    pub token_endpoint_auth_method: String,
    /// Client URI for more information
    pub client_uri: Option<String>,
    /// Logo URI for client branding
    pub logo_uri: Option<String>,
    /// Terms of service URI
    pub tos_uri: Option<String>,
    /// Privacy policy URI
    pub policy_uri: Option<String>,
    /// JWKS URI for public key verification
    pub jwks_uri: Option<String>,
    /// Whether the client is active
    pub is_active: bool,
}

impl OAuthClientRegistration {
    pub fn new(
        client_id: String,
        owner_id: String,
        client_name: String,
        client_type: String,
        redirect_uris: Vec<String>,
        grant_types: Vec<String>,
        response_types: Vec<String>,
        scope: String,
        token_endpoint_auth_method: String,
    ) -> Self {
        let pk = Partition::OAuthClient(client_id);
        let sk = EntityType::OAuthClientRegistration;
        let now = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            owner_id,
            status: "active".to_string(),
            client_type,
            client_name,
            client_secret: None,
            redirect_uris: serde_json::to_string(&redirect_uris).unwrap_or_default(),
            grant_types: serde_json::to_string(&grant_types).unwrap_or_default(),
            response_types: serde_json::to_string(&response_types).unwrap_or_default(),
            scope,
            token_endpoint_auth_method,
            client_uri: None,
            logo_uri: None,
            tos_uri: None,
            policy_uri: None,
            jwks_uri: None,
            is_active: true,
        }
    }

    /// Generate and set a client secret for confidential clients
    pub fn set_client_secret(&mut self, secret: String) {
        self.client_secret = Some(secret);
        self.updated_at = chrono::Utc::now().timestamp_micros();
    }

    /// Deactivate the client
    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.status = "inactive".to_string();
        self.updated_at = chrono::Utc::now().timestamp_micros();
    }

    /// Parse redirect URIs from JSON string
    pub fn get_redirect_uris(&self) -> Vec<String> {
        serde_json::from_str(&self.redirect_uris).unwrap_or_default()
    }

    /// Parse grant types from JSON string
    pub fn get_grant_types(&self) -> Vec<String> {
        serde_json::from_str(&self.grant_types).unwrap_or_default()
    }

    /// Parse response types from JSON string
    pub fn get_response_types(&self) -> Vec<String> {
        serde_json::from_str(&self.response_types).unwrap_or_default()
    }
}
