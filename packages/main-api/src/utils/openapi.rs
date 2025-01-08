use reqwest::Error;
use std::collections::HashMap;
use serde_json::Value;

pub async fn get_active_members() -> Result<Value, Error> {
    let config = crate::config::get();
    let mut params = HashMap::new();
    params.insert("KEY", config.openapi_key.to_string());
    params.insert("type", "json".to_string());
    params.insert("pIndex", "1".to_string()); // 페이지번호 default: 1, start from 1 not 0
    params.insert("pSize", "5".to_string()); // 페이지당 요청 건수 default: 300

    let client = reqwest::Client::new();
    let response = client
        .get("https://open.assembly.go.kr/portal/openapi/nwvrqwxyaytdsfvhu")
        .query(&params)
        .header(reqwest::header::USER_AGENT, "biyard") // 필수
        .send()
        .await?
        .text()
        .await?;

    if let Ok(json) = serde_json::from_str::<Value>(&response) {
        let response = json["nwvrqwxyaytdsfvhu"].clone();
        return Ok(response[1].clone());
    }

    Ok(Value::Null)
}

pub async fn get_active_member_en(
    code: String, // assembly member code
) -> Result<Value, Error> {
    let config = crate::config::get();
    let mut params = HashMap::new();
    params.insert("KEY", config.openapi_key.to_string());
    params.insert("type", "json".to_string());
    params.insert("NAAS_CD", code);

    let client = reqwest::Client::new();
    let response = client
        .get("https://open.assembly.go.kr/portal/openapi/ENNAMEMBER")
        .query(&params)
        .header(reqwest::header::USER_AGENT, "biyard") // 필수
        .send()
        .await?
        .text()
        .await?;

    if let Ok(json) = serde_json::from_str::<Value>(&response) {
        let response = json["ENNAMEMBER"].clone();
        return Ok(response[1].clone());
    }

    Ok(Value::Null)
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
        .header(reqwest::header::USER_AGENT, "biyard") // 필수
        .send()
        .await?
        .text()
        .await?;

    if let Ok(json) = serde_json::from_str::<Value>(&response) {
        let response = json["ALLNAMEMBER"].clone();
        let ret = response[1]["row"][0]["NAAS_PIC"].as_str().unwrap_or("");
        return Ok(ret.to_string());
    }

    Ok("232".to_string())
}