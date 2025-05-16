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

    pub async fn get_bill(&self, bill_id: i64) -> Result<CHAffair> {
        let bill_details: CHAffair = self.get(bill_id).await?;

        Ok(bill_details)
    }

    #[allow(dead_code)]
    async fn list<T>(&self, page_no: i64) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut params = HashMap::new();
        params.insert("format", "json".to_string());
        params.insert("lang", "en".to_string());
        params.insert("pretty", "true".to_string());
        params.insert("pageNumber", page_no.to_string());

        let client = reqwest::Client::new();

        let json: serde_json::Value = client
            .get(self.url.clone())
            .query(&params)
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

        let value = json.get("value").ok_or(Error::ChOpenDataApiEmptyRow)?;

        Ok(serde_json::from_value(value.clone())?)
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

        let client = reqwest::Client::new();

        let json: serde_json::Value = client
            .get(url)
            .query(&params)
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

        let value = json.get("value").ok_or(Error::ChOpenDataApiEmptyRow)?;

        Ok(serde_json::from_value(value.clone())?)
    }
}
