use std::sync::Arc;
use std::time::{Duration, Instant};

use super::*;
use crate::*;
use tokio::sync::RwLock;

const TOKEN_CACHE_TTL: Duration = Duration::from_secs(3600); // 1 hour

#[derive(Debug)]
struct CachedToken {
    token: TokenResponse,
    cached_at: Instant,
}

#[derive(Debug, Clone)]
pub struct Biyard {
    project_id: String,
    base_url: String,
    cli: reqwest::Client,
    token_cache: Arc<RwLock<Option<CachedToken>>>,
}

impl Biyard {
    pub fn new() -> Self {
        let biyard_conf = config::get().biyard;
        let cli = reqwest::Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    "Authorization",
                    reqwest::header::HeaderValue::from_str(&format!(
                        "Bearer {}",
                        biyard_conf.api_secret
                    ))
                    .unwrap(),
                );
                headers
            })
            .build()
            .unwrap();

        let base_url = biyard_conf.base_url.to_string();
        let project_id = biyard_conf.project_id.to_string();

        Self {
            cli,
            base_url,
            project_id,
            token_cache: Arc::new(RwLock::new(None)),
        }
    }

    fn convert_to_meta_user_id(user_pk: &Partition) -> String {
        match user_pk {
            Partition::User(id) => id.clone(),
            _ => panic!("Biyard user_pk must be of Partition::User type"),
        }
    }

    async fn fetch_token_from_api(&self) -> Result<TokenResponse> {
        let path = format!(
            "{}/v1/projects/{}/tokens",
            self.base_url, self.project_id
        );

        let res = self.cli.get(&path).send().await?;

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await.unwrap_or_default();
            return Err(Error::Unknown(format!(
                "Biyard API error: {} - {}",
                status, error_text
            )));
        }

        let token: TokenResponse = res.json().await?;
        Ok(token)
    }

    /// Get token info with TTL-based caching (refreshes after 1 hour)
    pub async fn get_token(&self) -> Result<TokenResponse> {
        // Check if cache is valid
        {
            let cache = self.token_cache.read().await;
            if let Some(cached) = cache.as_ref() {
                if cached.cached_at.elapsed() < TOKEN_CACHE_TTL {
                    return Ok(cached.token.clone());
                }
            }
        }

        // Cache miss or expired - fetch new token
        let token = self.fetch_token_from_api().await?;

        // Update cache
        {
            let mut cache = self.token_cache.write().await;
            *cache = Some(CachedToken {
                token: token.clone(),
                cached_at: Instant::now(),
            });
        }

        Ok(token)
    }

    pub async fn award_points(
        &self,
        user_pk: Partition,
        points: i64,
        description: String,
        month: Option<String>,
    ) -> Result<AwardPointResponse> {
        let path = format!("{}/v1/projects/{}/points", self.base_url, self.project_id);
        let body = vec![TransactPointRequest {
            tx_type: "Award".to_string(),
            to: Some(Self::convert_to_meta_user_id(&user_pk)),
            from: None,
            amount: points,
            description: Some(description),
            month,
        }];

        let res = self.cli.post(&path).json(&body).send().await?;

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await.unwrap_or_default();
            return Err(Error::Unknown(format!(
                "Biyard API error: {} - {}",
                status, error_text
            )));
        }

        let responses: Vec<TransactPointResponse> = res.json().await?;
        responses
            .into_iter()
            .next()
            .ok_or_else(|| Error::Unknown("Biyard API returned empty response".to_string()))
    }

    pub async fn get_user_balance(
        &self,
        user_pk: Partition,
        month: String,
    ) -> Result<UserPointBalanceResponse> {
        let path = format!(
            "{}/v1/projects/{}/points/{}?date={}",
            self.base_url,
            self.project_id,
            Self::convert_to_meta_user_id(&user_pk),
            month
        );

        let res = self.cli.get(&path).send().await?;

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await.unwrap_or_default();
            return Err(Error::Unknown(format!(
                "Biyard API error: {} - {}",
                status, error_text
            )));
        }

        let list_response: ListItemsResponse<UserPointBalanceResponse> = res.json().await?;
        list_response
            .items
            .into_iter()
            .next()
            .ok_or_else(|| Error::Unknown("No balance found for the specified month".to_string()))
    }

    pub async fn get_user_transactions(
        &self,
        user_pk: Partition,
        month: String,
        bookmark: Option<String>,
        limit: Option<i32>,
    ) -> Result<ListItemsResponse<UserPointTransactionResponse>> {
        let mut path = format!(
            "{}/v1/projects/{}/points/{}/transactions?month={}",
            self.base_url,
            self.project_id,
            Self::convert_to_meta_user_id(&user_pk),
            month
        );
        if let Some(bm) = bookmark {
            path = format!("{}&bookmark={}", path, bm);
        }
        if let Some(lim) = limit {
            path = format!("{}&limit={}", path, lim);
        }

        let res = self.cli.get(&path).send().await?;

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await.unwrap_or_default();
            return Err(Error::Unknown(format!(
                "Biyard API error: {} - {}",
                status, error_text
            )));
        }

        let transactions = res.json().await?;
        Ok(transactions)
    }

    pub async fn exchange_points(
        &self,
        user_pk: Partition,
        amount: i64,
        month: String,
    ) -> Result<TransactPointResponse> {
        let path = format!("{}/v1/projects/{}/points", self.base_url, self.project_id);
        let body = vec![TransactPointRequest {
            tx_type: "Exchange".to_string(),
            to: None,
            from: Some(Self::convert_to_meta_user_id(&user_pk)),
            amount,
            description: Some("Exchange points to tokens".to_string()),
            month: Some(month),
        }];

        let res = self.cli.post(&path).json(&body).send().await?;

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await.unwrap_or_default();
            return Err(Error::Unknown(format!(
                "Biyard API error: {} - {}",
                status, error_text
            )));
        }

        let responses: Vec<TransactPointResponse> = res.json().await?;
        responses
            .into_iter()
            .next()
            .ok_or_else(|| Error::Unknown("Biyard API returned empty response".to_string()))
    }

    pub async fn get_token_balance(&self, user_pk: Partition) -> Result<TokenBalanceResponse> {
        let path = format!(
            "{}/v1/projects/{}/tokens/balances/{}",
            self.base_url,
            self.project_id,
            Self::convert_to_meta_user_id(&user_pk)
        );

        let res = self.cli.get(&path).send().await?;

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await.unwrap_or_default();
            return Err(Error::Unknown(format!(
                "Biyard API error: {} - {}",
                status, error_text
            )));
        }

        let balance: TokenBalanceResponse = res.json().await?;
        Ok(balance)
    }

    pub async fn mint_tokens(
        &self,
        user_pk: Partition,
        amount: i64,
    ) -> Result<TokenBalanceResponse> {
        let path = format!(
            "{}/v1/projects/{}/tokens/mint/{}",
            self.base_url,
            self.project_id,
            Self::convert_to_meta_user_id(&user_pk)
        );

        let body = MintTokenRequest { amount };
        let res = self.cli.post(&path).json(&body).send().await?;

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await.unwrap_or_default();
            return Err(Error::Unknown(format!(
                "Biyard API error: {} - {}",
                status, error_text
            )));
        }

        let balance: TokenBalanceResponse = res.json().await?;
        Ok(balance)
    }
}
