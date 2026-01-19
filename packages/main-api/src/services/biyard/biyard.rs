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

    fn convert_to_meta_user_id(target_pk: &Partition) -> String {
        match target_pk {
            Partition::User(id) => id.clone(),
            Partition::Team(id) => id.clone(),
            _ => panic!("Biyard target_pk must be of Partition::User or Partition::Team type"),
        }
    }

    async fn fetch_token_from_api(&self) -> Result<TokenResponse> {
        let path = format!("{}/v1/projects/{}/tokens", self.base_url, self.project_id);

        let res = self.cli.get(&path).send().await?;

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await.unwrap_or_default();
            return Err(BiyardError::from_status(status, error_text).into());
        }

        let token: TokenResponse = res.json().await.map_err(BiyardError::parse_error)?;
        Ok(token)
    }

    pub async fn get_project_info(&self) -> Result<TokenResponse> {
        {
            let cache = self.token_cache.read().await;
            if let Some(cached) = cache.as_ref() {
                if cached.cached_at.elapsed() < TOKEN_CACHE_TTL {
                    return Ok(cached.token.clone());
                }
            }
        }

        let token = self.fetch_token_from_api().await?;

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
        target_pk: Partition,
        points: i64,
        description: String,
        month: Option<String>,
    ) -> Result<AwardPointResponse> {
        let path = format!("{}/v1/projects/{}/points", self.base_url, self.project_id);
        let body = vec![TransactPointRequest {
            tx_type: "Award".to_string(),
            to: Some(Self::convert_to_meta_user_id(&target_pk)),
            from: None,
            amount: points,
            description: Some(description),
            month,
        }];

        let res = self.cli.post(&path).json(&body).send().await?;

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await.unwrap_or_default();
            return Err(BiyardError::from_status(status, error_text).into());
        }

        let responses: Vec<TransactPointResponse> =
            res.json().await.map_err(BiyardError::parse_error)?;
        responses
            .into_iter()
            .next()
            .ok_or_else(|| BiyardError::EmptyResponse.into())
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
            return Err(BiyardError::from_status(status, error_text).into());
        }

        let response: UserPointBalanceResponse =
            res.json().await.map_err(BiyardError::parse_error)?;
        Ok(response)
    }

    pub async fn list_user_transactions(
        &self,
        user_pk: Partition,
        month: String,
        bookmark: Option<String>,
        limit: Option<i32>,
    ) -> Result<ListItemsResponse<UserPointTransactionResponse>> {
        let meta_user_id = Self::convert_to_meta_user_id(&user_pk);
        let mut path = format!(
            "{}/v1/projects/{}/points/{}/transactions?date={}",
            self.base_url, self.project_id, meta_user_id, month
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
            return Err(BiyardError::from_status(status, error_text).into());
        }

        let transactions = res.json().await.map_err(BiyardError::parse_error)?;
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
            return Err(BiyardError::from_status(status, error_text).into());
        }

        let responses: Vec<TransactPointResponse> =
            res.json().await.map_err(BiyardError::parse_error)?;
        responses
            .into_iter()
            .next()
            .ok_or_else(|| BiyardError::EmptyResponse.into())
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
            return Err(BiyardError::from_status(status, error_text).into());
        }

        let balance: TokenBalanceResponse = res.json().await.map_err(BiyardError::parse_error)?;
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
            return Err(BiyardError::from_status(status, error_text).into());
        }

        let balance: TokenBalanceResponse = res.json().await.map_err(BiyardError::parse_error)?;
        Ok(balance)
    }

    pub async fn list_transactions(
        &self,
        date: Option<String>,
        bookmark: Option<String>,
        limit: Option<i32>,
    ) -> Result<ListItemsResponse<ProjectPointTransactionResponse>> {
        let mut path = format!(
            "{}/v1/projects/{}/points/transactions",
            self.base_url, self.project_id
        );

        let mut query_params = vec![];
        if let Some(d) = date {
            query_params.push(format!("date={}", d));
        }
        if let Some(bm) = bookmark {
            query_params.push(format!("bookmark={}", bm));
        }
        if let Some(lim) = limit {
            query_params.push(format!("limit={}", lim));
        }
        if !query_params.is_empty() {
            path = format!("{}?{}", path, query_params.join("&"));
        }

        let res = self.cli.get(&path).send().await?;
        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await.unwrap_or_default();
            return Err(BiyardError::from_status(status, error_text).into());
        }

        let transactions: ListItemsResponse<ProjectPointTransactionResponse> =
            res.json().await.map_err(BiyardError::parse_error)?;
        Ok(transactions)
    }
}
