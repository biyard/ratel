use dto::Result;
use dto::ServiceError;
use easy_dynamodb::Document;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub r#type: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub deleted_at: Option<u64>,

    pub nickname: String,
    pub email: String,
    pub profile_url: String,
}

impl Document for User {
    fn document_type() -> String {
        "user".to_string()
    }
}

impl User {
    pub fn new(
        wallet_address: String,
        nickname: String,
        email: String,
        profile_url: String,
    ) -> Self {
        let now = chrono::Utc::now().timestamp() as u64;

        Self {
            id: wallet_address,
            r#type: User::document_type(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            nickname,
            email,
            profile_url,
        }
    }
}

impl Into<dto::User> for User {
    fn into(self) -> dto::User {
        dto::User {
            id: self.id,
            created_at: self.created_at,
            updated_at: self.updated_at,

            nickname: self.nickname,
            email: self.email,
            profile_url: self.profile_url,
        }
    }
}

impl User {
    pub async fn create(&self, log: &slog::Logger) -> Result<()> {
        let cli = easy_dynamodb::get_client(log);
        cli.create(self).await.map_err(|e| ServiceError::from(e))
    }

    pub async fn get(log: &slog::Logger, id: &str) -> Result<Option<User>> {
        let cli = easy_dynamodb::get_client(log);
        cli.get::<User>(id)
            .await
            .map_err(|e| ServiceError::DatabaseException(format!("{e}")))
    }
}
