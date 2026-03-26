use crate::common::*;

#[cfg(feature = "server")]
use reqwest;
#[cfg(feature = "server")]
use serde::de::DeserializeOwned;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactPointRequest {
    pub tx_type: String,
    pub to: Option<String>,
    pub from: Option<String>,
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactPointResponse {
    pub transaction_id: String,
    pub month: String,
    pub meta_user_id: String,
    pub transaction_type: String,
    pub amount: i64,
}

pub type AwardPointResponse = TransactPointResponse;
pub type ExchangePointResponse = TransactPointResponse;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TokenBalanceResponse {
    pub project_id: String,
    pub meta_user_id: String,
    pub balance: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MintTokenRequest {
    pub amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PointTransactionResponse {
    pub month: String,
    pub transaction_type: String,
    pub amount: i64,
    pub target_user_id: Option<String>,
    pub description: Option<String>,
    pub created_at: i64,
}

#[cfg(feature = "server")]
#[derive(Debug, Clone)]
pub struct BiyardService {
    project_id: String,
    base_url: String,
    cli: reqwest::Client,
}

#[cfg(feature = "server")]
impl BiyardService {
    pub fn new(api_secret: String, project_id: String, base_url: String) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Authorization",
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", api_secret)).unwrap(),
        );

        let cli = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            cli,
            base_url,
            project_id,
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
    ) -> Result<ListResponse<PointTransactionResponse>> {
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

    pub async fn award_points(
        &self,
        target_pk: Partition,
        points: i64,
        description: String,
        month: Option<String>,
    ) -> Result<AwardPointResponse> {
        let path = format!("{}/v1/projects/{}/points", self.base_url, self.project_id);
        let body = vec![TransactPointRequest {
            tx_type: "Award".to_string(),
            to: Some(Self::convert_user_id(&target_pk)?),
            from: None,
            amount: points,
            description: Some(description),
            month,
        }];

        let res = self
            .cli
            .post(&path)
            .json(&body)
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

        let responses: Vec<TransactPointResponse> = res
            .json()
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?;
        responses
            .into_iter()
            .next()
            .ok_or_else(|| Error::Unknown("Biyard API returned empty response".to_string()))
    }

    pub async fn exchange_points(
        &self,
        user_pk: Partition,
        amount: i64,
        month: String,
    ) -> Result<ExchangePointResponse> {
        let path = format!("{}/v1/projects/{}/points", self.base_url, self.project_id);
        let body = vec![TransactPointRequest {
            tx_type: "Exchange".to_string(),
            to: None,
            from: Some(Self::convert_user_id(&user_pk)?),
            amount,
            description: Some("Point-to-Token Exchange".to_string()),
            month: Some(month),
        }];

        let res = self
            .cli
            .post(&path)
            .json(&body)
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

        let responses: Vec<TransactPointResponse> = res
            .json()
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?;
        responses
            .into_iter()
            .next()
            .ok_or_else(|| Error::Unknown("Biyard API returned empty response".to_string()))
    }

    pub async fn get_token_balance(
        &self,
        user_pk: Partition,
    ) -> Result<TokenBalanceResponse> {
        let user_id = Self::convert_user_id(&user_pk)?;
        let path = format!(
            "{}/v1/projects/{}/tokens/balances/{}",
            self.base_url, self.project_id, user_id
        );
        self.get_json(path).await
    }

    pub async fn mint_tokens(
        &self,
        user_pk: Partition,
        amount: i64,
    ) -> Result<TokenBalanceResponse> {
        let user_id = Self::convert_user_id(&user_pk)?;
        let path = format!(
            "{}/v1/projects/{}/tokens/mint/{}",
            self.base_url, self.project_id, user_id
        );
        let body = MintTokenRequest { amount };

        let res = self
            .cli
            .post(&path)
            .json(&body)
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
