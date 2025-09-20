use bdk::prelude::*;
use crate::types::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct CredentialOffer {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,
    pub expires_at: i64,

    #[dynamo(prefix = "USER", name = "find_by_user", index = "gsi1", pk)]
    pub user_id: String,
    
    #[dynamo(prefix = "NONCE", name = "find_by_nonce", index = "gsi2", pk)]
    #[dynamo(index = "gsi2", sk)]
    pub nonce: String,

    pub credential_types_json: String,
    pub grant_type: String,
    pub pre_authorized_code: Option<String>,
    pub tx_code: Option<String>,
    pub used: bool,
}

impl CredentialOffer {
    pub fn new(
        offer_id: String,
        user_id: String,
        credential_types: Vec<String>,
        grant_type: String,
        pre_authorized_code: Option<String>,
        tx_code: Option<String>,
        expires_in: i64,
    ) -> Self {
        let pk = Partition::CredentialOffer(offer_id);
        let sk = EntityType::CredentialOffer;
        let now = chrono::Utc::now().timestamp_micros();
        let nonce = uuid::Uuid::new_v4().to_string();
        let credential_types_json = serde_json::to_string(&credential_types).unwrap_or_default();

        Self {
            pk,
            sk,
            created_at: now,
            expires_at: now + (expires_in * 1_000_000),
            user_id,
            nonce,
            credential_types_json,
            grant_type,
            pre_authorized_code,
            tx_code,
            used: false,
        }
    }
}
