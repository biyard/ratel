use std::collections::HashMap;

use bdk::prelude::reqwest;

use dto::{Error, Result};

use super::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct EuParliamentClient {
    url: String,
}

impl EuParliamentClient {
    pub fn new() -> Self {
        Self {
            url: "https://data.europarl.europa.eu/api/v2/adopted-texts".to_string(),
        }
    }

    pub async fn list_bill_id(
        &self,
        year: Option<i64>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<String>> {
        let bills: Vec<EUAdoptedTextSummary> = self.list(year, offset, limit).await?;

        let bill_ids = bills.iter().map(|text| text.identifier.clone()).collect();

        Ok(bill_ids)
    }

    pub async fn get_bill(&self, doc_id: String) -> Result<EUAdoptedText> {
        let bill_details: EUAdoptedText = self.get(doc_id).await?;

        Ok(bill_details)
    }

    async fn list<T>(&self, year: Option<i64>, offset: i64, limit: i64) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        if offset < 0 {
            return Err(Error::InvalidInputValue);
        }

        if limit < 1 {
            return Err(Error::InvalidInputValue);
        }
        let mut params = HashMap::new();
        params.insert("format", "application/ld+json".to_string());
        params.insert("offset", offset.to_string());
        params.insert("limit", limit.to_string());

        if let Some(year) = year {
            params.insert("year", year.to_string());
        }

        tracing::debug!("EU_parliament list url: {} param: {:?}", self.url, params);

        let client = reqwest::Client::new();

        let json: serde_json::Value = client
            .get(self.url.clone())
            .query(&params)
            .header(reqwest::header::USER_AGENT, "biyard-dev-1.0.0") // Required
            .header(reqwest::header::ACCEPT, "application/json")
            .header(reqwest::header::CONTENT_TYPE, "application/ld+json") // Required
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to send request: {}", e);
                Error::EuOpenDataApiRequestError
            })?
            .json()
            .await
            .map_err(|e| {
                tracing::error!("Failed to parse response: {}", e);
                Error::EuOpenDataApiResponseParsingError
            })?;

        let value = json.get("data").ok_or(Error::ApiEmptyRow)?;

        Ok(serde_json::from_value(value.clone())?)
    }

    async fn get<T>(&self, doc_id: String) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut params = HashMap::new();
        params.insert("format", "application/ld+json".to_string());
        params.insert("language", "en".to_string());

        let url = format!(
            "{}/{}",
            self.url,           // base url
            doc_id.to_string()  // bill id
        );

        tracing::debug!("EU_parliament get url: {} param: {:?}", url, params,);

        let client = reqwest::Client::new();

        let json: serde_json::Value = client
            .get(url)
            .query(&params)
            .header(reqwest::header::USER_AGENT, "biyard-dev-1.0.0") // Required
            .header(reqwest::header::ACCEPT, "application/json")
            .header(reqwest::header::CONTENT_TYPE, "application/ld+json") // Required
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to send request: {}", e);
                Error::EuOpenDataApiRequestError
            })?
            .json()
            .await
            .map_err(|e| {
                tracing::error!("Failed to parse response: {}", e);
                Error::EuOpenDataApiResponseParsingError
            })?;

        let value = json.get("data").ok_or(Error::ApiEmptyRow)?;

        if value.is_array() {
            if let Some(first_item) = value.get(0) {
                return Ok(serde_json::from_value(first_item.clone())?);
            }
        }

        Err(Error::ApiEmptyRow)
    }
}
