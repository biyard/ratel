use bdk::prelude::*;
use crate::types::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct OAuthAuthorizationCode {
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
    pub code_challenge: Option<String>,

    pub scope: String,
    pub redirect_uri: String,
    pub code_challenge_method: Option<String>,
    pub nonce: Option<String>,
    pub used: bool,
}

impl OAuthAuthorizationCode {
    pub fn new(
        code: String,
        user_id: String,
        client_id: String,
        scope: String,
        redirect_uri: String,
        code_challenge: Option<String>,
        code_challenge_method: Option<String>,
        nonce: Option<String>,
        expires_in: i64,
    ) -> Self {
        let pk = Partition::OAuthToken(code);
        let sk = EntityType::OAuthAuthorizationCode;
        let now = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk,
            created_at: now,
            expires_at: now + (expires_in * 1_000_000),
            user_id,
            client_id,
            scope,
            redirect_uri,
            code_challenge,
            code_challenge_method,
            nonce,
            used: false,
        }
    }
}
