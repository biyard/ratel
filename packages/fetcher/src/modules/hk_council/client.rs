use std::collections::HashMap;

use bdk::prelude::reqwest;

use dto::{Error, Result};

use super::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct HkCouncilClient {
    url: String,
}

impl HkCouncilClient {
    pub fn new() -> Self {
        Self {
            url: "https://app.legco.gov.hk/BillsDB/odata/Vbills".to_string(),
        }
    }

    pub async fn list_bills(&self, offset: i64, limit: i64) -> Result<Vec<HKBill>> {
        let bills: Vec<HKBill> = self.get(offset, limit, false, vec![], None, None).await?;

        Ok(bills)
    }

    pub async fn list_bills_by_date_range(
        &self,
        offset: i64,
        limit: i64,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<HKBill>> {
        let filter = format!(
            "bill_gazette_date ge datetime'{}T00:00:00' and bill_gazette_date lt datetime'{}T00:00:00'",
            start_date, end_date
        );

        let bills: Vec<HKBill> = self
            .get(offset, limit, true, vec![], None, Some(filter))
            .await?;

        Ok(bills)
    }

    pub async fn list_bills_by_year(
        &self,
        offset: i64,
        limit: i64,
        year: i64,
    ) -> Result<Vec<HKBill>> {
        let filter = format!("year(bill_gazette_date) eq {}", year);

        let bills: Vec<HKBill> = self
            .get(offset, limit, true, vec![], None, Some(filter))
            .await?;

        Ok(bills)
    }

    pub async fn get_bill(&self, internal_key: String) -> Result<HKBill> {
        let filter = format!("startswith(internal_key, '{}')", internal_key);
        let bills: Vec<HKBill> = self.get(0, 1, false, vec![], None, Some(filter)).await?;

        if bills.is_empty() {
            return Err(Error::ApiEmptyRow);
        }

        let bill = bills.into_iter().next().ok_or(Error::ApiEmptyRow)?;

        Ok(bill)
    }

    async fn get<T>(
        &self,
        offset: i64,              // starting from 0
        limit: i64,               // starting from 1
        total_count: bool,        // e.g., true -> $inlinecount=allpages
        select: Vec<String>,      // e.g., "bill_title_eng, bill_gazette_date"
        order_by: Option<String>, // need column name e.g., "bill_title_eng"
        filter: Option<String>,   // e.g., "$filter=year(bill_gazette_date) eq 2013"
    ) -> Result<T>
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
        params.insert("$top", limit.to_string());
        params.insert("$skip", offset.to_string());
        if total_count {
            params.insert("$inlinecount", "allpages".to_string());
        }
        if let Some(order_by) = order_by {
            params.insert("$orderby", order_by);
        }
        if !select.is_empty() {
            params.insert("$select", select.join(","));
        }
        if let Some(filter) = filter {
            params.insert("$filter", filter);
        }

        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true) // need to accept invalid certs
            .build()
            .map_err(|e| Error::ReqwestClientError(e.to_string()))?;

        let json: serde_json::Value = client
            .get(self.url.clone())
            .query(&params)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to send request: {}", e);
                Error::HkOpenDataApiRequestError
            })?
            .json()
            .await
            .map_err(|e| {
                tracing::error!("Failed to parse response: {}", e);
                Error::HkOpenDataApiResponseParsingError
            })?;

        let value = json.get("value").ok_or(Error::ApiEmptyRow)?;

        Ok(serde_json::from_value(value.clone())?)
    }
}
