use async_trait::async_trait;
use tower_sessions::{
    SessionManagerLayer, SessionStore,
    cookie::time::{Duration, OffsetDateTime},
    session::{Id, Record},
    session_store,
};

use crate::common::{
    models::session::Session,
    types::{EntityType, Partition},
};

pub fn get_session_layer(
    cli: &aws_sdk_dynamodb::Client,
    env: String,
) -> SessionManagerLayer<DynamoSessionStore> {
    let session_store = DynamoSessionStore::new(cli);

    // Always issue cross-site-capable cookies (`SameSite=None; Secure`).
    //
    // Why this is safe in local/test as well: the Tauri Android smoke
    // test runs the WebView at `http://tauri.localhost` and the
    // backend at `http://localhost:8080`. Those are different "sites"
    // for cookie purposes (no public-suffix entry for `.localhost`), so
    // a `SameSite=Lax` cookie would never reach the backend on the
    // subsequent XHRs after signup. Chromium treats every `*.localhost`
    // host (and bare `localhost`) as a secure context, so the `Secure`
    // attribute does not block the cookie over HTTP loopback either.
    //
    // For the same-origin web Playwright job (browser + backend both at
    // `http://localhost:8080`), `SameSite=None; Secure` is strictly
    // looser than `Lax`, so it remains compatible.
    let layer = SessionManagerLayer::new(session_store)
        .with_secure(true)
        .with_http_only(true)
        .with_same_site(tower_sessions::cookie::SameSite::None)
        .with_name(format!("{}_sid", env))
        .with_path("/")
        .with_expiry(tower_sessions::Expiry::AtDateTime(
            OffsetDateTime::now_utc()
                .checked_add(Duration::days(30))
                .unwrap(),
        ));

    layer
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
