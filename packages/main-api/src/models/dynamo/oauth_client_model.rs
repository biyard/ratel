use crate::types::dynamo_entity_type::EntityType;
use dto::AuthClient;

use super::base_model::{BaseModel, DynamoModel};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DynamoAuthClient {
    #[serde(flatten)]
    pub base: BaseModel,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uris: Vec<String>,
    pub scopes: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl DynamoAuthClient {
    pub fn from_postgres_client(client: &AuthClient) -> Self {
        Self {
            base: BaseModel {
                pk: format!("AUTH_CLIENT#{}", client.client_id),
                sk: format!("METADATA"),
                entity_type: EntityType::Metadata, // using metadata for auth clients
                created_at: client.created_at,
                updated_at: client.updated_at,
                gsi1_pk: Some(format!("AUTH_CLIENT#{}", client.client_id)),
                gsi1_sk: Some(format!("CREATED#{}", client.created_at)),
                gsi2_pk: None,
                gsi2_sk: None,
                ..Default::default()
            },
            client_id: client.client_id.clone(),
            client_secret: client.client_secret.clone(),
            redirect_uris: client.redirect_uris.clone(),
            scopes: client.scopes.clone(),
            created_at: client.created_at,
            updated_at: client.updated_at,
        }
    }
}

impl DynamoModel for DynamoAuthClient {
    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}
