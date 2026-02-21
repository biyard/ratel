use async_trait::async_trait;
use tower_sessions::{
    SessionManagerLayer, SessionStore,
    cookie::time::{Duration, OffsetDateTime},
    session::{Id, Record},
    session_store,
};

use crate::{
    models::session::Session,
    types::{EntityType, Partition},
    utils::aws::dynamo::DynamoClient,
};

pub fn get_session_layer(
    cli: &aws_sdk_dynamodb::Client,
    env: String,
) -> SessionManagerLayer<DynamoSessionStore> {
    let session_store = DynamoSessionStore::new(cli);

    let is_local = env == "local" || env == "test";

    SessionManagerLayer::new(session_store)
        .with_secure(!is_local)
        .with_http_only(!is_local)
        .with_same_site(if is_local {
            tower_sessions::cookie::SameSite::Lax
        } else {
            tower_sessions::cookie::SameSite::None
        })
        .with_name(format!("{}_sid", env))
        .with_path("/")
        .with_expiry(tower_sessions::Expiry::AtDateTime(
            OffsetDateTime::now_utc()
                .checked_add(Duration::days(30))
                .unwrap(),
        ))
}

#[derive(Debug, Clone)]
pub struct DynamoSessionStore {
    pub client: aws_sdk_dynamodb::Client,
}
impl DynamoSessionStore {
    pub fn new(client: &aws_sdk_dynamodb::Client) -> Self {
        Self {
            client: client.clone(),
        }
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
