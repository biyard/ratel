use std::collections::HashMap;

use bdk::prelude::reqwest;

use dto::{Error, Result};

use super::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct ChParliamentClient {
    url: String,
}

impl ChParliamentClient {
    pub fn new() -> Self {
        Self {
            url: "https://ws-old.parlament.ch/affairsummaries".to_string(),
        }
    }

    pub async fn list_bill_ids(&self, page_no: i64) -> Result<(Vec<i64>, bool)> {
        let bills: Vec<CHAffairSummary> = self.list(page_no).await?;

        let bill_ids = bills.iter().map(|bill| bill.id).collect();

        let is_last_page = if let Some(last_page) = bills.last() {
            last_page.has_more_pages.unwrap_or(false)
        } else {
            false
        };

        Ok((bill_ids, is_last_page))
    }

    pub async fn get_bill(&self, bill_id: i64) -> Result<CHAffair> {
        let bill_details: CHAffair = self.get(bill_id).await?;

        Ok(bill_details)
    }

    async fn list<T>(&self, page_no: i64) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut params = HashMap::new();
        params.insert("format", "json".to_string());
        params.insert("lang", "en".to_string());
        params.insert("pretty", "true".to_string());
        params.insert("pageNumber", page_no.to_string());

        tracing::debug!("ch_parliament list url: {} param: {:?}", self.url, params);

        let client = reqwest::Client::new();

        let result: T = client
            .get(self.url.clone())
            .query(&params)
            .header(reqwest::header::ACCEPT, "application/json") // Need to set the header to accept JSON
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to send request: {}", e);
                Error::ChOpenDataApiRequestError
            })?
            .json()
            .await
            .map_err(|e| {
                tracing::error!("Failed to parse response: {}", e);
                Error::ChOpenDataApiResponseParsingError
            })?;

        Ok(result)
    }

    async fn get<T>(&self, bill_id: i64) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut params = HashMap::new();
        params.insert("format", "json".to_string());
        params.insert("lang", "en".to_string());
        params.insert("pretty", "true".to_string());

        let url = format!(
            "{}/{}",
            self.url,            // base url
            bill_id.to_string()  // bill id
        );

        tracing::debug!(
            "ch_parliament get url: {} param: {:?} bill_id {:?}",
            url,
            params,
            bill_id
        );

        let client = reqwest::Client::new();

        let result: T = client
            .get(url)
            .query(&params)
            .header(reqwest::header::ACCEPT, "application/json") // Need to set the header to accept JSON
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to send request: {}", e);
                Error::ChOpenDataApiRequestError
            })?
            .json()
            .await
            .map_err(|e| {
                tracing::error!("Failed to parse response: {}", e);
                Error::ChOpenDataApiResponseParsingError
            })?;

        Ok(result)
    }
}
