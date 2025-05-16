use std::collections::HashMap;

use bdk::prelude::reqwest;

use dto::{Error, Result};

use super::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct AssemblyClient {
    key: String,
}

impl AssemblyClient {
    pub fn new(key: String) -> Self {
        Self { key }
    }

    pub async fn get_proposers(&self, bill_no: i64) -> Result<BillInfo> {
        let age = bill_no / 100000;

        let proposers: Vec<BillInfo> = self
            .get(
                GET_PROPOSERS,
                1,
                100,
                HashMap::from([("BILL_NO", bill_no.to_string()), ("AGE", age.to_string())]),
            )
            .await?;

        Ok(proposers[0].clone())
    }

    pub async fn get_bill(&self, bill_no: i64) -> Result<BillDetail> {
        let bills: Vec<BillDetail> = self
            .get(
                GET_BILL,
                1,
                100,
                HashMap::from([("BILL_NO", bill_no.to_string())]),
            )
            .await?;

        Ok(bills[0].clone())
    }

    #[allow(dead_code)]
    pub async fn list_active_members(&self, page: i64, page_size: i64) -> Result<Vec<Member>> {
        let members = self
            .get(LIST_ACTIVE_MEMBERS, page, page_size, HashMap::new())
            .await?;

        Ok(members)
    }

    #[allow(dead_code)]
    pub async fn get_registered_members(
        &self,
        page: i64,
        page_size: i64,
        code: String,
    ) -> Result<Vec<AssemblyMember>> {
        let members = self
            .get(
                GET_REGISTERED_MEMBERS,
                page,
                page_size,
                HashMap::from([("NAAS_CD", code)]),
            )
            .await?;

        Ok(members)
    }

    async fn get<T>(
        &self,
        endpoint: &str,
        page: i64,
        page_size: i64,
        mut params: HashMap<&str, String>,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        params.insert("KEY", self.key.clone());
        params.insert("type", "json".to_string());
        params.insert("pIndex", page.to_string());
        params.insert("pSize", page_size.to_string());
        let url = format!("https://open.assembly.go.kr/portal/openapi/{endpoint}");
        tracing::debug!("GET {}", url);
        let cli = reqwest::Client::new();
        let json: serde_json::Value = cli
            .get(url)
            .query(&params)
            .header(reqwest::header::USER_AGENT, "biyard") // Required
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to send request: {}", e);
                Error::NaOpenApiRequestError
            })?
            .json()
            .await
            .map_err(|e| {
                tracing::error!("Failed to parse response: {}", e);
                Error::NaOpenApiResponseParsingError
            })?;

        tracing::debug!("Response: {:?}", json);

        let response = json[endpoint].as_array().ok_or(Error::ApiEmptyRow)?;

        tracing::debug!("{} Response: {:?}", endpoint, response);

        if response.is_empty() {
            return Err(Error::ApiEmptyRow);
        }

        let rows = match response[1].get("row") {
            Some(rows) => rows,
            None => {
                return Err(Error::ApiEmptyRow);
            }
        };
        tracing::debug!("{} row data: {:?}", endpoint, rows);

        Ok(serde_json::from_value(rows.clone())?)
    }
}
