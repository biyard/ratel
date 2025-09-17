use bdk::prelude::*;
use crate::types::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct IssuedCredential {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    #[dynamo(prefix = "HOLDER", name = "find_by_holder", index = "gsi1", pk)]
    pub holder_id: String,
    
    #[dynamo(prefix = "TYPE", name = "find_by_type", index = "gsi2", pk)]
    pub credential_type: String,
    #[dynamo(prefix = "STATUS", index = "gsi2", sk)]
    pub status: String,

    pub credential_jwt: String,
    pub credential_subject: String,
    pub expires_at: Option<i64>,
    pub status_list_index: i64,
    pub revoked_at: Option<i64>,
    pub suspended_at: Option<i64>,
}

impl IssuedCredential {
    pub fn new(
        credential_id: String,
        holder_id: String,
        credential_type: String,
        credential_jwt: String,
        credential_subject: String,
        expires_at: Option<i64>,
        status_list_index: i64,
    ) -> Self {
        let pk = Partition::Credential(credential_id);
        let sk = EntityType::IssuedCredential;
        let now = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            holder_id,
            credential_type,
            status: "valid".to_string(),
            credential_jwt,
            credential_subject,
            expires_at,
            status_list_index,
            revoked_at: None,
            suspended_at: None,
        }
    }
}
