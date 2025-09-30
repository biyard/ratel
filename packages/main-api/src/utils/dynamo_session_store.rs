use async_trait::async_trait;
use tower_sessions::{
    SessionStore,
    session::{Id, Record},
    session_store,
};

use crate::{
    models::session::Session,
    types::{EntityType, Partition},
};

#[derive(Debug, Clone)]
pub struct DynamoSessionStore {
    pub client: aws_sdk_dynamodb::Client,
}
impl DynamoSessionStore {
    pub fn new(client: aws_sdk_dynamodb::Client) -> Self {
        Self { client }
    }
}
#[async_trait]
impl SessionStore for DynamoSessionStore {
    async fn create(&self, session_record: &mut Record) -> session_store::Result<()> {
        let expired_at = session_record.expiry_date.unix_timestamp();
        let data = serde_json::to_string(session_record).map_err(|e| {
            session_store::Error::Encode(format!("Failed to serialize session data: {}", e))
        })?;
        Session::new(session_record.id.to_string(), expired_at, data)
            .create(&self.client)
            .await
            .map_err(|e| session_store::Error::Backend(format!("Failed to create entry: {}", e)))?;

        Ok(())
    }

    async fn save(&self, session_record: &Record) -> session_store::Result<()> {
        let pk = session_record.id.to_string();
        Session::updater(Partition::Session(pk), &EntityType::Session)
            .with_data(serde_json::to_string(session_record).map_err(|e| {
                session_store::Error::Encode(format!("Failed to serialize session data: {}", e))
            })?)
            .with_expired_at(session_record.expiry_date.unix_timestamp())
            .execute(&self.client)
            .await
            .map_err(|e| session_store::Error::Backend(format!("Failed to save entry: {}", e)))?;
        Ok(())
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        let pk = session_id.to_string();
        let session = Session::get(
            &self.client,
            Partition::Session(pk),
            Some(EntityType::Session),
        )
        .await
        .map_err(|e| session_store::Error::Backend(format!("Failed to load entry: {}", e)))?;
        match session {
            None => Ok(None),
            Some(s) => {
                let record: Record = serde_json::from_str(&s.data).map_err(|e| {
                    session_store::Error::Decode(format!(
                        "Failed to deserialize session data: {}",
                        e
                    ))
                })?;
                Ok(Some(record))
            }
        }
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        tracing::debug!("Deleting session with ID: {}", session_id);
        let pk = session_id.to_string();
        Session::delete(
            &self.client,
            Partition::Session(pk),
            Some(EntityType::Session),
        )
        .await
        .map_err(|e| session_store::Error::Backend(format!("Failed to delete entry: {}", e)))?;
        Ok(())
    }
}
