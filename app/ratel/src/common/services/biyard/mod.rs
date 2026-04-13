use crate::common::*;

#[cfg(feature = "server")]
use super::ServiceError;
#[cfg(feature = "server")]
use reqwest;
#[cfg(feature = "server")]
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TokenResponse {
    pub name: String,
    pub symbol: String,
    #[serde(default)]
    pub contract_address: Option<String>,
    #[serde(default)]
    pub chain_id: Option<u64>,
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

#[derive(Debug, Clone, Serialize)]
struct TransactPointsBody {
    transactions: Vec<TransactPointRequest>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MonthlySummaryItem {
    pub month: String,
    pub total_earned: i64,
    pub total_spent: i64,
    pub balance: i64,
    pub project_total_points: i64,
    pub monthly_token_supply: i64,
    pub exchanged: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MonthlySummariesResponse {
    pub months: Vec<MonthlySummaryItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ClaimSignatureRequest {
    pub meta_user_id: String,
    pub month: String,
    pub wallet_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ClaimSignatureResponse {
    pub month_index: String,
    pub amount: String,
    pub max_claimable: String,
    pub nonce: String,
    pub deadline: String,
    pub signature: String,
    pub contract_address: String,
    pub chain_id: u64,
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
            "{}/v1/projects/{}/points/{}?month={}",
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
            "{}/v1/projects/{}/points/{}/transactions?month={}",
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
        let body = TransactPointsBody {
            transactions: vec![TransactPointRequest {
                tx_type: "Award".to_string(),
                to: Some(Self::convert_user_id(&target_pk)?),
                from: None,
                amount: points,
                description: Some(description),
                month,
            }],
        };

        let res = self.cli.post(&path).json(&body).send().await.map_err(|e| {
            crate::error!("Biyard API: {e}");
            ServiceError::BiyardApiRequestFailed
        })?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            crate::error!("Biyard API bad status: {status} {text}");
            return Err(Error::from(ServiceError::BiyardApiBadStatus));
        }

        let responses: Vec<TransactPointResponse> = res.json().await.map_err(|e| {
            crate::error!("Biyard API: {e}");
            ServiceError::BiyardApiRequestFailed
        })?;
        responses
            .into_iter()
            .next()
            .ok_or_else(|| Error::from(ServiceError::BiyardApiEmptyResponse))
    }

    pub async fn exchange_points(
        &self,
        user_pk: Partition,
        amount: i64,
        month: String,
    ) -> Result<ExchangePointResponse> {
        let path = format!("{}/v1/projects/{}/points", self.base_url, self.project_id);
        let body = TransactPointsBody {
            transactions: vec![TransactPointRequest {
                tx_type: "Exchange".to_string(),
                to: None,
                from: Some(Self::convert_user_id(&user_pk)?),
                amount,
                description: Some("Point-to-Token Exchange".to_string()),
                month: Some(month),
            }],
        };

        let res = self.cli.post(&path).json(&body).send().await.map_err(|e| {
            crate::error!("Biyard API: {e}");
            ServiceError::BiyardApiRequestFailed
        })?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            crate::error!("Biyard API bad status: {status} {text}");
            return Err(Error::from(ServiceError::BiyardApiBadStatus));
        }

        let responses: Vec<TransactPointResponse> = res.json().await.map_err(|e| {
            crate::error!("Biyard API: {e}");
            ServiceError::BiyardApiRequestFailed
        })?;
        responses
            .into_iter()
            .next()
            .ok_or_else(|| Error::from(ServiceError::BiyardApiEmptyResponse))
    }

    pub async fn get_monthly_summaries(
        &self,
        user_pk: Partition,
    ) -> Result<MonthlySummariesResponse> {
        let user_id = Self::convert_user_id(&user_pk)?;
        let path = format!(
            "{}/v1/projects/{}/points/{}/monthly-summaries",
            self.base_url, self.project_id, user_id
        );
        self.get_json(path).await
    }

    pub async fn request_claim_signature(
        &self,
        user_pk: Partition,
        month: String,
        wallet_address: String,
    ) -> Result<ClaimSignatureResponse> {
        let user_id = Self::convert_user_id(&user_pk)?;
        let path = format!(
            "{}/v1/projects/{}/tokens/claim-signature",
            self.base_url, self.project_id
        );
        let body = ClaimSignatureRequest {
            meta_user_id: user_id,
            month,
            wallet_address,
        };

        let res = self.cli.post(&path).json(&body).send().await.map_err(|e| {
            crate::error!("Biyard API: {e}");
            ServiceError::BiyardApiRequestFailed
        })?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            crate::error!("Biyard API bad status: {status} {text}");
            return Err(Error::from(ServiceError::BiyardApiBadStatus));
        }

        res.json().await.map_err(|e| {
            crate::error!("Biyard API: {e}");
            Error::from(ServiceError::BiyardApiRequestFailed)
        })
    }

    pub async fn get_token_balance(&self, user_pk: Partition) -> Result<TokenBalanceResponse> {
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

        let res = self.cli.post(&path).json(&body).send().await.map_err(|e| {
            crate::error!("Biyard API: {e}");
            ServiceError::BiyardApiRequestFailed
        })?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            crate::error!("Biyard API bad status: {status} {text}");
            return Err(Error::from(ServiceError::BiyardApiBadStatus));
        }

        res.json().await.map_err(|e| {
            crate::error!("Biyard API: {e}");
            Error::from(ServiceError::BiyardApiRequestFailed)
        })
    }

    fn convert_user_id(user_pk: &Partition) -> Result<String> {
        match user_pk {
            Partition::User(id) => Ok(id.clone()),
            Partition::Team(id) => Ok(id.clone()),
            _ => Err(Error::from(ServiceError::BiyardApiBadStatus)),
        }
    }

    async fn get_json<T: DeserializeOwned>(&self, path: String) -> Result<T> {
        let res = self.cli.get(&path).send().await.map_err(|e| {
            crate::error!("Biyard API: {e}");
            ServiceError::BiyardApiRequestFailed
        })?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            crate::error!("Biyard API bad status: {status} {text}");
            return Err(Error::from(ServiceError::BiyardApiBadStatus));
        }

        res.json().await.map_err(|e| {
            crate::error!("Biyard API: {e}");
            Error::from(ServiceError::BiyardApiRequestFailed)
        })
    }
}
