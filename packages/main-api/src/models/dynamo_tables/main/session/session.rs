use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[dynamo(table = "session", pk_name = "id", sk_name = "none")]
pub struct Session {
    pub id: String,

    pub created_at: i64,
    pub updated_at: i64,

    pub data: String,
    pub expired_at: i64,
}

impl Session {
    pub fn new(id: String, expired_at: i64, data: String) -> Self {
        let now = chrono::Utc::now().timestamp_micros();

        Self {
            id,
            created_at: now,
            updated_at: now,
            data,
            expired_at,
        }
    }
}
