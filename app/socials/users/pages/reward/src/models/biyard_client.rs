use crate::*;

#[cfg(feature = "server")]
use common::reqwest;
#[cfg(feature = "server")]
use common::serde::de::DeserializeOwned;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TokenResponse {
    pub name: String,
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserPointBalanceResponse {
    pub month: String,
    pub balance: i64,
    pub total_earned: i64,
    pub total_spent: i64,
    pub updated_at: i64,
    #[serde(default)]
    pub project_total_points: i64,
    #[serde(default)]
    pub monthly_token_supply: i64,
}

#[cfg(feature = "server")]
#[derive(Debug, Clone, Copy)]
struct BiyardConfig {
    api_secret: &'static str,
    project_id: &'static str,
    base_url: &'static str,
}

#[cfg(feature = "server")]
impl Default for BiyardConfig {
    fn default() -> Self {
        Self {
            api_secret: option_env!("BIYARD_API_KEY").unwrap_or_else(|| {
                tracing::warn!(
                    "BIYARD_API_KEY not set, using default value. Some features may not work properly."
                );
                "biyard_default_api_key"
            }),
            project_id: option_env!("BIYARD_PROJECT_ID").unwrap_or_else(|| {
                tracing::warn!(
                    "BIYARD_PROJECT_ID not set, using default value. Some features may not work properly."
                );
                "ratel_project_id"
            }),
            base_url: option_env!("BIYARD_API_URL").unwrap_or_else(|| {
                tracing::warn!(
                    "BIYARD_API_URL not set, using default value. Some features may not work properly."
                );
                "https://dev.biyard.co"
            }),
        }
    }
}

#[cfg(feature = "server")]
#[derive(Debug, Clone)]
pub struct BiyardClient {
    project_id: String,
    base_url: String,
    cli: reqwest::Client,
}

#[cfg(feature = "server")]
impl BiyardClient {
    pub fn new() -> Self {
        let config = BiyardConfig::default();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Authorization",
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", config.api_secret))
                .unwrap(),
        );

        let cli = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            cli,
            base_url: config.base_url.to_string(),
            project_id: config.project_id.to_string(),
        }
    }

    pub async fn get_project_info(&self) -> Result<TokenResponse> {
        let path = format!("{}/v1/projects/{}/tokens", self.base_url, self.project_id);
        self.get_json(path).await
    }

    pub async fn get_user_balance(
        &self,
        user_pk: Partition,
        month: String,
    ) -> Result<UserPointBalanceResponse> {
        let user_id = Self::convert_user_id(&user_pk)?;
        let path = format!(
            "{}/v1/projects/{}/points/{}?date={}",
            self.base_url, self.project_id, user_id, month
        );
        self.get_json(path).await
    }

    pub async fn list_user_transactions(
        &self,
        user_pk: Partition,
        month: String,
        bookmark: Option<String>,
        limit: Option<i32>,
    ) -> Result<ListResponse<crate::dto::PointTransactionResponse>> {
        let user_id = Self::convert_user_id(&user_pk)?;
        let mut path = format!(
            "{}/v1/projects/{}/points/{}/transactions?date={}",
            self.base_url, self.project_id, user_id, month
        );
        if let Some(bookmark) = bookmark {
            path = format!("{}&bookmark={}", path, bookmark);
        }
        if let Some(limit) = limit {
            path = format!("{}&limit={}", path, limit);
        }

        self.get_json(path).await
    }

    fn convert_user_id(user_pk: &Partition) -> Result<String> {
        match user_pk {
            Partition::User(id) => Ok(id.clone()),
            Partition::Team(id) => Ok(id.clone()),
            _ => Err(Error::BadRequest(
                "Biyard target pk must be user or team".to_string(),
            )),
        }
    }

    async fn get_json<T: DeserializeOwned>(&self, path: String) -> Result<T> {
        let res = self
            .cli
            .get(&path)
            .send()
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(Error::BadRequest(format!(
                "Biyard API error ({}): {}",
                status, text
            )));
        }

        res.json().await.map_err(|e| Error::Unknown(e.to_string()))
    }
}

#[cfg(not(feature = "server"))]
#[derive(Debug, Clone)]
pub struct BiyardClient;

#[cfg(not(feature = "server"))]
impl BiyardClient {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_project_info(&self) -> Result<TokenResponse> {
        Err(Error::NotSupported(
            "Biyard client is server-only".to_string(),
        ))
    }

    pub async fn get_user_balance(
        &self,
        _user_pk: Partition,
        _month: String,
    ) -> Result<UserPointBalanceResponse> {
        Err(Error::NotSupported(
            "Biyard client is server-only".to_string(),
        ))
    }

    pub async fn list_user_transactions(
        &self,
        _user_pk: Partition,
        _month: String,
        _bookmark: Option<String>,
        _limit: Option<i32>,
    ) -> Result<ListResponse<crate::dto::PointTransactionResponse>> {
        Err(Error::NotSupported(
            "Biyard client is server-only".to_string(),
        ))
    }
}
