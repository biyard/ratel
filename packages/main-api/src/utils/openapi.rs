use reqwest::Error;
use std::collections::HashMap;
use serde_json::Value;
use crate::models::openapi::member::{Member, EnMember};
use dto::ServiceError;

const DEFAULT_PAGE_INDEX: u32 = 1; // page num; start from 1 not 0
const DEFAULT_PAGE_SIZE: u32 = 300; // request per page

pub async fn get_active_members() -> Result<Vec<Member>, ServiceError> {
    let config = crate::config::get();
    let mut params = HashMap::new();
    params.insert("KEY", config.openapi_key.to_string());
    params.insert("type", "json".to_string());
    params.insert("pIndex", DEFAULT_PAGE_INDEX.to_string()); 
    params.insert("pSize", DEFAULT_PAGE_SIZE.to_string()); 

    let client = reqwest::Client::new();
    let response = client
        .get("https://open.assembly.go.kr/portal/openapi/nwvrqwxyaytdsfvhu")
        .query(&params)
        .header(reqwest::header::USER_AGENT, "biyard") // Required
        .send()
        .await?
        .text()
        .await?;

    if let Ok(json) = serde_json::from_str::<Value>(&response) {
        let response = json["nwvrqwxyaytdsfvhu"].clone();
        let rows = response[1]["row"].as_array().unwrap().clone();
        let rst: Vec<Member> = rows.into_iter().map(
            |row| match serde_json::from_value(row.clone()) {
                Ok(rst) => rst,
                Err(e) => {
                    return Err(ServiceError::JsonDeserializeError(e.to_string()));
                }
            }
        ).collect();
        return Ok(rst);
    } else {
        return Err(ServiceError::OpenApiResponseError("Failed to parse response".to_string()));    }
}

pub async fn get_active_member_en(
    code: String, // assembly member code
) -> Result<EnMember, ServiceError> {
    let config = crate::config::get();
    let mut params = HashMap::new();
    params.insert("KEY", config.openapi_key.to_string());
    params.insert("type", "json".to_string());
    params.insert("NAAS_CD", code);

    let client = reqwest::Client::new();
    let response = client
        .get("https://open.assembly.go.kr/portal/openapi/ENNAMEMBER")
        .query(&params)
        .header(reqwest::header::USER_AGENT, "biyard") // Required
        .send()
        .await?
        .text()
        .await?;

    if let Ok(json) = serde_json::from_str::<Value>(&response) {
        let response = json["ENNAMEMBER"].clone();
        let rows = response[1]["row"].as_array().unwrap().clone();
        let rst: EnMember = match serde_json::from_value(rows[0].clone()) {
            Ok(rst) => rst,
            Err(e) => {
                return Err(ServiceError::JsonDeserializeError(e.to_string()));
            }
        };
        return Ok(rst)
    } else {
        return Err(ServiceError::OpenApiResponseError("Failed to parse response".to_string()));
    }
}

pub async fn get_member_profile_image(
    code: String, // assembly member code
) -> Result<String, Error> {
    let config = crate::config::get();
    let mut params = HashMap::new();
    params.insert("KEY", config.openapi_key.to_string());
    params.insert("type", "json".to_string());
    params.insert("NAAS_CD", code);

    let client = reqwest::Client::new();
    let response = client
        .get("https://open.assembly.go.kr/portal/openapi/ALLNAMEMBER")
        .query(&params)
        .header(reqwest::header::USER_AGENT, "biyard") // Required
        .send()
        .await?
        .text()
        .await?;

    if let Ok(json) = serde_json::from_str::<Value>(&response) {
        let response = json["ALLNAMEMBER"].clone();
        let ret = response[1]["row"][0]["NAAS_PIC"].as_str().unwrap_or("");
        return Ok(ret.to_string());
    }

    Ok("".to_string())
}