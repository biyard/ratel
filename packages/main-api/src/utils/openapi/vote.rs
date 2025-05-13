use crate::models::openapi::national_vote::AssemblyVote;
use bdk::prelude::*;
use dto::Error;
use serde_json::Value;
use std::collections::HashMap;

const DEFAULT_PAGE_INDEX: u32 = 1; // page num; start from 1 not 0
const DEFAULT_PAGE_SIZE: u32 = 300; // request per page
const UNIT: u32 = 22; // 22nd assembly

pub async fn fetch_assembly_vote_result(bill_id: String) -> Result<Vec<AssemblyVote>, Error> {
    let config = crate::config::get();
    let mut params = HashMap::new();
    params.insert("KEY", config.openapi_key.to_string());
    params.insert("type", "json".to_string());
    params.insert("BILL_ID", bill_id);
    params.insert("AGE", UNIT.to_string());
    params.insert("pIndex", DEFAULT_PAGE_INDEX.to_string());
    params.insert("pSize", DEFAULT_PAGE_SIZE.to_string());

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/BILLJUDGE", config.openapi_url))
        .query(&params)
        .header(reqwest::header::USER_AGENT, "biyard") // Required
        .send()
        .await?
        .text()
        .await?;

    if let Ok(json) = serde_json::from_str::<Value>(&response) {
        let response = json["BILLJUDGE"].clone();
        let rows = match response[1]["row"].as_array() {
            Some(rows) => rows,
            None => {
                return Err(Error::OpenApiResponseError(
                    "Failed to parse response".to_string(),
                ));
            }
        };
        let rst: Vec<AssemblyVote> =
            match serde_json::from_value(serde_json::Value::Array(rows.clone())) {
                Ok(rst) => rst,
                Err(e) => {
                    return Err(Error::JsonDeserializeError(e.to_string()));
                }
            };
        return Ok(rst);
    } else {
        return Err(Error::OpenApiResponseError(
            "Failed to parse response".to_string(),
        ));
    }
}
