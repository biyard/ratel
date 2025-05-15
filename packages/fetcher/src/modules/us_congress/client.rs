use std::collections::HashMap;

use bdk::prelude::reqwest;

use dto::{Result, ServiceError};

use super::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct UsCongressClient {
    key: String,
    url: String,
}

impl UsCongressClient {
    pub fn new(key: String) -> Self {
        Self {
            key,
            url: "https://api.congress.gov/v3/bill".to_string(),
        }
    }

    pub async fn list_bills(&self, offset: i64, limit: i64) -> Result<Vec<BillInfo>> {
        let bills: Vec<BillInfo> = self
            .list(
                offset,
                limit,
                None,
                None,
                Some("updateDate+desc".to_string()),
                HashMap::new(),
            )
            .await?;

        Ok(bills)
    }

    pub async fn get_bill(
        &self,
        congress: i64,
        bill_type: &str,
        bill_no: i64,
    ) -> Result<BillDetail> {
        let bill_details: BillDetail = self.get(congress, bill_type, bill_no, None).await?;

        Ok(bill_details)
    }

    pub async fn get_bill_summary(
        &self,
        congress: i64,
        bill_type: &str,
        bill_no: i64,
    ) -> Result<BillSummaries> {
        let bill_summary: BillSummaries = self
            .get(congress, bill_type, bill_no, Some(SUMMARY.to_string()))
            .await?;

        Ok(bill_summary)
    }

    pub async fn get_bill_text(
        &self,
        congress: i64,
        bill_type: &str,
        bill_no: i64,
    ) -> Result<BillTexts> {
        let bill_text: BillTexts = self
            .get(congress, bill_type, bill_no, Some(TEXT.to_string()))
            .await?;

        Ok(bill_text)
    }

    pub async fn get_bill_titles(
        &self,
        congress: i64,
        bill_type: &str,
        bill_no: i64,
    ) -> Result<BillTitles> {
        let bill_titles: BillTitles = self
            .get(congress, bill_type, bill_no, Some(TITLES.to_string()))
            .await?;

        Ok(bill_titles)
    }

    async fn list<T>(
        &self,
        offset: i64,                    // starting from 0
        limit: i64,                     // max: 250
        from_date_time: Option<String>, // YYYY-MM-DDT00:00:00Z
        to_date_time: Option<String>,   // YYYY-MM-DDT23:59:59Z
        sort: Option<String>, // Sort by update date e.g., updateDate+asc or updateDate+desc.
        mut params: HashMap<&str, String>,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        params.insert("format", "json".to_string());
        params.insert("offset", offset.to_string());
        params.insert("limit", limit.to_string());
        if let Some(from_date_time) = from_date_time {
            params.insert("fromDateTime", from_date_time);
        }
        if let Some(to_date_time) = to_date_time {
            params.insert("toDateTime", to_date_time);
        }
        if let Some(sort) = sort {
            params.insert("sort", sort);
        }
        let client = reqwest::Client::new();
        let mut url = format!("{}/?", self.url);

        for (key, value) in params {
            url.push_str(&format!("{}={}&", key, value));
        }

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| ServiceError::FetchError(vec![(0, e.to_string())]))?;

        if response.status().is_success() {
            let result = response
                .json::<T>()
                .await
                .map_err(|e| ServiceError::JsonDeserializeError(e.to_string()))?;
            Ok(result)
        } else {
            Err(ServiceError::FetchError(vec![(
                0,
                "Request failed".to_string(),
            )]))
        }
    }

    async fn get<T>(
        &self,
        congress: i64,
        bill_type: &str,
        bill_no: i64,
        endpoint: Option<String>,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut params = HashMap::new();
        params.insert("api_key", self.key.clone());
        params.insert("format", "json".to_string());
        let client = reqwest::Client::new();

        let ep = endpoint.unwrap_or_else(|| "".to_string());

        let mut url = format!("{}/{congress}/{bill_type}/{bill_no}/{}?", self.url, ep);

        for (key, value) in params {
            url.push_str(&format!("{}={}&", key, value));
        }

        tracing::debug!("url: {}", url);

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| ServiceError::FetchError(vec![(0, e.to_string())]))?;

        if response.status().is_success() {
            let result = response
                .json::<T>()
                .await
                .map_err(|e| ServiceError::JsonDeserializeError(e.to_string()))?;
            Ok(result)
        } else {
            Err(ServiceError::FetchError(vec![(
                0,
                "Request failed".to_string(),
            )]))
        }
    }
}
