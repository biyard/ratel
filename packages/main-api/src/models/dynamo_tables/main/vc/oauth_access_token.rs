use bdk::prelude::*;
use crate::types::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct OAuthAccessToken {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,
    pub expires_at: i64,

    #[dynamo(prefix = "USER", name = "find_by_user", index = "gsi1", pk)]
    pub user_id: String,
    
    #[dynamo(prefix = "CLIENT", name = "find_by_client", index = "gsi2", pk)]
    pub client_id: String,
    #[dynamo(index = "gsi2", sk)]
    pub c_nonce: String,

    pub scope: String,
    pub token_type: String,
    pub c_nonce_expires_at: i64,
}

impl OAuthAccessToken {
    pub fn new(
        access_token: String,
        user_id: String,
        client_id: String,
        scope: String,
        token_type: String,
        expires_in: i64,
        c_nonce_expires_in: i64,
    ) -> Self {
        let pk = Partition::OAuthToken(access_token);
        let sk = EntityType::OAuthAccessToken;
        let now = chrono::Utc::now().timestamp_micros();
        let c_nonce = uuid::Uuid::new_v4().to_string();

        Self {
            pk,
            sk,
            created_at: now,
            expires_at: now + (expires_in * 1_000_000),
            user_id,
            client_id,
            scope,
            token_type,
            c_nonce,
            c_nonce_expires_at: now + (c_nonce_expires_in * 1_000_000),
        }
    }
}
