use crate::models::openapi::national_proposer::AssemblyProposer;
use bdk::prelude::*;
use dto::ServiceError;
use serde_json::Value;
use std::collections::HashMap;

const AGE: u32 = 22; // 22nd assembly

pub async fn fetch_proposers(index: u32, size: u32) -> Result<Vec<AssemblyProposer>, ServiceError> {
    let config = crate::config::get();
    let mut params = HashMap::new();
    params.insert("KEY", config.openapi_key.to_string());
    params.insert("type", "json".to_string());
    params.insert("pIndex", index.to_string());
    params.insert("pSize", size.to_string());
    params.insert("AGE", AGE.to_string());

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/nzmimeepazxkubdpn", config.openapi_url))
        .query(&params)
        .header(reqwest::header::USER_AGENT, "biyard") // Required
        .send()
        .await?
        .text()
        .await?;

    if let Ok(json) = serde_json::from_str::<Value>(&response) {
        let response = json["nzmimeepazxkubdpn"].clone();
        let rows = match response[1]["row"].as_array() {
            Some(rows) => rows,
            None => {
                return Err(ServiceError::OpenApiResponseError(
                    "Failed to parse response".to_string(),
                ));
            }
        };
        let rst: Vec<AssemblyProposer> =
            match serde_json::from_value(serde_json::Value::Array(rows.clone())) {
                Ok(rst) => rst,
                Err(e) => {
                    return Err(ServiceError::JsonDeserializeError(e.to_string()));
                }
            };
        return Ok(rst);
    } else {
        return Err(ServiceError::OpenApiResponseError(
            "Failed to parse response".to_string(),
        ));
    }
}

pub async fn fetch_proposer_by_bill_id(bill_no: String) -> Result<AssemblyProposer, ServiceError> {
    let config = crate::config::get();
    let mut params = HashMap::new();
    params.insert("KEY", config.openapi_key.to_string());
    params.insert("type", "json".to_string());
    params.insert("pIndex", "1".to_string());
    params.insert("pSize", "1".to_string());
    params.insert("AGE", AGE.to_string());
    params.insert("BILL_NO", bill_no.clone());

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/nzmimeepazxkubdpn", config.openapi_url))
        .query(&params)
        .header(reqwest::header::USER_AGENT, "biyard")
        .send()
        .await?
        .text()
        .await?;

    if let Ok(json) = serde_json::from_str::<Value>(&response) {
        let response = json["nzmimeepazxkubdpn"].clone();
        let rows = match response[1]["row"].as_array() {
            Some(rows) => rows,
            None => {
                return Err(ServiceError::OpenApiResponseError(
                    "Failed to parse response".to_string(),
                ));
            }
        };
        let rst: Vec<AssemblyProposer> =
            match serde_json::from_value(serde_json::Value::Array(rows.clone())) {
                Ok(rst) => rst,
                Err(e) => {
                    return Err(ServiceError::JsonDeserializeError(e.to_string()));
                }
            };
        for proposer in rst {
            if proposer.bill_no == bill_no {
                return Ok(proposer);
            }
        }
        return Err(ServiceError::OpenApiResponseError(
            "Failed to find proposer".to_string(),
        ));
    } else {
        return Err(ServiceError::OpenApiResponseError(
            "Failed to parse response".to_string(),
        ));
    }
}
